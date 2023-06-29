// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

pub(crate) mod burn;
pub(crate) mod error;
pub(crate) mod helpers;
pub(crate) mod remainder;
pub(crate) mod requirement;
pub(crate) mod transition;

use core::ops::Deref;
use std::collections::{HashMap, HashSet};

use packable::PackableExt;
pub(crate) use requirement::is_alias_transition;

pub use self::{
    burn::{Burn, BurnDto},
    error::Error,
    helpers::minimum_storage_deposit_basic_output,
    requirement::Requirement,
};
use crate::{
    client::{api::types::RemainderData, secret::types::InputSigningData},
    types::block::{
        address::{Address, AliasAddress, NftAddress},
        input::INPUT_COUNT_RANGE,
        output::{
            AliasOutput, AliasTransition, ChainId, FoundryOutput, NativeTokensBuilder, NftOutput, Output, OutputId,
            OUTPUT_COUNT_RANGE,
        },
        protocol::ProtocolParameters,
    },
    utils::unix_timestamp_now,
};

/// Working state for the input selection algorithm.
pub struct InputSelection {
    available_inputs: Vec<InputSigningData>,
    required_inputs: HashSet<OutputId>,
    forbidden_inputs: HashSet<OutputId>,
    selected_inputs: Vec<InputSigningData>,
    outputs: Vec<Output>,
    addresses: HashSet<Address>,
    burn: Option<Burn>,
    remainder_address: Option<Address>,
    protocol_parameters: ProtocolParameters,
    timestamp: u32,
    requirements: Vec<Requirement>,
    automatically_transitioned: HashMap<ChainId, Option<AliasTransition>>,
}

/// Result of the input selection algorithm.
#[derive(Clone, Debug)]
pub struct Selected {
    /// Selected inputs.
    pub inputs: Vec<InputSigningData>,
    /// Provided and created outputs.
    pub outputs: Vec<Output>,
    /// Remainder, if there was one.
    pub remainder: Option<RemainderData>,
}

impl InputSelection {
    fn required_alias_nft_addresses(&self, input: &InputSigningData) -> Result<Option<Requirement>, Error> {
        let alias_transition =
            is_alias_transition(&input.output, *input.output_id(), &self.outputs, self.burn.as_ref());
        let required_address = input
            .output
            .required_and_unlocked_address(self.timestamp, input.output_id(), alias_transition)?
            .0;

        match required_address {
            Address::Ed25519(_) => {
                if alias_transition.is_some() {
                    // Only add the requirement if the output is an alias because other types of output have been
                    // filtered by address already.
                    Ok(Some(Requirement::Ed25519(required_address)))
                } else {
                    Ok(None)
                }
            }
            Address::Alias(alias_address) => Ok(Some(Requirement::Alias(
                *alias_address.alias_id(),
                AliasTransition::State,
            ))),
            Address::Nft(nft_address) => Ok(Some(Requirement::Nft(*nft_address.nft_id()))),
        }
    }

    fn select_input(
        &mut self,
        input: InputSigningData,
        alias_transition: Option<AliasTransition>,
    ) -> Result<(), Error> {
        log::debug!("Selecting input {:?}", input.output_id());

        if let Some(output) = self.transition_input(&input, alias_transition)? {
            // No need to check for `outputs_requirements` because
            // - the sender feature doesn't need to be verified as it has been removed
            // - the issuer feature doesn't need to be verified as the chain is not new
            // - input doesn't need to be checked for as we just transitioned it
            // - foundry alias requirement should have been met already by a prior `required_alias_nft_addresses`
            self.outputs.push(output);
        }

        if let Some(requirement) = self.required_alias_nft_addresses(&input)? {
            log::debug!("Adding {requirement:?} from input {:?}", input.output_id());
            self.requirements.push(requirement);
        }

        self.selected_inputs.push(input);

        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        // Adds an initial amount requirement.
        self.requirements.push(Requirement::Amount);
        // Adds an initial native tokens requirement.
        self.requirements.push(Requirement::NativeTokens);

        // Removes forbidden inputs from available inputs.
        self.available_inputs
            .retain(|input| !self.forbidden_inputs.contains(input.output_id()));

        // This is to avoid a borrow of self since there is a mutable borrow in the loop already.
        let required_inputs = std::mem::take(&mut self.required_inputs);

        for required_input in required_inputs {
            // Checks that required input is not forbidden.
            if self.forbidden_inputs.contains(&required_input) {
                return Err(Error::RequiredInputIsForbidden(required_input));
            }

            // Checks that required input is available.
            match self
                .available_inputs
                .iter()
                .position(|input| input.output_id() == &required_input)
            {
                Some(index) => {
                    // Removes required input from available inputs.
                    let input = self.available_inputs.swap_remove(index);

                    // Selects required input.
                    self.select_input(input, None)?
                }
                None => return Err(Error::RequiredInputIsNotAvailable(required_input)),
            }
        }

        // Gets requirements from outputs.
        // TODO this may re-evaluate outputs added by inputs
        self.outputs_requirements();

        // Gets requirements from burn.
        self.burn_requirements()?;

        Ok(())
    }

    /// Creates a new [`InputSelection`].
    pub fn new(
        available_inputs: impl Into<Vec<InputSigningData>>,
        outputs: impl Into<Vec<Output>>,
        addresses: impl IntoIterator<Item = Address>,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        let available_inputs = available_inputs.into();
        let mut addresses = HashSet::from_iter(addresses);

        addresses.extend(available_inputs.iter().filter_map(|input| match &input.output {
            Output::Alias(output) => Some(Address::Alias(AliasAddress::from(
                output.alias_id_non_null(input.output_id()),
            ))),
            Output::Nft(output) => Some(Address::Nft(NftAddress::from(
                output.nft_id_non_null(input.output_id()),
            ))),
            _ => None,
        }));

        Self {
            available_inputs,
            required_inputs: HashSet::new(),
            forbidden_inputs: HashSet::new(),
            selected_inputs: Vec::new(),
            outputs: outputs.into(),
            addresses,
            burn: None,
            remainder_address: None,
            protocol_parameters,
            timestamp: unix_timestamp_now().as_secs() as u32,
            requirements: Vec::new(),
            automatically_transitioned: HashMap::new(),
        }
    }

    /// Sets the required inputs of an [`InputSelection`].
    pub fn required_inputs(mut self, inputs: impl Into<HashSet<OutputId>>) -> Self {
        self.required_inputs = inputs.into();
        self
    }

    /// Sets the forbidden inputs of an [`InputSelection`].
    pub fn forbidden_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.forbidden_inputs = inputs;
        self
    }

    /// Sets the burn of an [`InputSelection`].
    pub fn burn(mut self, burn: impl Into<Option<Burn>>) -> Self {
        self.burn = burn.into();
        self
    }

    /// Sets the remainder address of an [`InputSelection`].
    pub fn remainder_address(mut self, address: impl Into<Option<Address>>) -> Self {
        self.remainder_address = address.into();
        self
    }

    /// Sets the timestamp of an [`InputSelection`].
    pub fn timestamp(mut self, timestamp: u32) -> Self {
        self.timestamp = timestamp;
        self
    }

    fn filter_inputs(&mut self) {
        self.available_inputs.retain(|input| {
            // Keep alias outputs because at this point we do not know if a state or governor address will be required.
            if input.output.is_alias() {
                return true;
            }
            // Filter out non basic/foundry/nft outputs.
            else if !input.output.is_basic() && !input.output.is_foundry() && !input.output.is_nft() {
                return false;
            }

            // PANIC: safe to unwrap as non basic/alias/foundry/nft outputs are already filtered out.
            let unlock_conditions = input.output.unlock_conditions().unwrap();

            if unlock_conditions.is_time_locked(self.timestamp) {
                return false;
            }

            let required_address = input
                .output
                // Alias transition is irrelevant here as we keep aliases anyway.
                .required_and_unlocked_address(self.timestamp, input.output_id(), None)
                // PANIC: safe to unwrap as non basic/alias/foundry/nft outputs are already filtered out.
                .unwrap()
                .0;

            self.addresses.contains(&required_address)
        })
    }

    // Inputs need to be sorted before signing, because the reference unlock conditions can only reference a lower index
    pub(crate) fn sort_input_signing_data(
        mut inputs: Vec<InputSigningData>,
        outputs: &[Output],
        time: Option<u32>,
    ) -> Result<Vec<InputSigningData>, Error> {
        let time = time.unwrap_or_else(|| unix_timestamp_now().as_secs() as u32);
        // initially sort by output to make it deterministic
        // TODO: rethink this, we only need it deterministic for tests, for the protocol it doesn't matter, also there
        // might be a more efficient way to do this
        inputs.sort_by_key(|i| i.output.pack_to_vec());
        // filter for ed25519 address first
        let (mut sorted_inputs, alias_nft_address_inputs): (Vec<InputSigningData>, Vec<InputSigningData>) =
            inputs.into_iter().partition(|input_signing_data| {
                let alias_transition = is_alias_transition(
                    &input_signing_data.output,
                    *input_signing_data.output_id(),
                    outputs,
                    None,
                );
                let (input_address, _) = input_signing_data
                    .output
                    .required_and_unlocked_address(time, input_signing_data.output_id(), alias_transition)
                    // PANIC: safe to unwrap, because we filtered irrelevant outputs out before
                    .unwrap();

                input_address.is_ed25519()
            });

        for input in alias_nft_address_inputs {
            let alias_transition = is_alias_transition(&input.output, *input.output_id(), outputs, None);
            let (input_address, _) =
                input
                    .output
                    .required_and_unlocked_address(time, input.output_id(), alias_transition)?;

            match sorted_inputs.iter().position(|input_signing_data| match input_address {
                Address::Alias(unlock_address) => {
                    if let Output::Alias(alias_output) = &input_signing_data.output {
                        *unlock_address.alias_id() == alias_output.alias_id_non_null(input_signing_data.output_id())
                    } else {
                        false
                    }
                }
                Address::Nft(unlock_address) => {
                    if let Output::Nft(nft_output) = &input_signing_data.output {
                        *unlock_address.nft_id() == nft_output.nft_id_non_null(input_signing_data.output_id())
                    } else {
                        false
                    }
                }
                _ => false,
            }) {
                Some(position) => {
                    // Insert after the output we need
                    sorted_inputs.insert(position + 1, input);
                }
                None => {
                    // insert before address
                    let alias_or_nft_address = match &input.output {
                        Output::Alias(alias_output) => Some(Address::Alias(AliasAddress::new(
                            alias_output.alias_id_non_null(input.output_id()),
                        ))),
                        Output::Nft(nft_output) => Some(Address::Nft(NftAddress::new(
                            nft_output.nft_id_non_null(input.output_id()),
                        ))),
                        _ => None,
                    };

                    if let Some(alias_or_nft_address) = alias_or_nft_address {
                        // Check for existing outputs for this address, and insert before
                        match sorted_inputs.iter().position(|input_signing_data| {
                            let alias_transition = is_alias_transition(
                                &input_signing_data.output,
                                *input_signing_data.output_id(),
                                outputs,
                                None,
                            );
                            let (input_address, _) = input_signing_data
                                .output
                                .required_and_unlocked_address(time, input.output_id(), alias_transition)
                                // PANIC: safe to unwrap, because we filtered irrelevant outputs out before
                                .unwrap();

                            input_address == alias_or_nft_address
                        }) {
                            Some(position) => {
                                // Insert before the output with this address required for unlocking
                                sorted_inputs.insert(position, input);
                            }
                            // just push output
                            None => sorted_inputs.push(input),
                        }
                    } else {
                        // just push basic or foundry output
                        sorted_inputs.push(input);
                    }
                }
            }
        }

        Ok(sorted_inputs)
    }

    /// Selects inputs that meet the requirements of the outputs to satisfy the semantic validation of the overall
    /// transaction. Also creates a remainder output and chain transition outputs if required.
    pub fn select(mut self) -> Result<Selected, Error> {
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            // If burn is provided, outputs will be added later
            if !(self.outputs.is_empty() && self.burn.is_some()) {
                return Err(Error::InvalidOutputCount(self.outputs.len()));
            }
        }

        self.filter_inputs();

        if self.available_inputs.is_empty() {
            return Err(Error::NoAvailableInputsProvided);
        }

        // Creates the initial state, selected inputs and requirements, based on the provided outputs.
        self.init()?;

        // Process all the requirements until there are no more.
        while let Some(requirement) = self.requirements.pop() {
            // Fulfill the requirement.
            let inputs = self.fulfill_requirement(requirement)?;

            // Select suggested inputs.
            for (input, alias_transition) in inputs {
                self.select_input(input, alias_transition)?;
            }
        }

        if !INPUT_COUNT_RANGE.contains(&(self.selected_inputs.len() as u16)) {
            return Err(Error::InvalidInputCount(self.selected_inputs.len()));
        }

        let (remainder, storage_deposit_returns) = self.remainder_and_storage_deposit_return_outputs()?;

        if let Some(remainder) = &remainder {
            self.outputs.push(remainder.output.clone());
        }

        self.outputs.extend(storage_deposit_returns);

        // Check again, because more outputs may have been added.
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(Error::InvalidOutputCount(self.outputs.len()));
        }

        self.validate_transitions()?;

        Ok(Selected {
            inputs: Self::sort_input_signing_data(self.selected_inputs, &self.outputs, Some(self.timestamp))?,
            outputs: self.outputs,
            remainder,
        })
    }

    fn validate_transitions(&self) -> Result<(), Error> {
        let mut input_native_tokens_builder = NativeTokensBuilder::new();
        let mut output_native_tokens_builder = NativeTokensBuilder::new();
        let mut input_aliases = Vec::new();
        let mut input_chains_foundries = hashbrown::HashMap::new();
        let mut input_foundries = Vec::new();
        let mut input_nfts = Vec::new();
        for input in &self.selected_inputs {
            if let Some(native_tokens) = input.output.native_tokens() {
                input_native_tokens_builder.add_native_tokens(native_tokens.clone())?;
            }
            match &input.output {
                Output::Alias(_) => {
                    input_aliases.push(input);
                }
                Output::Foundry(foundry) => {
                    input_chains_foundries.insert(foundry.chain_id(), &input.output);
                    input_foundries.push(input);
                }
                Output::Nft(_) => {
                    input_nfts.push(input);
                }
                _ => {}
            }
        }

        for output in self.outputs.iter() {
            if let Some(native_token) = output.native_tokens() {
                output_native_tokens_builder.add_native_tokens(native_token.clone())?;
            }
        }

        // Validate utxo chain transitions
        for output in self.outputs.iter() {
            match output {
                Output::Alias(alias_output) => {
                    // Null id outputs are just minted and can't be a transition
                    if alias_output.alias_id().is_null() {
                        continue;
                    }

                    let alias_input = input_aliases
                        .iter()
                        .find(|i| {
                            if let Output::Alias(alias_input) = &i.output {
                                *alias_output.alias_id() == alias_input.alias_id_non_null(i.output_id())
                            } else {
                                false
                            }
                        })
                        .expect("ISA is broken because there is no alias input");

                    if let Err(err) = AliasOutput::transition_inner(
                        alias_input.output.as_alias(),
                        alias_output,
                        &input_chains_foundries,
                        &self.outputs,
                    ) {
                        log::debug!("validate_transitions error {err:?}");
                        let alias_transition =
                            if alias_input.output.as_alias().state_index() == alias_output.state_index() {
                                AliasTransition::Governance
                            } else {
                                AliasTransition::State
                            };
                        return Err(Error::UnfulfillableRequirement(Requirement::Alias(
                            *alias_output.alias_id(),
                            alias_transition,
                        )));
                    }
                }
                Output::Foundry(foundry_output) => {
                    let foundry_input = input_foundries.iter().find(|i| {
                        if let Output::Foundry(foundry_input) = &i.output {
                            foundry_output.id() == foundry_input.id()
                        } else {
                            false
                        }
                    });
                    if let Some(foundry_input) = foundry_input {
                        if let Err(err) = FoundryOutput::transition_inner(
                            foundry_input.output.as_foundry(),
                            foundry_output,
                            input_native_tokens_builder.deref(),
                            output_native_tokens_builder.deref(),
                        ) {
                            log::debug!("validate_transitions error {err:?}");
                            return Err(Error::UnfulfillableRequirement(Requirement::Foundry(
                                foundry_output.id(),
                            )));
                        }
                    }
                }
                Output::Nft(nft_output) => {
                    // Null id outputs are just minted and can't be a transition
                    if nft_output.nft_id().is_null() {
                        continue;
                    }

                    let nft_input = input_nfts
                        .iter()
                        .find(|i| {
                            if let Output::Nft(nft_input) = &i.output {
                                *nft_output.nft_id() == nft_input.nft_id_non_null(i.output_id())
                            } else {
                                false
                            }
                        })
                        .expect("ISA is broken because there is no nft input");

                    if let Err(err) = NftOutput::transition_inner(nft_input.output.as_nft(), nft_output) {
                        log::debug!("validate_transitions error {err:?}");
                        return Err(Error::UnfulfillableRequirement(Requirement::Nft(*nft_output.nft_id())));
                    }
                }
                // other output types don't do transitions
                _ => {}
            }
        }
        Ok(())
    }
}
