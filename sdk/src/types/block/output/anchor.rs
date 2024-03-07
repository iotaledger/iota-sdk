// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

use crate::types::block::{
    address::{Address, AnchorAddress},
    output::{
        feature::{verify_allowed_features, Feature, FeatureFlags, Features},
        unlock_condition::{
            verify_allowed_unlock_conditions, verify_restricted_addresses, UnlockCondition, UnlockConditionFlags,
            UnlockConditions,
        },
        ChainId, DecayedMana, MinimumOutputAmount, Output, OutputBuilderAmount, OutputError, OutputId, StorageScore,
        StorageScoreParameters,
    },
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    semantic::{SemanticValidationContext, TransactionFailureReason},
    slot::SlotIndex,
    unlock::Unlock,
};

crate::impl_id!(
    /// A unique identifier of an anchor.
    pub AnchorId {
        pub const LENGTH: usize = 32;
    }
);

impl From<&OutputId> for AnchorId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl AnchorId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }
}

impl From<AnchorId> for Address {
    fn from(value: AnchorId) -> Self {
        Self::Anchor(AnchorAddress::new(value))
    }
}

/// Types of anchor transition.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnchorTransition {
    /// State transition.
    State,
    /// Governance transition.
    Governance,
}

impl AnchorTransition {
    /// Checks whether the anchor transition is a state one.
    pub fn is_state(&self) -> bool {
        matches!(self, Self::State)
    }

    /// Checks whether the anchor transition is a governance one.
    pub fn is_governance(&self) -> bool {
        matches!(self, Self::Governance)
    }
}

impl core::fmt::Display for AnchorTransition {
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
pub struct AnchorOutputBuilder {
    amount: OutputBuilderAmount,
    mana: u64,
    anchor_id: AnchorId,
    state_index: u32,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
    immutable_features: BTreeSet<Feature>,
}

impl AnchorOutputBuilder {
    /// Creates an [`AnchorOutputBuilder`] with a provided amount.
    pub fn new_with_amount(amount: u64, anchor_id: AnchorId) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount), anchor_id)
    }

    /// Creates an [`AnchorOutputBuilder`] with a provided amount, unless it is below the minimum.
    pub fn new_with_amount_or_minimum(amount: u64, anchor_id: AnchorId, params: StorageScoreParameters) -> Self {
        Self::new(OutputBuilderAmount::AmountOrMinimum(amount, params), anchor_id)
    }

    /// Creates an [`AnchorOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    #[inline(always)]
    pub fn new_with_minimum_amount(params: StorageScoreParameters, anchor_id: AnchorId) -> Self {
        Self::new(OutputBuilderAmount::MinimumAmount(params), anchor_id)
    }

    fn new(amount: OutputBuilderAmount, anchor_id: AnchorId) -> Self {
        Self {
            amount,
            mana: Default::default(),
            anchor_id,
            state_index: 0,
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

    /// Sets the amount to the provided value, unless it is below the minimum.
    #[inline(always)]
    pub fn with_amount_or_minimum(mut self, amount: u64, params: StorageScoreParameters) -> Self {
        self.amount = OutputBuilderAmount::AmountOrMinimum(amount, params);
        self
    }

    /// Sets the amount to the minimum required amount.
    #[inline(always)]
    pub fn with_minimum_amount(mut self, params: StorageScoreParameters) -> Self {
        self.amount = OutputBuilderAmount::MinimumAmount(params);
        self
    }

    /// Sets the mana to the provided value.
    #[inline(always)]
    pub fn with_mana(mut self, mana: u64) -> Self {
        self.mana = mana;
        self
    }

    /// Sets the anchor ID to the provided value.
    #[inline(always)]
    pub fn with_anchor_id(mut self, anchor_id: AnchorId) -> Self {
        self.anchor_id = anchor_id;
        self
    }

    ///
    #[inline(always)]
    pub fn with_state_index(mut self, state_index: u32) -> Self {
        self.state_index = state_index;
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
    pub fn finish(self) -> Result<AnchorOutput, OutputError> {
        verify_index_counter(&self.anchor_id, self.state_index)?;

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions, &self.anchor_id)?;

        let features = Features::from_set(self.features)?;

        verify_restricted_addresses(
            &unlock_conditions,
            AnchorOutput::KIND,
            features.native_token(),
            self.mana,
        )?;
        verify_allowed_features(&features, AnchorOutput::ALLOWED_FEATURES)?;

        let immutable_features = Features::from_set(self.immutable_features)?;

        verify_allowed_features(&immutable_features, AnchorOutput::ALLOWED_IMMUTABLE_FEATURES)?;

        let mut output = AnchorOutput {
            amount: 0,
            mana: self.mana,
            anchor_id: self.anchor_id,
            state_index: self.state_index,
            unlock_conditions,
            features,
            immutable_features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::AmountOrMinimum(amount, params) => output.minimum_amount(params).max(amount),
            OutputBuilderAmount::MinimumAmount(params) => output.minimum_amount(params),
        };

        Ok(output)
    }

    /// Finishes the [`AnchorOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, OutputError> {
        Ok(Output::Anchor(self.finish()?))
    }
}

impl From<&AnchorOutput> for AnchorOutputBuilder {
    fn from(output: &AnchorOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            mana: output.mana,
            anchor_id: output.anchor_id,
            state_index: output.state_index,
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
            features: output.features.iter().cloned().collect(),
            immutable_features: output.immutable_features.iter().cloned().collect(),
        }
    }
}

/// Describes an anchor in the ledger that can be controlled by the state and governance controllers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AnchorOutput {
    /// Amount of IOTA coins held by the output.
    amount: u64,
    /// Amount of stored Mana held by the output.
    mana: u64,
    /// Unique identifier of the anchor.
    anchor_id: AnchorId,
    /// A counter that must increase by 1 every time the anchor is state transitioned.
    state_index: u32,
    /// Define how the output can be unlocked in a transaction.
    unlock_conditions: UnlockConditions,
    /// Features of the output.
    features: Features,
    /// Immutable features of the output.
    immutable_features: Features,
}

impl AnchorOutput {
    /// The [`Output`] kind of an [`AnchorOutput`].
    pub const KIND: u8 = 2;
    /// The set of allowed [`UnlockCondition`]s for an [`AnchorOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags =
        UnlockConditionFlags::STATE_CONTROLLER_ADDRESS.union(UnlockConditionFlags::GOVERNOR_ADDRESS);
    /// The set of allowed [`Feature`]s for an [`AnchorOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::METADATA.union(FeatureFlags::STATE_METADATA);
    /// The set of allowed immutable [`Feature`]s for an [`AnchorOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::ISSUER.union(FeatureFlags::METADATA);

    /// Creates a new [`AnchorOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, anchor_id: AnchorId) -> AnchorOutputBuilder {
        AnchorOutputBuilder::new_with_amount(amount, anchor_id)
    }

    /// Creates a new [`AnchorOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount.
    #[inline(always)]
    pub fn build_with_minimum_amount(params: StorageScoreParameters, anchor_id: AnchorId) -> AnchorOutputBuilder {
        AnchorOutputBuilder::new_with_minimum_amount(params, anchor_id)
    }

    ///
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount
    }

    #[inline(always)]
    pub fn mana(&self) -> u64 {
        self.mana
    }

    ///
    #[inline(always)]
    pub fn anchor_id(&self) -> &AnchorId {
        &self.anchor_id
    }

    /// Returns the anchor ID if not null, or creates it from the output ID.
    #[inline(always)]
    pub fn anchor_id_non_null(&self, output_id: &OutputId) -> AnchorId {
        self.anchor_id.or_from_output_id(output_id)
    }

    ///
    #[inline(always)]
    pub fn state_index(&self) -> u32 {
        self.state_index
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
        // An AnchorOutput must have a StateControllerAddressUnlockCondition.
        self.unlock_conditions
            .state_controller_address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    #[inline(always)]
    pub fn governor_address(&self) -> &Address {
        // An AnchorOutput must have a GovernorAddressUnlockCondition.
        self.unlock_conditions
            .governor_address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Anchor(self.anchor_id)
    }

    /// Returns the anchor address for this output.
    pub fn anchor_address(&self, output_id: &OutputId) -> AnchorAddress {
        AnchorAddress::new(self.anchor_id_non_null(output_id))
    }

    ///
    pub fn unlock(
        &self,
        output_id: &OutputId,
        unlock: &Unlock,
        context: &mut SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        let anchor_id = self.anchor_id_non_null(output_id);
        let next_state = context.output_chains.get(&ChainId::from(anchor_id));

        match next_state {
            Some((_, Output::Anchor(next_state))) => {
                if self.state_index() == next_state.state_index() {
                    context.address_unlock(self.governor_address(), unlock)?;
                } else {
                    context.address_unlock(self.state_controller_address(), unlock)?;
                    // Only a state transition can be used to consider the anchor address for output unlocks and
                    // sender/issuer validations.
                    context.unlocked_addresses.insert(Address::from(anchor_id));
                }
            }
            None => context.address_unlock(self.governor_address(), unlock)?,
            // The next state can only be an anchor output since it is identified by an anchor chain identifier.
            Some(_) => unreachable!(),
        };

        Ok(())
    }

    /// Returns all the mana held by the output, which is potential + stored, all decayed.
    pub fn available_mana(
        &self,
        protocol_parameters: &ProtocolParameters,
        creation_index: SlotIndex,
        target_index: SlotIndex,
    ) -> Result<u64, OutputError> {
        let decayed_mana = self.decayed_mana(protocol_parameters, creation_index, target_index)?;

        decayed_mana
            .stored
            .checked_add(decayed_mana.potential)
            .ok_or(OutputError::ConsumedManaOverflow)
    }

    /// Returns the decayed stored and potential mana of the output.
    pub fn decayed_mana(
        &self,
        protocol_parameters: &ProtocolParameters,
        creation_index: SlotIndex,
        target_index: SlotIndex,
    ) -> Result<DecayedMana, OutputError> {
        let generation_amount = self.generation_amount(protocol_parameters);
        let stored_mana = protocol_parameters.mana_with_decay(self.mana(), creation_index, target_index)?;
        let potential_mana =
            protocol_parameters.generate_mana_with_decay(generation_amount, creation_index, target_index)?;

        Ok(DecayedMana {
            stored: stored_mana,
            potential: potential_mana,
        })
    }

    /// Returns the generation amount of the output.
    pub fn generation_amount(&self, protocol_parameters: &ProtocolParameters) -> u64 {
        let min_deposit = self.minimum_amount(protocol_parameters.storage_score_parameters());
        self.amount().saturating_sub(min_deposit)
    }
}

impl StorageScore for AnchorOutput {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.output_offset()
            // Type byte
            + (1 + self.packed_len() as u64) * params.data_factor() as u64
            + self.unlock_conditions.storage_score(params)
            + self.features.storage_score(params)
            + self.immutable_features.storage_score(params)
    }
}

impl WorkScore for AnchorOutput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.output()
            + self.unlock_conditions.work_score(params)
            + self.features.work_score(params)
            + self.immutable_features.work_score(params)
    }
}

impl MinimumOutputAmount for AnchorOutput {}

impl Packable for AnchorOutput {
    type UnpackError = OutputError;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.mana.pack(packer)?;
        self.anchor_id.pack(packer)?;
        self.state_index.pack(packer)?;
        self.unlock_conditions.pack(packer)?;
        self.features.pack(packer)?;
        self.immutable_features.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let amount = u64::unpack_inner(unpacker, visitor).coerce()?;

        let mana = u64::unpack_inner(unpacker, visitor).coerce()?;

        let anchor_id = AnchorId::unpack_inner(unpacker, visitor).coerce()?;
        let state_index = u32::unpack_inner(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_index_counter(&anchor_id, state_index).map_err(UnpackError::Packable)?;
        }

        let unlock_conditions = UnlockConditions::unpack(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_unlock_conditions(&unlock_conditions, &anchor_id).map_err(UnpackError::Packable)?;
        }

        let features = Features::unpack_inner(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_restricted_addresses(&unlock_conditions, Self::KIND, features.native_token(), mana)
                .map_err(UnpackError::Packable)
                .coerce()?;
            verify_allowed_features(&features, Self::ALLOWED_FEATURES)
                .map_err(UnpackError::Packable)
                .coerce()?;
        }

        let immutable_features = Features::unpack_inner(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_allowed_features(&immutable_features, Self::ALLOWED_IMMUTABLE_FEATURES)
                .map_err(UnpackError::Packable)
                .coerce()?;
        }

        Ok(Self {
            amount,
            mana,
            anchor_id,
            state_index,
            unlock_conditions,
            features,
            immutable_features,
        })
    }
}

#[inline]
fn verify_index_counter(anchor_id: &AnchorId, state_index: u32) -> Result<(), OutputError> {
    if anchor_id.is_null() && state_index != 0 {
        Err(OutputError::NonZeroStateIndexOrFoundryCounter)
    } else {
        Ok(())
    }
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions, anchor_id: &AnchorId) -> Result<(), OutputError> {
    if let Some(unlock_condition) = unlock_conditions.state_controller_address() {
        if let Address::Anchor(anchor_address) = unlock_condition.address() {
            if anchor_address.anchor_id() == anchor_id {
                return Err(OutputError::SelfControlledAnchorOutput(*anchor_id));
            }
        }
    } else {
        return Err(OutputError::MissingStateControllerUnlockCondition);
    }

    if let Some(unlock_condition) = unlock_conditions.governor_address() {
        if let Address::Anchor(anchor_address) = unlock_condition.address() {
            if anchor_address.anchor_id() == anchor_id {
                return Err(OutputError::SelfControlledAnchorOutput(*anchor_id));
            }
        }
    } else {
        return Err(OutputError::MissingGovernorUnlockCondition);
    }

    Ok(verify_allowed_unlock_conditions(
        unlock_conditions,
        AnchorOutput::ALLOWED_UNLOCK_CONDITIONS,
    )?)
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{types::block::output::unlock_condition::UnlockCondition, utils::serde::string};

    /// Describes an anchor in the ledger that can be controlled by the state and governance controllers.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct AnchorOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub mana: u64,
        pub anchor_id: AnchorId,
        pub state_index: u32,
        pub unlock_conditions: Vec<UnlockCondition>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<Feature>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<Feature>,
    }

    impl From<&AnchorOutput> for AnchorOutputDto {
        fn from(value: &AnchorOutput) -> Self {
            Self {
                kind: AnchorOutput::KIND,
                amount: value.amount(),
                mana: value.mana(),
                anchor_id: *value.anchor_id(),
                state_index: value.state_index(),
                unlock_conditions: value.unlock_conditions().to_vec(),
                features: value.features().to_vec(),
                immutable_features: value.immutable_features().to_vec(),
            }
        }
    }

    impl TryFrom<AnchorOutputDto> for AnchorOutput {
        type Error = OutputError;

        fn try_from(dto: AnchorOutputDto) -> Result<Self, Self::Error> {
            let mut builder = AnchorOutputBuilder::new_with_amount(dto.amount, dto.anchor_id)
                .with_mana(dto.mana)
                .with_state_index(dto.state_index)
                .with_features(dto.features)
                .with_immutable_features(dto.immutable_features);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(u);
            }

            builder.finish()
        }
    }

    crate::impl_serde_typed_dto!(AnchorOutput, AnchorOutputDto, "anchor output");
}

#[cfg(all(test, feature = "protocol_parameters_samples"))]
mod tests {
    use super::*;
    use crate::types::block::{
        output::anchor::dto::AnchorOutputDto, protocol::iota_mainnet_protocol_parameters,
        rand::output::rand_anchor_output,
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = iota_mainnet_protocol_parameters();
        let anchor_output = rand_anchor_output(protocol_parameters.token_supply());
        let dto = AnchorOutputDto::from(&anchor_output);
        let output = Output::Anchor(AnchorOutput::try_from(dto).unwrap());
        assert_eq!(&anchor_output, output.as_anchor());
    }
}
