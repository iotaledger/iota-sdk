// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{hash_map::Values, HashSet};

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        api::input_selection::{is_account_transition, Burn, InputSelection, Selected},
        secret::{types::InputSigningData, SecretManage},
    },
    types::block::{
        address::Address,
        output::{Output, OutputId},
        slot::SlotIndex,
    },
    wallet::{
        account::{operations::helpers::time::can_output_be_unlocked_forever_from_now_on, OutputData},
        core::WalletData,
        Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Selects inputs for a transaction and locks them in the account, so they don't get used again
    pub(crate) async fn select_inputs(
        &self,
        outputs: Vec<Output>,
        custom_inputs: Option<HashSet<OutputId>>,
        mandatory_inputs: Option<HashSet<OutputId>>,
        remainder_address: Option<Address>,
        burn: Option<&Burn>,
    ) -> crate::wallet::Result<Selected> {
        log::debug!("[TRANSACTION] select_inputs");
        // Voting output needs to be requested before to prevent a deadlock
        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await?;
        // lock so the same inputs can't be selected in multiple transactions
        let mut wallet_data = self.data_mut().await;
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        #[cfg(feature = "events")]
        self.emit(
            todo!("wallet_data.index"),
            WalletEvent::TransactionProgress(TransactionProgressEvent::SelectingInputs),
        )
        .await;

        let slot_index = self.client().get_slot_index().await?;
        #[allow(unused_mut)]
        let mut forbidden_inputs = wallet_data.locked_outputs.clone();

        todo!("no need for a vec anymore");
        let addresses = vec![wallet_data.address.clone()];

        // TODO: remove
        // let addresses = wallet_data
        //     .public_addresses
        //     .iter()
        //     .chain(wallet_data.internal_addresses.iter())
        //     .map(|address| *address.address.as_ref())
        //     .collect::<Vec<_>>();

        // Prevent consuming the voting output if not actually wanted
        #[cfg(feature = "participation")]
        if let Some(voting_output) = &voting_output {
            let required = mandatory_inputs.as_ref().map_or(false, |mandatory_inputs| {
                mandatory_inputs.contains(&voting_output.output_id)
            });
            if !required {
                forbidden_inputs.insert(voting_output.output_id);
            }
        }

        // Filter inputs to not include inputs that require additional outputs for storage deposit return or could be
        // still locked.
        let available_outputs_signing_data = filter_inputs(
            &wallet_data,
            wallet_data.unspent_outputs.values(),
            slot_index,
            &outputs,
            burn,
            custom_inputs.as_ref(),
            mandatory_inputs.as_ref(),
        )?;

        // if custom inputs are provided we should only use them (validate if we have the outputs in this account and
        // that the amount is enough)
        if let Some(custom_inputs) = custom_inputs {
            // Check that no input got already locked
            for input in custom_inputs.iter() {
                if wallet_data.locked_outputs.contains(input) {
                    return Err(crate::wallet::Error::CustomInput(format!(
                        "provided custom input {input} is already used in another transaction",
                    )));
                }
            }

            let mut input_selection = InputSelection::new(
                available_outputs_signing_data,
                outputs,
                addresses,
                protocol_parameters.clone(),
            )
            .with_required_inputs(custom_inputs)
            .with_forbidden_inputs(forbidden_inputs);

            if let Some(address) = remainder_address {
                input_selection = input_selection.with_remainder_address(address);
            }

            if let Some(burn) = burn {
                input_selection = input_selection.with_burn(burn.clone());
            }

            let selected_transaction_data = input_selection.select()?;

            // lock outputs so they don't get used by another transaction
            for output in &selected_transaction_data.inputs {
                wallet_data.locked_outputs.insert(*output.output_id());
            }

            return Ok(selected_transaction_data);
        } else if let Some(mandatory_inputs) = mandatory_inputs {
            // Check that no input got already locked
            for input in mandatory_inputs.iter() {
                if wallet_data.locked_outputs.contains(input) {
                    return Err(crate::wallet::Error::CustomInput(format!(
                        "provided custom input {input} is already used in another transaction",
                    )));
                }
            }

            let mut input_selection = InputSelection::new(
                available_outputs_signing_data,
                outputs,
                addresses,
                protocol_parameters.clone(),
            )
            .with_required_inputs(mandatory_inputs)
            .with_forbidden_inputs(forbidden_inputs);

            if let Some(address) = remainder_address {
                input_selection = input_selection.with_remainder_address(address);
            }

            if let Some(burn) = burn {
                input_selection = input_selection.with_burn(burn.clone());
            }

            let selected_transaction_data = input_selection.select()?;

            // lock outputs so they don't get used by another transaction
            for output in &selected_transaction_data.inputs {
                wallet_data.locked_outputs.insert(*output.output_id());
            }

            // lock outputs so they don't get used by another transaction
            for output in &selected_transaction_data.inputs {
                wallet_data.locked_outputs.insert(*output.output_id());
            }

            return Ok(selected_transaction_data);
        }

        let mut input_selection = InputSelection::new(
            available_outputs_signing_data,
            outputs,
            addresses,
            protocol_parameters.clone(),
        )
        .with_forbidden_inputs(forbidden_inputs);

        if let Some(address) = remainder_address {
            input_selection = input_selection.with_remainder_address(address);
        }

        if let Some(burn) = burn {
            input_selection = input_selection.with_burn(burn.clone());
        }

        let selected_transaction_data = match input_selection.select() {
            Ok(r) => r,
            // TODO this error doesn't exist with the new ISA
            // Err(crate::client::Error::ConsolidationRequired(output_count)) => {
            //     #[cfg(feature = "events")]
            //     self.event_emitter
            //         .lock()
            //         .await
            //         .emit(account.index, WalletEvent::ConsolidationRequired);
            //     return Err(crate::wallet::Error::ConsolidationRequired {
            //         output_count,
            //         output_count_max: INPUT_COUNT_MAX,
            //     });
            // }
            Err(e) => return Err(e.into()),
        };

        // lock outputs so they don't get used by another transaction
        for output in &selected_transaction_data.inputs {
            log::debug!("[TRANSACTION] locking: {}", output.output_id());
            wallet_data.locked_outputs.insert(*output.output_id());
        }

        Ok(selected_transaction_data)
    }
}

/// Filter available outputs to only include outputs that don't have unlock conditions, that could create
/// conflicting transactions or need a new output for the storage deposit return
/// Also only include Account, Nft and Foundry outputs, if a corresponding output with the same id exists in the output,
/// so they don't get burned
///
/// Note: this is only for the default input selection, it's still possible to send these outputs by using
/// `claim_outputs` or providing their OutputId's in the custom_inputs
///
/// Some examples for which outputs should be included in the inputs to select from:
/// | Unlock conditions                                   | Include in inputs |
/// | --------------------------------------------------- | ----------------- |
/// | [Address]                                           | yes               |
/// | [Address, expired Timelock]                         | yes               |
/// | [Address, not expired Timelock, ...]                | no                |
/// | [Address, expired Expiration, ...]                  | yes               |
/// | [Address, not expired Expiration, ...]              | no                |
/// | [Address, StorageDepositReturn, ...]                | no                |
/// | [Address, StorageDepositReturn, expired Expiration] | yes               |
#[allow(clippy::too_many_arguments)]
fn filter_inputs(
    account: &WalletData,
    available_outputs: Values<'_, OutputId, OutputData>,
    slot_index: SlotIndex,
    outputs: &[Output],
    burn: Option<&Burn>,
    custom_inputs: Option<&HashSet<OutputId>>,
    mandatory_inputs: Option<&HashSet<OutputId>>,
) -> crate::wallet::Result<Vec<InputSigningData>> {
    let mut available_outputs_signing_data = Vec::new();

    for output_data in available_outputs {
        if !custom_inputs
            .map(|inputs| inputs.contains(&output_data.output_id))
            .unwrap_or(false)
            && !mandatory_inputs
                .map(|inputs| inputs.contains(&output_data.output_id))
                .unwrap_or(false)
        {
            let output_can_be_unlocked_now_and_in_future = can_output_be_unlocked_forever_from_now_on(
                // We use the addresses with unspent outputs, because other addresses of the
                // account without unspent outputs can't be related to this output
                todo!("&account.addresses_with_unspent_outputs"),
                &output_data.output,
                slot_index,
            );

            // Outputs that could get unlocked in the future will not be included
            if !output_can_be_unlocked_now_and_in_future {
                continue;
            }
        }

        // Defaults to state transition if it is not explicitly a governance transition or a burn.
        let account_state_transition = is_account_transition(&output_data.output, output_data.output_id, outputs, burn);

        if let Some(available_input) = output_data.input_signing_data(account, slot_index, account_state_transition)? {
            available_outputs_signing_data.push(available_input);
        }
    }

    Ok(available_outputs_signing_data)
}
