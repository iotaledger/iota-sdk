// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{TransactionBuilder, TransactionBuilderError};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput},
        output::{AccountId, DelegationOutputBuilder, Output},
    },
};

impl TransactionBuilder {
    // Clippy's suggestion greatly degrades readability.
    #[allow(clippy::useless_let_if_seq)]
    pub(crate) fn fulfill_context_inputs_requirements(&mut self, input: &InputSigningData) {
        match &input.output {
            // Transitioning an issuer account requires a BlockIssuanceCreditContextInput.
            Output::Account(account) => {
                if account.features().block_issuer().is_some() {
                    log::debug!("Adding block issuance context input for transitioned account output");
                    self.bic_context_inputs.insert(BlockIssuanceCreditContextInput::from(
                        account.account_id_non_null(input.output_id()),
                    ));
                }
            }
            // Transitioning an implicit account requires a BlockIssuanceCreditContextInput.
            Output::Basic(basic) => {
                if basic.is_implicit_account() {
                    log::debug!("Adding block issuance context input for transitioned implicit account output");
                    self.bic_context_inputs
                        .insert(BlockIssuanceCreditContextInput::from(AccountId::from(
                            input.output_id(),
                        )));
                }
            }
            _ => (),
        }

        if self.commitment_context_input.is_some() {
            return;
        }
        let mut needs_commitment_context = false;

        // Inputs with timelock or expiration unlock condition require a CommitmentContextInput
        if input
            .output
            .unlock_conditions()
            .iter()
            .any(|u| u.is_timelock() || u.is_expiration())
        {
            log::debug!("Adding commitment context input for timelocked or expiring output");
            needs_commitment_context = true;
        }

        if self.mana_rewards.get(input.output_id()).is_some() {
            log::debug!("Adding reward and commitment context input for output claiming mana rewards");
            self.reward_context_inputs.insert(*input.output_id());
            needs_commitment_context = true;
        }

        // BlockIssuanceCreditContextInput requires a CommitmentContextInput.
        if !self.bic_context_inputs.is_empty() {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            log::debug!("Adding commitment context input for output with block issuance credit context input");
            needs_commitment_context = true;
        }

        if needs_commitment_context {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            self.commitment_context_input
                .replace(CommitmentContextInput::new(self.latest_slot_commitment_id));
        }
    }

    pub(crate) fn fulfill_output_context_inputs_requirements(&mut self) -> Result<(), TransactionBuilderError> {
        let mut needs_commitment_context = false;

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

        if needs_commitment_context && self.commitment_context_input.is_none() {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            self.commitment_context_input
                .replace(CommitmentContextInput::new(self.latest_slot_commitment_id));
        }
        Ok(())
    }
}
