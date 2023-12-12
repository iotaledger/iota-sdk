// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

use crate::types::block::{
    address::{Address, NftAddress},
    output::{
        feature::{verify_allowed_features, Feature, FeatureFlags, Features},
        unlock_condition::{
            verify_allowed_unlock_conditions, AddressUnlockCondition, StorageDepositReturnUnlockCondition,
            UnlockCondition, UnlockConditionFlags, UnlockConditions,
        },
        BasicOutputBuilder, ChainId, MinimumOutputAmount, Output, OutputBuilderAmount, OutputId, StorageScore,
        StorageScoreParameters,
    },
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    semantic::StateTransitionError,
    Error,
};

crate::impl_id!(
    /// Unique identifier of the [`NftOutput`](crate::types::block::output::NftOutput),
    /// which is the BLAKE2b-256 hash of the [`OutputId`](crate::types::block::output::OutputId) that created it.
    pub NftId {
        pub const LENGTH: usize = 32;
    }
);

impl From<&OutputId> for NftId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl NftId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }
}

impl From<NftId> for Address {
    fn from(value: NftId) -> Self {
        Self::Nft(NftAddress::new(value))
    }
}

/// Builder for an [`NftOutput`].
#[derive(Clone)]
#[must_use]
pub struct NftOutputBuilder {
    amount: OutputBuilderAmount,
    mana: u64,
    nft_id: NftId,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
    immutable_features: BTreeSet<Feature>,
}

impl NftOutputBuilder {
    /// Creates an [`NftOutputBuilder`] with a provided amount.
    pub fn new_with_amount(amount: u64, nft_id: NftId) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount), nft_id)
    }

    /// Creates an [`NftOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    pub fn new_with_minimum_amount(params: StorageScoreParameters, nft_id: NftId) -> Self {
        Self::new(OutputBuilderAmount::MinimumAmount(params), nft_id)
    }

    fn new(amount: OutputBuilderAmount, nft_id: NftId) -> Self {
        Self {
            amount,
            mana: Default::default(),
            nft_id,
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

    /// Sets the NFT ID to the provided value.
    #[inline(always)]
    pub fn with_nft_id(mut self, nft_id: NftId) -> Self {
        self.nft_id = nft_id;
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

    /// Adds a storage deposit return unlock condition if one is needed to cover the current amount
    /// (i.e. `amount < minimum_amount`). This will increase the total amount to satisfy the `minimum_amount` with
    /// the additional unlock condition that will return the remainder to the provided `return_address`.
    pub fn with_sufficient_storage_deposit(
        mut self,
        return_address: impl Into<Address>,
        params: StorageScoreParameters,
    ) -> Result<Self, Error> {
        Ok(match self.amount {
            OutputBuilderAmount::Amount(amount) => {
                let return_address = return_address.into();
                // Get the current storage requirement
                let minimum_amount = self.clone().finish()?.minimum_amount(params);
                // Check whether we already have enough funds to cover it
                if amount < minimum_amount {
                    // Get the projected minimum amount of the return output
                    let return_min_amount = BasicOutputBuilder::new_with_minimum_amount(params)
                        .add_unlock_condition(AddressUnlockCondition::new(return_address.clone()))
                        .finish()?
                        .amount();
                    // Add a temporary storage deposit unlock condition so the new storage requirement can be calculated
                    self =
                        self.add_unlock_condition(StorageDepositReturnUnlockCondition::new(return_address.clone(), 1)?);
                    // Get the min amount of the output with the added storage deposit return unlock condition
                    let min_amount_with_sdruc = self.clone().finish()?.minimum_amount(params);
                    // If the return storage cost and amount are less than the required min
                    let (amount, sdruc_amount) = if min_amount_with_sdruc >= return_min_amount + amount {
                        // Then sending storage_cost_with_sdruc covers both minimum requirements
                        (min_amount_with_sdruc, min_amount_with_sdruc - amount)
                    } else {
                        // Otherwise we must use the total of the return minimum and the original amount
                        // which is unfortunately more than the storage_cost_with_sdruc
                        (return_min_amount + amount, return_min_amount)
                    };
                    // Add the required storage deposit unlock condition and the additional storage amount
                    self.with_amount(amount)
                        .replace_unlock_condition(StorageDepositReturnUnlockCondition::new(
                            return_address,
                            sdruc_amount,
                        )?)
                } else {
                    self
                }
            }
            OutputBuilderAmount::MinimumAmount(_) => self,
        })
    }

    ///
    pub fn finish(self) -> Result<NftOutput, Error> {
        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions, &self.nft_id)?;

        let features = Features::from_set(self.features)?;

        verify_allowed_features(&features, NftOutput::ALLOWED_FEATURES)?;

        let immutable_features = Features::from_set(self.immutable_features)?;

        verify_allowed_features(&immutable_features, NftOutput::ALLOWED_IMMUTABLE_FEATURES)?;

        let mut output = NftOutput {
            amount: 0,
            mana: self.mana,
            nft_id: self.nft_id,
            unlock_conditions,
            features,
            immutable_features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumAmount(params) => output.minimum_amount(params),
        };

        Ok(output)
    }

    /// Finishes the [`NftOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, Error> {
        Ok(Output::Nft(self.finish()?))
    }
}

impl From<&NftOutput> for NftOutputBuilder {
    fn from(output: &NftOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            mana: output.mana,
            nft_id: output.nft_id,
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
            features: output.features.iter().cloned().collect(),
            immutable_features: output.immutable_features.iter().cloned().collect(),
        }
    }
}

/// Describes an NFT output, a globally unique token with metadata attached.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NftOutput {
    /// Amount of IOTA coins held by the output.
    amount: u64,
    /// Amount of stored Mana held by the output.
    mana: u64,
    /// Unique identifier of the NFT.
    nft_id: NftId,
    /// Define how the output can be unlocked in a transaction.
    unlock_conditions: UnlockConditions,
    /// Features of the output.
    features: Features,
    /// Immutable features of the output.
    immutable_features: Features,
}

impl NftOutput {
    /// The [`Output`] kind of an [`NftOutput`].
    pub const KIND: u8 = 4;
    /// The set of allowed [`UnlockCondition`]s for an [`NftOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS
        .union(UnlockConditionFlags::STORAGE_DEPOSIT_RETURN)
        .union(UnlockConditionFlags::TIMELOCK)
        .union(UnlockConditionFlags::EXPIRATION);
    /// The set of allowed [`Feature`]s for an [`NftOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::SENDER
        .union(FeatureFlags::METADATA)
        .union(FeatureFlags::TAG);
    /// The set of allowed immutable [`Feature`]s for an [`NftOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::ISSUER.union(FeatureFlags::METADATA);

    /// Creates a new [`NftOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, nft_id: NftId) -> NftOutputBuilder {
        NftOutputBuilder::new_with_amount(amount, nft_id)
    }

    /// Creates a new [`NftOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    #[inline(always)]
    pub fn build_with_minimum_amount(params: StorageScoreParameters, nft_id: NftId) -> NftOutputBuilder {
        NftOutputBuilder::new_with_minimum_amount(params, nft_id)
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
    pub fn nft_id(&self) -> &NftId {
        &self.nft_id
    }

    /// Returns the nft ID if not null, or creates it from the output ID.
    #[inline(always)]
    pub fn nft_id_non_null(&self, output_id: &OutputId) -> NftId {
        self.nft_id.or_from_output_id(output_id)
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
    pub fn address(&self) -> &Address {
        // An NftOutput must have an AddressUnlockCondition.
        self.unlock_conditions
            .address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Nft(self.nft_id)
    }

    /// Returns the nft address for this output.
    pub fn nft_address(&self, output_id: &OutputId) -> NftAddress {
        NftAddress::new(self.nft_id_non_null(output_id))
    }

    // Transition, just without full SemanticValidationContext
    pub(crate) fn transition_inner(current_state: &Self, next_state: &Self) -> Result<(), StateTransitionError> {
        if current_state.immutable_features != next_state.immutable_features {
            return Err(StateTransitionError::MutatedImmutableField);
        }
        Ok(())
    }
}

impl StorageScore for NftOutput {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.output_offset()
            // Type byte
            + (1 + self.packed_len() as u64) * params.data_factor() as u64
            + self.unlock_conditions.storage_score(params)
            + self.features.storage_score(params)
            + self.immutable_features.storage_score(params)
    }
}

impl WorkScore for NftOutput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.output()
            + self.unlock_conditions.work_score(params)
            + self.features.work_score(params)
            + self.immutable_features.work_score(params)
    }
}

impl MinimumOutputAmount for NftOutput {}

impl Packable for NftOutput {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.mana.pack(packer)?;
        self.nft_id.pack(packer)?;
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

        let mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let nft_id = NftId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let unlock_conditions = UnlockConditions::unpack::<_, VERIFY>(unpacker, visitor)?;

        if VERIFY {
            verify_unlock_conditions(&unlock_conditions, &nft_id).map_err(UnpackError::Packable)?;
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
            mana,
            nft_id,
            unlock_conditions,
            features,
            immutable_features,
        })
    }
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions, nft_id: &NftId) -> Result<(), Error> {
    if let Some(unlock_condition) = unlock_conditions.address() {
        if let Address::Nft(nft_address) = unlock_condition.address() {
            if nft_address.nft_id() == nft_id {
                return Err(Error::SelfDepositNft(*nft_id));
            }
        }
    } else {
        return Err(Error::MissingAddressUnlockCondition);
    }

    verify_allowed_unlock_conditions(unlock_conditions, NftOutput::ALLOWED_UNLOCK_CONDITIONS)
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::block::{output::unlock_condition::UnlockCondition, Error},
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct NftOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub mana: u64,
        pub nft_id: NftId,
        pub unlock_conditions: Vec<UnlockCondition>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<Feature>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<Feature>,
    }

    impl From<&NftOutput> for NftOutputDto {
        fn from(value: &NftOutput) -> Self {
            Self {
                kind: NftOutput::KIND,
                amount: value.amount(),
                mana: value.mana(),
                nft_id: *value.nft_id(),
                unlock_conditions: value.unlock_conditions().to_vec(),
                features: value.features().to_vec(),
                immutable_features: value.immutable_features().to_vec(),
            }
        }
    }

    impl TryFrom<NftOutputDto> for NftOutput {
        type Error = Error;

        fn try_from(dto: NftOutputDto) -> Result<Self, Self::Error> {
            let mut builder = NftOutputBuilder::new_with_amount(dto.amount, dto.nft_id)
                .with_mana(dto.mana)
                .with_features(dto.features)
                .with_immutable_features(dto.immutable_features);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(u);
            }

            builder.finish()
        }
    }

    impl NftOutput {
        #[allow(clippy::too_many_arguments)]
        pub fn try_from_dtos(
            amount: OutputBuilderAmount,
            mana: u64,
            nft_id: &NftId,
            unlock_conditions: Vec<UnlockCondition>,
            features: Option<Vec<Feature>>,
            immutable_features: Option<Vec<Feature>>,
        ) -> Result<Self, Error> {
            let mut builder = match amount {
                OutputBuilderAmount::Amount(amount) => NftOutputBuilder::new_with_amount(amount, *nft_id),
                OutputBuilderAmount::MinimumAmount(params) => {
                    NftOutputBuilder::new_with_minimum_amount(params, *nft_id)
                }
            }
            .with_mana(mana);

            let unlock_conditions = unlock_conditions
                .into_iter()
                .map(UnlockCondition::from)
                .collect::<Vec<UnlockCondition>>();
            builder = builder.with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                builder = builder.with_features(features);
            }

            if let Some(immutable_features) = immutable_features {
                builder = builder.with_immutable_features(immutable_features);
            }

            builder.finish()
        }
    }

    crate::impl_serde_typed_dto!(NftOutput, NftOutputDto, "nft output");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::{
        output::nft::dto::NftOutputDto,
        protocol::protocol_parameters,
        rand::output::{
            feature::rand_allowed_features, rand_nft_output, unlock_condition::rand_address_unlock_condition,
        },
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = protocol_parameters();
        let nft_output = rand_nft_output(protocol_parameters.token_supply());
        let dto = NftOutputDto::from(&nft_output);
        let output = Output::Nft(NftOutput::try_from(dto).unwrap());
        assert_eq!(&nft_output, output.as_nft());

        let output_split = NftOutput::try_from_dtos(
            OutputBuilderAmount::Amount(nft_output.amount()),
            nft_output.mana(),
            nft_output.nft_id(),
            nft_output.unlock_conditions().to_vec(),
            Some(nft_output.features().to_vec()),
            Some(nft_output.immutable_features().to_vec()),
        )
        .unwrap();
        assert_eq!(nft_output, output_split);

        let test_split_dto = |builder: NftOutputBuilder| {
            let output_split = NftOutput::try_from_dtos(
                builder.amount,
                builder.mana,
                &builder.nft_id,
                builder.unlock_conditions.iter().cloned().collect(),
                Some(builder.features.iter().cloned().collect()),
                Some(builder.immutable_features.iter().cloned().collect()),
            )
            .unwrap();
            assert_eq!(builder.finish().unwrap(), output_split);
        };

        let builder = NftOutput::build_with_amount(100, NftId::null())
            .add_unlock_condition(rand_address_unlock_condition())
            .with_features(rand_allowed_features(NftOutput::ALLOWED_FEATURES))
            .with_immutable_features(rand_allowed_features(NftOutput::ALLOWED_IMMUTABLE_FEATURES));
        test_split_dto(builder);

        let builder =
            NftOutput::build_with_minimum_amount(protocol_parameters.storage_score_parameters(), NftId::null())
                .add_unlock_condition(rand_address_unlock_condition())
                .with_features(rand_allowed_features(NftOutput::ALLOWED_FEATURES))
                .with_immutable_features(rand_allowed_features(NftOutput::ALLOWED_IMMUTABLE_FEATURES));
        test_split_dto(builder);
    }
}
