// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use hashbrown::HashMap;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

use crate::types::block::{
    address::{AccountAddress, Address},
    output::{
        feature::{verify_allowed_features, Feature, FeatureFlags, Features},
        unlock_condition::{
            verify_allowed_unlock_conditions, verify_restricted_addresses, UnlockCondition, UnlockConditionError,
            UnlockConditionFlags, UnlockConditions,
        },
        ChainId, DecayedMana, MinimumOutputAmount, Output, OutputBuilderAmount, OutputError, OutputId, StorageScore,
        StorageScoreParameters,
    },
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    semantic::TransactionFailureReason,
    slot::SlotIndex,
};

crate::impl_id!(
    /// A unique identifier of an account.
    pub AccountId {
        pub const LENGTH: usize = 32;
    }
);

impl From<&OutputId> for AccountId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl AccountId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }
}

impl From<AccountId> for Address {
    fn from(value: AccountId) -> Self {
        Self::Account(AccountAddress::new(value))
    }
}

///
#[derive(Clone)]
#[must_use]
pub struct AccountOutputBuilder {
    amount: OutputBuilderAmount,
    mana: u64,
    account_id: AccountId,
    foundry_counter: Option<u32>,
    unlock_conditions: BTreeSet<UnlockCondition>,
    features: BTreeSet<Feature>,
    immutable_features: BTreeSet<Feature>,
}

impl AccountOutputBuilder {
    /// Creates an [`AccountOutputBuilder`] with a provided amount.
    pub fn new_with_amount(amount: u64, account_id: AccountId) -> Self {
        Self::new(OutputBuilderAmount::Amount(amount), account_id)
    }

    /// Creates an [`AccountOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount of the resulting output.
    pub fn new_with_minimum_amount(params: StorageScoreParameters, account_id: AccountId) -> Self {
        Self::new(OutputBuilderAmount::MinimumAmount(params), account_id)
    }

    fn new(amount: OutputBuilderAmount, account_id: AccountId) -> Self {
        Self {
            amount,
            mana: Default::default(),
            account_id,
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

    /// Sets the account ID to the provided value.
    #[inline(always)]
    pub fn with_account_id(mut self, account_id: AccountId) -> Self {
        self.account_id = account_id;
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
    pub fn finish(self) -> Result<AccountOutput, OutputError> {
        let foundry_counter = self.foundry_counter.unwrap_or(0);

        verify_index_counter(&self.account_id, foundry_counter)?;

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions, &self.account_id)?;

        let features = Features::from_set(self.features)?;

        verify_restricted_addresses(
            &unlock_conditions,
            AccountOutput::KIND,
            features.native_token(),
            self.mana,
        )
        .map_err(UnlockConditionError::from)?;
        verify_allowed_features(&features, AccountOutput::ALLOWED_FEATURES)?;

        let immutable_features = Features::from_set(self.immutable_features)?;

        verify_allowed_features(&immutable_features, AccountOutput::ALLOWED_IMMUTABLE_FEATURES)?;

        let mut output = AccountOutput {
            amount: 0,
            mana: self.mana,
            account_id: self.account_id,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumAmount(params) => output.minimum_amount(params),
        };

        verify_staked_amount(output.amount, &output.features)?;

        Ok(output)
    }

    /// Finishes the [`AccountOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, OutputError> {
        Ok(Output::Account(self.finish()?))
    }
}

impl From<&AccountOutput> for AccountOutputBuilder {
    fn from(output: &AccountOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            mana: output.mana,
            account_id: output.account_id,
            foundry_counter: Some(output.foundry_counter),
            unlock_conditions: output.unlock_conditions.iter().cloned().collect(),
            features: output.features.iter().cloned().collect(),
            immutable_features: output.immutable_features.iter().cloned().collect(),
        }
    }
}

/// Describes an account in the ledger that can be controlled by the state and governance controllers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AccountOutput {
    /// Amount of IOTA coins held by the output.
    amount: u64,
    /// Amount of stored Mana held by the output.
    mana: u64,
    // Unique identifier of the account.
    account_id: AccountId,
    // A counter that denotes the number of foundries created by this account.
    foundry_counter: u32,
    /// Define how the output can be unlocked in a transaction.
    unlock_conditions: UnlockConditions,
    /// Features of the output.
    features: Features,
    /// Immutable features of the output.
    immutable_features: Features,
}

impl AccountOutput {
    /// The [`Output`] kind of an [`AccountOutput`].
    pub const KIND: u8 = 1;
    /// The set of allowed [`UnlockCondition`]s for an [`AccountOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS;
    /// The set of allowed [`Feature`]s for an [`AccountOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::SENDER
        .union(FeatureFlags::METADATA)
        .union(FeatureFlags::BLOCK_ISSUER)
        .union(FeatureFlags::STAKING);
    /// The set of allowed immutable [`Feature`]s for an [`AccountOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::ISSUER.union(FeatureFlags::METADATA);

    /// Creates a new [`AccountOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, account_id: AccountId) -> AccountOutputBuilder {
        AccountOutputBuilder::new_with_amount(amount, account_id)
    }

    /// Creates a new [`AccountOutputBuilder`] with provided storage score parameters.
    /// The amount will be set to the minimum required amount.
    #[inline(always)]
    pub fn build_with_minimum_amount(params: StorageScoreParameters, account_id: AccountId) -> AccountOutputBuilder {
        AccountOutputBuilder::new_with_minimum_amount(params, account_id)
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
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// Returns the account ID if not null, or creates it from the output ID.
    #[inline(always)]
    pub fn account_id_non_null(&self, output_id: &OutputId) -> AccountId {
        self.account_id.or_from_output_id(output_id)
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
    pub fn address(&self) -> &Address {
        // An AccountOutput must have an AddressUnlockCondition.
        self.unlock_conditions
            .address()
            .map(|unlock_condition| unlock_condition.address())
            .unwrap()
    }

    ///
    #[inline(always)]
    pub fn chain_id(&self) -> ChainId {
        ChainId::Account(self.account_id)
    }

    /// Returns the account address for this output.
    pub fn account_address(&self, output_id: &OutputId) -> AccountAddress {
        AccountAddress::new(self.account_id_non_null(output_id))
    }

    /// Checks whether the account is a block issuer.
    pub fn is_block_issuer(&self) -> bool {
        self.features.block_issuer().is_some()
    }

    /// Returns whether the output can claim rewards based on its current and next state in a transaction.
    pub fn can_claim_rewards(&self, next_state: Option<&Self>) -> bool {
        self.features().staking().is_some() && next_state.map_or(true, |o| o.features().staking().is_none())
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
        let stored_mana = protocol_parameters.mana_with_decay(self.mana(), creation_index, target_index)?;
        let potential_mana =
            protocol_parameters.generate_mana_with_decay(generation_amount, creation_index, target_index)?;

        Ok(DecayedMana {
            stored: stored_mana,
            potential: potential_mana,
        })
    }

    // Transition, just without full SemanticValidationContext
    pub(crate) fn transition_inner(
        current_state: &Self,
        next_state: &Self,
        input_chains: &HashMap<ChainId, (&OutputId, &Output)>,
        outputs: &[Output],
    ) -> Result<(), TransactionFailureReason> {
        if current_state.immutable_features != next_state.immutable_features {
            return Err(TransactionFailureReason::ChainOutputImmutableFeaturesChanged);
        }

        // TODO update when TIP is updated
        // // Governance transition.
        // if current_state.amount != next_state.amount
        //     || current_state.foundry_counter != next_state.foundry_counter
        // {
        //     return Err(StateTransitionError::MutatedFieldWithoutRights);
        // }

        // // State transition.
        // if current_state.features.metadata() != next_state.features.metadata() {
        //     return Err(StateTransitionError::MutatedFieldWithoutRights);
        // }

        let created_foundries = outputs.iter().filter_map(|output| {
            if let Output::Foundry(foundry) = output {
                if foundry.account_address().account_id() == &next_state.account_id
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
                return Err(TransactionFailureReason::FoundrySerialInvalid);
            }
        }

        if current_state.foundry_counter + created_foundries_count != next_state.foundry_counter {
            return Err(TransactionFailureReason::AccountInvalidFoundryCounter);
        }

        Ok(())
    }
}

impl StorageScore for AccountOutput {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.output_offset()
            // Type byte
            + (1 + self.packed_len() as u64) * params.data_factor() as u64
            + self.unlock_conditions.storage_score(params)
            + self.features.storage_score(params)
            + self.immutable_features.storage_score(params)
    }
}

impl WorkScore for AccountOutput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.output()
            + self.unlock_conditions.work_score(params)
            + self.features.work_score(params)
            + self.immutable_features.work_score(params)
    }
}

impl MinimumOutputAmount for AccountOutput {}

impl Packable for AccountOutput {
    type UnpackError = OutputError;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.mana.pack(packer)?;
        self.account_id.pack(packer)?;
        self.foundry_counter.pack(packer)?;
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

        let account_id = AccountId::unpack_inner(unpacker, visitor).coerce()?;

        let foundry_counter = u32::unpack_inner(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_index_counter(&account_id, foundry_counter).map_err(UnpackError::Packable)?;
        }

        let unlock_conditions = UnlockConditions::unpack(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_unlock_conditions(&unlock_conditions, &account_id).map_err(UnpackError::Packable)?;
        }

        let features = Features::unpack_inner(unpacker, visitor).coerce()?;

        if visitor.is_some() {
            verify_restricted_addresses(&unlock_conditions, Self::KIND, features.native_token(), mana)
                .map_err(UnlockConditionError::from)
                .map_err(UnpackError::Packable)
                .coerce()?;
            verify_allowed_features(&features, Self::ALLOWED_FEATURES)
                .map_err(UnpackError::Packable)
                .coerce()?;
            verify_staked_amount(amount, &features).map_err(UnpackError::Packable)?;
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
            account_id,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        })
    }
}

#[inline]
fn verify_index_counter(account_id: &AccountId, foundry_counter: u32) -> Result<(), OutputError> {
    if account_id.is_null() && foundry_counter != 0 {
        Err(OutputError::NonZeroStateIndexOrFoundryCounter)
    } else {
        Ok(())
    }
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions, account_id: &AccountId) -> Result<(), OutputError> {
    if let Some(unlock_condition) = unlock_conditions.address() {
        if let Address::Account(account_address) = unlock_condition.address() {
            if !account_id.is_null() && account_address.account_id() == account_id {
                return Err(OutputError::SelfDepositAccount(*account_id));
            }
        }
    } else {
        return Err(OutputError::MissingAddressUnlockCondition);
    }

    Ok(verify_allowed_unlock_conditions(
        unlock_conditions,
        AccountOutput::ALLOWED_UNLOCK_CONDITIONS,
    )?)
}

fn verify_staked_amount(amount: u64, features: &Features) -> Result<(), OutputError> {
    if let Some(staking) = features.staking() {
        if amount < staking.staked_amount() {
            return Err(OutputError::InvalidStakedAmount);
        }
    }

    Ok(())
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{types::block::output::unlock_condition::UnlockCondition, utils::serde::string};

    /// Describes an account in the ledger that can be controlled by the state and governance controllers.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct AccountOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub mana: u64,
        pub account_id: AccountId,
        pub foundry_counter: u32,
        pub unlock_conditions: Vec<UnlockCondition>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<Feature>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<Feature>,
    }

    impl From<&AccountOutput> for AccountOutputDto {
        fn from(value: &AccountOutput) -> Self {
            Self {
                kind: AccountOutput::KIND,
                amount: value.amount(),
                mana: value.mana(),
                account_id: *value.account_id(),
                foundry_counter: value.foundry_counter(),
                unlock_conditions: value.unlock_conditions().to_vec(),
                features: value.features().to_vec(),
                immutable_features: value.immutable_features().to_vec(),
            }
        }
    }

    impl TryFrom<AccountOutputDto> for AccountOutput {
        type Error = OutputError;

        fn try_from(dto: AccountOutputDto) -> Result<Self, Self::Error> {
            let mut builder = AccountOutputBuilder::new_with_amount(dto.amount, dto.account_id)
                .with_mana(dto.mana)
                .with_foundry_counter(dto.foundry_counter)
                .with_features(dto.features)
                .with_immutable_features(dto.immutable_features);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(u);
            }

            builder.finish()
        }
    }

    crate::impl_serde_typed_dto!(AccountOutput, AccountOutputDto, "account output");
}

#[cfg(all(test, feature = "protocol_parameters_samples"))]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::{
        output::account::dto::AccountOutputDto, protocol::iota_mainnet_protocol_parameters,
        rand::output::rand_account_output,
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = iota_mainnet_protocol_parameters();
        let account_output = rand_account_output(protocol_parameters.token_supply());
        let dto = AccountOutputDto::from(&account_output);
        let output = Output::Account(AccountOutput::try_from(dto).unwrap());
        assert_eq!(&account_output, output.as_account());
    }
}
