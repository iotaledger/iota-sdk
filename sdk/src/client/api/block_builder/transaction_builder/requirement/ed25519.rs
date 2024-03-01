// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::{client::secret::types::InputSigningData, types::block::address::Address};

impl TransactionBuilder {
    // Checks if a selected input unlocks a given ED25519 address.
    fn selected_unlocks_ed25519_address(&self, input: &InputSigningData, address: &Address) -> bool {
        let required_address = input
            .output
            .required_address(
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.committable_age_range(),
            )
            // PANIC: safe to unwrap as outputs with no address have been filtered out already.
            .unwrap()
            .expect("expiration unlockable outputs already filtered out");

        &required_address == address
    }

    // Checks if an available input can unlock a given ED25519 address.
    // In case an account input is selected, also tells if it needs to be state or governance transitioned.
    fn available_has_ed25519_address(&self, input: &InputSigningData, address: &Address) -> bool {
        let required_address = input
            .output
            .required_address(
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.committable_age_range(),
            )
            // PANIC: safe to unwrap as outputs with no address have been filtered out already.
            .unwrap()
            .expect("expiration unlockable outputs already filtered out");

        required_address
            .backing_ed25519()
            .map_or(false, |a| a == address.as_ed25519())
    }

    /// Fulfills an ed25519 sender requirement by selecting an available input that unlocks its address.
    pub(crate) fn fulfill_ed25519_requirement(
        &mut self,
        address: &Address,
    ) -> Result<Vec<InputSigningData>, TransactionBuilderError> {
        // Checks if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| self.selected_unlocks_ed25519_address(input, address))
        {
            log::debug!(
                "{address:?} sender requirement already fulfilled by {:?}",
                input.output_id()
            );
            return Ok(Vec::new());
        }

        // Checks if the requirement can be fulfilled by a basic output.
        let found = if let Some((index, _)) = self
            .available_inputs
            .iter()
            .enumerate()
            .find(|(_, input)| input.output.is_basic() && self.available_has_ed25519_address(input, address))
        {
            Some(index)
        } else {
            // Otherwise, checks if the requirement can be fulfilled by a non-basic output.
            self.available_inputs.iter().enumerate().find_map(|(index, input)| {
                (!input.output.is_basic() && self.available_has_ed25519_address(input, address)).then_some(index)
            })
        };

        match found {
            Some(index) => {
                // Remove the input from the available inputs, swap to make it O(1).
                let input = self.available_inputs.swap_remove(index);

                log::debug!("{address:?} sender requirement fulfilled by {:?}", input.output_id(),);

                Ok(vec![input])
            }
            None => Err(TransactionBuilderError::UnfulfillableRequirement(Requirement::Ed25519(
                address.clone(),
            ))),
        }
    }
}
