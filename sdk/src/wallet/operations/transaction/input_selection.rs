// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;
use std::collections::HashMap;

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        api::input_selection::{InputSelection, Selected},
        secret::{types::InputSigningData, SecretManage},
    },
    types::block::{
        output::{Output, OutputId},
        protocol::CommittableAgeRange,
        slot::SlotIndex,
    },
    wallet::{
        core::WalletData, operations::helpers::time::can_output_be_unlocked_forever_from_now_on, types::OutputData,
        RemainderValueStrategy, TransactionOptions, Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Selects inputs for a transaction and locks them in the wallet, so they don't get used again
    pub(crate) async fn select_inputs(
        &self,
        outputs: Vec<Output>,
        options: TransactionOptions,
    ) -> crate::wallet::Result<Selected> {
        log::debug!("[TRANSACTION] select_inputs");
        // Voting output needs to be requested before to prevent a deadlock
        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await?;
        let protocol_parameters = self.client().get_protocol_parameters().await?;
        let slot_commitment_id = self.client().get_issuance().await?.latest_commitment.id();
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
        // lock so the same inputs can't be selected in multiple transactions
        let mut wallet_data = self.data_mut().await;

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::SelectingInputs,
        ))
        .await;

        #[allow(unused_mut)]
        let mut forbidden_inputs = wallet_data.locked_outputs.clone();

        // Prevent consuming the voting output if not actually wanted
        #[cfg(feature = "participation")]
        if let Some(voting_output) = &voting_output {
            if !options.mandatory_inputs.contains(&voting_output.output_id) {
                forbidden_inputs.insert(voting_output.output_id);
            }
        }

        // Filter inputs to not include inputs that require additional outputs for storage deposit return or could be
        // still locked.
        let available_outputs_signing_data = filter_inputs(
            &wallet_data,
            wallet_data.unspent_outputs.values(),
            slot_commitment_id.slot_index(),
            protocol_parameters.committable_age_range(),
            &options.mandatory_inputs,
        )?;

        let mut mana_rewards = HashMap::new();

        if let Some(burn) = &options.burn {
            for delegation_id in burn.delegations() {
                if let Some(output) = wallet_data.unspent_delegation_output(delegation_id) {
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
        for output_id in &options.mandatory_inputs {
            if wallet_data.locked_outputs.contains(output_id) {
                return Err(crate::wallet::Error::CustomInput(format!(
                    "provided custom input {output_id} is already used in another transaction",
                )));
            }
            if let Some(input) = wallet_data.outputs.get(output_id) {
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

        let mut input_selection = InputSelection::new(
            available_outputs_signing_data,
            options.context_inputs,
            outputs,
            Some(wallet_data.address.clone().into_inner()),
            slot_commitment_id,
            protocol_parameters.clone(),
        )
        .with_required_inputs(options.mandatory_inputs)
        .with_forbidden_inputs(forbidden_inputs)
        .with_mana_rewards(mana_rewards)
        .with_payload(options.tagged_data_payload)
        .with_mana_allotments(options.mana_allotments)
        .with_remainder_address(remainder_address)
        .with_burn(options.burn);

        if let (Some(account_id), Some(reference_mana_cost)) = (options.issuer_id, reference_mana_cost) {
            input_selection = input_selection.with_auto_mana_allotment(account_id, reference_mana_cost);
        }

        if !options.allow_additional_input_selection {
            input_selection = input_selection.disable_additional_input_selection();
        }

        let selected_transaction_data = input_selection.select()?;

        // lock outputs so they don't get used by another transaction
        for output in &selected_transaction_data.inputs {
            log::debug!("[TRANSACTION] locking: {}", output.output_id());
            wallet_data.locked_outputs.insert(*output.output_id());
        }

        Ok(selected_transaction_data)
    }
}

/// Filter available outputs to only include outputs that can be unlocked forever from this moment.
/// Note: this is only for the default input selection, it's still possible to send these outputs by using
/// `claim_outputs` or providing their OutputId's in the custom_inputs
#[allow(clippy::too_many_arguments)]
fn filter_inputs<'a>(
    wallet_data: &WalletData,
    available_outputs: impl IntoIterator<Item = &'a OutputData>,
    slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
    mandatory_inputs: &BTreeSet<OutputId>,
) -> crate::wallet::Result<Vec<InputSigningData>> {
    let mut available_outputs_signing_data = Vec::new();

    for output_data in available_outputs {
        if !mandatory_inputs.contains(&output_data.output_id) {
            let output_can_be_unlocked_now_and_in_future = can_output_be_unlocked_forever_from_now_on(
                // We use the addresses with unspent outputs, because other addresses of the
                // account without unspent outputs can't be related to this output
                &wallet_data.address.inner,
                &output_data.output,
                slot_index,
                committable_age_range,
            );

            // Outputs that could get unlocked in the future will not be included
            if !output_can_be_unlocked_now_and_in_future {
                continue;
            }
        }

        if let Some(available_input) = output_data.input_signing_data(wallet_data, slot_index, committable_age_range)? {
            available_outputs_signing_data.push(available_input);
        }
    }

    Ok(available_outputs_signing_data)
}
