// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{collections::BTreeSet, vec::Vec};

use hashbrown::HashMap;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::{
    block::{
        address::{AccountAddress, Address},
        output::{
            feature::{verify_allowed_features, Feature, FeatureFlags, Features},
            unlock_condition::{
                verify_allowed_unlock_conditions, UnlockCondition, UnlockConditionFlags, UnlockConditions,
            },
            verify_output_amount_min, verify_output_amount_packable, ChainId, NativeToken, NativeTokens, Output,
            OutputBuilderAmount, OutputId, Rent, RentStructure, StateTransitionError, StateTransitionVerifier,
        },
        payload::signed_transaction::TransactionCapabilityFlag,
        protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
        semantic::{SemanticValidationContext, TransactionFailureReason},
        unlock::Unlock,
        Error,
    },
    ValidationParams,
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
    native_tokens: BTreeSet<NativeToken>,
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

    /// Creates an [`AccountOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    pub fn new_with_minimum_storage_deposit(rent_structure: RentStructure, account_id: AccountId) -> Self {
        Self::new(OutputBuilderAmount::MinimumStorageDeposit(rent_structure), account_id)
    }

    fn new(amount: OutputBuilderAmount, account_id: AccountId) -> Self {
        Self {
            amount,
            mana: Default::default(),
            native_tokens: BTreeSet::new(),
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
    pub fn finish(self) -> Result<AccountOutput, Error> {
        let foundry_counter = self.foundry_counter.unwrap_or(0);

        verify_index_counter(&self.account_id, foundry_counter)?;

        let unlock_conditions = UnlockConditions::from_set(self.unlock_conditions)?;

        verify_unlock_conditions(&unlock_conditions, &self.account_id)?;

        let features = Features::from_set(self.features)?;

        verify_allowed_features(&features, AccountOutput::ALLOWED_FEATURES)?;

        let immutable_features = Features::from_set(self.immutable_features)?;

        verify_allowed_features(&immutable_features, AccountOutput::ALLOWED_IMMUTABLE_FEATURES)?;

        let mut output = AccountOutput {
            amount: 1,
            mana: self.mana,
            native_tokens: NativeTokens::from_set(self.native_tokens)?,
            account_id: self.account_id,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        };

        output.amount = match self.amount {
            OutputBuilderAmount::Amount(amount) => amount,
            OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                Output::Account(output.clone()).rent_cost(rent_structure)
            }
        };

        verify_output_amount_min(output.amount)?;

        Ok(output)
    }

    /// Finishes the [`AccountOutputBuilder`] into an [`Output`].
    pub fn finish_output(self) -> Result<Output, Error> {
        Ok(Output::Account(self.finish()?))
    }
}

impl From<&AccountOutput> for AccountOutputBuilder {
    fn from(output: &AccountOutput) -> Self {
        Self {
            amount: OutputBuilderAmount::Amount(output.amount),
            mana: output.mana,
            native_tokens: output.native_tokens.iter().copied().collect(),
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
    // Amount of IOTA coins held by the output.
    amount: u64,
    mana: u64,
    // Native tokens held by the output.
    native_tokens: NativeTokens,
    // Unique identifier of the account.
    account_id: AccountId,
    // A counter that denotes the number of foundries created by this account.
    foundry_counter: u32,
    unlock_conditions: UnlockConditions,
    //
    features: Features,
    //
    immutable_features: Features,
}

impl AccountOutput {
    /// The [`Output`](crate::types::block::output::Output) kind of an [`AccountOutput`].
    pub const KIND: u8 = 1;
    /// The set of allowed [`UnlockCondition`]s for an [`AccountOutput`].
    pub const ALLOWED_UNLOCK_CONDITIONS: UnlockConditionFlags = UnlockConditionFlags::ADDRESS;
    /// The set of allowed [`Feature`]s for an [`AccountOutput`].
    pub const ALLOWED_FEATURES: FeatureFlags = FeatureFlags::SENDER.union(FeatureFlags::METADATA);
    /// The set of allowed immutable [`Feature`]s for an [`AccountOutput`].
    pub const ALLOWED_IMMUTABLE_FEATURES: FeatureFlags = FeatureFlags::ISSUER.union(FeatureFlags::METADATA);

    /// Creates a new [`AccountOutputBuilder`] with a provided amount.
    #[inline(always)]
    pub fn build_with_amount(amount: u64, account_id: AccountId) -> AccountOutputBuilder {
        AccountOutputBuilder::new_with_amount(amount, account_id)
    }

    /// Creates a new [`AccountOutputBuilder`] with a provided rent structure.
    /// The amount will be set to the minimum storage deposit.
    #[inline(always)]
    pub fn build_with_minimum_storage_deposit(
        rent_structure: RentStructure,
        account_id: AccountId,
    ) -> AccountOutputBuilder {
        AccountOutputBuilder::new_with_minimum_storage_deposit(rent_structure, account_id)
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

    ///
    pub fn unlock(
        &self,
        output_id: &OutputId,
        unlock: &Unlock,
        context: &mut SemanticValidationContext<'_>,
    ) -> Result<(), TransactionFailureReason> {
        self.unlock_conditions()
            .locked_address(self.address(), context.transaction.creation_slot())
            .unlock(unlock, context)?;

        let account_id = if self.account_id().is_null() {
            AccountId::from(output_id)
        } else {
            *self.account_id()
        };

        context
            .unlocked_addresses
            .insert(Address::from(AccountAddress::from(account_id)));

        Ok(())
    }

    // Transition, just without full SemanticValidationContext
    pub(crate) fn transition_inner(
        current_state: &Self,
        next_state: &Self,
        input_chains: &HashMap<ChainId, &Output>,
        outputs: &[Output],
    ) -> Result<(), StateTransitionError> {
        if current_state.immutable_features != next_state.immutable_features {
            return Err(StateTransitionError::MutatedImmutableField);
        }

        // TODO update when TIP is updated
        // // Governance transition.
        // if current_state.amount != next_state.amount
        //     || current_state.native_tokens != next_state.native_tokens
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
                return Err(StateTransitionError::UnsortedCreatedFoundries);
            }
        }

        if current_state.foundry_counter + created_foundries_count != next_state.foundry_counter {
            return Err(StateTransitionError::InconsistentCreatedFoundriesCount);
        }

        Ok(())
    }
}

impl StateTransitionVerifier for AccountOutput {
    fn creation(next_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
        if !next_state.account_id.is_null() {
            return Err(StateTransitionError::NonZeroCreatedId);
        }

        if let Some(issuer) = next_state.immutable_features().issuer() {
            if !context.unlocked_addresses.contains(issuer.address()) {
                return Err(StateTransitionError::IssuerNotUnlocked);
            }
        }

        Ok(())
    }

    fn transition(
        current_state: &Self,
        next_state: &Self,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        Self::transition_inner(
            current_state,
            next_state,
            &context.input_chains,
            context.transaction.outputs(),
        )
    }

    fn destruction(_current_state: &Self, context: &SemanticValidationContext<'_>) -> Result<(), StateTransitionError> {
        if !context
            .transaction
            .has_capability(TransactionCapabilityFlag::DestroyAccountOutputs)
        {
            return Err(TransactionFailureReason::TransactionCapabilityAccountDestructionNotAllowed)?;
        }
        Ok(())
    }
}

impl WorkScore for AccountOutput {
    fn work_score(&self, work_score_params: WorkScoreParameters) -> u32 {
        let native_tokens_score = self.native_tokens().work_score(work_score_params);
        let features_score = self.features().work_score(work_score_params);
        let immutable_features_score = self.immutable_features().work_score(work_score_params);
        work_score_params.output() + native_tokens_score + features_score + immutable_features_score
    }
}

impl Packable for AccountOutput {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.amount.pack(packer)?;
        self.mana.pack(packer)?;
        self.native_tokens.pack(packer)?;
        self.account_id.pack(packer)?;
        self.foundry_counter.pack(packer)?;
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

        verify_output_amount_packable::<VERIFY>(&amount, visitor).map_err(UnpackError::Packable)?;

        let mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let native_tokens = NativeTokens::unpack::<_, VERIFY>(unpacker, &())?;
        let account_id = AccountId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let foundry_counter = u32::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        if VERIFY {
            verify_index_counter(&account_id, foundry_counter).map_err(UnpackError::Packable)?;
        }

        let unlock_conditions = UnlockConditions::unpack::<_, VERIFY>(unpacker, visitor)?;

        if VERIFY {
            verify_unlock_conditions(&unlock_conditions, &account_id).map_err(UnpackError::Packable)?;
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
            native_tokens,
            account_id,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        })
    }
}

#[inline]
fn verify_index_counter(account_id: &AccountId, foundry_counter: u32) -> Result<(), Error> {
    if account_id.is_null() && foundry_counter != 0 {
        Err(Error::NonZeroStateIndexOrFoundryCounter)
    } else {
        Ok(())
    }
}

fn verify_unlock_conditions(unlock_conditions: &UnlockConditions, account_id: &AccountId) -> Result<(), Error> {
    if let Some(unlock_condition) = unlock_conditions.address() {
        if let Address::Account(account_address) = unlock_condition.address() {
            if account_address.account_id() == account_id {
                return Err(Error::SelfDepositAccount(*account_id));
            }
        }
    } else {
        return Err(Error::MissingAddressUnlockCondition);
    }

    verify_allowed_unlock_conditions(unlock_conditions, AccountOutput::ALLOWED_UNLOCK_CONDITIONS)
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{
            block::{output::unlock_condition::dto::UnlockConditionDto, Error},
            TryFromDto,
        },
        utils::serde::string,
    };

    /// Describes an account in the ledger that can be controlled by the state and governance controllers.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "string")]
        pub amount: u64,
        #[serde(with = "string")]
        pub mana: u64,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        pub account_id: AccountId,
        pub foundry_counter: u32,
        pub unlock_conditions: Vec<UnlockConditionDto>,
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
                native_tokens: value.native_tokens().to_vec(),
                account_id: *value.account_id(),
                foundry_counter: value.foundry_counter(),
                unlock_conditions: value.unlock_conditions().iter().map(Into::into).collect::<_>(),
                features: value.features().to_vec(),
                immutable_features: value.immutable_features().to_vec(),
            }
        }
    }

    impl TryFromDto for AccountOutput {
        type Dto = AccountOutputDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let mut builder = AccountOutputBuilder::new_with_amount(dto.amount, dto.account_id)
                .with_mana(dto.mana)
                .with_foundry_counter(dto.foundry_counter)
                .with_native_tokens(dto.native_tokens)
                .with_features(dto.features)
                .with_immutable_features(dto.immutable_features);

            for u in dto.unlock_conditions {
                builder = builder.add_unlock_condition(UnlockCondition::try_from_dto_with_params(u, &params)?);
            }

            builder.finish()
        }
    }

    impl AccountOutput {
        #[allow(clippy::too_many_arguments)]
        pub fn try_from_dtos<'a>(
            amount: OutputBuilderAmount,
            mana: u64,
            native_tokens: Option<Vec<NativeToken>>,
            account_id: &AccountId,
            foundry_counter: Option<u32>,
            unlock_conditions: Vec<UnlockConditionDto>,
            features: Option<Vec<Feature>>,
            immutable_features: Option<Vec<Feature>>,
            params: impl Into<ValidationParams<'a>> + Send,
        ) -> Result<Self, Error> {
            let params = params.into();
            let mut builder = match amount {
                OutputBuilderAmount::Amount(amount) => AccountOutputBuilder::new_with_amount(amount, *account_id),
                OutputBuilderAmount::MinimumStorageDeposit(rent_structure) => {
                    AccountOutputBuilder::new_with_minimum_storage_deposit(rent_structure, *account_id)
                }
            }
            .with_mana(mana);

            if let Some(native_tokens) = native_tokens {
                builder = builder.with_native_tokens(native_tokens);
            }

            if let Some(foundry_counter) = foundry_counter {
                builder = builder.with_foundry_counter(foundry_counter);
            }

            let unlock_conditions = unlock_conditions
                .into_iter()
                .map(|u| UnlockCondition::try_from_dto_with_params(u, &params))
                .collect::<Result<Vec<UnlockCondition>, Error>>()?;
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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::{
        block::{
            output::{dto::OutputDto, FoundryId, SimpleTokenScheme, TokenId},
            protocol::protocol_parameters,
            rand::{
                address::rand_account_address,
                output::{
                    feature::rand_allowed_features, rand_account_id, rand_account_output,
                    unlock_condition::rand_address_unlock_condition_different_from_account_id,
                },
            },
        },
        TryFromDto,
    };

    #[test]
    fn to_from_dto() {
        let protocol_parameters = protocol_parameters();
        let output = rand_account_output(protocol_parameters.token_supply());
        let dto = OutputDto::Account((&output).into());
        let output_unver = Output::try_from_dto(dto.clone()).unwrap();
        assert_eq!(&output, output_unver.as_account());
        let output_ver = Output::try_from_dto_with_params(dto, &protocol_parameters).unwrap();
        assert_eq!(&output, output_ver.as_account());

        let output_split = AccountOutput::try_from_dtos(
            OutputBuilderAmount::Amount(output.amount()),
            output.mana(),
            Some(output.native_tokens().to_vec()),
            output.account_id(),
            output.foundry_counter().into(),
            output.unlock_conditions().iter().map(Into::into).collect(),
            Some(output.features().to_vec()),
            Some(output.immutable_features().to_vec()),
            &protocol_parameters,
        )
        .unwrap();
        assert_eq!(output, output_split);

        let account_id = rand_account_id();
        let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);
        let address = rand_address_unlock_condition_different_from_account_id(&account_id);

        let test_split_dto = |builder: AccountOutputBuilder| {
            let output_split = AccountOutput::try_from_dtos(
                builder.amount,
                builder.mana,
                Some(builder.native_tokens.iter().copied().collect()),
                &builder.account_id,
                builder.foundry_counter,
                builder.unlock_conditions.iter().map(Into::into).collect(),
                Some(builder.features.iter().cloned().collect()),
                Some(builder.immutable_features.iter().cloned().collect()),
                &protocol_parameters,
            )
            .unwrap();
            assert_eq!(builder.finish().unwrap(), output_split);
        };

        let builder = AccountOutput::build_with_amount(100, account_id)
            .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
            .add_unlock_condition(address.clone())
            .with_features(rand_allowed_features(AccountOutput::ALLOWED_FEATURES))
            .with_immutable_features(rand_allowed_features(AccountOutput::ALLOWED_IMMUTABLE_FEATURES));
        test_split_dto(builder);

        let builder =
            AccountOutput::build_with_minimum_storage_deposit(protocol_parameters.rent_structure(), account_id)
                .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
                .add_unlock_condition(address)
                .with_features(rand_allowed_features(AccountOutput::ALLOWED_FEATURES))
                .with_immutable_features(rand_allowed_features(AccountOutput::ALLOWED_IMMUTABLE_FEATURES));
        test_split_dto(builder);
    }
}
