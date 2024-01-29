// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use super::{Error, InputSelection, Requirement};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        address::Address,
        input::INPUT_COUNT_MAX,
        output::{
            unlock_condition::StorageDepositReturnUnlockCondition, AccountOutputBuilder, FoundryOutputBuilder,
            MinimumOutputAmount, NftOutputBuilder, Output, OutputId,
        },
        slot::SlotIndex,
    },
};

/// Get the `StorageDepositReturnUnlockCondition`, if not expired.
pub(crate) fn sdruc_not_expired(
    output: &Output,
    slot_index: SlotIndex,
) -> Option<&StorageDepositReturnUnlockCondition> {
    // PANIC: safe to unwrap as outputs without unlock conditions have been filtered out already.
    let unlock_conditions = output.unlock_conditions().unwrap();

    unlock_conditions.storage_deposit_return().and_then(|sdr| {
        let expired = unlock_conditions
            .expiration()
            .map_or(false, |expiration| slot_index >= expiration.slot_index());

        // We only have to send the storage deposit return back if the output is not expired
        if !expired {
            Some(sdr)
        } else {
            None
        }
    })
}

pub(crate) fn amount_sums(
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
    slot_index: SlotIndex,
) -> (u64, u64, HashMap<Address, u64>, HashMap<Address, u64>) {
    let mut inputs_sum = 0;
    let mut outputs_sum = 0;
    let mut inputs_sdr = HashMap::new();
    let mut outputs_sdr = HashMap::new();

    for selected_input in selected_inputs {
        inputs_sum += selected_input.output.amount();

        if let Some(sdruc) = sdruc_not_expired(&selected_input.output, slot_index) {
            *inputs_sdr.entry(sdruc.return_address().clone()).or_default() += sdruc.amount();
        }
    }

    for output in outputs {
        outputs_sum += output.amount();

        if let Output::Basic(output) = output {
            if let Some(address) = output.simple_deposit_address() {
                *outputs_sdr.entry(address.clone()).or_default() += output.amount();
            }
        }
    }

    // TODO explanation about that
    for (sdr_address, input_sdr_amount) in &inputs_sdr {
        let output_sdr_amount = outputs_sdr.get(sdr_address).unwrap_or(&0);

        if input_sdr_amount > output_sdr_amount {
            outputs_sum += input_sdr_amount - output_sdr_amount;
        }
    }

    (inputs_sum, outputs_sum, inputs_sdr, outputs_sdr)
}

#[derive(Debug, Clone)]
struct AmountSelection {
    newly_selected_inputs: HashMap<OutputId, InputSigningData>,
    inputs_sum: u64,
    outputs_sum: u64,
    inputs_sdr: HashMap<Address, u64>,
    outputs_sdr: HashMap<Address, u64>,
    remainder_amount: u64,
    native_tokens_remainder: bool,
    slot_index: SlotIndex,
}

impl AmountSelection {
    fn new(input_selection: &InputSelection) -> Result<Self, Error> {
        let (inputs_sum, outputs_sum, inputs_sdr, outputs_sdr) = amount_sums(
            &input_selection.selected_inputs,
            &input_selection.outputs,
            input_selection.slot_index,
        );
        let (remainder_amount, native_tokens_remainder) = input_selection.remainder_amount()?;

        Ok(Self {
            newly_selected_inputs: HashMap::new(),
            inputs_sum,
            outputs_sum,
            inputs_sdr,
            outputs_sdr,
            remainder_amount,
            native_tokens_remainder,
            slot_index: input_selection.slot_index,
        })
    }

    fn missing_amount(&self) -> u64 {
        // If there is already a remainder, make sure it's enough to cover the storage deposit.
        if self.inputs_sum > self.outputs_sum {
            let diff = self.inputs_sum - self.outputs_sum;

            if self.remainder_amount > diff {
                self.remainder_amount - diff
            } else {
                0
            }
        } else if self.inputs_sum < self.outputs_sum {
            self.outputs_sum - self.inputs_sum
        } else if self.native_tokens_remainder {
            self.remainder_amount
        } else {
            0
        }
    }

    fn fulfil<'a>(&mut self, inputs: impl Iterator<Item = &'a InputSigningData>) -> bool {
        for input in inputs {
            if self.newly_selected_inputs.contains_key(input.output_id()) {
                continue;
            }

            if let Some(sdruc) = sdruc_not_expired(&input.output, self.slot_index) {
                // Skip if no additional amount is made available
                if input.output.amount() == sdruc.amount() {
                    continue;
                }
                let input_sdr = self.inputs_sdr.get(sdruc.return_address()).unwrap_or(&0) + sdruc.amount();
                let output_sdr = *self.outputs_sdr.get(sdruc.return_address()).unwrap_or(&0);

                if input_sdr > output_sdr {
                    let diff = input_sdr - output_sdr;
                    self.outputs_sum += diff;
                    *self.outputs_sdr.entry(sdruc.return_address().clone()).or_default() += sdruc.amount();
                }

                *self.inputs_sdr.entry(sdruc.return_address().clone()).or_default() += sdruc.amount();
            }

            self.inputs_sum += input.output.amount();
            self.newly_selected_inputs.insert(*input.output_id(), input.clone());

            if self.missing_amount() == 0 {
                return true;
            }
        }

        false
    }

    fn into_newly_selected_inputs(self) -> Vec<InputSigningData> {
        self.newly_selected_inputs.into_values().collect()
    }
}

impl InputSelection {
    fn fulfil<'a>(
        &self,
        base_inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
        amount_selection: &mut AmountSelection,
    ) -> bool {
        // No native token, expired SDRUC.
        let inputs = base_inputs.clone().filter(|input| {
            input.output.native_token().is_none() && sdruc_not_expired(&input.output, self.slot_index).is_none()
        });

        if amount_selection.fulfil(inputs) {
            return true;
        }

        // No native token, unexpired SDRUC.
        let inputs = base_inputs.clone().filter(|input| {
            input.output.native_token().is_none() && sdruc_not_expired(&input.output, self.slot_index).is_some()
        });

        if amount_selection.fulfil(inputs) {
            return true;
        }

        // Native token, expired SDRUC.
        let inputs = base_inputs.clone().filter(|input| {
            input.output.native_token().is_some() && sdruc_not_expired(&input.output, self.slot_index).is_none()
        });

        if amount_selection.fulfil(inputs) {
            return true;
        }

        // Native token, unexpired SDRUC.
        let inputs = base_inputs.clone().filter(|input| {
            input.output.native_token().is_some() && sdruc_not_expired(&input.output, self.slot_index).is_some()
        });

        if amount_selection.fulfil(inputs) {
            return true;
        }

        // Everything else.
        if amount_selection.fulfil(base_inputs) {
            return true;
        }

        false
    }

    fn reduce_funds_of_chains(&mut self, amount_selection: &mut AmountSelection) -> Result<(), Error> {
        // Only consider automatically transitioned outputs.
        let outputs = self.outputs.iter_mut().filter(|output| {
            output
                .chain_id()
                .as_ref()
                .map(|chain_id| self.automatically_transitioned.contains(chain_id))
                .unwrap_or(false)
        });

        for output in outputs {
            let diff = amount_selection.missing_amount();
            let amount = output.amount();
            let minimum_amount = output.minimum_amount(self.protocol_parameters.storage_score_parameters());

            let new_amount = if amount >= diff + minimum_amount {
                amount - diff
            } else {
                minimum_amount
            };

            // TODO check that new_amount is enough for the storage cost

            let new_output = match output {
                Output::Account(output) => {
                    // Mana generated or stored by an issuer account is locked to that account.
                    if output.is_block_issuer() {
                        continue;
                    }

                    AccountOutputBuilder::from(&*output)
                        .with_amount(new_amount)
                        .finish_output()?
                }
                Output::Foundry(output) => FoundryOutputBuilder::from(&*output)
                    .with_amount(new_amount)
                    .finish_output()?,
                Output::Nft(output) => NftOutputBuilder::from(&*output)
                    .with_amount(new_amount)
                    .finish_output()?,
                _ => panic!("only account, nft and foundry can be automatically created"),
            };

            // PANIC: unwrap is fine as non-chain outputs have been filtered out already.
            log::debug!(
                "Reducing amount of {} to {} to fulfill amount requirement",
                output.chain_id().unwrap(),
                new_amount
            );

            amount_selection.outputs_sum -= amount - new_amount;
            *output = new_output;

            if amount_selection.missing_amount() == 0 {
                return Ok(());
            }
        }

        Err(Error::InsufficientAmount {
            found: amount_selection.inputs_sum,
            required: amount_selection.inputs_sum + amount_selection.missing_amount(),
        })
    }

    pub(crate) fn fulfill_amount_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let mut amount_selection = AmountSelection::new(self)?;

        if amount_selection.missing_amount() == 0 {
            log::debug!("Amount requirement already fulfilled");
            return Ok(amount_selection.into_newly_selected_inputs());
        } else {
            log::debug!(
                "Fulfilling amount requirement with input {}, output {}, input sdrs {:?} and output sdrs {:?}",
                amount_selection.inputs_sum,
                amount_selection.outputs_sum,
                amount_selection.inputs_sdr,
                amount_selection.outputs_sdr
            );
        }

        // TODO if consolidate strategy: sum all the lowest amount until diff is covered.
        // TODO this would be lowest amount of input strategy.

        // Try to select outputs first with ordering from low to high amount, if that fails, try reversed.

        log::debug!("Ordering inputs from low to high amount");
        // Sort inputs per amount, low to high.
        self.available_inputs
            .sort_by(|left, right| left.output.amount().cmp(&right.output.amount()));

        if let Some(r) = self.fulfill_amount_requirement_inner(&mut amount_selection) {
            return Ok(r);
        }

        if self.selected_inputs.len() + amount_selection.newly_selected_inputs.len() > INPUT_COUNT_MAX.into() {
            // Clear before trying with reversed ordering.
            log::debug!("Clearing amount selection");
            amount_selection = AmountSelection::new(self)?;

            log::debug!("Ordering inputs from high to low amount");
            // Sort inputs per amount, high to low.
            self.available_inputs
                .sort_by(|left, right| right.output.amount().cmp(&left.output.amount()));

            if let Some(r) = self.fulfill_amount_requirement_inner(&mut amount_selection) {
                return Ok(r);
            }
        }

        if self.selected_inputs.len() + amount_selection.newly_selected_inputs.len() > INPUT_COUNT_MAX.into() {
            return Err(Error::InvalidInputCount(
                self.selected_inputs.len() + amount_selection.newly_selected_inputs.len(),
            ));
        }

        if amount_selection.missing_amount() != 0 {
            self.reduce_funds_of_chains(&mut amount_selection)?;
        }

        log::debug!(
            "Outputs {:?} selected to fulfill the amount requirement",
            amount_selection.newly_selected_inputs
        );

        self.available_inputs
            .retain(|input| !amount_selection.newly_selected_inputs.contains_key(input.output_id()));

        Ok(amount_selection.into_newly_selected_inputs())
    }

    fn fulfill_amount_requirement_inner(
        &mut self,
        amount_selection: &mut AmountSelection,
    ) -> Option<Vec<InputSigningData>> {
        let basic_ed25519_inputs = self.available_inputs.iter().filter(|input| {
            if let Output::Basic(output) = &input.output {
                output
                    .unlock_conditions()
                    .locked_address(
                        output.address(),
                        self.slot_index,
                        self.protocol_parameters.committable_age_range(),
                    )
                    .expect("slot index was provided")
                    .expect("expiration unlockable outputs already filtered out")
                    .is_ed25519()
            } else {
                false
            }
        });

        if self.fulfil(basic_ed25519_inputs, amount_selection) {
            return None;
        }

        let basic_non_ed25519_inputs = self.available_inputs.iter().filter(|input| {
            if let Output::Basic(output) = &input.output {
                !output
                    .unlock_conditions()
                    .locked_address(
                        output.address(),
                        self.slot_index,
                        self.protocol_parameters.committable_age_range(),
                    )
                    .expect("slot index was provided")
                    .expect("expiration unlockable outputs already filtered out")
                    .is_ed25519()
            } else {
                false
            }
        });

        if self.fulfil(basic_non_ed25519_inputs, amount_selection) {
            return None;
        }

        // Other kinds of outputs.

        log::debug!("Trying other types of outputs");

        let mut inputs = self
            .available_inputs
            .iter()
            .filter(|input| !input.output.is_basic())
            .peekable();

        if inputs.peek().is_some() {
            amount_selection.fulfil(inputs);

            log::debug!(
                "Outputs {:?} selected to fulfill the amount requirement",
                amount_selection.newly_selected_inputs
            );
            log::debug!("Triggering another amount round as non-basic outputs need to be transitioned first");

            if self.selected_inputs.len() + amount_selection.newly_selected_inputs.len() <= INPUT_COUNT_MAX.into() {
                self.available_inputs
                    .retain(|input| !amount_selection.newly_selected_inputs.contains_key(input.output_id()));

                // TODO explanation of Amount
                self.requirements.push(Requirement::Amount);

                Some(amount_selection.clone().into_newly_selected_inputs())
            } else {
                None
            }
        } else {
            None
        }
    }
}
