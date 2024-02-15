// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection};
use crate::client::secret::types::InputSigningData;

impl InputSelection {
    pub(crate) fn fulfill_mana_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let (mut selected_mana, required_mana) = self.mana_sums(true)?;

        log::debug!("Mana requirement selected mana: {selected_mana}, required mana: {required_mana}");

        if selected_mana >= required_mana {
            log::debug!("Mana requirement already fulfilled");
            Ok(Vec::new())
        } else {
            let mut inputs = Vec::new();

            // TODO we should do as for the amount and have preferences on which inputs to pick.
            while let Some(input) = self.available_inputs.pop() {
                selected_mana += self.total_mana(&input)?;
                inputs.push(input);

                if selected_mana >= required_mana {
                    break;
                }
            }
            if selected_mana < required_mana {
                return Err(Error::InsufficientMana {
                    found: selected_mana,
                    required: required_mana,
                });
            }

            Ok(inputs)
        }
    }

    pub(crate) fn mana_sums(&self, include_remainders: bool) -> Result<(u64, u64), Error> {
        let required_mana = if include_remainders {
            self.all_outputs().map(|o| o.mana()).sum::<u64>() + self.remainders.added_mana
        } else {
            self.non_remainder_outputs().map(|o| o.mana()).sum::<u64>()
        } + self.mana_allotments.values().sum::<u64>();
        let mut selected_mana = 0;

        for input in &self.selected_inputs {
            selected_mana += self.total_mana(input)?;
        }
        Ok((selected_mana, required_mana))
    }

    fn total_mana(&self, input: &InputSigningData) -> Result<u64, Error> {
        Ok(self.mana_rewards.get(input.output_id()).copied().unwrap_or_default()
            + input.output.available_mana(
                &self.protocol_parameters,
                input.output_id().transaction_id().slot_index(),
                self.creation_slot,
            )?)
    }
}
