// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection};
use crate::client::secret::types::InputSigningData;

impl InputSelection {
    pub(crate) fn fulfill_mana_requirement(&mut self, allotments: u64) -> Result<Vec<InputSigningData>, Error> {
        let required_mana = self.outputs.iter().map(|o| o.mana()).sum::<u64>() + allotments;

        // Checks if the requirement is already fulfilled.
        let selected_mana = self.selected_inputs.iter().map(|o| o.output.mana()).sum::<u64>();

        if selected_mana >= required_mana {
            log::debug!("Mana requirement already fulfilled");
            return Ok(Vec::new());
        }

        Ok(vec![])
    }
}
