// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::{
    block::{
        address::Address,
        output::{
            account_id::AccountId,
            chain_id::ChainId,
            feature::FeatureFlags,
            unlock_condition::{
                verify_allowed_unlock_conditions, UnlockCondition, UnlockConditionFlags, UnlockConditions,
            },
            verify_output_amount_min, verify_output_amount_packable, verify_output_amount_supply, Output,
            OutputBuilderAmount, OutputId, Rent, RentStructure,
        },
        protocol::ProtocolParameters,
        semantic::{TransactionFailureReason, ValidationContext},
        slot::EpochIndex,
        unlock::Unlock,
        Error,
    },
    ValidationParams,
};

impl_id!(pub DelegationId, 32, "Unique identifier of the Delegation Output, which is the BLAKE2b-256 hash of the Output ID that created it.");

#[cfg(feature = "serde")]
string_serde_impl!(DelegationId);

impl From<&OutputId> for DelegationId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl DelegationId {
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }
}

/// Builder for a [`DelegationOutput`].
#[derive(Clone)]
#[must_use]
pub struct DelegationOutputBuilder {
    amount: OutputBuilderAmount,
    delegated_amount: u64,
    delegation_id: DelegationId,
    validator_id: AccountId,
    start_epoch: EpochIndex,
    end_epoch: EpochIndex,
    unlock_conditions: BTreeSet<UnlockCondition>,
}

impl DelegationOutputBuilder {
    /// Creates a [`DelegationOutputBuilder`] with a provided amount.
    pub fn new_with_amount(
        amount: u64,
        delegated_amount: u64,
        delegation_id: DelegationId,
        validator_id: AccountId,
    ) -> Self {
        Self::new(
            OutputBuilderAmount::Amount(amount),
            delegated_amount,
            delegation_id,
            validator_id,
        )
    }

    /// Creates a [`DelegationOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    pub fn new_with_minimum_storage_deposit(
        rent_structure: RentStructure,
        delegated_amount: u64,
        delegation_id: DelegationId,
        validator_id: AccountId,
    ) -> Self {
        Self::new(
            OutputBuilderAmount::MinimumStorageDeposit(rent_structure),
            delegated_amount,
            delegation_id,
            validator_id,
        )
    }

    fn new(
        amount: OutputBuilderAmount,
        delegated_amount: u64,
        delegation_id: DelegationId,
        validator_id: AccountId,
    ) -> Self {
        Self {
            amount,
            delegated_amount,
            delegation_id,
            validator_id,
            start_epoch: 0.into(),
            end_epoch: 0.into(),
            unlock_conditions: BTreeSet::new(),
        }
    }

    /// Sets the amount to the provided value.
    pub fn with_amount(mut self, amount: u64) -> Self {
        self.amount = OutputBuilderAmount::Amount(amount);
        self
    }

    /// Sets the amount to the minimum storage deposit.
    pub fn with_minimum_storage_deposit(mut self, rent_structure: RentStructure) -> Self {
        self.amount = OutputBuilderAmount::MinimumStorageDeposit(rent_structure);
        self
    }

    /// Sets the delegation ID to the provided value.
    pub fn with_delegation_id(mut self, delegation_id: DelegationId) -> Self {
        self.delegation_id = delegation_id;
        self
    }

    /// Sets the validator ID to the provided value.
    pub fn with_validator_id(mut self, validator_id: AccountId) -> Self {
        self.validator_id = validator_id;
        self
    }

    /// Sets the start epoch to the provided value.
    pub fn with_start_epoch(mut self, start_epoch: impl Into<EpochIndex>) -> Self {
        self.start_epoch = start_epoch.into();
        self
    }

    /// Sets the end epoch to the provided value.
    pub fn with_end_epoch(mut self, end_epoch: impl Into<EpochIndex>) -> Self {
        self.end_epoch = end_epoch.into();
        self
    }

    /// Adds an [`UnlockCondition`] to the builder, if one does not already exist of that type.
    pub fn add_unlock_condition(mut self, unlock_condition: impl Into<UnlockCondition>) -> Self {
        self.unlock_conditions.insert(unlock_condition.into());
        self
    }

    /// Sets the [`UnlockConditions`]s in the builder, overwriting any existing values.
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
    pub fn clear_unlock_conditions(mut self) -> Self {
        self.unlock_conditions.clear();
        self
    }

    /// Finishes the builder into a [`DelegationOutput`] without amount verification.
    pub fn finish(self) -> Result<DelegationOutput, Error> {
        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions::<true>(&unlock_conditions)?;

        let mut output = DelegationOutput {
            amount: 1u64,
            delegated_amount: self.delegated_amount,
            delegation_id: self.delegation_id,
            validator_id: self.validator_id,
            start_epoch: self.start_epoch,
            end_epoch: self.end_epoch,
            unlock_conditions,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                Output::Delegation(output.clone()).rent_cost(rent_structure)
            }
        };

        verify_output_amount_min(output.amount)?;

        Ok(output)
    }

    /// Finishes the builder into a [`DelegationOutput`] with amount verification.
    pub fn finish_with_params<'a>(
        self,
        params: impl Into<ValidationParams<'a>> + Send,
    ) -> Result<DelegationOutput, Error> {
        let output = self.finish()?;

        if let Some(token_supply) = params.into().token_supply() {
            verify_output_amount_supply(output.amount, token_supply)?;
        }

        Ok(output)
    }

    /// Finishes the [`DelegationOutputBuilder`] into an [`Output`].
    pub fn finish_output(self, token_supply: u64) -> Result<Output, Error> {
        Ok(Output::Delegation(self.finish_with_params(token_supply)?))
    }
}

impl From<&DelegationOutput> for DelegationOutputBuilder {
    fn from(output: &DelegationOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            delegated_amount: output.delegated_amount,
            delegation_id: output.delegation_id,
            validator_id: output.validator_id,
            start_epoch: output.start_epoch,
            end_epoch: output.end_epoch,
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
        }
    }
}

/// Describes a Delegation output, which delegates its contained IOTA tokens as voting power to a validator.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DelegationOutput {
    // Amount of IOTA tokens held by the output.
    amount: u64,
    /// The amount of delegated coins.
    delegated_amount: u64,
    /// Unique identifier of the Delegation Output, which is the BLAKE2b-256 hash of the Output ID that created it.
    delegation_id: DelegationId,
    /// The Account ID of the validator to which this output is delegating.
    validator_id: AccountId,
    /// The index of the first epoch for which this output delegates.
    start_epoch: EpochIndex,
    /// The index of the last epoch for which this output delegates.
    end_epoch: EpochIndex,
    unlock_conditions: UnlockConditions,
}

impl DelegationOutput {
    /// The [`Output`](crate::types::block::output::Output) kind of a [`DelegationOutput`].
    pub const KIND: u8 = 7;
    /// The set of allowed [`UnlockCondition`]s for a [`DelegationOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS;
    /// The set of allowed immutable [`Feature`]s for a [`DelegationOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::ISSUER;

    /// Creates a new [`DelegationOutputBuilder`] with a provided amount.
    pub fn build_with_amount(
        amount: u64,
        delegated_amount: u64,
        delegation_id: DelegationId,
        validator_id: AccountId,
    ) -> DelegationOutputBuilder {
        DelegationOutputBuilder::new_with_amount(amount, delegated_amount, delegation_id, validator_id)
    }

    /// Creates a new [`DelegationOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    pub fn build_with_minimum_storage_deposit(
        rent_structure: RentStructure,
        delegated_amount: u64,
        delegation_id: DelegationId,
        validator_id: AccountId,
    ) -> DelegationOutputBuilder {
        DelegationOutputBuilder::new_with_minimum_storage_deposit(
            rent_structure,
            delegated_amount,
            delegation_id,
            validator_id,
        )
    }

    /// Returns the amount of the [`DelegationOutput`].
    pub fn amount(&self) -> u64 {
        self.amount
    }

    /// Returns the delegated amount of the [`DelegationOutput`].
    pub fn delegated_amount(&self) -> u64 {
        self.delegated_amount
    }

    /// Returns the delegation ID of the [`DelegationOutput`].
    pub fn delegation_id(&self) -> &DelegationId {
        &self.delegation_id
    }

    /// Returns the delegation ID of the [`DelegationOutput`] if not null, or creates it from the output ID.
    pub fn delegation_id_non_null(&self, output_id: &OutputId) -> DelegationId {
        self.delegation_id.or_from_output_id(output_id)
    }

    /// Returns the validator ID of the [`DelegationOutput`].
    pub fn validator_id(&self) -> &AccountId {
        &self.validator_id
    }

    /// Returns the start epoch of the [`DelegationOutput`].
    pub fn start_epoch(&self) -> EpochIndex {
        self.start_epoch
    }

    /// Returns the end epoch of the [`DelegationOutput`].
    pub fn end_epoch(&self) -> EpochIndex {
        self.end_epoch
    }

    /// Returns the unlock conditions of the [`DelegationOutput`].
    pub fn unlock_conditions(&self) -> &UnlockConditions {
        &self.unlock_conditions
    }

    /// Returns the address of the [`DelegationOutput`].
    pub fn address(&self) -> &Address {
        // An DelegationOutput must have an AddressUnlockCondition.
        self.unlock_conditions
            .address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    /// Returns the chain ID of the [`DelegationOutput`].
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Delegation(self.delegation_id)
    }

    /// Tries to unlock the [`DelegationOutput`].
    pub fn unlock(
        &self,
        _output_id: &OutputId,
        unlock: &Unlock,
        inputs: &[(&OutputId, &Output)],
        context: &mut ValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        self.unlock_conditions()
            .locked_address(self.address(), context.slot_index)
            .unlock(unlock, inputs, context)
    }
}

impl Packable for DelegationOutput {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.delegated_amount.pack(packer)?;
        self.delegation_id.pack(packer)?;
        self.validator_id.pack(packer)?;
        self.start_epoch.pack(packer)?;
        self.end_epoch.pack(packer)?;
        self.unlock_conditions.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let amount = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        verify_output_amount_packable::<VERIFY>(&amount, visitor).map_err(UnpackError::Packable)?;

        let delegated_amount = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let delegation_id = DelegationId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let validator_id = AccountId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let start_epoch = EpochIndex::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let end_epoch = EpochIndex::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let unlock_conditions = UnlockConditions::unpack::<_, VERIFY>(unpacker, visitor)?;

        verify_unlock_conditions::<VERIFY>(&unlock_conditions).map_err(UnpackError::Packable)?;

        Ok(Self {
            amount,
            delegated_amount,
            delegation_id,
            validator_id,
            start_epoch,
            end_epoch,
            unlock_conditions,
        })
    }
}

fn verify_unlock_conditions<const VERIFY: bool>(unlock_conditions: &UnlockConditions) -> Result<(), Error> {
    if VERIFY {
        if unlock_conditions.address().is_none() {
            Err(Error::MissingAddressUnlockCondition)
        } else {
            verify_allowed_unlock_conditions(unlock_conditions, DelegationOutput::ALLOWED_UNLOCK_CONDITIONS)
        }
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{
            block::{
                output::{unlock_condition::dto::UnlockConditionDto, OutputBuilderAmount},
                Error,
            },
            TryFromDto,
        },
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DelegationOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub delegated_amount: u64,
        pub delegation_id: DelegationId,
        pub validator_id: AccountId,
        start_epoch: EpochIndex,
        end_epoch: EpochIndex,
        pub unlock_conditions: Vec<UnlockConditionDto>,
    }

    impl From<&DelegationOutput> for DelegationOutputDto {
        fn from(value: &DelegationOutput) -> Self {
            Self {
                kind: DelegationOutput::KIND,
                amount: value.amount(),
                delegated_amount: value.delegated_amount(),
                delegation_id: *value.delegation_id(),
                validator_id: *value.validator_id(),
                start_epoch: value.start_epoch(),
                end_epoch: value.end_epoch(),
                unlock_conditions: value.unlock_conditions().iter().map(Into::into).collect::<_>(),
            }
        }
    }

    impl TryFromDto for DelegationOutput {
        type Dto = DelegationOutputDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: Self::Dto,
            params: crate::types::ValidationParams<'_>,
        ) -> Result<Self, Self::Error> {
            let mut builder = DelegationOutputBuilder::new_with_amount(
                dto.amount,
                dto.delegated_amount,
                dto.delegation_id,
                dto.validator_id,
            )
            .with_start_epoch(dto.start_epoch)
            .with_end_epoch(dto.end_epoch);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(UnlockCondition::try_from_dto_with_params(u, &params)?);
            }

            builder.finish_with_params(params)
        }
    }

    impl DelegationOutput {
        #[allow(clippy::too_many_arguments)]
        pub fn try_from_dtos<'a>(
            amount: OutputBuilderAmount,
            delegated_amount: u64,
            delegation_id: &DelegationId,
            validator_id: &AccountId,
            start_epoch: impl Into<EpochIndex>,
            end_epoch: impl Into<EpochIndex>,
            unlock_conditions: Vec<UnlockConditionDto>,
            params: impl Into<ValidationParams<'a>> + Send,
        ) -> Result<Self, Error> {
            let params = params.into();
            let mut builder = match amount {
                OutputBuilderAmount::Amount(amount) => {
                    DelegationOutputBuilder::new_with_amount(amount, delegated_amount, *delegation_id, *validator_id)
                }
                OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                    DelegationOutputBuilder::new_with_minimum_storage_deposit(
                        rent_structure,
                        delegated_amount,
                        *delegation_id,
                        *validator_id,
                    )
                }
            }
            .with_start_epoch(start_epoch)
            .with_end_epoch(end_epoch);

            let unlock_conditions = unlock_conditions
                .into_iter()
                .map(|u| UnlockCondition::try_from_dto_with_params(u, &params))
                .collect::<Result<Vec<UnlockCondition>, Error>>()?;
            builder = builder.with_unlock_conditions(unlock_conditions);

            builder.finish_with_params(params)
        }
    }
}
