// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Builder for transactions

pub(crate) mod burn;
pub(crate) mod context_inputs;
pub(crate) mod error;
pub(crate) mod remainder;
pub(crate) mod requirement;
pub(crate) mod transition;

use alloc::collections::{BTreeMap, VecDeque};
use core::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use crypto::keys::bip44::Bip44;

pub use self::{burn::Burn, error::TransactionBuilderError, requirement::Requirement, transition::Transitions};
use crate::{
    client::{
        api::{
            options::{RemainderValueStrategy, TransactionOptions},
            PreparedTransactionData, RemainderData,
        },
        node_api::indexer::query_parameters::OutputQueryParameters,
        secret::types::InputSigningData,
        Client, ClientError,
    },
    types::block::{
        address::{AccountAddress, Address, NftAddress, ToBech32Ext},
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput, ContextInput, RewardContextInput},
        input::{Input, UtxoInput, INPUT_COUNT_MAX, INPUT_COUNT_RANGE},
        mana::ManaAllotment,
        output::{
            AccountId, AccountOutputBuilder, BasicOutputBuilder, ChainId, FoundryOutputBuilder, NftOutputBuilder,
            Output, OutputId, OUTPUT_COUNT_RANGE,
        },
        payload::{
            signed_transaction::{Transaction, TransactionCapabilities, TransactionCapabilityFlag},
            TaggedDataPayload,
        },
        protocol::ProtocolParameters,
        slot::{SlotCommitmentId, SlotIndex},
    },
};

impl Client {
    /// Builds a transaction using the given inputs, outputs, addresses, and options.
    pub async fn build_transaction(
        &self,
        addresses: impl IntoIterator<Item = (Address, Bip44)> + Send,
        outputs: impl IntoIterator<Item = Output> + Send,
        options: TransactionOptions,
    ) -> Result<PreparedTransactionData, ClientError> {
        let addresses = addresses.into_iter().collect::<HashMap<_, _>>();
        let protocol_parameters = self.get_protocol_parameters().await?;
        let slot_commitment_id = self.get_issuance().await?.latest_commitment.id();

        let hrp = protocol_parameters.bech32_hrp();

        let mut available_inputs = Vec::new();
        for (address, chain) in &addresses {
            let output_ids = self
                .output_ids(OutputQueryParameters::new().unlockable_by_address(address.clone().to_bech32(hrp)))
                .await?
                .items;
            available_inputs.extend(
                self.get_outputs_with_metadata(&output_ids)
                    .await?
                    .into_iter()
                    .map(|res| {
                        Ok(InputSigningData {
                            output: res.output,
                            output_metadata: res.metadata,
                            chain: Some(*chain),
                        })
                    })
                    .collect::<Result<Vec<_>, ClientError>>()?,
            );
        }

        self.build_transaction_inner(
            addresses.into_keys(),
            available_inputs,
            outputs,
            options,
            slot_commitment_id,
            protocol_parameters,
        )
        .await
    }

    /// Builds a transaction using the given inputs, outputs, addresses, and options.
    pub(crate) async fn build_transaction_inner(
        &self,
        addresses: impl IntoIterator<Item = Address> + Send,
        available_inputs: impl IntoIterator<Item = InputSigningData> + Send,
        outputs: impl IntoIterator<Item = Output> + Send,
        options: TransactionOptions,
        slot_commitment_id: SlotCommitmentId,
        protocol_parameters: ProtocolParameters,
    ) -> Result<PreparedTransactionData, ClientError> {
        let outputs = outputs.into_iter().collect::<Vec<_>>();
        let creation_slot = self.get_slot_index().await?;

        let reference_mana_cost = if let Some(issuer_id) = options.issuer_id {
            Some(self.get_account_congestion(&issuer_id, None).await?.reference_mana_cost)
        } else {
            None
        };
        let remainder_address = match options.remainder_value_strategy {
            RemainderValueStrategy::ReuseAddress => None,
            RemainderValueStrategy::CustomAddress(address) => Some(address),
        };

        let mut mana_rewards = HashMap::new();

        if let Some(burn) = &options.burn {
            for delegation_id in burn.delegations() {
                let output_id = self.delegation_output_id(*delegation_id).await?;
                mana_rewards.insert(
                    output_id,
                    self.get_output_mana_rewards(&output_id, slot_commitment_id.slot_index())
                        .await?
                        .rewards,
                );
            }
        }

        for output_id in &options.required_inputs {
            let input = self.get_output(output_id).await?;
            if input.output.can_claim_rewards(outputs.iter().find(|o| {
                input
                    .output
                    .chain_id()
                    .map(|chain_id| chain_id.or_from_output_id(output_id))
                    == o.chain_id()
            })) {
                mana_rewards.insert(
                    *output_id,
                    self.get_output_mana_rewards(output_id, slot_commitment_id.slot_index())
                        .await?
                        .rewards,
                );
            }
        }

        let mut transaction_builder = TransactionBuilder::new(
            available_inputs,
            outputs,
            addresses,
            creation_slot,
            slot_commitment_id,
            protocol_parameters,
        )
        .with_required_inputs(options.required_inputs)
        .with_mana_rewards(mana_rewards)
        .with_payload(options.tagged_data_payload)
        .with_mana_allotments(options.mana_allotments)
        .with_remainder_address(remainder_address)
        .with_transitions(options.transitions)
        .with_issuer_id(options.issuer_id)
        .with_burn(options.burn);

        if let (Some(account_id), Some(reference_mana_cost)) = (options.issuer_id, reference_mana_cost) {
            transaction_builder = transaction_builder.with_min_mana_allotment(account_id, reference_mana_cost);
        }

        if !options.allow_additional_input_selection {
            transaction_builder = transaction_builder.disable_additional_input_selection();
        }

        Ok(transaction_builder.finish()?)
    }
}

/// Working state for the transaction builder algorithm.
#[derive(Debug)]
pub struct TransactionBuilder {
    available_inputs: Vec<InputSigningData>,
    required_inputs: HashSet<OutputId>,
    selected_inputs: OrderedInputs,
    bic_context_inputs: HashSet<BlockIssuanceCreditContextInput>,
    commitment_context_input: Option<CommitmentContextInput>,
    reward_context_inputs: HashSet<OutputId>,
    provided_outputs: Vec<Output>,
    added_outputs: Vec<Output>,
    addresses: HashSet<Address>,
    transitions: Option<Transitions>,
    burn: Option<Burn>,
    remainders: Remainders,
    creation_slot: SlotIndex,
    latest_slot_commitment_id: SlotCommitmentId,
    requirements: Vec<Requirement>,
    min_mana_allotment: Option<MinManaAllotment>,
    issuer_id: Option<AccountId>,
    mana_allotments: BTreeMap<AccountId, u64>,
    mana_rewards: HashMap<OutputId, u64>,
    payload: Option<TaggedDataPayload>,
    allow_additional_input_selection: bool,
    transaction_capabilities: TransactionCapabilities,
    protocol_parameters: ProtocolParameters,
}

/// Account and RMC for automatic mana allotment
#[derive(Copy, Clone, Debug)]
pub(crate) struct MinManaAllotment {
    issuer_id: AccountId,
    reference_mana_cost: u64,
    allotment_debt: u64,
    required_allotment: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Remainders {
    address: Option<Address>,
    data: Vec<RemainderData>,
    storage_deposit_returns: Vec<Output>,
    added_amount: HashMap<Option<ChainId>, u64>,
    added_mana: HashMap<Option<ChainId>, u64>,
}

impl TransactionBuilder {
    /// Creates a new [`TransactionBuilder`].
    pub fn new(
        available_inputs: impl IntoIterator<Item = InputSigningData>,
        outputs: impl IntoIterator<Item = Output>,
        addresses: impl IntoIterator<Item = Address>,
        creation_slot_index: impl Into<SlotIndex>,
        latest_slot_commitment_id: SlotCommitmentId,
        protocol_parameters: ProtocolParameters,
    ) -> Self {
        let available_inputs = available_inputs.into_iter().collect::<Vec<_>>();

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
            selected_inputs: Default::default(),
            bic_context_inputs: HashSet::new(),
            commitment_context_input: None,
            reward_context_inputs: HashSet::new(),
            provided_outputs: outputs.into_iter().collect(),
            added_outputs: Vec::new(),
            addresses,
            transitions: None,
            burn: None,
            remainders: Default::default(),
            creation_slot: creation_slot_index.into(),
            latest_slot_commitment_id,
            requirements: Vec::new(),
            min_mana_allotment: None,
            issuer_id: None,
            mana_allotments: Default::default(),
            mana_rewards: Default::default(),
            allow_additional_input_selection: true,
            transaction_capabilities: Default::default(),
            payload: None,
            protocol_parameters,
        }
    }

    fn init(&mut self) -> Result<(), TransactionBuilderError> {
        // If automatic min mana allotment is enabled, we need to initialize the allotment debt.
        if let Some(MinManaAllotment {
            issuer_id,
            allotment_debt,
            ..
        }) = self.min_mana_allotment.as_mut()
        {
            // Add initial debt from any passed-in allotments
            *allotment_debt = self.mana_allotments.get(issuer_id).copied().unwrap_or_default();
        }
        // Add initial requirements
        self.requirements
            .extend([Requirement::Mana, Requirement::Amount, Requirement::NativeTokens]);

        self.fulfill_output_context_inputs_requirements()?;

        for required_input in self.required_inputs.clone() {
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
                    self.select_input(input)?;
                }
                None => return Err(TransactionBuilderError::RequiredInputIsNotAvailable(required_input)),
            }
        }

        // Gets requirements from outputs.
        self.outputs_requirements();

        // Gets requirements from burn.
        self.burn_requirements()?;

        Ok(())
    }

    /// Selects inputs that meet the requirements of the outputs to satisfy the semantic validation of the overall
    /// transaction. Also creates a remainder output and chain transition outputs if required.
    pub fn finish(mut self) -> Result<PreparedTransactionData, TransactionBuilderError> {
        if !OUTPUT_COUNT_RANGE.contains(&(self.provided_outputs.len() as u16)) {
            // If burn or mana allotments are provided, outputs will be added later, in the other cases it will just
            // create remainder outputs.
            if !self.provided_outputs.is_empty()
                || (self.burn.is_none() && self.mana_allotments.is_empty() && self.required_inputs.is_empty())
            {
                return Err(TransactionBuilderError::InvalidOutputCount(self.provided_outputs.len()));
            }
        }

        self.filter_inputs();

        if self.available_inputs.is_empty() {
            return Err(TransactionBuilderError::NoAvailableInputsProvided);
        }

        // Creates the initial state, selected inputs and requirements, based on the provided outputs.
        self.init()?;

        // Process all the requirements until there are no more.
        while let Some(requirement) = self.requirements.pop() {
            // Fulfill the requirement.
            self.fulfill_requirement(&requirement)?;
        }

        let (input_mana, output_mana) = self.mana_sums(false)?;

        if input_mana < output_mana {
            let total_generation_amount = self
                .selected_inputs
                .iter()
                .map(|o| o.output.mana_generation_amount(&self.protocol_parameters))
                .sum::<u64>();
            let slots_remaining = self.protocol_parameters.slots_until_generated(
                self.creation_slot,
                total_generation_amount,
                self.total_selected_mana(false)?,
                output_mana - input_mana,
            )?;
            return Err(TransactionBuilderError::InsufficientMana {
                found: input_mana,
                required: output_mana,
                slots_remaining,
            });
        }

        // If there is no min allotment calculation, then we should update the remainders as the last step
        if self.min_mana_allotment.is_none() {
            self.update_remainders()?;
        }

        if !INPUT_COUNT_RANGE.contains(&(self.selected_inputs.len() as u16)) {
            return Err(TransactionBuilderError::InvalidInputCount(self.selected_inputs.len()));
        }

        let remainder_address = self
            .get_remainder_address()?
            .ok_or(TransactionBuilderError::MissingInputWithEd25519Address)?
            .0;

        let mut added_amount_mana = HashMap::<Option<ChainId>, (u64, u64)>::new();
        for (chain_id, added_amount) in self.remainders.added_amount.drain() {
            added_amount_mana.entry(chain_id).or_default().0 = added_amount;
        }
        for (chain_id, added_mana) in self.remainders.added_mana.drain() {
            added_amount_mana.entry(chain_id).or_default().1 = added_mana;
        }

        for (chain_id, (added_amount, added_mana)) in added_amount_mana {
            let mut output = self.get_output_for_remainder(chain_id, &remainder_address);
            if output.is_none() {
                output = self.get_output_for_remainder(None, &remainder_address);
            }
            if let Some(output) = output {
                log::debug!(
                    "Adding {added_amount} excess amount and {added_mana} excess mana to output with address {remainder_address} and {chain_id:?}"
                );
                let new_amount = output.amount() + added_amount;
                let new_mana = output.mana() + added_mana;
                *output = match output {
                    Output::Basic(b) => BasicOutputBuilder::from(&*b)
                        .with_amount(new_amount)
                        .with_mana(new_mana)
                        .finish_output()?,
                    Output::Account(a) => AccountOutputBuilder::from(&*a)
                        .with_amount(new_amount)
                        .with_mana(new_mana)
                        .finish_output()?,
                    Output::Nft(n) => NftOutputBuilder::from(&*n)
                        .with_amount(new_amount)
                        .with_mana(new_mana)
                        .finish_output()?,
                    Output::Foundry(f) => FoundryOutputBuilder::from(&*f)
                        .with_amount(new_amount)
                        .finish_output()?,
                    _ => unreachable!(),
                };
            }
        }
        // If we're burning generated mana, set the capability flag.
        if self.burn.as_ref().map_or(false, |b| b.generated_mana()) {
            // Get the mana sums with generated mana to see whether there's a difference.
            if !self
                .transaction_capabilities
                .has_capability(TransactionCapabilityFlag::BurnMana)
                && input_mana < self.total_selected_mana(true)?
            {
                self.transaction_capabilities
                    .add_capability(TransactionCapabilityFlag::BurnMana);
            }
        }

        let outputs = self
            .provided_outputs
            .into_iter()
            .chain(self.added_outputs)
            .chain(self.remainders.storage_deposit_returns)
            .chain(self.remainders.data.iter().map(|r| r.output.clone()))
            .collect::<Vec<_>>();

        // Check again, because more outputs may have been added.
        if !OUTPUT_COUNT_RANGE.contains(&(outputs.len() as u16)) {
            return Err(TransactionBuilderError::InvalidOutputCount(outputs.len()));
        }

        let inputs_data: Vec<InputSigningData> = self.selected_inputs.into_sorted_iter().collect();
        for output_id in self.mana_rewards.keys() {
            if !inputs_data.iter().any(|i| output_id == i.output_id()) {
                return Err(TransactionBuilderError::ExtraManaRewards(*output_id));
            }
        }

        let mut inputs: Vec<Input> = Vec::new();
        let mut context_inputs = self
            .bic_context_inputs
            .into_iter()
            .map(ContextInput::from)
            .chain(self.commitment_context_input.map(ContextInput::from))
            .collect::<Vec<_>>();

        for (idx, input) in inputs_data.iter().enumerate() {
            inputs.push(Input::Utxo(UtxoInput::from(*input.output_id())));
            if self.reward_context_inputs.contains(input.output_id()) {
                context_inputs.push(RewardContextInput::new(idx as u16).unwrap().into());
            }
        }

        let mana_allotments = self
            .mana_allotments
            .into_iter()
            .map(|(account_id, mana)| ManaAllotment::new(account_id, mana))
            .collect::<Result<Vec<_>, _>>()?;

        // Build transaction

        let mut builder = Transaction::builder(self.protocol_parameters.network_id())
            .with_inputs(inputs)
            .with_outputs(outputs)
            .with_mana_allotments(mana_allotments)
            .with_context_inputs(context_inputs)
            .with_creation_slot(self.creation_slot)
            .with_capabilities(self.transaction_capabilities);

        if let Some(payload) = self.payload {
            builder = builder.with_payload(payload);
        }

        let transaction = builder.finish_with_params(&self.protocol_parameters)?;

        let data = PreparedTransactionData {
            transaction,
            inputs_data,
            remainders: self.remainders.data,
            mana_rewards: self.mana_rewards.into_iter().collect(),
            issuer_id: self.issuer_id,
        };

        data.verify_semantic(&self.protocol_parameters)?;

        Ok(data)
    }

    /// Select an input and return whether an output was created.
    fn select_input(&mut self, input: InputSigningData) -> Result<bool, TransactionBuilderError> {
        log::debug!("Selecting input {:?}", input.output_id());

        if self.selected_inputs.len() >= INPUT_COUNT_MAX as usize {
            return Err(TransactionBuilderError::InvalidInputCount(
                self.selected_inputs.len() + 1,
            ));
        }

        let mut added_output = false;
        if let Some(output) = self.transition_input(&input)? {
            // No need to check for `outputs_requirements` because
            // - the sender feature doesn't need to be verified as it has been removed
            // - the issuer feature doesn't need to be verified as the chain is not new
            // - input doesn't need to be checked for as we just transitioned it
            // - foundry account requirement should have been met already by a prior `required_account_nft_addresses`
            self.added_outputs.push(output);
            added_output = true;
        }

        if let Some(requirement) = self.required_account_nft_addresses(&input)? {
            log::debug!("Adding {requirement:?} from input {:?}", input.output_id());
            self.requirements.push(requirement);
        }

        // New input may need context inputs
        self.fulfill_context_inputs_requirements(&input);

        let required_address = input
            .output
            .required_address(
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.committable_age_range(),
            )
            // PANIC: safe to unwrap as non basic/account/foundry/nft/delegation outputs are already filtered out.
            .unwrap()
            .expect("expiration unlockable outputs already filtered out");
        self.selected_inputs.insert(required_address, input);

        Ok(added_output)
    }

    /// Sets the required inputs of a [`TransactionBuilder`].
    pub fn with_required_inputs(mut self, inputs: impl IntoIterator<Item = OutputId>) -> Self {
        self.required_inputs = inputs.into_iter().collect();
        self
    }

    /// Sets the transitions of a [`TransactionBuilder`].
    pub fn with_transitions(mut self, transitions: impl Into<Option<Transitions>>) -> Self {
        self.transitions = transitions.into();
        self
    }

    /// Sets the burn of a [`TransactionBuilder`].
    pub fn with_burn(mut self, burn: impl Into<Option<Burn>>) -> Self {
        self.burn = burn.into();
        self
    }

    /// Sets the remainder address of a [`TransactionBuilder`].
    pub fn with_remainder_address(mut self, address: impl Into<Option<Address>>) -> Self {
        self.remainders.address = address.into();
        self
    }

    /// Sets the mana allotments of a [`TransactionBuilder`].
    pub fn with_mana_allotments(mut self, mana_allotments: impl IntoIterator<Item = (AccountId, u64)>) -> Self {
        self.mana_allotments = mana_allotments.into_iter().collect();
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

    /// Add a transaction data payload.
    pub fn with_payload(mut self, payload: impl Into<Option<TaggedDataPayload>>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Specifies an account to which the minimum required mana allotment will be added.
    pub fn with_min_mana_allotment(mut self, account_id: AccountId, reference_mana_cost: u64) -> Self {
        self.min_mana_allotment.replace(MinManaAllotment {
            issuer_id: account_id,
            reference_mana_cost,
            allotment_debt: 0,
            required_allotment: None,
        });
        self
    }

    /// Specifies the block issuer id that should be used when sending a block.
    pub fn with_issuer_id(mut self, issuer_id: impl Into<Option<AccountId>>) -> Self {
        self.issuer_id = issuer_id.into();
        self
    }

    /// Disables selecting additional inputs.
    pub fn disable_additional_input_selection(mut self) -> Self {
        self.allow_additional_input_selection = false;
        self
    }

    pub(crate) fn all_outputs(&self) -> impl Iterator<Item = &Output> {
        self.non_remainder_outputs().chain(self.remainder_outputs())
    }

    pub(crate) fn non_remainder_outputs(&self) -> impl Iterator<Item = &Output> {
        self.provided_outputs.iter().chain(&self.added_outputs)
    }

    pub(crate) fn remainder_outputs(&self) -> impl Iterator<Item = &Output> {
        self.remainders
            .data
            .iter()
            .map(|r| &r.output)
            .chain(&self.remainders.storage_deposit_returns)
    }

    pub(crate) fn context_inputs(&self) -> impl Iterator<Item = ContextInput> + '_ {
        self.bic_context_inputs
            .iter()
            .copied()
            .map(ContextInput::from)
            .chain(self.commitment_context_input.map(ContextInput::from))
            .chain(self.selected_inputs.iter().enumerate().filter_map(|(idx, input)| {
                self.reward_context_inputs
                    .contains(input.output_id())
                    .then_some(RewardContextInput::new(idx as u16).unwrap().into())
            }))
    }

    fn required_account_nft_addresses(
        &self,
        input: &InputSigningData,
    ) -> Result<Option<Requirement>, TransactionBuilderError> {
        let required_address = input
            .output
            .required_address(
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.committable_age_range(),
            )?
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

            if input.output.unlock_conditions().is_timelocked(
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.min_committable_age(),
            ) {
                return false;
            }

            let required_address = input
                .output
                // Account transition is irrelevant here as we keep accounts anyway.
                .required_address(
                    self.latest_slot_commitment_id.slot_index(),
                    self.protocol_parameters.committable_age_range(),
                )
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
}

#[derive(Clone, Debug, Default)]
pub(crate) struct OrderedInputs {
    ed25519: VecDeque<InputSigningData>,
    other: BTreeMap<Address, VecDeque<InputSigningData>>,
    len: usize,
}

impl OrderedInputs {
    pub(crate) fn sorted_iter(&self) -> OrderedInputsIter<&Address, &InputSigningData> {
        OrderedInputsIter {
            queue: self.ed25519.iter().collect(),
            other: self.other.iter().map(|(k, v)| (k, v.iter().collect())).collect(),
        }
    }

    pub(crate) fn into_sorted_iter(self) -> OrderedInputsIter<Address, InputSigningData> {
        OrderedInputsIter {
            queue: self.ed25519,
            other: self.other,
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &InputSigningData> + Clone {
        self.ed25519.iter().chain(self.other.values().flatten())
    }

    pub(crate) fn insert(&mut self, required_address: Address, input: InputSigningData) {
        if required_address.is_ed25519_backed() {
            self.ed25519.push_back(input);
        } else {
            self.other.entry(required_address).or_default().push_back(input);
        }
        self.len += 1;
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Debug)]
pub(crate) struct OrderedInputsIter<A: Borrow<Address> + Ord + core::hash::Hash, I: Borrow<InputSigningData>> {
    queue: VecDeque<I>,
    other: BTreeMap<A, VecDeque<I>>,
}

impl<A: Borrow<Address> + Ord + core::hash::Hash, I: Borrow<InputSigningData>> Iterator for OrderedInputsIter<A, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        // Inputs that are unlocked by Ed25519 addresses go in the queue first
        // because they do not need to reference other inputs for their unlocks.
        // Each one may have additional dependents which are added to the front of
        // the queue to be sorted immediately after the input they depend upon.
        // Those can also have dependents which will go after them.
        // This creates a tree structure with many to one relationship, which is
        // flattened by this loop in insertion order.
        if let Some(input) = self.queue.pop_front() {
            // Add associated inputs to the front of the queue
            match &input.borrow().output {
                Output::Account(account_output) => {
                    for input in self
                        .other
                        .remove(&Address::Account(AccountAddress::new(
                            account_output.account_id_non_null(input.borrow().output_id()),
                        )))
                        .into_iter()
                        .flatten()
                        .rev()
                    {
                        self.queue.push_front(input);
                    }
                }
                Output::Nft(nft_output) => {
                    for input in self
                        .other
                        .remove(&Address::Nft(NftAddress::new(
                            nft_output.nft_id_non_null(input.borrow().output_id()),
                        )))
                        .into_iter()
                        .flatten()
                        .rev()
                    {
                        self.queue.push_front(input);
                    }
                }
                _ => (),
            };
            return Some(input);
        }
        // When the queue is empty, just add anything that is left over to the end of the list.
        if let Some(mut entry) = self.other.first_entry() {
            if let Some(input) = entry.get_mut().pop_front() {
                // Since the structure is a list-of-lists, we need to pop
                // the inner list if it's empty.
                if entry.get().is_empty() {
                    self.other.pop_first();
                }
                return Some(input);
            }
        }
        None
    }
}
