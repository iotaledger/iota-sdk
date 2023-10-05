// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::Packable;

use crate::types::{
    block::{
        address::Address,
        output::{
            feature::{verify_allowed_features, Feature, FeatureFlags, Features},
            unlock_condition::{
                verify_allowed_unlock_conditions, UnlockCondition, UnlockConditionFlags, UnlockConditions,
            },
            verify_output_amount_min, verify_output_amount_packable, verify_output_amount_supply, NativeToken,
            NativeTokens, Output, OutputBuilderAmount, OutputId, Rent, RentStructure,
        },
        protocol::ProtocolParameters,
        semantic::{TransactionFailureReason, ValidationContext},
        unlock::Unlock,
        Error,
    },
    ValidationParams,
};

/// Builder for a [`BasicOutput`].
#[derive(Clone)]
#[must_use]
pub struct BasicOutputBuilder {
    amount: OutputBuilderAmount,
    mana: u64,
    native_tokens: BTreeSet<NativeToken>,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
}

impl BasicOutputBuilder {
    /// Creates a [`BasicOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn new_with_amount(amount: u64) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount))
    }

    /// Creates an [`BasicOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    #[inline(always)]
    pub fn new_with_minimum_storage_deposit(rent_structure: RentStructure) -> Self {
        Self::new(OutputBuilderAmount::MinimumStorageDeposit(rent_structure))
    }

    fn new(amount: OutputBuilderAmount) -> Self {
        Self {
            amount,
            mana: Default::default(),
            native_tokens: BTreeSet::new(),
            unlock_conditions: BTreeSet::new(),
            features: BTreeSet::new(),
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

    /// Sets the mana to the provided value.
    #[inline(always)]
    pub fn with_mana(mut self, mana: u64) -> Self {
        self.mana = mana;
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

    ///
    pub fn finish(self) -> Result<BasicOutput, Error> {
        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions::<true>(&unlock_conditions)?;

        let features = Features::from_set(self.features)?;

        verify_features::<true>(&features)?;

        let mut output = BasicOutput {
            amount: 1u64,
            mana: self.mana,
            native_tokens: NativeTokens::from_set(self.native_tokens)?,
            unlock_conditions,
            features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                Output::Basic(output.clone()).rent_cost(rent_structure)
            }
        };

        verify_output_amount_min(output.amount)?;

        Ok(output)
    }

    ///
    pub fn finish_with_params<'a>(self, params: impl Into<ValidationParams<'a>> + Send) -> Result<BasicOutput, Error> {
        let output = self.finish()?;

        if let Some(token_supply) = params.into().token_supply() {
            verify_output_amount_supply(output.amount, token_supply)?;
        }

        Ok(output)
    }

    /// Finishes the [`BasicOutputBuilder`] into an [`Output`].
    pub fn finish_output<'a>(self, params: impl Into<ValidationParams<'a>> + Send) -> Result<Output, Error> {
        Ok(Output::Basic(self.finish_with_params(params)?))
    }
}

impl From<&BasicOutput> for BasicOutputBuilder {
    fn from(output: &BasicOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            mana: output.mana,
            native_tokens: output.native_tokens.iter().copied().collect(),
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
            features: output.features.iter().cloned().collect(),
        }
    }
}

/// Describes a basic output with optional features.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct BasicOutput {
    /// Amount of IOTA coins to deposit with this output.
    #[packable(verify_with = verify_output_amount_packable)]
    amount: u64,
    /// Amount of stored Mana held by this output.
    mana: u64,
    /// Native tokens held by this output.
    native_tokens: NativeTokens,
    /// Define how the output can be unlocked in a transaction.
    #[packable(verify_with = verify_unlock_conditions_packable)]
    unlock_conditions: UnlockConditions,
    #[packable(verify_with = verify_features_packable)]
    /// Features of the output.
    features: Features,
}

impl BasicOutput {
    /// The [`Output`](crate::types::block::output::Output) kind of an [`BasicOutput`].
    pub const KIND: u8 = 3;

    /// The set of allowed [`UnlockCondition`]s for an [`BasicOutput`].
    const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS
        .union(UnlockConditionFlags::STORAGE_DEPOSIT_RETURN)
        .union(UnlockConditionFlags::TIMELOCK)
        .union(UnlockConditionFlags::EXPIRATION);
    /// The set of allowed [`Feature`]s for an [`BasicOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::SENDER
        .union(FeatureFlags::METADATA)
        .union(FeatureFlags::TAG);

    /// Creates a new [`BasicOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_amount(amount)
    }

    /// Creates a new [`BasicOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    #[inline(always)]
    pub fn build_with_minimum_storage_deposit(rent_structure: RentStructure) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
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
    pub fn native_tokens(&self) -> &NativeTokens {
        &self.native_tokens
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
    pub fn address(&self) -> &Address {
        // A BasicOutput must have an AddressUnlockCondition.
        self.unlock_conditions
            .address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    pub fn unlock(
        &self,
        _output_id: &OutputId,
        unlock: &Unlock,
        inputs: &[(&OutputId, &Output)],
        context: &mut ValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        self.unlock_conditions()
            .locked_address(self.address(), context.essence.creation_slot())
            .unlock(unlock, inputs, context)
    }

    /// Returns the address of the unlock conditions if the output is a simple deposit.
    /// Simple deposit outputs are basic outputs with only an address unlock condition, no native tokens and no
    /// features. They are used to return storage deposits.
    pub fn simple_deposit_address(&self) -> Option<&Address> {
        if let [UnlockCondition::Address(address)] = self.unlock_conditions().as_ref() {
            if self.mana == 0 && self.native_tokens.is_empty() && self.features.is_empty() {
                return Some(address.address());
            }
        }

        None
    }
}

fn verify_unlock_conditions<const VERIFY: bool>(unlock_conditions: &UnlockConditions) -> Result<(), Error> {
    if VERIFY {
        if unlock_conditions.address().is_none() {
            Err(Error::MissingAddressUnlockCondition)
        } else {
            verify_allowed_unlock_conditions(unlock_conditions, BasicOutput::ALLOWED_UNLOCK_CONDITIONS)
        }
    } else {
        Ok(())
    }
}

fn verify_unlock_conditions_packable<const VERIFY: bool>(
    unlock_conditions: &UnlockConditions,
    _: &ProtocolParameters,
) -> Result<(), Error> {
    verify_unlock_conditions::<VERIFY>(unlock_conditions)
}

fn verify_features<const VERIFY: bool>(features: &Features) -> Result<(), Error> {
    if VERIFY {
        verify_allowed_features(features, BasicOutput::ALLOWED_FEATURES)
    } else {
        Ok(())
    }
}

fn verify_features_packable<const VERIFY: bool>(features: &Features, _: &ProtocolParameters) -> Result<(), Error> {
    verify_features::<VERIFY>(features)
}

pub(crate) mod dto {
    use alloc::vec::Vec;

    use super::*;
    use crate::{
        types::{
            block::{output::unlock_condition::dto::UnlockConditionDto, Error},
            TryFromDto,
        },
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(
        feature = "serde_types",
        derive(serde::Serialize, serde::Deserialize),
        serde(rename_all = "camelCase")
    )]
    pub struct BasicOutputDto {
        #[cfg_attr(feature = "serde_types", serde(rename = "type"))]
        pub kind: u8,
        #[cfg_attr(feature = "serde_types", serde(with = "string"))]
        pub amount: u64,
        #[cfg_attr(feature = "serde_types", serde(with = "string"))]
        pub mana: u64,
        #[cfg_attr(feature = "serde_types", serde(skip_serializing_if = "Vec::is_empty", default))]
        pub native_tokens: Vec<NativeToken>,
        pub unlock_conditions: Vec<UnlockConditionDto>,
        #[cfg_attr(feature = "serde_types", serde(skip_serializing_if = "Vec::is_empty", default))]
        pub features: Vec<Feature>,
    }

    impl From<&BasicOutput> for BasicOutputDto {
        fn from(value: &BasicOutput) -> Self {
            Self {
                kind: BasicOutput::KIND,
                amount: value.amount(),
                mana: value.mana(),
                native_tokens: value.native_tokens().to_vec(),
                unlock_conditions: value.unlock_conditions().iter().map(Into::into).collect::<_>(),
                features: value.features().to_vec(),
            }
        }
    }

    impl TryFromDto<BasicOutputDto> for BasicOutput {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: BasicOutputDto,
            params: ValidationParams<'_>,
        ) -> Result<Self, Self::Error> {
            let mut builder = BasicOutputBuilder::new_with_amount(dto.amount)
                .with_native_tokens(dto.native_tokens)
                .with_mana(dto.mana)
                .with_features(dto.features);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(UnlockCondition::try_from_dto_with_params(u, &params)?);
            }

            builder.finish_with_params(params)
        }
    }

    impl BasicOutput {
        pub fn try_from_dtos<'a>(
            amount: OutputBuilderAmount,
            mana: u64,
            native_tokens: Option<Vec<NativeToken>>,
            unlock_conditions: Vec<UnlockConditionDto>,
            features: Option<Vec<Feature>>,
            params: impl Into<ValidationParams<'a>> + Send,
        ) -> Result<Self, Error> {
            let params = params.into();
            let mut builder = match amount {
                OutputBuilderAmount::Amount(amount) => BasicOutputBuilder::new_with_amount(amount),
                OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                    BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
                }
            }
            .with_mana(mana);

            if let Some(native_tokens) = native_tokens {
                builder = builder.with_native_tokens(native_tokens);
            }

            let unlock_conditions = unlock_conditions
                .into_iter()
                .map(|u| UnlockCondition::try_from_dto_with_params(u, &params))
                .collect::<Result<Vec<UnlockCondition>, Error>>()?;
            builder = builder.with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                builder = builder.with_features(features);
            }

            builder.finish_with_params(params)
        }
    }
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for BasicOutput {
        fn to_json(&self) -> Value {
            let mut res = crate::json! ({
                "type": Self::KIND,
                "amount": self.amount(),
                "mana": self.mana(),
                "unlockConditions": self.unlock_conditions().as_list(),
            });
            if !self.native_tokens().is_empty() {
                res["nativeTokens"] = self.native_tokens().to_json();
            }
            if !self.features().is_empty() {
                res["features"] = self.features().to_json();
            }
            res
        }
    }

    impl ToJson for dto::BasicOutputDto {
        fn to_json(&self) -> Value {
            let mut res = crate::json! ({
                "type": BasicOutput::KIND,
                "amount": self.amount,
                "mana": self.mana,
                "unlockConditions": self.unlock_conditions,
            });
            if !self.native_tokens.is_empty() {
                res["nativeTokens"] = self.native_tokens.to_json();
            }
            if !self.features.is_empty() {
                res["features"] = self.features.to_json();
            }
            res
        }
    }

    impl FromJson for dto::BasicOutputDto {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != BasicOutput::KIND {
                return Err(Error::invalid_type::<BasicOutput>(BasicOutput::KIND, &value["type"]));
            }
            Ok(Self {
                kind: BasicOutput::KIND,
                amount: value["amount"].to_u64()?,
                mana: value["mana"].to_u64()?,
                native_tokens: value["nativeTokens"].take_opt_or_default()?,
                unlock_conditions: value["unlockCondition"].take_vec()?,
                features: value["features"].take_opt_or_default()?,
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::{
        block::{
            output::{dto::OutputDto, FoundryId, SimpleTokenScheme, TokenId},
            protocol::protocol_parameters,
            rand::{
                address::rand_account_address,
                output::{
                    feature::rand_allowed_features, rand_basic_output, unlock_condition::rand_address_unlock_condition,
                },
            },
        },
        TryFromDto,
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = protocol_parameters();
        let output = rand_basic_output(protocol_parameters.token_supply());
        let dto = OutputDto::Basic((&output).into());
        let output_unver = Output::try_from_dto(dto.clone()).unwrap();
        assert_eq!(&output, output_unver.as_basic());
        let output_ver = Output::try_from_dto_with_params(dto, &protocol_parameters).unwrap();
        assert_eq!(&output, output_ver.as_basic());

        let output_split = BasicOutput::try_from_dtos(
            OutputBuilderAmount::Amount(output.amount()),
            output.mana(),
            Some(output.native_tokens().to_vec()),
            output.unlock_conditions().iter().map(Into::into).collect(),
            Some(output.features().to_vec()),
            protocol_parameters.token_supply(),
        )
        .unwrap();
        assert_eq!(output, output_split);

        let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);
        let address = rand_address_unlock_condition();

        let test_split_dto = |builder: BasicOutputBuilder| {
            let output_split = BasicOutput::try_from_dtos(
                builder.amount,
                builder.mana,
                Some(builder.native_tokens.iter().copied().collect()),
                builder.unlock_conditions.iter().map(Into::into).collect(),
                Some(builder.features.iter().cloned().collect()),
                protocol_parameters.token_supply(),
            )
            .unwrap();
            assert_eq!(
                builder.finish_with_params(protocol_parameters.token_supply()).unwrap(),
                output_split
            );
        };

        let builder = BasicOutput::build_with_amount(100)
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(address)
            .with_features(rand_allowed_features(BasicOutput::ALLOWED_FEATURES));
        test_split_dto(builder);

        let builder = BasicOutput::build_with_minimum_storage_deposit(protocol_parameters.rent_structure())
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(address)
            .with_features(rand_allowed_features(BasicOutput::ALLOWED_FEATURES));
        test_split_dto(builder);
    }
}
