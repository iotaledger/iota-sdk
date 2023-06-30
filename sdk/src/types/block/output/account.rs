// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{collections::BTreeSet, vec::Vec};

use hashbrown::HashMap;
use packable::{
    bounded::BoundedU16,
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::BoxedSlicePrefix,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{
    address::{AccountAddress, Address},
    output::{
        feature::{verify_allowed_features, Feature, FeatureFlags, Features},
        unlock_condition::{verify_allowed_unlock_conditions, UnlockCondition, UnlockConditionFlags, UnlockConditions},
        verify_output_amount, AccountId, ChainId, NativeToken, NativeTokens, Output, OutputBuilderAmount, OutputId,
        Rent, RentStructure, StateTransitionError, StateTransitionVerifier,
    },
    protocol::ProtocolParameters,
    semantic::{ConflictReason, ValidationContext},
    unlock::Unlock,
    Error,
};

/// Types of account transition.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccountTransition {
    /// State transition.
    State,
    /// Governance transition.
    Governance,
}

impl AccountTransition {
    /// Checks whether the alias transition is a state one.
    pub fn is_state(&self) -> bool {
        matches!(self, Self::State)
    }

    /// Checks whether the alias transition is a governance one.
    pub fn is_governance(&self) -> bool {
        matches!(self, Self::Governance)
    }
}

impl core::fmt::Display for AccountTransition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::State => write!(f, "state"),
            Self::Governance => write!(f, "governance"),
        }
    }
}

///
#[derive(Clone)]
#[must_use]
pub struct AccountOutputBuilder {
    amount: OutputBuilderAmount,
    native_tokens: BTreeSet<NativeToken>,
    alias_id: AccountId,
    state_index: Option<u32>,
    state_metadata: Vec<u8>,
    foundry_counter: Option<u32>,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
    immutable_features: BTreeSet<Feature>,
}

impl AccountOutputBuilder {
    /// Creates an [`AccountOutputBuilder`] with a provided amount.
    pub fn new_with_amount(amount: u64, alias_id: AccountId) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount), alias_id)
    }

    /// Creates an [`AccountOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    pub fn new_with_minimum_storage_deposit(rent_structure: RentStructure, alias_id: AccountId) -> Self {
        Self::new(OutputBuilderAmount::MinimumStorageDeposit(rent_structure), alias_id)
    }

    fn new(amount: OutputBuilderAmount, alias_id: AccountId) -> Self {
        Self {
            amount,
            native_tokens: BTreeSet::new(),
            alias_id,
            state_index: None,
            state_metadata: Vec::new(),
            foundry_counter: None,
            unlock_conditions: BTreeSet::new(),
            features: BTreeSet::new(),
            immutable_features: BTreeSet::new(),
        }
    }

    /// Sets the amount to the provided value.
    #[inline(always)]
    pub fn with_amount(mut self, amount: u64) -> Self {
        self.amount = OutputBuilderAmount::Amount(amount);
        self
    }

    /// Sets the amount to the minimum storage deposit.
    #[inline(always)]
    pub fn with_minimum_storage_deposit(mut self, rent_structure: RentStructure) -> Self {
        self.amount = OutputBuilderAmount::MinimumStorageDeposit(rent_structure);
        self
    }

    ///
    #[inline(always)]
    pub fn add_native_token(mut self, native_token: NativeToken) -> Self {
        self.native_tokens.insert(native_token);
        self
    }

    ///
    #[inline(always)]
    pub fn with_native_tokens(mut self, native_tokens: impl IntoIterator<Item = NativeToken>) -> Self {
        self.native_tokens = native_tokens.into_iter().collect();
        self
    }

    /// Sets the alias ID to the provided value.
    #[inline(always)]
    pub fn with_alias_id(mut self, alias_id: AccountId) -> Self {
        self.alias_id = alias_id;
        self
    }

    ///
    #[inline(always)]
    pub fn with_state_index(mut self, state_index: impl Into<Option<u32>>) -> Self {
        self.state_index = state_index.into();
        self
    }

    ///
    #[inline(always)]
    pub fn with_state_metadata(mut self, state_metadata: impl Into<Vec<u8>>) -> Self {
        self.state_metadata = state_metadata.into();
        self
    }

    ///
    #[inline(always)]
    pub fn with_foundry_counter(mut self, foundry_counter: impl Into<Option<u32>>) -> Self {
        self.foundry_counter = foundry_counter.into();
        self
    }

    /// Adds an [`UnlockCondition`] to the builder, if one does not already exist of that type.
    #[inline(always)]
    pub fn add_unlock_condition(mut self, unlock_condition: impl Into<UnlockCondition>) -> Self {
        self.unlock_conditions.insert(unlock_condition.into());
        self
    }

    /// Sets the [`UnlockConditions`]s in the builder, overwriting any existing values.
    #[inline(always)]
    pub fn with_unlock_conditions(
        mut self,
        unlock_conditions: impl IntoIterator<Item = impl Into<UnlockCondition>>,
    ) -> Self {
        self.unlock_conditions = unlock_conditions.into_iter().map(Into::into).collect();
        self
    }

    /// Replaces an [`UnlockCondition`] of the builder with a new one, or adds it.
    pub fn replace_unlock_condition(mut self, unlock_condition: impl Into<UnlockCondition>) -> Self {
        self.unlock_conditions.replace(unlock_condition.into());
        self
    }

    /// Clears all [`UnlockConditions`]s from the builder.
    #[inline(always)]
    pub fn clear_unlock_conditions(mut self) -> Self {
        self.unlock_conditions.clear();
        self
    }

    /// Adds a [`Feature`] to the builder, if one does not already exist of that type.
    #[inline(always)]
    pub fn add_feature(mut self, feature: impl Into<Feature>) -> Self {
        self.features.insert(feature.into());
        self
    }

    /// Sets the [`Feature`]s in the builder, overwriting any existing values.
    #[inline(always)]
    pub fn with_features(mut self, features: impl IntoIterator<Item = impl Into<Feature>>) -> Self {
        self.features = features.into_iter().map(Into::into).collect();
        self
    }

    /// Replaces a [`Feature`] of the builder with a new one, or adds it.
    pub fn replace_feature(mut self, feature: impl Into<Feature>) -> Self {
        self.features.replace(feature.into());
        self
    }

    /// Clears all [`Feature`]s from the builder.
    #[inline(always)]
    pub fn clear_features(mut self) -> Self {
        self.features.clear();
        self
    }

    /// Adds an immutable [`Feature`] to the builder, if one does not already exist of that type.
    #[inline(always)]
    pub fn add_immutable_feature(mut self, immutable_feature: impl Into<Feature>) -> Self {
        self.immutable_features.insert(immutable_feature.into());
        self
    }

    /// Sets the immutable [`Feature`]s in the builder, overwriting any existing values.
    #[inline(always)]
    pub fn with_immutable_features(mut self, immutable_features: impl IntoIterator<Item = impl Into<Feature>>) -> Self {
        self.immutable_features = immutable_features.into_iter().map(Into::into).collect();
        self
    }

    /// Replaces an immutable [`Feature`] of the builder with a new one, or adds it.
    pub fn replace_immutable_feature(mut self, immutable_feature: impl Into<Feature>) -> Self {
        self.immutable_features.replace(immutable_feature.into());
        self
    }

    /// Clears all immutable [`Feature`]s from the builder.
    #[inline(always)]
    pub fn clear_immutable_features(mut self) -> Self {
        self.immutable_features.clear();
        self
    }

    ///
    pub fn finish_unverified(self) -> Result<AccountOutput, Error> {
        let state_index = self.state_index.unwrap_or(0);
        let foundry_counter = self.foundry_counter.unwrap_or(0);

        let state_metadata = self
            .state_metadata
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidStateMetadataLength)?;

        verify_index_counter(&self.alias_id, state_index, foundry_counter)?;

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions, &self.alias_id)?;

        let features = Features::from_set(self.features)?;

        verify_allowed_features(&features, AccountOutput::ALLOWED_FEATURES)?;

        let immutable_features = Features::from_set(self.immutable_features)?;

        verify_allowed_features(&immutable_features, AccountOutput::ALLOWED_IMMUTABLE_FEATURES)?;

        let mut output = AccountOutput {
            amount: 1,
            native_tokens: NativeTokens::from_set(self.native_tokens)?,
            alias_id: self.alias_id,
            state_index,
            state_metadata,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                Output::Account(output.clone()).rent_cost(&rent_structure)
            }
        };

        Ok(output)
    }

    ///
    pub fn finish(self, token_supply: u64) -> Result<AccountOutput, Error> {
        let output = self.finish_unverified()?;

        verify_output_amount::<true>(&output.amount, &token_supply)?;

        Ok(output)
    }

    /// Finishes the [`AccountOutputBuilder`] into an [`Output`].
    pub fn finish_output(self, token_supply: u64) -> Result<Output, Error> {
        Ok(Output::Account(self.finish(token_supply)?))
    }
}

impl From<&AccountOutput> for AccountOutputBuilder {
    fn from(output: &AccountOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            native_tokens: output.native_tokens.iter().copied().collect(),
            alias_id: output.alias_id,
            state_index: Some(output.state_index),
            state_metadata: output.state_metadata.to_vec(),
            foundry_counter: Some(output.foundry_counter),
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
            features: output.features.iter().cloned().collect(),
            immutable_features: output.immutable_features.iter().cloned().collect(),
        }
    }
}

pub(crate) type StateMetadataLength = BoundedU16<0, { AccountOutput::STATE_METADATA_LENGTH_MAX }>;

/// Describes an alias account in the ledger that can be controlled by the state and governance controllers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountOutput {
    // Amount of IOTA tokens held by the output.
    amount: u64,
    // Native tokens held by the output.
    native_tokens: NativeTokens,
    // Unique identifier of the alias.
    alias_id: AccountId,
    // A counter that must increase by 1 every time the alias is state transitioned.
    state_index: u32,
    // Metadata that can only be changed by the state controller.
    state_metadata: BoxedSlicePrefix<u8, StateMetadataLength>,
    // A counter that denotes the number of foundries created by this alias account.
    foundry_counter: u32,
    unlock_conditions: UnlockConditions,
    //
    features: Features,
    //
    immutable_features: Features,
}

impl AccountOutput {
    /// The [`Output`](crate::types::block::output::Output) kind of an [`AccountOutput`].
    pub const KIND: u8 = 4;
    /// Maximum possible length in bytes of the state metadata.
    pub const STATE_METADATA_LENGTH_MAX: u16 = 8192;
    /// The set of allowed [`UnlockCondition`]s for an [`AccountOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags =
        UnlockConditionFlags::STATE_CONTROLLER_ADDRESS.union(UnlockConditionFlags::GOVERNOR_ADDRESS);
    /// The set of allowed [`Feature`]s for an [`AccountOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::SENDER.union(FeatureFlags::METADATA);
    /// The set of allowed immutable [`Feature`]s for an [`AccountOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::ISSUER.union(FeatureFlags::METADATA);

    /// Creates a new [`AccountOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, alias_id: AccountId) -> AccountOutputBuilder {
        AccountOutputBuilder::new_with_amount(amount, alias_id)
    }

    /// Creates a new [`AccountOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    #[inline(always)]
    pub fn build_with_minimum_storage_deposit(
        rent_structure: RentStructure,
        alias_id: AccountId,
    ) -> AccountOutputBuilder {
        AccountOutputBuilder::new_with_minimum_storage_deposit(rent_structure, alias_id)
    }

    ///
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount
    }

    ///
    #[inline(always)]
    pub fn native_tokens(&self) -> &NativeTokens {
        &self.native_tokens
    }

    ///
    #[inline(always)]
    pub fn alias_id(&self) -> &AccountId {
        &self.alias_id
    }

    /// Returns the alias ID if not null, or creates it from the output ID.
    #[inline(always)]
    pub fn alias_id_non_null(&self, output_id: &OutputId) -> AccountId {
        self.alias_id.or_from_output_id(output_id)
    }

    ///
    #[inline(always)]
    pub fn state_index(&self) -> u32 {
        self.state_index
    }

    ///
    #[inline(always)]
    pub fn state_metadata(&self) -> &[u8] {
        &self.state_metadata
    }

    ///
    #[inline(always)]
    pub fn foundry_counter(&self) -> u32 {
        self.foundry_counter
    }

    ///
    #[inline(always)]
    pub fn unlock_conditions(&self) -> &UnlockConditions {
        &self.unlock_conditions
    }

    ///
    #[inline(always)]
    pub fn features(&self) -> &Features {
        &self.features
    }

    ///
    #[inline(always)]
    pub fn immutable_features(&self) -> &Features {
        &self.immutable_features
    }

    ///
    #[inline(always)]
    pub fn state_controller_address(&self) -> &Address {
        // An AccountOutput must have a StateControllerAddressUnlockCondition.
        self.unlock_conditions
            .state_controller_address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    #[inline(always)]
    pub fn governor_address(&self) -> &Address {
        // An AccountOutput must have a GovernorAddressUnlockCondition.
        self.unlock_conditions
            .governor_address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Account(self.alias_id)
    }

    /// Returns the alias address for this output.
    pub fn alias_address(&self, output_id: &OutputId) -> AccountAddress {
        AccountAddress::new(self.alias_id_non_null(output_id))
    }

    ///
    pub fn unlock(
        &self,
        output_id: &OutputId,
        unlock: &Unlock,
        inputs: &[(OutputId, &Output)],
        context: &mut ValidationContext<'_>,
    ) -> Result<(), ConflictReason> {
        let alias_id = if self.alias_id().is_null() {
            AccountId::from(output_id)
        } else {
            *self.alias_id()
        };
        let next_state = context.output_chains.get(&ChainId::from(alias_id));

        match next_state {
            Some(Output::Account(next_state)) => {
                if self.state_index() == next_state.state_index() {
                    self.governor_address().unlock(unlock, inputs, context)?;
                } else {
                    self.state_controller_address().unlock(unlock, inputs, context)?;
                    // Only a state transition can be used to consider the alias address for output unlocks and
                    // sender/issuer validations.
                    context
                        .unlocked_addresses
                        .insert(Address::from(AccountAddress::from(alias_id)));
                }
            }
            None => self.governor_address().unlock(unlock, inputs, context)?,
            // The next state can only be an alias output since it is identified by an alias chain identifier.
            Some(_) => unreachable!(),
        };

        Ok(())
    }

    // Transition, just without full ValidationContext
    pub(crate) fn transition_inner(
        current_state: &Self,
        next_state: &Self,
        input_chains: &HashMap<ChainId, &Output>,
        outputs: &[Output],
    ) -> Result<(), StateTransitionError> {
        if current_state.immutable_features != next_state.immutable_features {
            return Err(StateTransitionError::MutatedImmutableField);
        }

        if next_state.state_index == current_state.state_index + 1 {
            // State transition.
            if current_state.state_controller_address() != next_state.state_controller_address()
                || current_state.governor_address() != next_state.governor_address()
                || current_state.features.metadata() != next_state.features.metadata()
            {
                return Err(StateTransitionError::MutatedFieldWithoutRights);
            }

            let created_foundries = outputs.iter().filter_map(|output| {
                if let Output::Foundry(foundry) = output {
                    if foundry.alias_address().alias_id() == &next_state.alias_id
                        && !input_chains.contains_key(&foundry.chain_id())
                    {
                        Some(foundry)
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

            let mut created_foundries_count = 0;

            for foundry in created_foundries {
                created_foundries_count += 1;

                if foundry.serial_number() != current_state.foundry_counter + created_foundries_count {
                    return Err(StateTransitionError::UnsortedCreatedFoundries);
                }
            }

            if current_state.foundry_counter + created_foundries_count != next_state.foundry_counter {
                return Err(StateTransitionError::InconsistentCreatedFoundriesCount);
            }
        } else if next_state.state_index == current_state.state_index {
            // Governance transition.
            if current_state.amount != next_state.amount
                || current_state.native_tokens != next_state.native_tokens
                || current_state.state_metadata != next_state.state_metadata
                || current_state.foundry_counter != next_state.foundry_counter
            {
                return Err(StateTransitionError::MutatedFieldWithoutRights);
            }
        } else {
            return Err(StateTransitionError::UnsupportedStateIndexOperation {
                current_state: current_state.state_index,
                next_state: next_state.state_index,
            });
        }

        Ok(())
    }
}

impl StateTransitionVerifier for AccountOutput {
    fn creation(next_state: &Self, context: &ValidationContext<'_>) -> Result<(), StateTransitionError> {
        if !next_state.alias_id.is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if let Some(issuer) = next_state.immutable_features().issuer() {
            if !context.unlocked_addresses.contains(issuer.address()) {
                return Err(StateTransitionError::IssuerNotUnlocked);
            }
        }

        Ok(())
    }

    fn transition(
        current_state: &Self,
        next_state: &Self,
        context: &ValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(
            current_state,
            next_state,
            &context.input_chains,
            context.essence.outputs(),
        )
    }

    fn destruction(_current_state: &Self, _context: &ValidationContext<'_>) -> Result<(), StateTransitionError> {
        Ok(())
    }
}

impl Packable for AccountOutput {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.native_tokens.pack(packer)?;
        self.alias_id.pack(packer)?;
        self.state_index.pack(packer)?;
        self.state_metadata.pack(packer)?;
        self.foundry_counter.pack(packer)?;
        self.unlock_conditions.pack(packer)?;
        self.features.pack(packer)?;
        self.immutable_features.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let amount = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        verify_output_amount::<VERIFY>(&amount, &visitor.token_supply()).map_err(UnpackError::Packable)?;

        let native_tokens = NativeTokens::unpack::<_, VERIFY>(unpacker, &())?;
        let alias_id = AccountId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let state_index = u32::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let state_metadata = BoxedSlicePrefix::<u8, StateMetadataLength>::unpack::<_, VERIFY>(unpacker, &())
            .map_packable_err(|err| Error::InvalidStateMetadataLength(err.into_prefix_err().into()))?;

        let foundry_counter = u32::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        if VERIFY {
            verify_index_counter(&alias_id, state_index, foundry_counter).map_err(UnpackError::Packable)?;
        }

        let unlock_conditions = UnlockConditions::unpack::<_, VERIFY>(unpacker, visitor)?;

        if VERIFY {
            verify_unlock_conditions(&unlock_conditions, &alias_id).map_err(UnpackError::Packable)?;
        }

        let features = Features::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_allowed_features(&features, Self::ALLOWED_FEATURES).map_err(UnpackError::Packable)?;
        }

        let immutable_features = Features::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_allowed_features(&immutable_features, Self::ALLOWED_IMMUTABLE_FEATURES)
                .map_err(UnpackError::Packable)?;
        }

        Ok(Self {
            amount,
            native_tokens,
            alias_id,
            state_index,
            state_metadata,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        })
    }
}

#[inline]
fn verify_index_counter(alias_id: &AccountId, state_index: u32, foundry_counter: u32) -> Result<(), Error> {
    if alias_id.is_null() && (state_index != 0 || foundry_counter != 0) {
        Err(Error::NonZeroStateIndexOrFoundryCounter)
    } else {
        Ok(())
    }
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions, alias_id: &AccountId) -> Result<(), Error> {
    if let Some(unlock_condition) = unlock_conditions.state_controller_address() {
        if let Address::Alias(alias_address) = unlock_condition.address() {
            if alias_address.alias_id() == alias_id {
                return Err(Error::SelfControlledAccountOutput(*alias_id));
            }
        }
    } else {
        return Err(Error::MissingStateControllerUnlockCondition);
    }

    if let Some(unlock_condition) = unlock_conditions.governor_address() {
        if let Address::Alias(alias_address) = unlock_condition.address() {
            if alias_address.alias_id() == alias_id {
                return Err(Error::SelfControlledAccountOutput(*alias_id));
            }
        }
    } else {
        return Err(Error::MissingGovernorUnlockCondition);
    }

    verify_allowed_unlock_conditions(unlock_conditions, AccountOutput::ALLOWED_UNLOCK_CONDITIONS)
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::{
        output::{dto::OutputBuilderAmountDto, feature::dto::FeatureDto, unlock_condition::dto::UnlockConditionDto},
        Error,
    };

    /// Describes an alias account in the ledger that can be controlled by the state and governance controllers.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        // Amount of IOTA tokens held by the output.
        pub amount: String,
        // Native tokens held by the output.
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        // Unique identifier of the alias.
        pub alias_id: AccountId,
        // A counter that must increase by 1 every time the alias is state transitioned.
        pub state_index: u32,
        // Metadata that can only be changed by the state controller.
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub state_metadata: String,
        // A counter that denotes the number of foundries created by this alias account.
        pub foundry_counter: u32,
        //
        pub unlock_conditions: Vec<UnlockConditionDto>,
        //
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<FeatureDto>,
        //
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<FeatureDto>,
    }

    impl From<&AccountOutput> for AccountOutputDto {
        fn from(value: &AccountOutput) -> Self {
            Self {
                kind: AccountOutput::KIND,
                amount: value.amount().to_string(),
                native_tokens: value.native_tokens().to_vec(),
                alias_id: *value.alias_id(),
                state_index: value.state_index(),
                state_metadata: prefix_hex::encode(value.state_metadata()),
                foundry_counter: value.foundry_counter(),
                unlock_conditions: value.unlock_conditions().iter().map(Into::into).collect::<_>(),
                features: value.features().iter().map(Into::into).collect::<_>(),
                immutable_features: value.immutable_features().iter().map(Into::into).collect::<_>(),
            }
        }
    }

    impl AccountOutput {
        pub fn try_from_dto(value: AccountOutputDto, token_supply: u64) -> Result<Self, Error> {
            let mut builder = AccountOutputBuilder::new_with_amount(
                value.amount.parse::<u64>().map_err(|_| Error::InvalidField("amount"))?,
                value.alias_id,
            );

            builder = builder.with_state_index(value.state_index);

            if !value.state_metadata.is_empty() {
                builder = builder.with_state_metadata(
                    prefix_hex::decode::<Vec<_>>(&value.state_metadata)
                        .map_err(|_| Error::InvalidField("state_metadata"))?,
                );
            }

            builder = builder.with_foundry_counter(value.foundry_counter);

            for t in value.native_tokens {
                builder = builder.add_native_token(t);
            }

            for b in value.features {
                builder = builder.add_feature(Feature::try_from(b)?);
            }

            for b in value.immutable_features {
                builder = builder.add_immutable_feature(Feature::try_from(b)?);
            }

            for u in value.unlock_conditions {
                builder = builder.add_unlock_condition(UnlockCondition::try_from_dto(u, token_supply)?);
            }

            builder.finish(token_supply)
        }

        pub fn try_from_dto_unverified(value: AccountOutputDto) -> Result<Self, Error> {
            let mut builder = AccountOutputBuilder::new_with_amount(
                value.amount.parse::<u64>().map_err(|_| Error::InvalidField("amount"))?,
                value.alias_id,
            );

            builder = builder.with_state_index(value.state_index);

            if !value.state_metadata.is_empty() {
                builder = builder.with_state_metadata(
                    prefix_hex::decode::<Vec<_>>(&value.state_metadata)
                        .map_err(|_| Error::InvalidField("state_metadata"))?,
                );
            }

            builder = builder.with_foundry_counter(value.foundry_counter);

            for t in value.native_tokens {
                builder = builder.add_native_token(t);
            }

            for b in value.features {
                builder = builder.add_feature(Feature::try_from(b)?);
            }

            for b in value.immutable_features {
                builder = builder.add_immutable_feature(Feature::try_from(b)?);
            }

            for u in value.unlock_conditions {
                builder = builder.add_unlock_condition(UnlockCondition::try_from_dto_unverified(u)?);
            }

            builder.finish_unverified()
        }

        #[allow(clippy::too_many_arguments)]
        pub fn try_from_dtos(
            amount: OutputBuilderAmountDto,
            native_tokens: Option<Vec<NativeToken>>,
            alias_id: &AccountId,
            state_index: Option<u32>,
            state_metadata: Option<Vec<u8>>,
            foundry_counter: Option<u32>,
            unlock_conditions: Vec<UnlockConditionDto>,
            features: Option<Vec<FeatureDto>>,
            immutable_features: Option<Vec<FeatureDto>>,
            token_supply: u64,
        ) -> Result<Self, Error> {
            let mut builder = match amount {
                OutputBuilderAmountDto::Amount(amount) => AccountOutputBuilder::new_with_amount(
                    amount.parse().map_err(|_| Error::InvalidField("amount"))?,
                    *alias_id,
                ),
                OutputBuilderAmountDto::MinimumStorageDeposit(rent_structure) => {
                    AccountOutputBuilder::new_with_minimum_storage_deposit(rent_structure, *alias_id)
                }
            };

            if let Some(native_tokens) = native_tokens {
                builder = builder.with_native_tokens(native_tokens);
            }

            if let Some(state_index) = state_index {
                builder = builder.with_state_index(state_index);
            }

            if let Some(state_metadata) = state_metadata {
                builder = builder.with_state_metadata(state_metadata);
            }

            if let Some(foundry_counter) = foundry_counter {
                builder = builder.with_foundry_counter(foundry_counter);
            }

            let unlock_conditions = unlock_conditions
                .into_iter()
                .map(|u| UnlockCondition::try_from_dto(u, token_supply))
                .collect::<Result<Vec<UnlockCondition>, Error>>()?;
            builder = builder.with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                let features = features
                    .into_iter()
                    .map(Feature::try_from)
                    .collect::<Result<Vec<Feature>, Error>>()?;
                builder = builder.with_features(features);
            }

            if let Some(immutable_features) = immutable_features {
                let immutable_features = immutable_features
                    .into_iter()
                    .map(Feature::try_from)
                    .collect::<Result<Vec<Feature>, Error>>()?;
                builder = builder.with_immutable_features(immutable_features);
            }

            builder.finish(token_supply)
        }
    }
}

#[cfg(test)]
mod tests {
    use packable::PackableExt;

    use super::*;
    use crate::types::block::{
        address::AccountAddress,
        output::{
            dto::{OutputBuilderAmountDto, OutputDto},
            FoundryId, SimpleTokenScheme, TokenId,
        },
        protocol::protocol_parameters,
        rand::{
            address::rand_account_address,
            output::{
                feature::{rand_allowed_features, rand_issuer_feature, rand_metadata_feature, rand_sender_feature},
                rand_account_id, rand_account_output,
                unlock_condition::{
                    rand_governor_address_unlock_condition_different_from,
                    rand_state_controller_address_unlock_condition_different_from,
                },
            },
        },
    };

    #[test]
    fn builder() {
        let protocol_parameters = protocol_parameters();
        let alias_id = rand_account_id();
        let foundry_id = FoundryId::build(&AccountAddress::from(alias_id), 0, SimpleTokenScheme::KIND);
        let gov_address_1 = rand_governor_address_unlock_condition_different_from(&alias_id);
        let gov_address_2 = rand_governor_address_unlock_condition_different_from(&alias_id);
        let state_address_1 = rand_state_controller_address_unlock_condition_different_from(&alias_id);
        let state_address_2 = rand_state_controller_address_unlock_condition_different_from(&alias_id);
        let sender_1 = rand_sender_feature();
        let sender_2 = rand_sender_feature();
        let issuer_1 = rand_issuer_feature();
        let issuer_2 = rand_issuer_feature();

        let mut builder = AccountOutput::build_with_amount(0, alias_id)
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000.into()).unwrap())
            .add_unlock_condition(gov_address_1)
            .add_unlock_condition(state_address_1)
            .add_feature(sender_1)
            .replace_feature(sender_2)
            .replace_immutable_feature(issuer_1)
            .add_immutable_feature(issuer_2);

        let output = builder.clone().finish_unverified().unwrap();
        assert_eq!(output.unlock_conditions().governor_address(), Some(&gov_address_1));
        assert_eq!(
            output.unlock_conditions().state_controller_address(),
            Some(&state_address_1)
        );
        assert_eq!(output.features().sender(), Some(&sender_2));
        assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));

        builder = builder
            .clear_unlock_conditions()
            .clear_features()
            .clear_immutable_features()
            .replace_unlock_condition(gov_address_2)
            .replace_unlock_condition(state_address_2);
        let output = builder.clone().finish_unverified().unwrap();
        assert_eq!(output.unlock_conditions().governor_address(), Some(&gov_address_2));
        assert_eq!(
            output.unlock_conditions().state_controller_address(),
            Some(&state_address_2)
        );
        assert!(output.features().is_empty());
        assert!(output.immutable_features().is_empty());

        let metadata = rand_metadata_feature();

        let output = builder
            .with_minimum_storage_deposit(*protocol_parameters.rent_structure())
            .add_unlock_condition(rand_state_controller_address_unlock_condition_different_from(&alias_id))
            .add_unlock_condition(rand_governor_address_unlock_condition_different_from(&alias_id))
            .with_features([Feature::from(metadata.clone()), sender_1.into()])
            .with_immutable_features([Feature::from(metadata.clone()), issuer_1.into()])
            .finish(protocol_parameters.token_supply())
            .unwrap();

        assert_eq!(
            output.amount(),
            Output::Account(output.clone()).rent_cost(protocol_parameters.rent_structure())
        );
        assert_eq!(output.features().metadata(), Some(&metadata));
        assert_eq!(output.features().sender(), Some(&sender_1));
        assert_eq!(output.immutable_features().metadata(), Some(&metadata));
        assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));
    }

    #[test]
    fn pack_unpack() {
        let protocol_parameters = protocol_parameters();
        let output = rand_account_output(protocol_parameters.token_supply());
        let bytes = output.pack_to_vec();
        let output_unpacked = AccountOutput::unpack_verified(bytes, &protocol_parameters).unwrap();
        assert_eq!(output, output_unpacked);
    }

    #[test]
    fn to_from_dto() {
        let protocol_parameters = protocol_parameters();
        let output = rand_account_output(protocol_parameters.token_supply());
        let dto = OutputDto::Account((&output).into());
        let output_unver = Output::try_from_dto_unverified(dto.clone()).unwrap();
        assert_eq!(&output, output_unver.as_alias());
        let output_ver = Output::try_from_dto(dto, protocol_parameters.token_supply()).unwrap();
        assert_eq!(&output, output_ver.as_alias());

        let output_split = AccountOutput::try_from_dtos(
            OutputBuilderAmountDto::Amount(output.amount().to_string()),
            Some(output.native_tokens().to_vec()),
            output.alias_id(),
            output.state_index().into(),
            output.state_metadata().to_owned().into(),
            output.foundry_counter().into(),
            output.unlock_conditions().iter().map(Into::into).collect(),
            Some(output.features().iter().map(Into::into).collect()),
            Some(output.immutable_features().iter().map(Into::into).collect()),
            protocol_parameters.token_supply(),
        )
        .unwrap();
        assert_eq!(output, output_split);

        let alias_id = rand_account_id();
        let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);
        let gov_address = rand_governor_address_unlock_condition_different_from(&alias_id);
        let state_address = rand_state_controller_address_unlock_condition_different_from(&alias_id);

        let test_split_dto = |builder: AccountOutputBuilder| {
            let output_split = AccountOutput::try_from_dtos(
                (&builder.amount).into(),
                Some(builder.native_tokens.iter().copied().collect()),
                &builder.alias_id,
                builder.state_index,
                builder.state_metadata.to_owned().into(),
                builder.foundry_counter,
                builder.unlock_conditions.iter().map(Into::into).collect(),
                Some(builder.features.iter().map(Into::into).collect()),
                Some(builder.immutable_features.iter().map(Into::into).collect()),
                protocol_parameters.token_supply(),
            )
            .unwrap();
            assert_eq!(
                builder.finish(protocol_parameters.token_supply()).unwrap(),
                output_split
            );
        };

        let builder = AccountOutput::build_with_amount(100, alias_id)
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000.into()).unwrap())
            .add_unlock_condition(gov_address)
            .add_unlock_condition(state_address)
            .with_features(rand_allowed_features(AccountOutput::ALLOWED_FEATURES))
            .with_immutable_features(rand_allowed_features(AccountOutput::ALLOWED_IMMUTABLE_FEATURES));
        test_split_dto(builder);

        let builder =
            AccountOutput::build_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), alias_id)
                .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000.into()).unwrap())
                .add_unlock_condition(gov_address)
                .add_unlock_condition(state_address)
                .with_features(rand_allowed_features(AccountOutput::ALLOWED_FEATURES))
                .with_immutable_features(rand_allowed_features(AccountOutput::ALLOWED_IMMUTABLE_FEATURES));
        test_split_dto(builder);
    }
}
