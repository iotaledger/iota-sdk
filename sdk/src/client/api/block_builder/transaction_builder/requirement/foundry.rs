// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::types::block::output::{FoundryId, Output};

/// Checks if an output is a foundry with a given foundry ID.
pub(crate) fn is_foundry_with_id(output: &Output, foundry_id: &FoundryId) -> bool {
    if let Output::Foundry(foundry) = output {
        &foundry.id() == foundry_id
    } else {
        false
    }
}

impl TransactionBuilder {
    /// Fulfills a foundry requirement by selecting the appropriate foundry from the available inputs.
    pub(crate) fn fulfill_foundry_requirement(&mut self, foundry_id: FoundryId) -> Result<(), TransactionBuilderError> {
        // Check if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| is_foundry_with_id(&input.output, &foundry_id))
        {
            log::debug!(
                "{foundry_id:?} requirement already fulfilled by {:?}",
                input.output_id()
            );
            return Ok(());
        }

        if !self.allow_additional_input_selection {
            return Err(TransactionBuilderError::AdditionalInputsRequired(Requirement::Foundry(
                foundry_id,
            )));
        }

        // Check if the requirement can be fulfilled.
        let index = self
            .available_inputs
            .iter()
            .position(|input| is_foundry_with_id(&input.output, &foundry_id))
            .ok_or(TransactionBuilderError::UnfulfillableRequirement(Requirement::Foundry(
                foundry_id,
            )))?;
        // Remove the input from the available inputs, swap to make it O(1).
        let input = self.available_inputs.swap_remove(index);

        log::debug!("{foundry_id:?} requirement fulfilled by {:?}", input.output_id());

        self.select_input(input)?;

        Ok(())
    }
}
