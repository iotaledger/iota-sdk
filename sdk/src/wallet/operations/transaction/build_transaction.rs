// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;
use std::collections::HashSet;

use crypto::keys::bip44::Bip44;

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        api::{options::TransactionOptions, PreparedTransactionData},
        secret::{types::InputSigningData, SecretManage},
    },
    types::block::{
        address::{Address, Bech32Address},
        output::{Output, OutputId},
        protocol::CommittableAgeRange,
        slot::SlotIndex,
    },
    wallet::{operations::helpers::time::can_output_be_unlocked_from_now_on, types::OutputData, Wallet, WalletError},
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

        let slot_commitment_id = self.client().get_issuance().await?.latest_commitment.id();
        if options.issuer_id.is_none() {
            options.issuer_id = self.ledger().await.first_account_id();
        }

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::BuildingTransaction,
        ))
        .await;

        let wallet_ledger = self.ledger().await;

        #[allow(unused_mut)]
        let mut forbidden_inputs = wallet_ledger.locked_outputs.clone();

        // Prevent consuming the voting output if not actually wanted
        #[cfg(feature = "participation")]
        if let Some(voting_output) = &voting_output {
            if !options.required_inputs.contains(&voting_output.output_id) {
                forbidden_inputs.insert(voting_output.output_id);
            }
        }

        let wallet_address = self.address().await;
        let controlled_addresses = wallet_ledger.controlled_addresses(wallet_address.inner().clone());
        // Filter inputs to not include inputs that require additional outputs for storage deposit return or could be
        // still locked.
        let available_inputs = filter_inputs(
            &wallet_address,
            &controlled_addresses,
            self.bip_path().await,
            wallet_ledger
                .unspent_outputs
                .iter()
                .filter_map(|(id, data)| (!forbidden_inputs.contains(id)).then_some(data)),
            slot_commitment_id.slot_index(),
            protocol_parameters.committable_age_range(),
            &options.required_inputs,
        )?;

        // Check that no input got already locked
        for output_id in &options.required_inputs {
            if wallet_ledger.locked_outputs.contains(output_id) {
                return Err(WalletError::CustomInput(format!(
                    "provided custom input {output_id} is already used in another transaction",
                )));
            }
        }

        Ok(self
            .client()
            .build_transaction_inner(
                [self.address().await.into_inner()],
                available_inputs,
                outputs,
                options,
                slot_commitment_id,
                protocol_parameters,
            )
            .await?)
    }
}

/// Filter available outputs to only include outputs that can be unlocked forever from this moment.
/// Note: this is only for the default transaction builder, it's still possible to send these outputs by using
/// `claim_outputs` or providing their OutputId's in the custom_inputs
#[allow(clippy::too_many_arguments)]
fn filter_inputs<'a>(
    wallet_address: &Bech32Address,
    controlled_addresses: &HashSet<Address>,
    wallet_bip_path: Option<Bip44>,
    available_outputs: impl IntoIterator<Item = &'a OutputData>,
    slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
    required_inputs: &BTreeSet<OutputId>,
) -> Result<Vec<InputSigningData>, WalletError> {
    let mut available_outputs_signing_data = Vec::new();

    for output_data in available_outputs {
        if !required_inputs.contains(&output_data.output_id) {
            let output_can_be_unlocked_now_and_in_future = can_output_be_unlocked_from_now_on(
                controlled_addresses,
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
