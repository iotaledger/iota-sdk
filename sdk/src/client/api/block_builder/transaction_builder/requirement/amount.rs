// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        address::Address,
        input::{UtxoInput, INPUT_COUNT_MAX},
        output::{
            unlock_condition::StorageDepositReturnUnlockCondition, AccountOutput, BasicOutput, ChainId, FoundryOutput,
            NftOutput, Output,
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
        if self.available_inputs.is_empty() {
            return Err(TransactionBuilderError::InsufficientAmount {
                found: input_amount,
                required: output_amount,
            });
        }

        while let Some(input) = self.next_input_for_amount(output_amount - input_amount, self.latest_slot_commitment_id)
        {
            self.select_input(input)?;
            (input_amount, output_amount) = self.amount_balance()?;
            if input_amount >= output_amount {
                break;
            }
        }
        if output_amount > input_amount {
            return Err(TransactionBuilderError::InsufficientAmount {
                found: input_amount,
                required: output_amount,
            });
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

    pub(crate) fn amount_balance(&mut self) -> Result<(u64, u64), TransactionBuilderError> {
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

    fn next_input_for_amount(
        &mut self,
        missing_amount: u64,
        slot_commitment_id: SlotCommitmentId,
    ) -> Option<InputSigningData> {
        self.available_inputs
            .iter()
            .enumerate()
            .filter_map(|(idx, input)| {
                self.score_for_amount(input, missing_amount, slot_commitment_id.slot_index())
                    .map(|score| (score, idx))
            })
            .max_by_key(|(score, _)| *score)
            .map(|(_, idx)| self.available_inputs.swap_remove(idx))
    }

    // Score an input based on how desirable it is.
    fn score_for_amount(
        &self,
        input: &InputSigningData,
        mut missing_amount: u64,
        slot_index: SlotIndex,
    ) -> Option<usize> {
        ([
            BasicOutput::KIND,
            NftOutput::KIND,
            AccountOutput::KIND,
            FoundryOutput::KIND,
        ]
        .contains(&input.output.kind()))
        .then(|| {
            let mut work_score = self
                .protocol_parameters
                .work_score(&UtxoInput::from(*input.output_id()));
            let amount = input.output.amount();
            let mut remainder_work_score = 0;
            if let Some(sdruc) = sdruc_not_expired(&input.output, slot_index) {
                missing_amount += sdruc.amount();
                remainder_work_score = self.protocol_parameters.work_score(self.basic_remainder())
            }

            if let Ok(Some(output)) = self.transition_input(input) {
                missing_amount += output.amount();
                work_score += self.protocol_parameters.work_score(&output);
            } else if input.output.native_token().is_some() {
                missing_amount += self.native_token_remainder().amount();
                remainder_work_score += self.protocol_parameters.work_score(self.native_token_remainder());
            } else if amount > missing_amount {
                missing_amount += self.basic_remainder().amount();
                remainder_work_score = self.protocol_parameters.work_score(self.basic_remainder());
            }
            work_score += remainder_work_score;

            let amount_diff = amount.abs_diff(missing_amount) as f64;
            // Exp(-x) creates a curve which is 1 when x is 0, and approaches 0 as x increases
            // If the amount is insufficient, the score will decrease the more inputs are selected
            let amount_score = if amount >= missing_amount {
                (-amount_diff / u64::MAX as f64).exp()
            } else {
                (-amount_diff / missing_amount as f64).exp()
                    * ((INPUT_COUNT_MAX as f64 - self.selected_inputs.len() as f64) / INPUT_COUNT_MAX as f64)
            };
            let work_score = (-(work_score as f64) / u32::MAX as f64).exp();
            // Normalize scores between 0..1 with 1 being desirable
            (amount_score * work_score * usize::MAX as f64).round() as _
        })
    }
}
