// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{account::is_account_with_id_non_null, foundry::is_foundry_with_id, nft::is_nft_with_id_non_null},
    TransactionBuilder, TransactionBuilderError,
};
use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        context_input::CommitmentContextInput,
        output::{
            AccountOutput, AccountOutputBuilder, FoundryOutput, FoundryOutputBuilder, NftOutput, NftOutputBuilder,
            Output, OutputId,
        },
        payload::signed_transaction::TransactionCapabilityFlag,
    },
};

impl TransactionBuilder {
    /// Transitions an account input by creating a new account output if required.
    fn transition_account_input(
        &mut self,
        input: &AccountOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        let account_id = input.account_id_non_null(output_id);

        // Do not create an account output if the account input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.accounts.contains(&account_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{account_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create an account output if it already exists.
        if self
            .non_remainder_outputs()
            .any(|output| is_account_with_id_non_null(output, &account_id))
        {
            log::debug!("No transition of {output_id:?}/{account_id:?} as output already exists");
            return Ok(None);
        }

        let mut highest_foundry_serial_number = 0;
        for output in self.non_remainder_outputs() {
            if let Output::Foundry(foundry) = output {
                if *foundry.account_address().account_id() == account_id {
                    highest_foundry_serial_number = u32::max(highest_foundry_serial_number, foundry.serial_number());
                }
            }
        }

        // Remove potential sender feature because it will not be needed anymore as it only needs to be verified once.
        let features = input.features().iter().filter(|feature| !feature.is_sender()).cloned();

        let mut builder = AccountOutputBuilder::from(input)
            .with_amount_or_minimum(input.amount(), self.protocol_parameters.storage_score_parameters())
            .with_account_id(account_id)
            .with_foundry_counter(u32::max(highest_foundry_serial_number, input.foundry_counter()))
            .with_features(features);

        if input.is_block_issuer() {
            if self.commitment_context_input.is_none() {
                // TODO https://github.com/iotaledger/iota-sdk/issues/1740
                self.commitment_context_input
                    .replace(CommitmentContextInput::new(self.latest_slot_commitment_id));
            }

            if !self.burn.as_ref().map_or(false, |b| b.generated_mana()) {
                builder = builder.with_mana(input.available_mana(
                    &self.protocol_parameters,
                    output_id.transaction_id().slot_index(),
                    self.creation_slot,
                )?)
            } else {
                self.transaction_capabilities
                    .add_capability(TransactionCapabilityFlag::BurnMana);
            }
        }

        let output = builder.finish_output()?;

        log::debug!("Automatic transition of {output_id:?}/{account_id:?}");

        Ok(Some(output))
    }

    /// Transitions an nft input by creating a new nft output if required.
    fn transition_nft_input(
        &mut self,
        input: &NftOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        let nft_id = input.nft_id_non_null(output_id);

        // Do not create an nft output if the nft input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.nfts.contains(&nft_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{nft_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create an nft output if it already exists.
        if self
            .non_remainder_outputs()
            .any(|output| is_nft_with_id_non_null(output, &nft_id))
        {
            log::debug!("No transition of {output_id:?}/{nft_id:?} as output already exists");
            return Ok(None);
        }

        // Remove potential sender feature because it will not be needed anymore as it only needs to be verified once.
        let features = input.features().iter().filter(|feature| !feature.is_sender()).cloned();

        let output = NftOutputBuilder::from(input)
            .with_amount_or_minimum(input.amount(), self.protocol_parameters.storage_score_parameters())
            .with_nft_id(nft_id)
            .with_features(features)
            .finish_output()?;

        log::debug!("Automatic transition of {output_id:?}/{nft_id:?}");

        Ok(Some(output))
    }

    /// Transitions a foundry input by creating a new foundry output if required.
    fn transition_foundry_input(
        &mut self,
        input: &FoundryOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        let foundry_id = input.id();

        // Do not create a foundry output if the foundry input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.foundries.contains(&foundry_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{foundry_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create a foundry output if it already exists.
        if self
            .non_remainder_outputs()
            .any(|output| is_foundry_with_id(output, &foundry_id))
        {
            log::debug!("No transition of {output_id:?}/{foundry_id:?} as output already exists");
            return Ok(None);
        }

        let output = FoundryOutputBuilder::from(input)
            .with_amount_or_minimum(input.amount(), self.protocol_parameters.storage_score_parameters())
            .finish_output()?;

        log::debug!("Automatic transition of {output_id:?}/{foundry_id:?}");

        Ok(Some(output))
    }

    /// Transitions an input by creating a new output if required.
    /// If no `account_transition` is provided, assumes a state transition.
    pub(crate) fn transition_input(
        &mut self,
        input: &InputSigningData,
    ) -> Result<Option<Output>, TransactionBuilderError> {
        match &input.output {
            Output::Account(account_input) => self.transition_account_input(account_input, input.output_id()),
            Output::Foundry(foundry_input) => self.transition_foundry_input(foundry_input, input.output_id()),
            Output::Nft(nft_input) => self.transition_nft_input(nft_input, input.output_id()),
            _ => Ok(None),
        }
    }
}
