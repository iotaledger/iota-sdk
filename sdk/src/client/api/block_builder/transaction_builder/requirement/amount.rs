// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, sync::OnceLock};

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::{
    client::{api::transaction_builder::requirement::PriorityMap, secret::types::InputSigningData},
    types::block::{
        address::Address,
        output::{
            unlock_condition::StorageDepositReturnUnlockCondition, AccountOutput, AccountOutputBuilder, BasicOutput,
            ChainId, FoundryOutput, FoundryOutputBuilder, MinimumOutputAmount, NftOutput, NftOutputBuilder, Output,
        },
        slot::{SlotCommitmentId, SlotIndex},
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
        (!expired).then_some(sdr)
    })
}

impl TransactionBuilder {
    pub(crate) fn fulfill_amount_requirement(&mut self) -> Result<(), TransactionBuilderError> {
        let (mut input_amount, mut output_amount) = self.amount_balance()?;
        if input_amount >= output_amount {
            log::debug!("Amount requirement already fulfilled");
            return Ok(());
        }

        log::debug!("Fulfilling amount requirement with input amount {input_amount}, output amount {output_amount}");

        if !self.allow_additional_input_selection {
            return Err(TransactionBuilderError::AdditionalInputsRequired(Requirement::Amount));
        }
        // If we have no inputs to balance with, try reducing outputs instead
        if self.available_inputs.is_empty() {
            if !self.reduce_funds_of_chains(input_amount, &mut output_amount)? {
                return Err(TransactionBuilderError::InsufficientAmount {
                    found: input_amount,
                    required: output_amount,
                });
            }
        } else {
            let mut priority_map = PriorityMap::<AmountPriority>::generate(&mut self.available_inputs);
            loop {
                let Some(input) = priority_map.next(output_amount - input_amount, self.latest_slot_commitment_id)
                else {
                    break;
                };
                log::debug!("selecting input with amount {}", input.output.amount());
                self.select_input(input)?;
                (input_amount, output_amount) = self.amount_balance()?;
                // Try to reduce output funds
                if self.reduce_funds_of_chains(input_amount, &mut output_amount)? {
                    break;
                }
            }
            // Return unselected inputs to the available list
            for input in priority_map.into_inputs() {
                self.available_inputs.push(input);
            }
            if output_amount > input_amount {
                return Err(TransactionBuilderError::InsufficientAmount {
                    found: input_amount,
                    required: output_amount,
                });
            }
        }

        Ok(())
    }

    pub(crate) fn amount_sums(&self) -> (u64, u64, HashMap<Address, u64>, HashMap<Address, u64>) {
        let mut inputs_sum = 0;
        let mut outputs_sum = 0;
        let mut inputs_sdr = HashMap::new();
        let mut outputs_sdr = HashMap::new();

        for selected_input in &self.selected_inputs {
            inputs_sum += selected_input.output.amount();

            if let Some(sdruc) = sdruc_not_expired(&selected_input.output, self.latest_slot_commitment_id.slot_index())
            {
                *inputs_sdr.entry(sdruc.return_address().clone()).or_default() += sdruc.amount();
            }
        }

        for output in self.non_remainder_outputs() {
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

    pub(crate) fn amount_balance(&self) -> Result<(u64, u64), TransactionBuilderError> {
        let (inputs_sum, mut outputs_sum, _, _) = self.amount_sums();
        let (remainder_amount, native_tokens_remainder, mana_remainder) = self.required_remainder_amount()?;
        if inputs_sum > outputs_sum {
            let diff = inputs_sum - outputs_sum;

            if remainder_amount > diff {
                outputs_sum += remainder_amount - diff
            }
        } else if native_tokens_remainder || mana_remainder {
            outputs_sum += remainder_amount
        }
        Ok((inputs_sum, outputs_sum))
    }

    fn reduce_funds_of_chains(
        &mut self,
        input_amount: u64,
        output_amount: &mut u64,
    ) -> Result<bool, TransactionBuilderError> {
        if *output_amount > input_amount {
            // Only consider automatically transitioned outputs.
            for output in self.added_outputs.iter_mut() {
                let missing_amount = *output_amount - input_amount;
                let amount = output.amount();
                let minimum_amount = output.minimum_amount(self.protocol_parameters.storage_score_parameters());

                let new_amount = if amount >= missing_amount + minimum_amount {
                    *output_amount = input_amount;
                    amount - missing_amount
                } else {
                    *output_amount -= amount - minimum_amount;
                    minimum_amount
                };

                // PANIC: unwrap is fine as non-chain outputs have been filtered out already.
                log::debug!(
                    "Reducing amount of {} to {} to fulfill amount requirement",
                    output.chain_id().unwrap(),
                    new_amount
                );

                *output = match output {
                    Output::Account(output) => AccountOutputBuilder::from(&*output)
                        .with_amount(new_amount)
                        .finish_output()?,
                    Output::Foundry(output) => FoundryOutputBuilder::from(&*output)
                        .with_amount(new_amount)
                        .finish_output()?,
                    Output::Nft(output) => NftOutputBuilder::from(&*output)
                        .with_amount(new_amount)
                        .finish_output()?,
                    _ => continue,
                };

                if *output_amount == input_amount {
                    break;
                }
            }
        }

        Ok(input_amount >= *output_amount)
    }

    pub(crate) fn amount_chains(&self) -> Result<HashMap<ChainId, (u64, u64)>, TransactionBuilderError> {
        let mut res = self
            .non_remainder_outputs()
            .filter_map(|o| o.chain_id().map(|id| (id, (0, o.amount()))))
            .collect::<HashMap<_, _>>();
        for input in &self.selected_inputs {
            if let Some(chain_id) = input
                .output
                .chain_id()
                .map(|id| id.or_from_output_id(input.output_id()))
            {
                res.entry(chain_id).or_default().0 += input.output.amount();
            }
        }
        Ok(res)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct AmountPriority {
    kind_priority: usize,
    has_native_token: bool,
}

impl PartialOrd for AmountPriority {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AmountPriority {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.kind_priority, self.has_native_token).cmp(&(other.kind_priority, other.has_native_token))
    }
}

impl From<&InputSigningData> for Option<AmountPriority> {
    fn from(value: &InputSigningData) -> Self {
        sort_order_type()
            .get(&value.output.kind())
            .map(|&kind_priority| AmountPriority {
                kind_priority,
                has_native_token: value.output.native_token().is_some(),
            })
    }
}

/// Establish the order in which we want to pick an input
pub fn sort_order_type() -> &'static HashMap<u8, usize> {
    static MAP: OnceLock<HashMap<u8, usize>> = OnceLock::new();
    MAP.get_or_init(|| {
        [
            BasicOutput::KIND,
            AccountOutput::KIND,
            NftOutput::KIND,
            FoundryOutput::KIND,
        ]
        .into_iter()
        .zip(0_usize..)
        .collect::<HashMap<_, _>>()
    })
}

impl PriorityMap<AmountPriority> {
    fn next(&mut self, missing_amount: u64, slot_committment_id: SlotCommitmentId) -> Option<InputSigningData> {
        let amount_sort = |output: &Output| {
            let mut amount = output.amount();
            if let Some(sdruc) = sdruc_not_expired(output, slot_committment_id.slot_index()) {
                amount -= sdruc.amount();
            }
            // If the amount is greater than the missing amount, we want the smallest ones first
            if amount >= missing_amount {
                (false, amount)
            // Otherwise, we want the biggest first
            } else {
                (true, u64::MAX - amount)
            }
        };
        if let Some((priority, mut inputs)) = self.0.pop_first() {
            // Sort in reverse so we can pop from the back
            inputs.sort_unstable_by(|i1, i2| amount_sort(&i2.output).cmp(&amount_sort(&i1.output)));
            let input = inputs.pop();
            if !inputs.is_empty() {
                self.0.insert(priority, inputs);
            }
            return input;
        }
        None
    }
}
