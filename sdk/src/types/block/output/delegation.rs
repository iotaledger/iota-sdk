// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use packable::{Packable, PackableExt};

use crate::types::block::{
    address::{AccountAddress, Address, AddressError},
    output::{
        chain_id::ChainId,
        unlock_condition::{
            verify_allowed_unlock_conditions, verify_restricted_addresses, UnlockCondition, UnlockConditionFlags,
            UnlockConditions,
        },
        DecayedMana, MinimumOutputAmount, Output, OutputBuilderAmount, OutputError, OutputId, StorageScore,
        StorageScoreParameters,
    },
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    semantic::TransactionFailureReason,
    slot::{EpochIndex, SlotIndex},
};

crate::impl_id!(
    /// Unique identifier of the [`DelegationOutput`](crate::types::block::output::DelegationOutput),
    /// which is the BLAKE2b-256 hash of the [`OutputId`](crate::types::block::output::OutputId) that created it.
    pub DelegationId {
        pub const LENGTH: usize = 32;
    }
);

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

// TODO maybe can be removed as part of https://github.com/iotaledger/iota-sdk/issues/1938
#[derive(Copy, Clone)]
pub enum DelegatedAmount {
    Amount(u64),
    MinimumAmount(StorageScoreParameters),
}

/// Builder for a [`DelegationOutput`].
#[derive(Clone)]
#[must_use]
pub struct DelegationOutputBuilder {
    // TODO https://github.com/iotaledger/iota-sdk/issues/1938
    amount: Option<OutputBuilderAmount>,
    delegated_amount: DelegatedAmount,
    delegation_id: DelegationId,
    validator_address: AccountAddress,
    start_epoch: EpochIndex,
    end_epoch: EpochIndex,
    unlock_conditions: BTreeSet<UnlockCondition>,
}

impl DelegationOutputBuilder {
    /// Creates a [`DelegationOutputBuilder`] with a provided amount.
    /// Will set the delegated amount field to match.
    pub fn new_with_amount(amount: u64, delegation_id: DelegationId, validator_address: AccountAddress) -> Self {
        Self::new(DelegatedAmount::Amount(amount), delegation_id, validator_address)
    }

    /// Creates a [`DelegationOutputBuilder`] with a provided amount, unless it is below the minimum.
    /// Will set the delegated amount field to match.
    pub fn new_with_amount_or_minimum(
        amount: u64,
        delegation_id: DelegationId,
        validator_address: AccountAddress,
        params: StorageScoreParameters,
    ) -> Self {
        Self::new(DelegatedAmount::Amount(amount), delegation_id, validator_address)
            .with_amount_or_minimum(amount, params)
    }

    /// Creates a [`DelegationOutputBuilder`] with provided storage score parameters.
    /// The amount and delegated amount will be set to the minimum required amount of the resulting output.
    pub fn new_with_minimum_amount(
        params: StorageScoreParameters,
        delegation_id: DelegationId,
        validator_address: AccountAddress,
    ) -> Self {
        Self::new(DelegatedAmount::MinimumAmount(params), delegation_id, validator_address)
    }

    fn new(delegated_amount: DelegatedAmount, delegation_id: DelegationId, validator_address: AccountAddress) -> Self {
        Self {
            amount: None,
            delegated_amount,
            delegation_id,
            validator_address,
            start_epoch: 0.into(),
            end_epoch: 0.into(),
            unlock_conditions: BTreeSet::new(),
        }
    }

    /// Sets the amount to the provided value.
    pub fn with_amount(mut self, amount: u64) -> Self {
        self.amount = Some(OutputBuilderAmount::Amount(amount));
        self
    }

    /// Sets the amount to the provided value, unless it is below the minimum.
    #[inline(always)]
    pub fn with_amount_or_minimum(mut self, amount: u64, params: StorageScoreParameters) -> Self {
        self.amount = Some(OutputBuilderAmount::AmountOrMinimum(amount, params));
        self
    }

    /// Sets the amount to the minimum required amount.
    pub fn with_minimum_amount(mut self, params: StorageScoreParameters) -> Self {
        if matches!(self.delegated_amount, DelegatedAmount::MinimumAmount(_)) {
            self.amount = None;
        } else {
            self.amount = Some(OutputBuilderAmount::MinimumAmount(params));
        }
        self
    }

    /// Sets the delegation ID to the provided value.
    pub fn with_delegation_id(mut self, delegation_id: DelegationId) -> Self {
        self.delegation_id = delegation_id;
        self
    }

    /// Sets the validator address to the provided value.
    pub fn with_validator_address(mut self, validator_address: AccountAddress) -> Self {
        self.validator_address = validator_address;
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

    /// Finishes the builder into a [`DelegationOutput`] without parameters verification.
    pub fn finish(self) -> Result<DelegationOutput, OutputError> {
        let validator_address = Address::from(self.validator_address);

        verify_validator_address(&validator_address)?;

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions)?;
        verify_restricted_addresses(&unlock_conditions, DelegationOutput::KIND, None, 0)?;

        let mut output = DelegationOutput {
            amount: 0,
            delegated_amount: 0,
            delegation_id: self.delegation_id,
            validator_address,
            start_epoch: self.start_epoch,
            end_epoch: self.end_epoch,
            unlock_conditions,
        };

        match self.delegated_amount {
            DelegatedAmount::Amount(amount) => {
                output.delegated_amount = amount;
                output.amount = self.amount.map_or(amount, |builder_amount| match builder_amount {
                    OutputBuilderAmount::Amount(amount) => amount,
                    OutputBuilderAmount::AmountOrMinimum(amount, params) => output.minimum_amount(params).max(amount),
                    OutputBuilderAmount::MinimumAmount(params) => output.minimum_amount(params),
                });
            }
            DelegatedAmount::MinimumAmount(params) => {
                let min = output.minimum_amount(params);
                output.delegated_amount = min;
                output.amount = self.amount.map_or(min, |builder_amount| match builder_amount {
                    OutputBuilderAmount::Amount(amount) => amount,
                    OutputBuilderAmount::AmountOrMinimum(amount, params) => output.minimum_amount(params).max(amount),
                    OutputBuilderAmount::MinimumAmount(params) => output.minimum_amount(params),
                });
            }
        }

        Ok(output)
    }

    /// Finishes the [`DelegationOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, OutputError> {
        Ok(Output::Delegation(self.finish()?))
    }
}

impl From<&DelegationOutput> for DelegationOutputBuilder {
    fn from(output: &DelegationOutput) -> Self {
        Self {
            amount: Some(OutputBuilderAmount::Amount(output.amount)),
            delegated_amount: DelegatedAmount::Amount(output.delegated_amount),
            delegation_id: output.delegation_id,
            validator_address: *output.validator_address.as_account(),
            start_epoch: output.start_epoch,
            end_epoch: output.end_epoch,
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
        }
    }
}

/// An output which delegates its contained IOTA coins as voting power to a validator.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Packable)]
#[packable(unpack_error = OutputError)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(verify_with = verify_delegation_output)]
pub struct DelegationOutput {
    /// Amount of IOTA coins held by the output.
    amount: u64,
    /// Amount of delegated IOTA coins.
    delegated_amount: u64,
    /// Unique identifier of the delegation output.
    delegation_id: DelegationId,
    /// Account address of the validator to which this output is delegating.
    #[packable(verify_with = verify_validator_address_packable)]
    #[packable(unpack_error_with = OutputError::ValidatorAddress)]
    validator_address: Address,
    /// Index of the first epoch for which this output delegates.
    start_epoch: EpochIndex,
    /// Index of the last epoch for which this output delegates.
    end_epoch: EpochIndex,
    /// Define how the output can be unlocked in a transaction.
    #[packable(verify_with = verify_unlock_conditions_packable)]
    unlock_conditions: UnlockConditions,
}

impl DelegationOutput {
    /// The [`Output`] kind of a [`DelegationOutput`].
    pub const KIND: u8 = 5;
    /// The set of allowed [`UnlockCondition`]s for a [`DelegationOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS;

    /// Creates a new [`DelegationOutputBuilder`] with a provided amount.
    pub fn build_with_amount(
        amount: u64,
        delegation_id: DelegationId,
        validator_address: AccountAddress,
    ) -> DelegationOutputBuilder {
        DelegationOutputBuilder::new_with_amount(amount, delegation_id, validator_address)
    }

    /// Creates a new [`DelegationOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount.
    pub fn build_with_minimum_amount(
        params: StorageScoreParameters,
        delegation_id: DelegationId,
        validator_address: AccountAddress,
    ) -> DelegationOutputBuilder {
        DelegationOutputBuilder::new_with_minimum_amount(params, delegation_id, validator_address)
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

    /// Returns the validator address of the [`DelegationOutput`].
    pub fn validator_address(&self) -> &AccountAddress {
        self.validator_address.as_account()
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

    /// Returns whether the output can claim rewards based on its current and next state in a transaction.
    pub fn can_claim_rewards(&self, next_state: Option<&Self>) -> bool {
        next_state.is_none()
    }

    /// Returns the chain ID of the [`DelegationOutput`].
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Delegation(self.delegation_id)
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
        let min_deposit = self.minimum_amount(protocol_parameters.storage_score_parameters());
        let generation_amount = self.amount().saturating_sub(min_deposit);
        let potential_mana =
            protocol_parameters.generate_mana_with_decay(generation_amount, creation_index, target_index)?;

        Ok(DecayedMana {
            stored: 0,
            potential: potential_mana,
        })
    }

    // Transition, just without full SemanticValidationContext.
    pub(crate) fn transition_inner(current_state: &Self, next_state: &Self) -> Result<(), TransactionFailureReason> {
        if !current_state.delegation_id.is_null() || next_state.delegation_id.is_null() {
            return Err(TransactionFailureReason::DelegationOutputTransitionedTwice);
        }

        if current_state.delegated_amount != next_state.delegated_amount
            || current_state.start_epoch != next_state.start_epoch
            || current_state.validator_address != next_state.validator_address
        {
            return Err(TransactionFailureReason::DelegationModified);
        }

        Ok(())
    }
}

impl StorageScore for DelegationOutput {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.output_offset()
            // Type byte
            + (1 + self.packed_len() as u64) * params.data_factor() as u64
            + params.delegation_offset()
            + self.unlock_conditions.storage_score(params)
    }
}

impl WorkScore for DelegationOutput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.output() + self.unlock_conditions.work_score(params)
    }
}

impl MinimumOutputAmount for DelegationOutput {}

fn verify_validator_address(validator_address: &Address) -> Result<(), OutputError> {
    if let Address::Account(validator_address) = validator_address {
        if validator_address.is_null() {
            return Err(OutputError::NullDelegationValidatorId);
        }
    } else {
        return Err(OutputError::ValidatorAddress(AddressError::Kind(
            validator_address.kind(),
        )));
    }

    Ok(())
}

fn verify_validator_address_packable(validator_address: &Address, _: &ProtocolParameters) -> Result<(), OutputError> {
    verify_validator_address(validator_address)
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions) -> Result<(), OutputError> {
    if unlock_conditions.address().is_none() {
        Err(OutputError::MissingAddressUnlockCondition)
    } else {
        Ok(verify_allowed_unlock_conditions(
            unlock_conditions,
            DelegationOutput::ALLOWED_UNLOCK_CONDITIONS,
        )?)
    }
}

fn verify_unlock_conditions_packable(
    unlock_conditions: &UnlockConditions,
    _: &ProtocolParameters,
) -> Result<(), OutputError> {
    verify_unlock_conditions(unlock_conditions)
}

fn verify_delegation_output(output: &DelegationOutput, _: &ProtocolParameters) -> Result<(), OutputError> {
    Ok(verify_restricted_addresses(
        output.unlock_conditions(),
        DelegationOutput::KIND,
        None,
        0,
    )?)
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{types::block::output::unlock_condition::UnlockCondition, utils::serde::string};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DelegationOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub delegated_amount: u64,
        pub delegation_id: DelegationId,
        pub validator_address: AccountAddress,
        start_epoch: EpochIndex,
        end_epoch: EpochIndex,
        pub unlock_conditions: Vec<UnlockCondition>,
    }

    impl From<&DelegationOutput> for DelegationOutputDto {
        fn from(value: &DelegationOutput) -> Self {
            Self {
                kind: DelegationOutput::KIND,
                amount: value.amount(),
                delegated_amount: value.delegated_amount(),
                delegation_id: *value.delegation_id(),
                validator_address: *value.validator_address(),
                start_epoch: value.start_epoch(),
                end_epoch: value.end_epoch(),
                unlock_conditions: value.unlock_conditions().to_vec(),
            }
        }
    }

    impl TryFrom<DelegationOutputDto> for DelegationOutput {
        type Error = OutputError;

        fn try_from(dto: DelegationOutputDto) -> Result<Self, Self::Error> {
            let mut builder = DelegationOutputBuilder::new_with_amount(
                dto.delegated_amount,
                dto.delegation_id,
                dto.validator_address,
            )
            .with_amount(dto.amount)
            .with_start_epoch(dto.start_epoch)
            .with_end_epoch(dto.end_epoch);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(u);
            }

            builder.finish()
        }
    }

    crate::impl_serde_typed_dto!(DelegationOutput, DelegationOutputDto, "delegation output");
}
