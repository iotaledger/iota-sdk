// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{account::is_account_transition, Error, InputSelection, Requirement};
use crate::{client::secret::types::InputSigningData, types::block::address::Address};

impl InputSelection {
    // Checks if a selected input unlocks a given ED25519 address.
    fn selected_unlocks_ed25519_address(&self, input: &InputSigningData, address: &Address) -> bool {
        let account_transition = is_account_transition(
            &input.output,
            *input.output_id(),
            self.outputs.as_slice(),
            self.burn.as_ref(),
        );

        // PANIC: safe to unwrap as outputs with no address have been filtered out already.
        let required_address = input
            .output
            .required_and_unlocked_address(self.slot_index, input.output_id())
            .unwrap()
            .0;

        if account_transition.is_some() {
            // Only check if we own the required address if the input is an account because other types of output have
            // been filtered by address already.
            &required_address == address && self.addresses.contains(address)
        } else {
            &required_address == address
        }
    }

    // Checks if an available input can unlock a given ED25519 address.
    // In case an account input is selected, also tells if it needs to be state or governance transitioned.
    fn available_has_ed25519_address(&self, input: &InputSigningData, address: &Address) -> bool {
        if input.output.is_account() {
            // PANIC: safe to unwrap as outputs without unlock conditions have been filtered out already.
            let unlock_conditions = input.output.unlock_conditions().unwrap();

            // PANIC: safe to unwrap as accounts have a state controller address.
            if unlock_conditions.state_controller_address().unwrap().address() == address {
                return self.addresses.contains(address);
            }

            // PANIC: safe to unwrap as accounts have a governor address.
            if unlock_conditions.governor_address().unwrap().address() == address {
                return self.addresses.contains(address);
            }

            false
        } else {
            let (required_address, _) = input
                .output
                .required_and_unlocked_address(self.slot_index, input.output_id())
                .unwrap();

            &required_address == address
        }
    }

    /// Fulfills an ed25519 sender requirement by selecting an available input that unlocks its address.
    pub(crate) fn fulfill_ed25519_requirement(&mut self, address: Address) -> Result<Vec<InputSigningData>, Error> {
        // Checks if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| self.selected_unlocks_ed25519_address(input, &address))
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
            .find(|(_, input)| input.output.is_basic() && self.available_has_ed25519_address(input, &address))
        {
            Some((index, None))
        } else {
            // Otherwise, checks if the requirement can be fulfilled by a non-basic output.
            self.available_inputs.iter().enumerate().find_map(|(index, input)| {
                if !input.output.is_basic() {
                    if let (true, account_transition) = self.available_has_ed25519_address(input, &address) {
                        Some(index)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        };

        match found {
            Some((index, account_transition)) => {
                // Remove the input from the available inputs, swap to make it O(1).
                let input = self.available_inputs.swap_remove(index);

                log::debug!(
                    "{address:?} sender requirement fulfilled by {:?} (account transition {:?})",
                    input.output_id(),
                    account_transition
                );

                Ok(vec![input])
            }
            None => Err(Error::UnfulfillableRequirement(Requirement::Ed25519(address))),
        }
    }
}
