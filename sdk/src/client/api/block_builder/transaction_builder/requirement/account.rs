// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::{
    client::secret::types::InputSigningData,
    types::block::output::{AccountId, Output, OutputId},
};

/// Checks if an output is an account with output ID that matches the given account ID.
pub(crate) fn is_account_with_id(output: &Output, account_id: &AccountId, output_id: &OutputId) -> bool {
    match output {
        Output::Basic(basic) => basic.is_implicit_account() && &AccountId::from(output_id) == account_id,
        Output::Account(account) => &account.account_id_non_null(output_id) == account_id,
        _ => false,
    }
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

impl TransactionBuilder {
    /// Fulfills an account requirement by selecting the appropriate account from the available inputs.
    pub(crate) fn fulfill_account_requirement(
        &mut self,
        account_id: AccountId,
    ) -> Result<Vec<InputSigningData>, TransactionBuilderError> {
        // Check if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| is_account_with_id(&input.output, &account_id, input.output_id()))
        {
            log::debug!(
                "{account_id:?} requirement already fulfilled by {:?}",
                input.output_id()
            );
            return Ok(Vec::new());
        }

        // Check if the requirement can be fulfilled.
        let index = self
            .available_inputs
            .iter()
            .position(|input| is_account_with_id(&input.output, &account_id, input.output_id()))
            .ok_or(TransactionBuilderError::UnfulfillableRequirement(Requirement::Account(
                account_id,
            )))?;
        // Remove the input from the available inputs, swap to make it O(1).
        let input = self.available_inputs.swap_remove(index);

        log::debug!("{account_id:?} requirement fulfilled by {:?}", input.output_id());

        Ok(vec![input])
    }
}
