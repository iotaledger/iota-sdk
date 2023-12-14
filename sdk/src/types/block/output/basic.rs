// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::{Packable, PackableExt};

use crate::types::block::{
    address::Address,
    output::{
        feature::{verify_allowed_features, Feature, FeatureFlags, Features, NativeTokenFeature},
        unlock_condition::{
            verify_allowed_unlock_conditions, AddressUnlockCondition, StorageDepositReturnUnlockCondition,
            UnlockCondition, UnlockConditionFlags, UnlockConditions,
        },
        MinimumOutputAmount, NativeToken, Output, OutputBuilderAmount, StorageScore, StorageScoreParameters,
    },
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    Error,
};

/// Builder for a [`BasicOutput`].
#[derive(Clone)]
#[must_use]
pub struct BasicOutputBuilder {
    amount: OutputBuilderAmount,
    mana: u64,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
}

impl BasicOutputBuilder {
    /// Creates a [`BasicOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn new_with_amount(amount: u64) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount))
    }

    /// Creates an [`BasicOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    #[inline(always)]
    pub fn new_with_minimum_amount(params: StorageScoreParameters) -> Self {
        Self::new(OutputBuilderAmount::MinimumAmount(params))
    }

    fn new(amount: OutputBuilderAmount) -> Self {
        Self {
            amount,
            mana: Default::default(),
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
                    let return_min_amount = Self::new_with_minimum_amount(params)
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
    pub fn finish(self) -> Result<BasicOutput, Error> {
        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions::<true>(&unlock_conditions)?;

        let features = Features::from_set(self.features)?;

        verify_features::<true>(&features)?;

        let mut output = BasicOutput {
            amount: 0,
            mana: self.mana,
            unlock_conditions,
            features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumAmount(params) => output.minimum_amount(params),
        };

        Ok(output)
    }

    /// Finishes the [`BasicOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, Error> {
        Ok(Output::Basic(self.finish()?))
    }
}

impl From<&BasicOutput> for BasicOutputBuilder {
    fn from(output: &BasicOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            mana: output.mana,
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
    /// Amount of IOTA coins held by the output.
    amount: u64,
    /// Amount of stored Mana held by the output.
    mana: u64,
    /// Define how the output can be unlocked in a transaction.
    #[packable(verify_with = verify_unlock_conditions_packable)]
    unlock_conditions: UnlockConditions,
    /// Features of the output.
    #[packable(verify_with = verify_features_packable)]
    features: Features,
}

impl BasicOutput {
    /// The [`Output`] kind of an [`BasicOutput`].
    pub const KIND: u8 = 0;
    /// The set of allowed [`UnlockCondition`]s for an [`BasicOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS
        .union(UnlockConditionFlags::STORAGE_DEPOSIT_RETURN)
        .union(UnlockConditionFlags::TIMELOCK)
        .union(UnlockConditionFlags::EXPIRATION);
    /// The set of allowed [`Feature`]s for an [`BasicOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::SENDER
        .union(FeatureFlags::METADATA)
        .union(FeatureFlags::TAG)
        .union(FeatureFlags::NATIVE_TOKEN);

    /// Creates a new [`BasicOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_amount(amount)
    }

    /// Creates a new [`BasicOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount.
    #[inline(always)]
    pub fn build_with_minimum_amount(params: StorageScoreParameters) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_minimum_amount(params)
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
    pub fn address(&self) -> &Address {
        // A BasicOutput must have an AddressUnlockCondition.
        self.unlock_conditions
            .address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    /// Returns the address of the unlock conditions if the output is a simple deposit.
    /// Simple deposit outputs are basic outputs with only an address unlock condition, no native tokens and no
    /// features. They are used to return storage deposits.
    pub fn simple_deposit_address(&self) -> Option<&Address> {
        if let [UnlockCondition::Address(address)] = self.unlock_conditions().as_ref() {
            if self.mana == 0 && self.features.is_empty() {
                return Some(address.address());
            }
        }

        None
    }

    /// Checks whether the basic output is an implicit account.
    pub fn is_implicit_account(&self) -> bool {
        if let [UnlockCondition::Address(uc)] = self.unlock_conditions().as_ref() {
            uc.address().is_implicit_account_creation()
        } else {
            false
        }
    }

    /// Computes the minimum amount of the most Basic Output.
    pub fn minimum_amount(address: &Address, params: StorageScoreParameters) -> u64 {
        // PANIC: This can never fail because the amount will always be within the valid range. Also, the actual value
        // is not important, we are only interested in the storage requirements of the type.
        BasicOutputBuilder::new_with_minimum_amount(params)
            .add_unlock_condition(AddressUnlockCondition::new(address.clone()))
            .finish()
            .unwrap()
            .amount()
    }
}

impl StorageScore for BasicOutput {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.output_offset()
            // Type byte
            + (1 + self.packed_len() as u64) * params.data_factor() as u64
            + self.unlock_conditions.storage_score(params)
            + self.features.storage_score(params)
    }
}

impl WorkScore for BasicOutput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.output() + self.unlock_conditions.work_score(params) + self.features.work_score(params)
    }
}

impl MinimumOutputAmount for BasicOutput {}

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
    pub(crate) struct BasicOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub mana: u64,
        pub unlock_conditions: Vec<UnlockCondition>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<Feature>,
    }

    impl From<&BasicOutput> for BasicOutputDto {
        fn from(value: &BasicOutput) -> Self {
            Self {
                kind: BasicOutput::KIND,
                amount: value.amount(),
                mana: value.mana(),
                unlock_conditions: value.unlock_conditions().to_vec(),
                features: value.features().to_vec(),
            }
        }
    }

    impl TryFrom<BasicOutputDto> for BasicOutput {
        type Error = Error;

        fn try_from(dto: BasicOutputDto) -> Result<Self, Self::Error> {
            let mut builder = BasicOutputBuilder::new_with_amount(dto.amount)
                .with_mana(dto.mana)
                .with_features(dto.features);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(u);
            }

            builder.finish()
        }
    }

    impl BasicOutput {
        pub fn try_from_dtos(
            amount: OutputBuilderAmount,
            mana: u64,
            unlock_conditions: Vec<UnlockCondition>,
            features: Option<Vec<Feature>>,
        ) -> Result<Self, Error> {
            let mut builder = match amount {
                OutputBuilderAmount::Amount(amount) => BasicOutputBuilder::new_with_amount(amount),
                OutputBuilderAmount::MinimumAmount(params) => BasicOutputBuilder::new_with_minimum_amount(params),
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

            builder.finish()
        }
    }

    crate::impl_serde_typed_dto!(BasicOutput, BasicOutputDto, "basic output");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::{
        output::{basic::dto::BasicOutputDto, FoundryId, SimpleTokenScheme, TokenId},
        protocol::protocol_parameters,
        rand::{
            address::rand_account_address,
            output::{
                feature::rand_allowed_features, rand_basic_output, unlock_condition::rand_address_unlock_condition,
            },
        },
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = protocol_parameters();
        let basic_output = rand_basic_output(protocol_parameters.token_supply());
        let dto = BasicOutputDto::from(&basic_output);
        let output = Output::Basic(BasicOutput::try_from(dto).unwrap());
        assert_eq!(&basic_output, output.as_basic());

        let output_split = BasicOutput::try_from_dtos(
            OutputBuilderAmount::Amount(basic_output.amount()),
            basic_output.mana(),
            basic_output.unlock_conditions().to_vec(),
            Some(basic_output.features().to_vec()),
        )
        .unwrap();
        assert_eq!(basic_output, output_split);

        let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);
        let address = rand_address_unlock_condition();

        let test_split_dto = |builder: BasicOutputBuilder| {
            let output_split = BasicOutput::try_from_dtos(
                builder.amount,
                builder.mana,
                builder.unlock_conditions.iter().cloned().collect(),
                Some(builder.features.iter().cloned().collect()),
            )
            .unwrap();
            assert_eq!(builder.finish().unwrap(), output_split);
        };

        let builder = BasicOutput::build_with_amount(100)
            .with_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(address.clone())
            .with_features(rand_allowed_features(BasicOutput::ALLOWED_FEATURES));
        test_split_dto(builder);

        let builder = BasicOutput::build_with_minimum_amount(protocol_parameters.storage_score_parameters())
            .with_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(address)
            .with_features(rand_allowed_features(BasicOutput::ALLOWED_FEATURES));
        test_split_dto(builder);
    }

    // TODO: re-enable when rent is figured out
    // #[test]
    // fn storage_deposit() {
    //     let protocol_parameters = protocol_parameters();
    //     let address_unlock = rand_address_unlock_condition();
    //     let return_address = rand_address();

    //     let builder_1 = BasicOutput::build_with_amount(1).add_unlock_condition(address_unlock.clone());

    //     let builder_2 = BasicOutput::build_with_minimum_amount(protocol_parameters.storage_score_parameters())
    //         .add_unlock_condition(address_unlock);

    //     assert_eq!(
    //         builder_1.storage_cost(protocol_parameters.storage_score_parameters()),
    //         builder_2.amount()
    //     );
    //     assert_eq!(
    //         builder_1.clone().finish_output(&protocol_parameters),
    //         Err(Error::InsufficientStorageDepositAmount {
    //             amount: 1,
    //             required: builder_1.storage_cost(protocol_parameters.storage_score_parameters())
    //         })
    //     );

    //     let builder_1 = builder_1
    //         .with_sufficient_storage_deposit(
    //             return_address.clone(),
    //             protocol_parameters.storage_score_parameters(),
    //             protocol_parameters.token_supply(),
    //         )
    //         .unwrap();

    //     let sdruc_cost =
    //         StorageDepositReturnUnlockCondition::new(return_address, 1, protocol_parameters.token_supply())
    //             .unwrap()
    //             .storage_cost(protocol_parameters.storage_score_parameters());

    //     assert_eq!(builder_1.amount(), builder_2.amount() + sdruc_cost);
    // }
}
