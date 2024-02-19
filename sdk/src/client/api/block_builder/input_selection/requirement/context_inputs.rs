// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput, RewardContextInput},
        output::{AccountId, DelegationOutputBuilder, Output},
    },
};

impl InputSelection {
    pub(crate) fn fulfill_context_inputs_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let mut needs_commitment_context = false;

        for (idx, input) in self.selected_inputs.iter().enumerate() {
            match &input.output {
                // Transitioning an issuer account requires a BlockIssuanceCreditContextInput.
                Output::Account(account) => {
                    if account.features().block_issuer().is_some() {
                        log::debug!("Adding block issuance context input for transitioned account output");
                        self.context_inputs.insert(
                            BlockIssuanceCreditContextInput::from(account.account_id_non_null(input.output_id()))
                                .into(),
                        );
                    }
                }
                // Transitioning an implicit account requires a BlockIssuanceCreditContextInput.
                Output::Basic(basic) => {
                    if basic.is_implicit_account() {
                        log::debug!("Adding block issuance context input for transitioned implicit account output");
                        self.context_inputs
                            .insert(BlockIssuanceCreditContextInput::from(AccountId::from(input.output_id())).into());
                    }
                }
                _ => (),
            }

            // Inputs with timelock or expiration unlock condition require a CommitmentContextInput
            if input
                .output
                .unlock_conditions()
                .map_or(false, |u| u.iter().any(|u| u.is_timelock() || u.is_expiration()))
            {
                log::debug!("Adding commitment context input for timelocked or expiring output");
                needs_commitment_context = true;
            }

            if self.mana_rewards.get(input.output_id()).is_some() {
                log::debug!("Adding reward and commitment context input for output claiming mana rewards");
                self.context_inputs.insert(RewardContextInput::new(idx as _)?.into());
                needs_commitment_context = true;
            }
        }
        for output in self
            .provided_outputs
            .iter_mut()
            .chain(&mut self.added_outputs)
            .filter(|o| o.is_delegation())
        {
            // Created delegations have their start epoch set, and delayed delegations have their end set
            if output.as_delegation().delegation_id().is_null() {
                let start_epoch = self
                    .protocol_parameters
                    .delegation_start_epoch(self.latest_slot_commitment_id);
                log::debug!("Setting created delegation start epoch to {start_epoch}");
                *output = DelegationOutputBuilder::from(output.as_delegation())
                    .with_start_epoch(start_epoch)
                    .finish_output()?;
            } else {
                let end_epoch = self
                    .protocol_parameters
                    .delegation_end_epoch(self.latest_slot_commitment_id);
                log::debug!("Setting delayed delegation end epoch to {end_epoch}");
                *output = DelegationOutputBuilder::from(output.as_delegation())
                    .with_end_epoch(end_epoch)
                    .finish_output()?;
            }
            log::debug!("Adding commitment context input for delegation output");
            needs_commitment_context = true;
        }
        // BlockIssuanceCreditContextInput requires a CommitmentContextInput.
        if self
            .context_inputs
            .iter()
            .any(|c| c.kind() == BlockIssuanceCreditContextInput::KIND)
        {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            log::debug!("Adding commitment context input for output with block issuance credit context input");
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
                .insert(CommitmentContextInput::new(self.latest_slot_commitment_id).into());
        }
        Ok(Vec::new())
    }
}
