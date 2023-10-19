// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use getset::{CopyGetters, Getters};
use packable::{
    error::{UnpackError, UnpackErrorExt},
    Packable,
};

use super::{
    feature::{MetadataFeature, SenderFeature, TagFeature},
    unlock_condition::{ExpirationUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition},
    verify_output_amount, AddressUnlockCondition,
};
use crate::types::{
    block::{
        address::Address,
        output::{
            feature::{Feature, Features},
            unlock_condition::{UnlockCondition, UnlockConditions},
            verify_output_amount_min, verify_output_amount_supply, NativeToken, NativeTokens, Output,
            OutputBuilderAmount, OutputId, Rent, RentStructure,
        },
        protocol::ProtocolParameters,
        semantic::{TransactionFailureReason, ValidationContext},
        slot::SlotIndex,
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
    address_unlock_condition: AddressUnlockCondition,
    storage_deposit_return_unlock_condition: Option<StorageDepositReturnUnlockCondition>,
    timelock_unlock_condition: Option<TimelockUnlockCondition>,
    expiration_unlock_condition: Option<ExpirationUnlockCondition>,
    sender_feature: Option<SenderFeature>,
    metadata_feature: Option<MetadataFeature>,
    tag_feature: Option<TagFeature>,
}

impl BasicOutputBuilder {
    /// Creates a [`BasicOutputBuilder`] with a provided amount and an address.
    #[inline(always)]
    pub fn new_with_amount(amount: u64, address: impl Into<Address>) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount), address)
    }

    /// Creates an [`BasicOutputBuilder`] with a provided rent structure and an address.
    /// The amount will be set to the minimum storage deposit.
    #[inline(always)]
    pub fn new_with_minimum_storage_deposit(rent_structure: RentStructure, address: impl Into<Address>) -> Self {
        Self::new(OutputBuilderAmount::MinimumStorageDeposit(rent_structure), address)
    }

    fn new(amount: OutputBuilderAmount, address: impl Into<Address>) -> Self {
        Self {
            amount,
            mana: Default::default(),
            native_tokens: BTreeSet::new(),
            address_unlock_condition: AddressUnlockCondition::new(address),
            storage_deposit_return_unlock_condition: Default::default(),
            timelock_unlock_condition: Default::default(),
            expiration_unlock_condition: Default::default(),
            sender_feature: Default::default(),
            metadata_feature: Default::default(),
            tag_feature: Default::default(),
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

    /// Sets the address.
    #[inline(always)]
    pub fn with_address(mut self, address: impl Into<Address>) -> Self {
        self.address_unlock_condition = AddressUnlockCondition::from(address.into());
        self
    }

    /// Sets the address.
    #[inline(always)]
    pub fn with_address_unlock_condition(mut self, address: impl Into<AddressUnlockCondition>) -> Self {
        self.address_unlock_condition = address.into();
        self
    }

    /// Clears the unlock conditions (other than the address).
    #[inline(always)]
    pub fn clear_unlock_conditions(mut self) -> Self {
        self.storage_deposit_return_unlock_condition = None;
        self.timelock_unlock_condition = None;
        self.expiration_unlock_condition = None;
        self
    }

    /// Sets the storage deposit return unlock condition.
    #[inline(always)]
    pub fn with_storage_deposit_return_unlock_condition(
        mut self,
        storage_deposit_return_unlock_condition: impl Into<Option<StorageDepositReturnUnlockCondition>>,
    ) -> Self {
        self.storage_deposit_return_unlock_condition = storage_deposit_return_unlock_condition.into();
        self
    }

    /// Sets the timelock unlock condition.
    #[inline(always)]
    pub fn with_timelock_unlock_condition(
        mut self,
        timelock_unlock_condition: impl Into<Option<TimelockUnlockCondition>>,
    ) -> Self {
        self.timelock_unlock_condition = timelock_unlock_condition.into();
        self
    }

    /// Sets the expiration unlock condition.
    #[inline(always)]
    pub fn with_expiration_unlock_condition(
        mut self,
        expiration_unlock_condition: impl Into<Option<ExpirationUnlockCondition>>,
    ) -> Self {
        self.expiration_unlock_condition = expiration_unlock_condition.into();
        self
    }

    /// Sets the sender feature.
    #[inline(always)]
    pub fn with_sender_feature(mut self, sender_feature: impl Into<Option<SenderFeature>>) -> Self {
        self.sender_feature = sender_feature.into();
        self
    }

    /// Sets the metadata feature.
    #[inline(always)]
    pub fn with_metadata_feature(mut self, metadata_feature: impl Into<Option<MetadataFeature>>) -> Self {
        self.metadata_feature = metadata_feature.into();
        self
    }

    /// Sets the tag feature.
    #[inline(always)]
    pub fn with_tag_feature(mut self, tag_feature: impl Into<Option<TagFeature>>) -> Self {
        self.tag_feature = tag_feature.into();
        self
    }

    /// Clears the features.
    #[inline(always)]
    pub fn clear_features(mut self) -> Self {
        self.sender_feature = None;
        self.metadata_feature = None;
        self.tag_feature = None;
        self
    }

    ///
    pub fn finish(self) -> Result<BasicOutput, Error> {
        let mut output = BasicOutput {
            amount: 1u64,
            mana: self.mana,
            native_tokens: NativeTokens::from_set(self.native_tokens)?,
            address_unlock_condition: self.address_unlock_condition,
            storage_deposit_return_unlock_condition: self.storage_deposit_return_unlock_condition,
            timelock_unlock_condition: self.timelock_unlock_condition,
            expiration_unlock_condition: self.expiration_unlock_condition,
            sender_feature: self.sender_feature,
            metadata_feature: self.metadata_feature,
            tag_feature: self.tag_feature,
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
            address_unlock_condition: output.address_unlock_condition.clone(),
            storage_deposit_return_unlock_condition: output.storage_deposit_return_unlock_condition.clone(),
            timelock_unlock_condition: output.timelock_unlock_condition.clone(),
            expiration_unlock_condition: output.expiration_unlock_condition.clone(),
            sender_feature: output.sender_feature.clone(),
            metadata_feature: output.metadata_feature.clone(),
            tag_feature: output.tag_feature.clone(),
        }
    }
}

/// Describes a basic output with optional features.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Getters, CopyGetters)]
pub struct BasicOutput {
    /// Amount of IOTA coins to deposit with this output.
    #[getset(get_copy = "pub")]
    amount: u64,
    /// Amount of stored Mana held by this output.
    #[getset(get_copy = "pub")]
    mana: u64,
    /// Native tokens held by this output.
    #[getset(get = "pub")]
    native_tokens: NativeTokens,
    /// The condition for unlocking this output with an address.
    #[getset(get = "pub")]
    address_unlock_condition: AddressUnlockCondition,
    /// The optional condition for unlocking this output with a returned storage deposit.
    storage_deposit_return_unlock_condition: Option<StorageDepositReturnUnlockCondition>,
    /// The optional condition for unlocking this output with a timelock.
    timelock_unlock_condition: Option<TimelockUnlockCondition>,
    /// The optional condition for unlocking this output with an expiration.
    expiration_unlock_condition: Option<ExpirationUnlockCondition>,
    /// Optional sender feature of the output.
    sender_feature: Option<SenderFeature>,
    /// Optional metadata feature of the output.
    metadata_feature: Option<MetadataFeature>,
    /// Optional tag feature of the output.
    tag_feature: Option<TagFeature>,
}

impl BasicOutput {
    /// The [`Output`](crate::types::block::output::Output) kind of an [`BasicOutput`].
    pub const KIND: u8 = 0;

    /// Creates a new [`BasicOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, address: impl Into<Address>) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_amount(amount, address)
    }

    /// Creates a new [`BasicOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    #[inline(always)]
    pub fn build_with_minimum_storage_deposit(
        rent_structure: RentStructure,
        address: impl Into<Address>,
    ) -> BasicOutputBuilder {
        BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure, address)
    }

    /// Gets the address from the [`AddressUnlockCondition`]`.
    #[inline(always)]
    pub fn address(&self) -> &Address {
        self.address_unlock_condition.address()
    }

    pub fn storage_deposit_return_unlock_condition(&self) -> Option<&StorageDepositReturnUnlockCondition> {
        self.storage_deposit_return_unlock_condition.as_ref()
    }

    pub fn timelock_unlock_condition(&self) -> Option<&TimelockUnlockCondition> {
        self.timelock_unlock_condition.as_ref()
    }

    pub fn expiration_unlock_condition(&self) -> Option<&ExpirationUnlockCondition> {
        self.expiration_unlock_condition.as_ref()
    }

    pub fn unlock_conditions(&self) -> UnlockConditions {
        // Unwrap: safe because we know the unlock conditions are valid
        UnlockConditions::from_vec(
            [
                Some(self.address_unlock_condition.clone().into()),
                self.storage_deposit_return_unlock_condition
                    .clone()
                    .map(UnlockCondition::from),
                self.timelock_unlock_condition.clone().map(UnlockCondition::from),
                self.expiration_unlock_condition.clone().map(UnlockCondition::from),
            ]
            .into_iter()
            .filter_map(|v| v)
            .collect(),
        )
        .unwrap()
    }

    pub fn sender_feature(&self) -> Option<&SenderFeature> {
        self.sender_feature.as_ref()
    }

    pub fn metadata_feature(&self) -> Option<&MetadataFeature> {
        self.metadata_feature.as_ref()
    }

    pub fn tag_feature(&self) -> Option<&TagFeature> {
        self.tag_feature.as_ref()
    }

    pub fn features(&self) -> Features {
        // Unwrap: safe because we know the features are valid
        Features::from_vec(
            [
                self.sender_feature.clone().map(Feature::from),
                self.tag_feature.clone().map(Feature::from),
                self.metadata_feature.clone().map(Feature::from),
            ]
            .into_iter()
            .filter_map(|v| v)
            .collect(),
        )
        .unwrap()
    }

    ///
    pub fn unlock(
        &self,
        unlock: &Unlock,
        inputs: &[(&OutputId, &Output)],
        context: &mut ValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        self.expiration_unlock_condition
            .as_ref()
            .and_then(|uc| uc.return_address_expired(context.essence.creation_slot()))
            .unwrap_or_else(|| self.address())
            .unlock(unlock, inputs, context)
    }

    /// Returns the address of the unlock conditions if the output is a simple deposit.
    /// Simple deposit outputs are basic outputs with only an address unlock condition, no native tokens and no
    /// features. They are used to return storage deposits.
    pub fn simple_deposit_address(&self) -> Option<&Address> {
        if self.storage_deposit_return_unlock_condition.is_none()
            && self.timelock_unlock_condition.is_none()
            && self.expiration_unlock_condition.is_none()
            && self.sender_feature.is_none()
            && self.metadata_feature.is_none()
            && self.tag_feature.is_none()
            && self.mana == 0
            && self.native_tokens.is_empty()
        {
            return Some(self.address());
        }

        None
    }

    /// Returns the address to be unlocked.
    #[inline(always)]
    pub fn locked_address<'a>(&'a self, address: &'a Address, slot_index: SlotIndex) -> &'a Address {
        self.expiration_unlock_condition()
            .and_then(|e| e.return_address_expired(slot_index))
            .unwrap_or(address)
    }

    /// Returns whether a time lock exists and is still relevant.
    #[inline(always)]
    pub fn is_time_locked(&self, slot_index: impl Into<SlotIndex>) -> bool {
        let slot_index = slot_index.into();

        self.timelock_unlock_condition()
            .map_or(false, |timelock| slot_index < timelock.slot_index())
    }

    /// Returns whether an expiration exists and is expired.
    #[inline(always)]
    pub fn is_expired(&self, slot_index: impl Into<SlotIndex>) -> bool {
        let slot_index = slot_index.into();

        self.expiration_unlock_condition()
            .map_or(false, |expiration| slot_index >= expiration.slot_index())
    }
}

impl Packable for BasicOutput {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.mana.pack(packer)?;
        self.native_tokens.pack(packer)?;
        let unlock_conditions = self.unlock_conditions();
        unlock_conditions.pack(packer)?;
        let features = self.features();
        features.pack(packer)?;
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        params: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let amount = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        verify_output_amount(amount, params.token_supply()).map_err(UnpackError::Packable)?;
        let mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let native_tokens = NativeTokens::unpack::<_, VERIFY>(unpacker, &())?;
        let unlock_conditions = UnlockConditions::unpack::<_, VERIFY>(unpacker, params)?;
        let (
            mut address_unlock_condition,
            mut storage_deposit_return_unlock_condition,
            mut timelock_unlock_condition,
            mut expiration_unlock_condition,
        ) = Default::default();
        for (index, unlock_condition) in unlock_conditions.into_iter().enumerate() {
            match unlock_condition {
                UnlockCondition::Address(uc) => address_unlock_condition = Some(uc),
                UnlockCondition::StorageDepositReturn(uc) => storage_deposit_return_unlock_condition = Some(uc),
                UnlockCondition::Timelock(uc) => timelock_unlock_condition = Some(uc),
                UnlockCondition::Expiration(uc) => expiration_unlock_condition = Some(uc),
                _ => {
                    return Err(UnpackError::Packable(Error::DisallowedUnlockCondition {
                        index,
                        kind: unlock_condition.kind(),
                    }));
                }
            }
        }
        let Some(address_unlock_condition) = address_unlock_condition else {
            return Err(UnpackError::Packable(Error::MissingAddressUnlockCondition));
        };
        let features = Features::unpack::<_, VERIFY>(unpacker, &())?;
        let (mut sender_feature, mut metadata_feature, mut tag_feature) = Default::default();
        for (index, feature) in features.into_iter().enumerate() {
            match feature {
                Feature::Sender(f) => sender_feature = Some(f),
                Feature::Metadata(f) => metadata_feature = Some(f),
                Feature::Tag(f) => tag_feature = Some(f),
                _ => {
                    return Err(UnpackError::Packable(Error::DisallowedFeature {
                        index,
                        kind: feature.kind(),
                    }));
                }
            }
        }
        Ok(Self {
            amount,
            mana,
            native_tokens,
            address_unlock_condition,
            storage_deposit_return_unlock_condition,
            timelock_unlock_condition,
            expiration_unlock_condition,
            sender_feature,
            metadata_feature,
            tag_feature,
        })
    }
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
            if let Some(token_supply) = params.token_supply() {
                verify_output_amount(dto.amount, token_supply)?;
            }
            let (
                mut address_unlock_condition,
                mut storage_deposit_return_unlock_condition,
                mut timelock_unlock_condition,
                mut expiration_unlock_condition,
            ) = Default::default();
            for (index, unlock_condition) in dto.unlock_conditions.into_iter().enumerate() {
                match unlock_condition {
                    UnlockConditionDto::Address(uc) => address_unlock_condition = Some(uc),
                    UnlockConditionDto::StorageDepositReturn(uc) => {
                        storage_deposit_return_unlock_condition = Some(
                            StorageDepositReturnUnlockCondition::try_from_dto_with_params(uc, &params)?,
                        )
                    }
                    UnlockConditionDto::Timelock(uc) => timelock_unlock_condition = Some(uc),
                    UnlockConditionDto::Expiration(uc) => expiration_unlock_condition = Some(uc),
                    _ => {
                        return Err(Error::DisallowedUnlockCondition {
                            index,
                            kind: unlock_condition.kind(),
                        });
                    }
                }
            }
            let Some(address_unlock_condition) = address_unlock_condition else {
                return Err(Error::MissingAddressUnlockCondition);
            };
            let (mut sender_feature, mut metadata_feature, mut tag_feature) = Default::default();
            for (index, feature) in dto.features.into_iter().enumerate() {
                match feature {
                    Feature::Sender(f) => sender_feature = Some(f),
                    Feature::Metadata(f) => metadata_feature = Some(f),
                    Feature::Tag(f) => tag_feature = Some(f),
                    _ => {
                        return Err(Error::DisallowedFeature {
                            index,
                            kind: feature.kind(),
                        });
                    }
                }
            }

            Ok(Self {
                amount: dto.amount,
                mana: dto.mana,
                native_tokens: NativeTokens::from_vec(dto.native_tokens)?,
                address_unlock_condition,
                storage_deposit_return_unlock_condition,
                timelock_unlock_condition,
                expiration_unlock_condition,
                sender_feature,
                metadata_feature,
                tag_feature,
            })
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
            let (
                mut address_unlock_condition,
                mut storage_deposit_return_unlock_condition,
                mut timelock_unlock_condition,
                mut expiration_unlock_condition,
            ) = Default::default();
            for (index, unlock_condition) in unlock_conditions.into_iter().enumerate() {
                match unlock_condition {
                    UnlockConditionDto::Address(uc) => address_unlock_condition = Some(uc),
                    UnlockConditionDto::StorageDepositReturn(uc) => {
                        storage_deposit_return_unlock_condition = Some(
                            StorageDepositReturnUnlockCondition::try_from_dto_with_params(uc, &params)?,
                        )
                    }
                    UnlockConditionDto::Timelock(uc) => timelock_unlock_condition = Some(uc),
                    UnlockConditionDto::Expiration(uc) => expiration_unlock_condition = Some(uc),
                    _ => {
                        return Err(Error::DisallowedUnlockCondition {
                            index,
                            kind: unlock_condition.kind(),
                        });
                    }
                }
            }
            let Some(address_unlock_condition) = address_unlock_condition else {
                return Err(Error::MissingAddressUnlockCondition);
            };
            let (mut sender_feature, mut metadata_feature, mut tag_feature) = Default::default();
            for (index, feature) in features.into_iter().flatten().enumerate() {
                match feature {
                    Feature::Sender(f) => sender_feature = Some(f),
                    Feature::Metadata(f) => metadata_feature = Some(f),
                    Feature::Tag(f) => tag_feature = Some(f),
                    _ => {
                        return Err(Error::DisallowedFeature {
                            index,
                            kind: feature.kind(),
                        });
                    }
                }
            }
            match amount {
                OutputBuilderAmount::Amount(amount) => {
                    BasicOutputBuilder::new_with_amount(amount, address_unlock_condition.address().clone())
                }
                OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                    BasicOutputBuilder::new_with_minimum_storage_deposit(
                        rent_structure,
                        address_unlock_condition.address().clone(),
                    )
                }
            }
            .with_mana(mana)
            .with_native_tokens(native_tokens.into_iter().flatten())
            .with_storage_deposit_return_unlock_condition(storage_deposit_return_unlock_condition)
            .with_timelock_unlock_condition(timelock_unlock_condition)
            .with_expiration_unlock_condition(expiration_unlock_condition)
            .with_sender_feature(sender_feature)
            .with_metadata_feature(metadata_feature)
            .with_tag_feature(tag_feature)
            .finish_with_params(params)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        block::{
            output::{
                dto::OutputDto, unlock_condition::dto::UnlockConditionDto, FoundryId, SimpleTokenScheme, TokenId,
            },
            protocol::protocol_parameters,
            rand::{
                address::{rand_account_address, rand_address},
                output::{
                    feature::{rand_metadata_feature, rand_sender_feature, rand_tag_feature},
                    rand_basic_output,
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
        let address = rand_address();

        let test_split_dto = |builder: BasicOutputBuilder| {
            let output_split = BasicOutput::try_from_dtos(
                builder.amount,
                builder.mana,
                Some(builder.native_tokens.iter().copied().collect()),
                [
                    Some(builder.address_unlock_condition.clone().into()),
                    builder
                        .storage_deposit_return_unlock_condition
                        .clone()
                        .map(UnlockCondition::from),
                    builder.timelock_unlock_condition.clone().map(UnlockCondition::from),
                    builder.expiration_unlock_condition.clone().map(UnlockCondition::from),
                ]
                .into_iter()
                .filter_map(|v| v.as_ref().map(UnlockConditionDto::from))
                .collect(),
                Some(
                    [
                        builder.sender_feature.clone().map(Feature::from),
                        builder.metadata_feature.clone().map(Feature::from),
                        builder.tag_feature.clone().map(Feature::from),
                    ]
                    .into_iter()
                    .filter_map(|v| v)
                    .collect(),
                ),
                protocol_parameters.token_supply(),
            )
            .unwrap();
            assert_eq!(
                builder.finish_with_params(protocol_parameters.token_supply()).unwrap(),
                output_split
            );
        };

        let builder = BasicOutput::build_with_amount(100, address.clone())
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .with_sender_feature(rand_sender_feature())
            .with_metadata_feature(rand_metadata_feature())
            .with_tag_feature(rand_tag_feature());
        test_split_dto(builder);

        let builder = BasicOutput::build_with_minimum_storage_deposit(protocol_parameters.rent_structure(), address)
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .with_sender_feature(rand_sender_feature())
            .with_metadata_feature(rand_metadata_feature())
            .with_tag_feature(rand_tag_feature());
        test_split_dto(builder);
    }
}
