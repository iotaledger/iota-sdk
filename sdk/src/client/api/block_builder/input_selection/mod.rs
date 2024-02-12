// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

pub(crate) mod burn;
pub(crate) mod error;
pub(crate) mod remainder;
pub(crate) mod requirement;
pub(crate) mod transition;

use core::ops::Deref;
use std::collections::{HashMap, HashSet};

use packable::PackableExt;

use self::requirement::account::is_account_with_id;
pub use self::{burn::Burn, error::Error, requirement::Requirement};
use crate::{
    client::{api::types::RemainderData, secret::types::InputSigningData},
    types::block::{
        address::{AccountAddress, Address, NftAddress},
        input::INPUT_COUNT_RANGE,
        mana::ManaAllotment,
        output::{
            AccountOutput, ChainId, FoundryOutput, NativeTokensBuilder, NftOutput, Output, OutputId, OUTPUT_COUNT_RANGE,
        },
        payload::signed_transaction::TransactionCapabilities,
        protocol::{CommittableAgeRange, ProtocolParameters},
        slot::SlotIndex,
    },
};

/// Working state for the input selection algorithm.
#[derive(Debug)]
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
    slot_index: SlotIndex,
    requirements: Vec<Requirement>,
    automatically_transitioned: HashSet<ChainId>,
    mana_allotments: u64,
    mana_rewards: HashMap<OutputId, u64>,
}

/// Result of the input selection algorithm.
#[derive(Clone, Debug)]
pub struct Selected {
    /// Selected inputs.
    pub inputs: Vec<InputSigningData>,
    /// Provided and created outputs.
    pub outputs: Vec<Output>,
    /// Remainder outputs information.
    pub remainders: Vec<RemainderData>,
    /// Mana rewards by input.
    pub mana_rewards: HashMap<OutputId, u64>,
}

impl InputSelection {
    fn required_account_nft_addresses(&self, input: &InputSigningData) -> Result<Option<Requirement>, Error> {
        let required_address = input
            .output
            .required_address(self.slot_index, self.protocol_parameters.committable_age_range())?
            .expect("expiration unlockable outputs already filtered out");

        let required_address = if let Address::Restricted(restricted) = &required_address {
            restricted.address()
        } else {
            &required_address
        };

        match required_address {
            Address::Account(account_address) => Ok(Some(Requirement::Account(*account_address.account_id()))),
            Address::Nft(nft_address) => Ok(Some(Requirement::Nft(*nft_address.nft_id()))),
            _ => Ok(None),
        }
    }

    fn select_input(&mut self, input: InputSigningData) -> Result<(), Error> {
        log::debug!("Selecting input {:?}", input.output_id());

        if let Some(output) = self.transition_input(&input)? {
            // No need to check for `outputs_requirements` because
            // - the sender feature doesn't need to be verified as it has been removed
            // - the issuer feature doesn't need to be verified as the chain is not new
            // - input doesn't need to be checked for as we just transitioned it
            // - foundry account requirement should have been met already by a prior `required_account_nft_addresses`
            self.outputs.push(output);
        }

        if let Some(requirement) = self.required_account_nft_addresses(&input)? {
            log::debug!("Adding {requirement:?} from input {:?}", input.output_id());
            self.requirements.push(requirement);
        }

        self.selected_inputs.push(input);

        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        // Adds an initial mana requirement.
        self.requirements.push(Requirement::Mana);
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
                    self.select_input(input)?
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
        slot_index: impl Into<SlotIndex>,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        let available_inputs = available_inputs.into();

        let mut addresses = HashSet::from_iter(addresses.into_iter().map(|a| {
            // Get a potential Ed25519 address directly since we're only interested in that
            #[allow(clippy::option_if_let_else)] // clippy's suggestion requires a clone
            if let Some(address) = a.backing_ed25519() {
                Address::Ed25519(*address)
            } else {
                a
            }
        }));

        addresses.extend(available_inputs.iter().filter_map(|input| match &input.output {
            Output::Account(output) => Some(Address::Account(AccountAddress::from(
                output.account_id_non_null(input.output_id()),
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
            // Should be set from a commitment context input
            slot_index: slot_index.into(),
            requirements: Vec::new(),
            automatically_transitioned: HashSet::new(),
            mana_allotments: 0,
            mana_rewards: Default::default(),
        }
    }

    /// Sets the required inputs of an [`InputSelection`].
    pub fn with_required_inputs(mut self, inputs: impl Into<HashSet<OutputId>>) -> Self {
        self.required_inputs = inputs.into();
        self
    }

    /// Sets the forbidden inputs of an [`InputSelection`].
    pub fn with_forbidden_inputs(mut self, inputs: HashSet<OutputId>) -> Self {
        self.forbidden_inputs = inputs;
        self
    }

    /// Sets the burn of an [`InputSelection`].
    pub fn with_burn(mut self, burn: impl Into<Option<Burn>>) -> Self {
        self.burn = burn.into();
        self
    }

    /// Sets the remainder address of an [`InputSelection`].
    pub fn with_remainder_address(mut self, address: impl Into<Option<Address>>) -> Self {
        self.remainder_address = address.into();
        self
    }

    /// Sets the mana allotments sum of an [`InputSelection`].
    pub fn with_mana_allotments<'a>(mut self, mana_allotments: impl Iterator<Item = &'a ManaAllotment>) -> Self {
        self.mana_allotments = mana_allotments.map(ManaAllotment::mana).sum();
        self
    }

    /// Sets the total mana rewards for required inputs.
    pub fn with_mana_rewards(mut self, mana_rewards: HashMap<OutputId, u64>) -> Self {
        self.mana_rewards = mana_rewards;
        self
    }

    /// Sets the mana rewards for the given input.
    pub fn add_mana_rewards(mut self, input: OutputId, mana_rewards: u64) -> Self {
        self.mana_rewards.insert(input, mana_rewards);
        self
    }

    fn filter_inputs(&mut self) {
        self.available_inputs.retain(|input| {
            // TODO what about other kinds?
            // Filter out non basic/account/foundry/nft outputs.
            if !(input.output.is_basic()
                || input.output.is_account()
                || input.output.is_foundry()
                || input.output.is_nft())
            {
                // Keep burned outputs
                if let Some(burn) = &self.burn {
                    if let Some(delegation) = input.output.as_delegation_opt() {
                        return burn
                            .delegations()
                            .contains(&delegation.delegation_id_non_null(input.output_id()));
                    }
                }
                return false;
            }

            // PANIC: safe to unwrap as non basic/account/foundry/nft outputs are already filtered out.
            let unlock_conditions = input.output.unlock_conditions().unwrap();

            if unlock_conditions.is_timelocked(self.slot_index, self.protocol_parameters.min_committable_age()) {
                return false;
            }

            let required_address = input
                .output
                // Account transition is irrelevant here as we keep accounts anyway.
                .required_address(self.slot_index, self.protocol_parameters.committable_age_range())
                // PANIC: safe to unwrap as non basic/account/foundry/nft outputs are already filtered out.
                .unwrap();

            let required_address = match &required_address {
                Some(address) => {
                    if let Address::Restricted(restricted) = address {
                        restricted.address()
                    } else {
                        address
                    }
                }
                // Time in which no address can unlock the output because of an expiration unlock condition
                None => return false,
            };

            match required_address {
                Address::Anchor(_) => false,
                Address::ImplicitAccountCreation(implicit_account_creation) => {
                    self.required_inputs.contains(input.output_id())
                        && self
                            .addresses
                            .contains(&Address::from(*implicit_account_creation.ed25519_address()))
                }
                _ => self.addresses.contains(required_address),
            }
        })
    }

    // Inputs need to be sorted before signing, because the reference unlock conditions can only reference a lower index
    pub(crate) fn sort_input_signing_data(
        mut inputs: Vec<InputSigningData>,
        slot_index: SlotIndex,
        committable_age_range: CommittableAgeRange,
    ) -> Result<Vec<InputSigningData>, Error> {
        // initially sort by output to make it deterministic
        // TODO: rethink this, we only need it deterministic for tests, for the protocol it doesn't matter, also there
        // might be a more efficient way to do this
        inputs.sort_by_key(|i| i.output.pack_to_vec());
        // filter for ed25519 address first
        let (mut sorted_inputs, account_nft_address_inputs): (Vec<InputSigningData>, Vec<InputSigningData>) =
            inputs.into_iter().partition(|input_signing_data| {
                let required_address = input_signing_data
                    .output
                    .required_address(slot_index, committable_age_range)
                    // PANIC: safe to unwrap as non basic/account/foundry/nft outputs are already filtered out.
                    .unwrap()
                    .expect("expiration unlockable outputs already filtered out");

                required_address.is_ed25519()
            });

        for input in account_nft_address_inputs {
            let required_address = input
                .output
                .required_address(slot_index, committable_age_range)?
                .expect("expiration unlockable outputs already filtered out");

            match sorted_inputs
                .iter()
                .position(|input_signing_data| match required_address {
                    Address::Account(unlock_address) => {
                        if let Output::Account(account_output) = &input_signing_data.output {
                            *unlock_address.account_id()
                                == account_output.account_id_non_null(input_signing_data.output_id())
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
                    let account_or_nft_address = match &input.output {
                        Output::Account(account_output) => Some(Address::Account(AccountAddress::new(
                            account_output.account_id_non_null(input.output_id()),
                        ))),
                        Output::Nft(nft_output) => Some(Address::Nft(NftAddress::new(
                            nft_output.nft_id_non_null(input.output_id()),
                        ))),
                        _ => None,
                    };

                    if let Some(account_or_nft_address) = account_or_nft_address {
                        // Check for existing outputs for this address, and insert before
                        match sorted_inputs.iter().position(|input_signing_data| {
                            let required_address = input_signing_data
                                .output
                                .required_address(slot_index, committable_age_range)
                                // PANIC: safe to unwrap as non basic/alias/foundry/nft outputs are already filtered
                                .unwrap()
                                .expect("expiration unlockable outputs already filtered out");

                            required_address == account_or_nft_address
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
            // If burn or mana allotments are provided, outputs will be added later, in the other cases it will just
            // create remainder outputs.
            if !(self.outputs.is_empty()
                && (self.burn.is_some() || self.mana_allotments != 0 || !self.required_inputs.is_empty()))
            {
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
            for input in inputs {
                self.select_input(input)?;
            }
        }

        if !INPUT_COUNT_RANGE.contains(&(self.selected_inputs.len() as u16)) {
            return Err(Error::InvalidInputCount(self.selected_inputs.len()));
        }

        let (storage_deposit_returns, remainders) = self.storage_deposit_returns_and_remainders()?;

        self.outputs.extend(storage_deposit_returns);
        self.outputs.extend(remainders.iter().map(|r| r.output.clone()));

        // Check again, because more outputs may have been added.
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(Error::InvalidOutputCount(self.outputs.len()));
        }

        self.validate_transitions()?;

        for output_id in self.mana_rewards.keys() {
            if !self.selected_inputs.iter().any(|i| output_id == i.output_id()) {
                return Err(Error::ExtraManaRewards(*output_id));
            }
        }

        Ok(Selected {
            inputs: Self::sort_input_signing_data(
                self.selected_inputs,
                self.slot_index,
                self.protocol_parameters.committable_age_range(),
            )?,
            outputs: self.outputs,
            remainders,
            mana_rewards: self.mana_rewards,
        })
    }

    fn validate_transitions(&self) -> Result<(), Error> {
        let mut input_native_tokens_builder = NativeTokensBuilder::new();
        let mut output_native_tokens_builder = NativeTokensBuilder::new();
        let mut input_accounts = Vec::new();
        let mut input_chains_foundries = hashbrown::HashMap::new();
        let mut input_foundries = Vec::new();
        let mut input_nfts = Vec::new();

        for input in &self.selected_inputs {
            if let Some(native_token) = input.output.native_token() {
                input_native_tokens_builder.add_native_token(*native_token)?;
            }
            match &input.output {
                Output::Basic(basic) => {
                    if basic.is_implicit_account() {
                        input_accounts.push(input);
                    }
                }
                Output::Account(_) => {
                    input_accounts.push(input);
                }
                Output::Foundry(foundry) => {
                    input_chains_foundries.insert(foundry.chain_id(), (input.output_id(), &input.output));
                    input_foundries.push(input);
                }
                Output::Nft(_) => {
                    input_nfts.push(input);
                }
                _ => {}
            }
        }

        for output in self.outputs.iter() {
            if let Some(native_token) = output.native_token() {
                output_native_tokens_builder.add_native_token(*native_token)?;
            }
        }

        // Validate utxo chain transitions
        for output in self.outputs.iter() {
            match output {
                Output::Account(account_output) => {
                    // Null id outputs are just minted and can't be a transition
                    if account_output.account_id().is_null() {
                        continue;
                    }

                    let account_input = input_accounts
                        .iter()
                        .find(|i| is_account_with_id(&i.output, account_output.account_id(), i.output_id()))
                        .expect("ISA is broken because there is no account input");

                    match &account_input.output {
                        Output::Account(account) => {
                            if let Err(err) = AccountOutput::transition_inner(
                                account,
                                account_output,
                                &input_chains_foundries,
                                &self.outputs,
                            ) {
                                log::debug!("validate_transitions error {err:?}");
                                return Err(Error::UnfulfillableRequirement(Requirement::Account(
                                    *account_output.account_id(),
                                )));
                            }
                        }
                        Output::Basic(_) => {
                            // TODO https://github.com/iotaledger/iota-sdk/issues/1664
                        }
                        _ => panic!(
                            "unreachable: \"input_accounts\" only contains account outputs and implicit account (basic) outputs"
                        ),
                    }
                }
                Output::Foundry(foundry_output) => {
                    let foundry_id = foundry_output.id();
                    let foundry_input = input_foundries.iter().find(|i| {
                        if let Output::Foundry(foundry_input) = &i.output {
                            foundry_id == foundry_input.id()
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
                            // We use `all` capabilities here because this transition may be burning
                            // native tokens, and validation will fail without the capability.
                            &TransactionCapabilities::all(),
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
