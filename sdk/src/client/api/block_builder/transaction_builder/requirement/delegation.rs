// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Requirement, TransactionBuilder, TransactionBuilderError};
use crate::{
    client::secret::types::InputSigningData,
    types::block::output::{DelegationId, Output, OutputId},
};

/// Checks if an output is an delegation with a given delegation ID.
/// Assumes that the output delegation ID can be null and hashes the output ID.
/// Use when not sure if the delegation has been assigned a proper ID already.
pub(crate) fn is_delegation_with_id(output: &Output, delegation_id: &DelegationId, output_id: &OutputId) -> bool {
    if let Output::Delegation(delegation) = output {
        &delegation.delegation_id_non_null(output_id) == delegation_id
    } else {
        false
    }
}

/// Checks if an output is an delegation with a given delegation ID.
/// Assumes that the output delegation ID is non null to avoid an output ID hash.
/// Only use when sure that the delegation has been assigned a proper ID already.
pub(crate) fn is_delegation_with_id_non_null(output: &Output, delegation_id: &DelegationId) -> bool {
    if let Output::Delegation(delegation) = output {
        delegation.delegation_id() == delegation_id
    } else {
        false
    }
}

impl TransactionBuilder {
    /// Fulfills an delegation requirement by selecting the appropriate delegation from the available inputs.
    pub(crate) fn fulfill_delegation_requirement(
        &mut self,
        delegation_id: DelegationId,
    ) -> Result<Vec<InputSigningData>, TransactionBuilderError> {
        // Check if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| is_delegation_with_id(&input.output, &delegation_id, input.output_id()))
        {
            log::debug!(
                "{delegation_id:?} requirement already fulfilled by {:?}",
                input.output_id()
            );
            return Ok(Vec::new());
        }

        // Check if the requirement can be fulfilled.
        let index = self
            .available_inputs
            .iter()
            .position(|input| is_delegation_with_id(&input.output, &delegation_id, input.output_id()))
            .ok_or(TransactionBuilderError::UnfulfillableRequirement(
                Requirement::Delegation(delegation_id),
            ))?;
        // Remove the input from the available inputs, swap to make it O(1).
        let input = self.available_inputs.swap_remove(index);

        log::debug!("{delegation_id:?} requirement fulfilled by {:?}", input.output_id());

        Ok(vec![input])
    }
}
