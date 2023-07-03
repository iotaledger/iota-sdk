// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection, Requirement};
use crate::{
    client::{api::input_selection::Burn, secret::types::InputSigningData},
    types::block::output::{AccountId, AccountTransition, Output, OutputId},
};

pub fn is_account_transition<'a>(
    input: &Output,
    input_id: OutputId,
    outputs: &[Output],
    burn: impl Into<Option<&'a Burn>>,
) -> Option<AccountTransition> {
    if let Output::Account(account_input) = &input {
        let account_id = account_input.account_id_non_null(&input_id);
        // Checks if the account exists in the outputs and gets the transition type.
        for output in outputs.iter() {
            if let Output::Account(account_output) = output {
                if *account_output.account_id() == account_id {
                    if account_output.state_index() == account_input.state_index() {
                        // Governance transition.
                        return Some(AccountTransition::Governance);
                    } else {
                        // State transition.
                        return Some(AccountTransition::State);
                    }
                }
            }
        }
        if let Some(burn) = burn.into() {
            if burn.accounts().contains(&account_id) {
                return Some(AccountTransition::Governance);
            }
        }
    }
    None
}

/// Checks if an output is an account with a given non null account ID.
/// Calling it with a null account ID may lead to undefined behavior.
pub(crate) fn is_account_with_id_non_null(output: &Output, account_id: &AccountId) -> bool {
    if let Output::Account(account) = output {
        account.account_id() == account_id
    } else {
        false
    }
}

/// Checks if an output is an account with output ID that matches the given account ID.
pub(crate) fn is_account_with_id(output: &Output, output_id: &OutputId, account_id: &AccountId) -> bool {
    if let Output::Account(account) = output {
        &account.account_id_non_null(output_id) == account_id
    } else {
        false
    }
}

impl InputSelection {
    /// Fulfills an account requirement by selecting the appropriate account from the available inputs.
    pub(crate) fn fulfill_account_requirement(
        &mut self,
        account_id: AccountId,
        account_transition: AccountTransition,
    ) -> Result<Vec<(InputSigningData, Option<AccountTransition>)>, Error> {
        // Check that the account is not burned when a state transition is required.
        if account_transition.is_state()
            && self
                .burn
                .as_ref()
                .map_or(false, |burn| burn.accounts.contains(&account_id))
        {
            return Err(Error::UnfulfillableRequirement(Requirement::Account(
                account_id,
                account_transition,
            )));
        }

        let selected_input = self
            .selected_inputs
            .iter()
            .find(|input| is_account_with_id(&input.output, input.output_id(), &account_id));

        // If a state transition is not required and the account has already been selected, no additional check has to
        // be performed.
        if !account_transition.is_state() && selected_input.is_some() {
            log::debug!(
                "{account_id:?}/{account_transition:?} requirement already fulfilled by {:?}",
                selected_input.unwrap().output_id()
            );
            return Ok(Vec::new());
        }

        let available_index = self
            .available_inputs
            .iter()
            .position(|input| is_account_with_id(&input.output, input.output_id(), &account_id));

        // If the account was not already selected and it not available, the requirement can't be fulfilled.
        if selected_input.is_none() && available_index.is_none() {
            return Err(Error::UnfulfillableRequirement(Requirement::Account(
                account_id,
                account_transition,
            )));
        }

        // If a state transition is not required, we can simply select the account.
        if !account_transition.is_state() {
            // Remove the output from the available inputs, swap to make it O(1).
            let input = self.available_inputs.swap_remove(available_index.unwrap());

            log::debug!(
                "{account_id:?}/{account_transition:?} requirement fulfilled by {:?}",
                input.output_id()
            );

            // PANIC: safe to unwrap as it's been checked that it can't be None when a state transition is not required.
            return Ok(vec![(input, None)]);
        }

        // At this point, a state transition is required so we need to verify that an account output describing a
        // governance transition was not provided.

        // PANIC: safe to unwrap as it's been checked that both can't be None at the same time.
        let input = selected_input.unwrap_or_else(|| &self.available_inputs[available_index.unwrap()]);

        if is_account_transition(&input.output, *input.output_id(), &self.outputs, self.burn.as_ref())
            == Some(AccountTransition::Governance)
        {
            return Err(Error::UnfulfillableRequirement(Requirement::Account(
                account_id,
                account_transition,
            )));
        }

        if let Some(available_index) = available_index {
            // Remove the output from the available inputs, swap to make it O(1).
            let input = self.available_inputs.swap_remove(available_index);

            log::debug!(
                "{account_id:?}/{account_transition:?} requirement fulfilled by {:?}",
                input.output_id()
            );

            return Ok(vec![(input, None)]);
        }

        log::debug!(
            "{account_id:?}/{account_transition:?} requirement already fulfilled by {:?}",
            selected_input.unwrap().output_id()
        );

        Ok(Vec::new())
    }
}
