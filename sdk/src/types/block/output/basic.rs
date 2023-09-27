// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::Packable;

use crate::types::{
    block::{
        address::{Address, Ed25519Address},
        output::{
            feature::{verify_allowed_features, Feature, FeatureFlags, Features},
            unlock_condition::{
                verify_allowed_unlock_conditions, AddressUnlockCondition, StorageDepositReturnUnlockCondition,
                UnlockCondition, UnlockConditionFlags, UnlockConditions,
            },
            verify_output_amount_min, verify_output_amount_packable, verify_output_amount_supply, NativeToken,
            NativeTokens, Output, OutputBuilderAmount, OutputId, Rent, RentBuilder, RentStructure,
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
    /// The amount will be set to the rent cost of the resulting output.
    #[inline(always)]
    pub fn new_with_minimum_amount(rent_structure: RentStructure) -> Self {
        Self::new(OutputBuilderAmount::RentCost(rent_structure))
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

    /// Gets the current amount as a concrete value.
    pub fn amount(&self) -> u64 {
        match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::RentCost(rent_structure) => self.rent_cost(rent_structure),
        }
    }

    /// Sets the amount to the provided value.
    #[inline(always)]
    pub fn with_amount(mut self, amount: u64) -> Self {
        self.amount = OutputBuilderAmount::Amount(amount);
        self
    }

    /// Sets the amount to the rent cost.
    #[inline(always)]
    pub fn with_minimum_amount(mut self, rent_structure: RentStructure) -> Self {
        self.amount = OutputBuilderAmount::RentCost(rent_structure);
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

    /// Adds a storage deposit return unlock condition if one is needed to cover the current amount
    /// (i.e. `amount < rent_cost`). This will increase the total amount to equal the `rent_cost` with
    /// the additional unlock condition that will return the remainder to the provided `return_address`.
    pub fn with_sufficient_storage_deposit(
        mut self,
        return_address: impl Into<Address>,
        rent_structure: RentStructure,
        token_supply: u64,
    ) -> Result<Self, Error> {
        Ok(match self.amount {
            OutputBuilderAmount::Amount(amount) => {
                let return_address = return_address.into();
                // Get the current rent requirement
                let rent_cost = self.rent_cost(rent_structure);
                // Check whether we already have enough funds to cover it
                if amount < rent_cost {
                    // Get the projected rent cost of the return output
                    let return_rent_cost = Self::new_with_amount(0)
                        .add_unlock_condition(AddressUnlockCondition::new(return_address))
                        .rent_cost(rent_structure);
                    // Add a temporary storage deposit unlock condition so the new rent requirement can be calculated
                    self = self.add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                        return_address,
                        1,
                        token_supply,
                    )?);
                    // Get the rent cost of the output with the added storage deposit return unlock condition
                    let rent_cost_with_sdruc = self.rent_cost(rent_structure);
                    // If the return rent cost and amount are less than the required min
                    let (amount, sdruc_amount) = if rent_cost_with_sdruc >= return_rent_cost + amount {
                        // Then sending rent_cost_with_sdruc covers both minimum requirements
                        (rent_cost_with_sdruc, rent_cost_with_sdruc - amount)
                    } else {
                        // Otherwise we must use the total of the return minimum and the original amount
                        // which is unfortunately more than the rent_cost_with_sdruc
                        (return_rent_cost + amount, return_rent_cost)
                    };
                    // Add the required storage deposit unlock condition and the additional rent amount
                    self.with_amount(amount)
                        .replace_unlock_condition(StorageDepositReturnUnlockCondition::new(
                            return_address,
                            sdruc_amount,
                            token_supply,
                        )?)
                } else {
                    self
                }
            }
            OutputBuilderAmount::RentCost(_) => self,
        })
    }

    pub fn min_storage_deposit_amount(&self, rent_structure: RentStructure, token_supply: u64) -> Result<u64, Error> {
        Ok(self
            .clone()
            // TODO: Probably not good to use ed25519 always here, even if technically it's the same for now..
            .with_sufficient_storage_deposit(Ed25519Address::null(), rent_structure, token_supply)?
            .amount())
    }

    ///
    pub fn finish(self) -> Result<BasicOutput, Error> {
        let amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::RentCost(rent_structure) => self.rent_cost(rent_structure),
        };
        verify_output_amount_min(amount)?;

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions::<true>(&unlock_conditions)?;

        let features = Features::from_set(self.features)?;

        verify_features::<true>(&features)?;

        Ok(BasicOutput {
            amount,
            mana: self.mana,
            native_tokens: NativeTokens::from_set(self.native_tokens)?,
            unlock_conditions,
            features,
        })
    }

    ///
    pub fn finish_with_params<'a>(self, params: impl Into<ValidationParams<'a>> + Send) -> Result<BasicOutput, Error> {
        let output = self.finish()?;
        let params = params.into();

        if let Some(token_supply) = params.token_supply() {
            verify_output_amount_supply(output.amount, token_supply)?;
        }

        if let Some(params) = params.protocol_parameters() {
            let rent_cost = output.rent_cost(params.rent_structure());
            if output.amount < rent_cost {
                return Err(Error::InsufficientStorageDepositAmount {
                    amount: output.amount,
                    required: rent_cost,
                });
            }
        }

        Ok(output)
    }

    /// Finishes the [`BasicOutputBuilder`] into an [`Output`].
    pub fn finish_output<'a>(self, params: impl Into<ValidationParams<'a>> + Send) -> Result<Output, Error> {
        Ok(Output::Basic(self.finish_with_params(params)?))
    }
}

impl Rent for BasicOutputBuilder {
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder {
        Output::byte_offset(builder)
            // Kind
            .data_field::<u8>()
            // Amount
            .data_field::<u64>()
            // Mana
            .data_field::<u64>()
            // Native Tokens
            .data_field::<u8>()
            .iter_field(&self.native_tokens)
            // Unlock Conditions
            .data_field::<u8>()
            .iter_field(&self.unlock_conditions)
            // Features
            .data_field::<u8>()
            .iter_field(&self.features)
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
    pub fn build_with_minimum_amount(rent_structure: RentStructure) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_minimum_amount(rent_structure)
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

impl Rent for BasicOutput {
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder {
        Output::byte_offset(builder)
            // Kind
            .data_field::<u8>()
            // Amount
            .data_field::<u64>()
            // Mana
            .data_field::<u64>()
            // Native Tokens
            .packable_data_field(&self.native_tokens)
            // Unlock Conditions
            .packable_data_field(&self.unlock_conditions)
            // Features
            .packable_data_field(&self.features)
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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{
            block::{output::unlock_condition::dto::UnlockConditionDto, Error},
            TryFromDto,
        },
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BasicOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub mana: u64,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        pub unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
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

    impl TryFromDto for BasicOutput {
        type Dto = BasicOutputDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
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
                OutputBuilderAmount::RentCost(rent_structure) => {
                    BasicOutputBuilder::new_with_minimum_amount(rent_structure)
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::{
        block::{
            output::{dto::OutputDto, FoundryId, SimpleTokenScheme, TokenId},
            protocol::protocol_parameters,
            rand::{
                address::{rand_account_address, rand_address},
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

        let builder = BasicOutput::build_with_minimum_amount(protocol_parameters.rent_structure())
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(address)
            .with_features(rand_allowed_features(BasicOutput::ALLOWED_FEATURES));
        test_split_dto(builder);
    }

    #[test]
    fn storage_deposit() {
        let protocol_parameters = protocol_parameters();
        let address_unlock = rand_address_unlock_condition();
        let return_address = rand_address();

        let builder_1 = BasicOutput::build_with_amount(1).add_unlock_condition(address_unlock);

        let builder_2 = BasicOutput::build_with_minimum_amount(protocol_parameters.rent_structure())
            .add_unlock_condition(address_unlock);

        assert_eq!(
            builder_1.rent_cost(protocol_parameters.rent_structure()),
            builder_2.amount()
        );
        assert_eq!(
            builder_1.clone().finish_output(&protocol_parameters),
            Err(Error::InsufficientStorageDepositAmount {
                amount: 1,
                required: builder_1.rent_cost(protocol_parameters.rent_structure())
            })
        );

        let builder_1 = builder_1
            .with_sufficient_storage_deposit(
                return_address,
                protocol_parameters.rent_structure(),
                protocol_parameters.token_supply(),
            )
            .unwrap();

        let sdruc_cost =
            StorageDepositReturnUnlockCondition::new(return_address, 1, protocol_parameters.token_supply())
                .unwrap()
                .rent_cost(protocol_parameters.rent_structure());

        assert_eq!(builder_1.amount(), builder_2.amount() + sdruc_cost);
    }
}
