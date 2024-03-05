// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;
use std::collections::HashMap;

use crypto::keys::bip44::Bip44;

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        api::{
            options::{RemainderValueStrategy, TransactionOptions},
            transaction_builder::TransactionBuilder,
            PreparedTransactionData,
        },
        secret::{types::InputSigningData, SecretManage},
    },
    types::block::{
        address::Bech32Address,
        output::{Output, OutputId},
        protocol::CommittableAgeRange,
        slot::SlotIndex,
    },
    wallet::{
        operations::helpers::time::can_output_be_unlocked_forever_from_now_on, types::OutputData, Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Builds a transaction using the given outputs and options.
    pub(crate) async fn build_transaction(
        &self,
        outputs: Vec<Output>,
        mut options: TransactionOptions,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] build_transaction");
        // Voting output needs to be requested before to prevent a deadlock
        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await;
        let protocol_parameters = self.client().get_protocol_parameters().await?;
        let creation_slot = self.client().get_slot_index().await?;

        let slot_commitment_id = self.client().get_issuance().await?.latest_commitment.id();
        if options.issuer_id.is_none() {
            options.issuer_id = self.ledger().await.first_account_id();
        }
        let reference_mana_cost = if let Some(issuer_id) = options.issuer_id {
            Some(
                self.client()
                    .get_account_congestion(&issuer_id, None)
                    .await?
                    .reference_mana_cost,
            )
        } else {
            None
        };
        let remainder_address = match options.remainder_value_strategy {
            RemainderValueStrategy::ReuseAddress => None,
            RemainderValueStrategy::CustomAddress(address) => Some(address),
        };

        let wallet_ledger = self.ledger().await;

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::BuildingTransaction,
        ))
        .await;

        #[allow(unused_mut)]
        let mut forbidden_inputs = wallet_ledger.locked_outputs.clone();

        // Prevent consuming the voting output if not actually wanted
        #[cfg(feature = "participation")]
        if let Some(voting_output) = &voting_output {
            if !options.required_inputs.contains(&voting_output.output_id) {
                forbidden_inputs.insert(voting_output.output_id);
            }
        }

        // Filter inputs to not include inputs that require additional outputs for storage deposit return or could be
        // still locked.
        let available_outputs_signing_data = filter_inputs(
            &self.address().await,
            self.bip_path().await,
            wallet_ledger
                .unspent_outputs
                .iter()
                .filter_map(|(id, data)| (!forbidden_inputs.contains(id)).then_some(data)),
            slot_commitment_id.slot_index(),
            protocol_parameters.committable_age_range(),
            &options.required_inputs,
        )?;

        let mut mana_rewards = HashMap::new();

        if let Some(burn) = &options.burn {
            for delegation_id in burn.delegations() {
                if let Some(output) = wallet_ledger.unspent_delegation_output(delegation_id) {
                    mana_rewards.insert(
                        output.output_id,
                        self.client()
                            .get_output_mana_rewards(&output.output_id, slot_commitment_id.slot_index())
                            .await?
                            .rewards,
                    );
                }
            }
        }

        // Check that no input got already locked
        for output_id in &options.required_inputs {
            if wallet_ledger.locked_outputs.contains(output_id) {
                return Err(WalletError::CustomInput(format!(
                    "provided custom input {output_id} is already used in another transaction",
                )));
            }
            if let Some(input) = wallet_ledger.outputs.get(output_id) {
                if input.output.can_claim_rewards(outputs.iter().find(|o| {
                    input
                        .output
                        .chain_id()
                        .map(|chain_id| chain_id.or_from_output_id(output_id))
                        == o.chain_id()
                })) {
                    mana_rewards.insert(
                        *output_id,
                        self.client()
                            .get_output_mana_rewards(output_id, slot_commitment_id.slot_index())
                            .await?
                            .rewards,
                    );
                }
            }
        }

        let mut transaction_builder = TransactionBuilder::new(
            available_outputs_signing_data,
            outputs,
            Some(self.address().await.into_inner()),
            creation_slot,
            slot_commitment_id,
            protocol_parameters.clone(),
        )
        .with_required_inputs(options.required_inputs)
        .with_mana_rewards(mana_rewards)
        .with_payload(options.tagged_data_payload)
        .with_mana_allotments(options.mana_allotments)
        .with_remainder_address(remainder_address)
        .with_burn(options.burn);

        if let (Some(account_id), Some(reference_mana_cost)) = (options.issuer_id, reference_mana_cost) {
            transaction_builder = transaction_builder.with_min_mana_allotment(account_id, reference_mana_cost);
        }

        if !options.allow_additional_input_selection {
            transaction_builder = transaction_builder.disable_additional_input_selection();
        }

        let prepared_transaction_data = transaction_builder.finish()?;

        prepared_transaction_data.transaction.validate_length()?;

        Ok(prepared_transaction_data)
    }
}

/// Filter available outputs to only include outputs that can be unlocked forever from this moment.
/// Note: this is only for the default transaction builder, it's still possible to send these outputs by using
/// `claim_outputs` or providing their OutputId's in the custom_inputs
#[allow(clippy::too_many_arguments)]
fn filter_inputs<'a>(
    wallet_address: &Bech32Address,
    wallet_bip_path: Option<Bip44>,
    available_outputs: impl IntoIterator<Item = &'a OutputData>,
    slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
    required_inputs: &BTreeSet<OutputId>,
) -> Result<Vec<InputSigningData>, WalletError> {
    let mut available_outputs_signing_data = Vec::new();

    for output_data in available_outputs {
        if !required_inputs.contains(&output_data.output_id) {
            let output_can_be_unlocked_now_and_in_future = can_output_be_unlocked_forever_from_now_on(
                // We use the addresses with unspent outputs, because other addresses of the
                // account without unspent outputs can't be related to this output
                wallet_address.inner(),
                &output_data.output,
                slot_index,
                committable_age_range,
            );

            // Outputs that could get unlocked in the future will not be included
            if !output_can_be_unlocked_now_and_in_future {
                continue;
            }
        }

        if let Some(available_input) =
            output_data.input_signing_data(wallet_address, wallet_bip_path, slot_index, committable_age_range)?
        {
            available_outputs_signing_data.push(available_input);
        }
    }

    Ok(available_outputs_signing_data)
}
