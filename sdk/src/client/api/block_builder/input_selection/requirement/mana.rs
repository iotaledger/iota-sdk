// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection};
use crate::client::secret::types::InputSigningData;

impl InputSelection {
    pub(crate) fn fulfill_mana_requirement(&mut self, allotments: u64) -> Result<Vec<InputSigningData>, Error> {
        let required_mana = self.outputs.iter().map(|o| o.mana()).sum::<u64>() + allotments;
        let mut selected_mana = 0;

        for input in &self.selected_inputs {
            selected_mana += input.output.available_mana(
                &self.protocol_parameters,
                input.output_id().transaction_id().slot_index(),
                self.slot_index,
            )?;
            // TODO rewards https://github.com/iotaledger/iota-sdk/issues/1310
        }

        if selected_mana >= required_mana {
            log::debug!("Mana requirement already fulfilled");
            Ok(Vec::new())
        } else {
            let mut inputs = Vec::new();

            // TODO we should do as for the amount and have preferences on which inputs to pick.
            while let Some(input) = self.available_inputs.pop() {
                selected_mana += input.output.mana();
                inputs.push(input);

                if selected_mana >= required_mana {
                    break;
                }
            }

            Ok(inputs)
        }
    }
}
