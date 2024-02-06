// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput, RewardContextInput},
        output::{DelegationOutputBuilder, Output},
    },
};

impl InputSelection {
    pub(crate) fn fulfill_context_inputs_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let mut needs_commitment_context = false;

        for (idx, input) in self.selected_inputs.iter().enumerate() {
            // Transitioning an issuer account requires a BlockIssuanceCreditContextInput.
            if let Output::Account(account) = &input.output {
                if account.features().block_issuer().is_some() {
                    self.context_inputs.insert(
                        BlockIssuanceCreditContextInput::from(account.account_id_non_null(input.output_id())).into(),
                    );
                }
            }

            // Inputs with timelock or expiration unlock condition require a CommitmentContextInput
            if input
                .output
                .unlock_conditions()
                .map_or(false, |u| u.iter().any(|u| u.is_timelock() || u.is_expiration()))
            {
                needs_commitment_context = true;
            }

            if self.mana_rewards.get(input.output_id()).is_some() {
                self.context_inputs.insert(RewardContextInput::new(idx as _)?.into());
                needs_commitment_context = true;
            }
        }
        for output in self.outputs.iter_mut().filter(|o| o.is_delegation()) {
            // Created delegations have their start epoch set, and delayed delegations have their end set
            if output.as_delegation().delegation_id().is_null() {
                *output = DelegationOutputBuilder::from(output.as_delegation())
                    .with_start_epoch(self.protocol_parameters.delegation_start_epoch(self.slot_commitment_id))
                    .finish_output()?;
            } else {
                *output = DelegationOutputBuilder::from(output.as_delegation())
                    .with_end_epoch(self.protocol_parameters.delegation_end_epoch(self.slot_commitment_id))
                    .finish_output()?;
            }
            needs_commitment_context = true;
        }
        // BlockIssuanceCreditContextInput requires a CommitmentContextInput.
        if self
            .context_inputs
            .iter()
            .any(|c| c.kind() == BlockIssuanceCreditContextInput::KIND)
        {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            needs_commitment_context = true;
        }

        if needs_commitment_context
            && !self
                .context_inputs
                .iter()
                .any(|c| c.kind() == CommitmentContextInput::KIND)
        {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            self.context_inputs
                .insert(CommitmentContextInput::new(self.slot_commitment_id).into());
        }
        Ok(Vec::new())
    }
}
