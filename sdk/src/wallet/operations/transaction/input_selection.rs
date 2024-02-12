// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{hash_map::Values, HashMap, HashSet};

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        api::input_selection::{Burn, InputSelection, Selected},
        secret::{types::InputSigningData, SecretManage},
    },
    types::block::{
        address::Address,
        mana::ManaAllotment,
        output::{Output, OutputId},
        protocol::CommittableAgeRange,
        slot::SlotIndex,
    },
    wallet::{
        core::WalletData, operations::helpers::time::can_output_be_unlocked_forever_from_now_on, types::OutputData,
        Wallet,
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
        custom_inputs: Option<HashSet<OutputId>>,
        mandatory_inputs: Option<HashSet<OutputId>>,
        remainder_address: Option<Address>,
        burn: Option<&Burn>,
        mana_allotments: Option<Vec<ManaAllotment>>,
    ) -> crate::wallet::Result<Selected> {
        log::debug!("[TRANSACTION] select_inputs");
        // Voting output needs to be requested before to prevent a deadlock
        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await?;
        let protocol_parameters = self.client().get_protocol_parameters().await?;
        let slot_index = self.client().get_slot_index().await?;
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
            protocol_parameters.committable_age_range(),
            custom_inputs.as_ref(),
            mandatory_inputs.as_ref(),
        )?;

        let mut mana_rewards = HashMap::new();

        if let Some(burn) = burn {
            for delegation_id in burn.delegations() {
                if let Some(output) = wallet_data.unspent_delegation_output(delegation_id) {
                    mana_rewards.insert(
                        output.output_id,
                        self.client()
                            .get_output_mana_rewards(&output.output_id, slot_index)
                            .await?
                            .rewards,
                    );
                }
            }
        }

        // if custom inputs are provided we should only use them (validate if we have the outputs in this account and
        // that the amount is enough)
        if let Some(custom_inputs) = custom_inputs {
            // Check that no input got already locked
            for output_id in &custom_inputs {
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
                                .get_output_mana_rewards(output_id, slot_index)
                                .await?
                                .rewards,
                        );
                    }
                }
            }

            let mut input_selection = InputSelection::new(
                available_outputs_signing_data,
                outputs,
                Some(wallet_data.address.clone().into_inner()),
                slot_index,
                protocol_parameters.clone(),
            )
            .with_required_inputs(custom_inputs)
            .with_forbidden_inputs(forbidden_inputs)
            .with_mana_rewards(mana_rewards);

            if let Some(address) = remainder_address {
                input_selection = input_selection.with_remainder_address(address);
            }

            if let Some(burn) = burn {
                input_selection = input_selection.with_burn(burn.clone());
            }

            if let Some(mana_allotments) = mana_allotments {
                input_selection = input_selection.with_mana_allotments(mana_allotments.iter());
            }

            let selected_transaction_data = input_selection.select()?;

            // lock outputs so they don't get used by another transaction
            for output in &selected_transaction_data.inputs {
                wallet_data.locked_outputs.insert(*output.output_id());
            }

            return Ok(selected_transaction_data);
        } else if let Some(mandatory_inputs) = mandatory_inputs {
            // Check that no input got already locked
            for output_id in &mandatory_inputs {
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
                                .get_output_mana_rewards(output_id, slot_index)
                                .await?
                                .rewards,
                        );
                    }
                }
            }

            let mut input_selection = InputSelection::new(
                available_outputs_signing_data,
                outputs,
                Some(wallet_data.address.clone().into_inner()),
                slot_index,
                protocol_parameters.clone(),
            )
            .with_required_inputs(mandatory_inputs)
            .with_forbidden_inputs(forbidden_inputs)
            .with_mana_rewards(mana_rewards);

            if let Some(address) = remainder_address {
                input_selection = input_selection.with_remainder_address(address);
            }

            if let Some(burn) = burn {
                input_selection = input_selection.with_burn(burn.clone());
            }

            if let Some(mana_allotments) = mana_allotments {
                input_selection = input_selection.with_mana_allotments(mana_allotments.iter());
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
            Some(wallet_data.address.clone().into_inner()),
            slot_index,
            protocol_parameters.clone(),
        )
        .with_forbidden_inputs(forbidden_inputs)
        .with_mana_rewards(mana_rewards);

        if let Some(address) = remainder_address {
            input_selection = input_selection.with_remainder_address(address);
        }

        if let Some(burn) = burn {
            input_selection = input_selection.with_burn(burn.clone());
        }

        if let Some(mana_allotments) = mana_allotments {
            input_selection = input_selection.with_mana_allotments(mana_allotments.iter());
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
fn filter_inputs(
    wallet_data: &WalletData,
    available_outputs: Values<'_, OutputId, OutputData>,
    slot_index: impl Into<SlotIndex> + Copy,
    committable_age_range: CommittableAgeRange,
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
