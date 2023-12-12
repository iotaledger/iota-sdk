// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::{BTreeMap, BTreeSet};
use core::cmp::Ordering;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::{Packer, SlicePacker},
    unpacker::Unpacker,
    Packable, PackableExt,
};
use primitive_types::U256;

use crate::types::block::{
    address::{AccountAddress, Address},
    output::{
        account::AccountId,
        feature::{verify_allowed_features, Feature, FeatureFlags, Features, NativeTokenFeature},
        unlock_condition::{verify_allowed_unlock_conditions, UnlockCondition, UnlockConditionFlags, UnlockConditions},
        ChainId, MinimumOutputAmount, NativeToken, Output, OutputBuilderAmount, StorageScore, StorageScoreParameters,
        TokenId, TokenScheme,
    },
    payload::signed_transaction::{TransactionCapabilities, TransactionCapabilityFlag},
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    semantic::{StateTransitionError, TransactionFailureReason},
    Error,
};

crate::impl_id!(
    /// Unique identifier of the [`FoundryOutput`](crate::types::block::output::FoundryOutput),
    /// which is the BLAKE2b-256 hash of the [`OutputId`](crate::types::block::output::OutputId) that created it.
    pub FoundryId {
        pub const LENGTH: usize = 38;
    }
);

impl From<TokenId> for FoundryId {
    fn from(token_id: TokenId) -> Self {
        Self::new(*token_id)
    }
}

impl FoundryId {
    /// Builds a new [`FoundryId`] from its components.
    pub fn build(account_address: &AccountAddress, serial_number: u32, token_scheme_kind: u8) -> Self {
        let mut bytes = [0u8; Self::LENGTH];
        let mut packer = SlicePacker::new(&mut bytes);

        // PANIC: packing to an array of the correct length can't fail.
        Address::Account(*account_address).pack(&mut packer).unwrap();
        serial_number.pack(&mut packer).unwrap();
        token_scheme_kind.pack(&mut packer).unwrap();

        Self::new(bytes)
    }

    /// Returns the [`AccountAddress`] of the [`FoundryId`].
    pub fn account_address(&self) -> AccountAddress {
        // PANIC: the lengths are known.
        AccountAddress::from(AccountId::new(self.0[1..AccountId::LENGTH + 1].try_into().unwrap()))
    }

    /// Returns the serial number of the [`FoundryId`].
    pub fn serial_number(&self) -> u32 {
        // PANIC: the lengths are known.
        u32::from_le_bytes(
            self.0[AccountId::LENGTH + 1..AccountId::LENGTH + 1 + core::mem::size_of::<u32>()]
                .try_into()
                .unwrap(),
        )
    }

    /// Returns the [`TokenScheme`] kind of the [`FoundryId`].
    pub fn token_scheme_kind(&self) -> u8 {
        // PANIC: the length is known.
        *self.0.last().unwrap()
    }
}

/// Builder for a [`FoundryOutput`].
#[derive(Clone)]
#[must_use]
pub struct FoundryOutputBuilder {
    amount: OutputBuilderAmount,
    serial_number: u32,
    token_scheme: TokenScheme,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
    immutable_features: BTreeSet<Feature>,
}

impl FoundryOutputBuilder {
    /// Creates a [`FoundryOutputBuilder`] with a provided amount.
    pub fn new_with_amount(amount: u64, serial_number: u32, token_scheme: TokenScheme) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount), serial_number, token_scheme)
    }

    /// Creates a [`FoundryOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    pub fn new_with_minimum_amount(
        params: StorageScoreParameters,
        serial_number: u32,
        token_scheme: TokenScheme,
    ) -> Self {
        Self::new(OutputBuilderAmount::MinimumAmount(params), serial_number, token_scheme)
    }

    fn new(amount: OutputBuilderAmount, serial_number: u32, token_scheme: TokenScheme) -> Self {
        Self {
            amount,
            serial_number,
            token_scheme,
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

    /// Sets the serial number to the provided value.
    #[inline(always)]
    pub fn with_serial_number(mut self, serial_number: u32) -> Self {
        self.serial_number = serial_number;
        self
    }

    /// Sets the token scheme to the provided value.
    #[inline(always)]
    pub fn with_token_scheme(mut self, token_scheme: TokenScheme) -> Self {
        self.token_scheme = token_scheme;
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

    /// Sets the native token of the builder.
    #[inline(always)]
    pub fn with_native_token(self, native_token: impl Into<NativeToken>) -> Self {
        self.add_feature(NativeTokenFeature::from(native_token.into()))
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
    pub fn finish(self) -> Result<FoundryOutput, Error> {
        if self.serial_number == 0 {
            return Err(Error::InvalidFoundryZeroSerialNumber);
        }

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions)?;

        let features = Features::from_set(self.features)?;

        verify_allowed_features(&features, FoundryOutput::ALLOWED_FEATURES)?;

        let immutable_features = Features::from_set(self.immutable_features)?;

        verify_allowed_features(&immutable_features, FoundryOutput::ALLOWED_IMMUTABLE_FEATURES)?;

        let mut output = FoundryOutput {
            amount: 0,
            serial_number: self.serial_number,
            token_scheme: self.token_scheme,
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

    /// Finishes the [`FoundryOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, Error> {
        Ok(Output::Foundry(self.finish()?))
    }
}

impl From<&FoundryOutput> for FoundryOutputBuilder {
    fn from(output: &FoundryOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            serial_number: output.serial_number,
            token_scheme: output.token_scheme.clone(),
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
            features: output.features.iter().cloned().collect(),
            immutable_features: output.immutable_features.iter().cloned().collect(),
        }
    }
}

/// Describes a foundry output that is controlled by an account.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FoundryOutput {
    /// Amount of IOTA coins held by the output.
    amount: u64,
    /// The serial number of the foundry with respect to the controlling account.
    serial_number: u32,
    /// Define the supply control scheme of the native tokens controlled by the foundry.
    token_scheme: TokenScheme,
    /// Define how the output can be unlocked in a transaction.
    unlock_conditions: UnlockConditions,
    /// Features of the output.
    features: Features,
    /// Immutable features of the output.
    immutable_features: Features,
}

impl FoundryOutput {
    /// The [`Output`] kind of a [`FoundryOutput`].
    pub const KIND: u8 = 3;
    /// The set of allowed [`UnlockCondition`]s for a [`FoundryOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::IMMUTABLE_ACCOUNT_ADDRESS;
    /// The set of allowed [`Feature`]s for a [`FoundryOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::METADATA.union(FeatureFlags::NATIVE_TOKEN);
    /// The set of allowed immutable [`Feature`]s for a [`FoundryOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::METADATA;

    /// Creates a new [`FoundryOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, serial_number: u32, token_scheme: TokenScheme) -> FoundryOutputBuilder {
        FoundryOutputBuilder::new_with_amount(amount, serial_number, token_scheme)
    }

    /// Creates a new [`FoundryOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    #[inline(always)]
    pub fn build_with_minimum_amount(
        params: StorageScoreParameters,
        serial_number: u32,
        token_scheme: TokenScheme,
    ) -> FoundryOutputBuilder {
        FoundryOutputBuilder::new_with_minimum_amount(params, serial_number, token_scheme)
    }

    ///
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount
    }

    ///
    #[inline(always)]
    pub fn serial_number(&self) -> u32 {
        self.serial_number
    }

    ///
    #[inline(always)]
    pub fn token_scheme(&self) -> &TokenScheme {
        &self.token_scheme
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
    pub fn native_token(&self) -> Option<&NativeToken> {
        self.features.native_token().map(|f| f.native_token())
    }

    ///
    #[inline(always)]
    pub fn immutable_features(&self) -> &Features {
        &self.immutable_features
    }

    ///
    #[inline(always)]
    pub fn account_address(&self) -> &AccountAddress {
        // A FoundryOutput must have an ImmutableAccountAddressUnlockCondition.
        self.unlock_conditions
            .immutable_account_address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    /// Returns the [`FoundryId`] of the [`FoundryOutput`].
    pub fn id(&self) -> FoundryId {
        FoundryId::build(self.account_address(), self.serial_number, self.token_scheme.kind())
    }

    /// Returns the [`TokenId`] of the [`FoundryOutput`].
    pub fn token_id(&self) -> TokenId {
        TokenId::from(self.id())
    }

    ///
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Foundry(self.id())
    }

    // Transition, just without full SemanticValidationContext
    pub(crate) fn transition_inner(
        current_state: &Self,
        next_state: &Self,
        input_native_tokens: &BTreeMap<TokenId, U256>,
        output_native_tokens: &BTreeMap<TokenId, U256>,
        capabilities: &TransactionCapabilities,
    ) -> Result<(), StateTransitionError> {
        if current_state.account_address() != next_state.account_address()
            || current_state.serial_number != next_state.serial_number
            || current_state.immutable_features != next_state.immutable_features
        {
            return Err(StateTransitionError::MutatedImmutableField);
        }

        let token_id = next_state.token_id();
        let input_tokens = input_native_tokens.get(&token_id).copied().unwrap_or_default();
        let output_tokens = output_native_tokens.get(&token_id).copied().unwrap_or_default();
        let TokenScheme::Simple(ref current_token_scheme) = current_state.token_scheme;
        let TokenScheme::Simple(ref next_token_scheme) = next_state.token_scheme;

        if current_token_scheme.maximum_supply() != next_token_scheme.maximum_supply() {
            return Err(StateTransitionError::MutatedImmutableField);
        }

        if current_token_scheme.minted_tokens() > next_token_scheme.minted_tokens()
            || current_token_scheme.melted_tokens() > next_token_scheme.melted_tokens()
        {
            return Err(StateTransitionError::NonMonotonicallyIncreasingNativeTokens);
        }

        match input_tokens.cmp(&output_tokens) {
            Ordering::Less => {
                // Mint

                // This can't underflow as it is known that current_minted_tokens <= next_minted_tokens.
                let minted_diff = next_token_scheme.minted_tokens() - current_token_scheme.minted_tokens();
                // This can't underflow as it is known that input_tokens < output_tokens (Ordering::Less).
                let token_diff = output_tokens - input_tokens;

                if minted_diff != token_diff {
                    return Err(StateTransitionError::InconsistentNativeTokensMint);
                }

                if current_token_scheme.melted_tokens() != next_token_scheme.melted_tokens() {
                    return Err(StateTransitionError::InconsistentNativeTokensMint);
                }
            }
            Ordering::Equal => {
                // Transition

                if current_token_scheme.minted_tokens() != next_token_scheme.minted_tokens()
                    || current_token_scheme.melted_tokens() != next_token_scheme.melted_tokens()
                {
                    return Err(StateTransitionError::InconsistentNativeTokensTransition);
                }
            }
            Ordering::Greater => {
                // Melt / Burn

                if current_token_scheme.melted_tokens() != next_token_scheme.melted_tokens()
                    && current_token_scheme.minted_tokens() != next_token_scheme.minted_tokens()
                {
                    return Err(StateTransitionError::InconsistentNativeTokensMeltBurn);
                }

                // This can't underflow as it is known that current_melted_tokens <= next_melted_tokens.
                let melted_diff = next_token_scheme.melted_tokens() - current_token_scheme.melted_tokens();
                // This can't underflow as it is known that input_tokens > output_tokens (Ordering::Greater).
                let token_diff = input_tokens - output_tokens;

                if melted_diff > token_diff {
                    return Err(StateTransitionError::InconsistentNativeTokensMeltBurn);
                }

                let burned_diff = token_diff - melted_diff;

                if !burned_diff.is_zero() && !capabilities.has_capability(TransactionCapabilityFlag::BurnNativeTokens) {
                    return Err(TransactionFailureReason::TransactionCapabilityManaBurningNotAllowed)?;
                }
            }
        }

        Ok(())
    }
}

impl StorageScore for FoundryOutput {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.output_offset()
            // Type byte
            + (1 + self.packed_len() as u64) * params.data_factor() as u64
            + self.unlock_conditions.storage_score(params)
            + self.features.storage_score(params)
            + self.immutable_features.storage_score(params)
    }
}

impl WorkScore for FoundryOutput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.output()
            + self.token_scheme.work_score(params)
            + self.unlock_conditions.work_score(params)
            + self.features.work_score(params)
            + self.immutable_features.work_score(params)
    }
}

impl MinimumOutputAmount for FoundryOutput {}

impl Packable for FoundryOutput {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.serial_number.pack(packer)?;
        self.token_scheme.pack(packer)?;
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

        let serial_number = u32::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let token_scheme = TokenScheme::unpack::<_, VERIFY>(unpacker, &())?;

        let unlock_conditions = UnlockConditions::unpack::<_, VERIFY>(unpacker, visitor)?;

        if VERIFY {
            verify_unlock_conditions(&unlock_conditions).map_err(UnpackError::Packable)?;
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
            serial_number,
            token_scheme,
            unlock_conditions,
            features,
            immutable_features,
        })
    }
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions) -> Result<(), Error> {
    if unlock_conditions.immutable_account_address().is_none() {
        Err(Error::MissingAddressUnlockCondition)
    } else {
        verify_allowed_unlock_conditions(unlock_conditions, FoundryOutput::ALLOWED_UNLOCK_CONDITIONS)
    }
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
    pub(crate) struct FoundryOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        pub serial_number: u32,
        pub token_scheme: TokenScheme,
        pub unlock_conditions: Vec<UnlockCondition>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<Feature>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<Feature>,
    }

    impl From<&FoundryOutput> for FoundryOutputDto {
        fn from(value: &FoundryOutput) -> Self {
            Self {
                kind: FoundryOutput::KIND,
                amount: value.amount(),
                serial_number: value.serial_number(),
                token_scheme: value.token_scheme().clone(),
                unlock_conditions: value.unlock_conditions().to_vec(),
                features: value.features().to_vec(),
                immutable_features: value.immutable_features().to_vec(),
            }
        }
    }

    impl TryFrom<FoundryOutputDto> for FoundryOutput {
        type Error = Error;

        fn try_from(dto: FoundryOutputDto) -> Result<Self, Self::Error> {
            let mut builder: FoundryOutputBuilder =
                FoundryOutputBuilder::new_with_amount(dto.amount, dto.serial_number, dto.token_scheme);

            for b in dto.features {
                builder = builder.add_feature(b);
            }

            for b in dto.immutable_features {
                builder = builder.add_immutable_feature(b);
            }

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(u);
            }

            builder.finish()
        }
    }

    impl FoundryOutput {
        #[allow(clippy::too_many_arguments)]
        pub fn try_from_dtos(
            amount: OutputBuilderAmount,
            serial_number: u32,
            token_scheme: TokenScheme,
            unlock_conditions: Vec<UnlockCondition>,
            features: Option<Vec<Feature>>,
            immutable_features: Option<Vec<Feature>>,
        ) -> Result<Self, Error> {
            let mut builder = match amount {
                OutputBuilderAmount::Amount(amount) => {
                    FoundryOutputBuilder::new_with_amount(amount, serial_number, token_scheme)
                }
                OutputBuilderAmount::MinimumAmount(params) => {
                    FoundryOutputBuilder::new_with_minimum_amount(params, serial_number, token_scheme)
                }
            };

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

    crate::impl_serde_typed_dto!(FoundryOutput, FoundryOutputDto, "foundry output");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::{
        output::{
            foundry::dto::FoundryOutputDto, unlock_condition::ImmutableAccountAddressUnlockCondition, FoundryId,
            SimpleTokenScheme, TokenId,
        },
        protocol::protocol_parameters,
        rand::{
            address::rand_account_address,
            output::{
                feature::{rand_allowed_features, rand_metadata_feature},
                rand_foundry_output, rand_token_scheme,
            },
        },
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = protocol_parameters();
        let foundry_output = rand_foundry_output(protocol_parameters.token_supply());
        let dto = FoundryOutputDto::from(&foundry_output);
        let output = Output::Foundry(FoundryOutput::try_from(dto).unwrap());
        assert_eq!(&foundry_output, output.as_foundry());

        let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);

        let test_split_dto = |builder: FoundryOutputBuilder| {
            let output_split = FoundryOutput::try_from_dtos(
                builder.amount,
                builder.serial_number,
                builder.token_scheme.clone(),
                builder.unlock_conditions.iter().cloned().collect(),
                Some(builder.features.iter().cloned().collect()),
                Some(builder.immutable_features.iter().cloned().collect()),
            )
            .unwrap();
            assert_eq!(builder.finish().unwrap(), output_split);
        };

        let builder = FoundryOutput::build_with_amount(100, 123, rand_token_scheme())
            .with_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(rand_account_address()))
            .add_immutable_feature(rand_metadata_feature())
            .with_features(rand_allowed_features(FoundryOutput::ALLOWED_FEATURES));
        test_split_dto(builder);

        let builder = FoundryOutput::build_with_minimum_amount(
            protocol_parameters.storage_score_parameters(),
            123,
            rand_token_scheme(),
        )
        .with_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
        .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(rand_account_address()))
        .add_immutable_feature(rand_metadata_feature())
        .with_features(rand_allowed_features(FoundryOutput::ALLOWED_FEATURES));
        test_split_dto(builder);
    }
}
