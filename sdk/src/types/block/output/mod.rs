// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod anchor;
mod chain_id;
mod delegation;
mod metadata;
mod native_token;
mod output_id;
mod state_transition;
mod storage_score;
mod token_scheme;

///
pub mod account;
///
pub mod basic;
///
pub mod feature;
///
pub mod foundry;
///
pub mod nft;
///
pub mod unlock_condition;

use core::ops::RangeInclusive;

use derive_more::From;
use packable::Packable;

pub use self::{
    account::{AccountId, AccountOutput, AccountOutputBuilder},
    anchor::{AnchorId, AnchorOutput, AnchorTransition},
    basic::{BasicOutput, BasicOutputBuilder},
    chain_id::ChainId,
    delegation::{DelegationId, DelegationOutput, DelegationOutputBuilder},
    feature::{Feature, Features},
    foundry::{FoundryId, FoundryOutput, FoundryOutputBuilder},
    metadata::OutputMetadata,
    native_token::{NativeToken, NativeTokens, NativeTokensBuilder, TokenId},
    nft::{NftId, NftOutput, NftOutputBuilder},
    output_id::OutputId,
    state_transition::{StateTransitionError, StateTransitionVerifier},
    storage_score::{StorageScore, StorageScoreParameters},
    token_scheme::{SimpleTokenScheme, TokenScheme},
    unlock_condition::{UnlockCondition, UnlockConditions},
};
pub(crate) use self::{
    anchor::StateMetadataLength,
    feature::{MetadataFeatureLength, TagFeatureLength},
    native_token::NativeTokenCount,
    output_id::OutputIndex,
    unlock_condition::AddressUnlockCondition,
};
use super::protocol::ProtocolParameters;
use crate::types::block::{address::Address, semantic::SemanticValidationContext, slot::SlotIndex, Error};

/// The maximum number of outputs of a transaction.
pub const OUTPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of outputs of a transaction.
pub const OUTPUT_COUNT_RANGE: RangeInclusive<u16> = 1..=OUTPUT_COUNT_MAX; // [1..128]
/// The maximum index of outputs of a transaction.
pub const OUTPUT_INDEX_MAX: u16 = OUTPUT_COUNT_MAX - 1; // 127
/// The range of valid indices of outputs of a transaction.
pub const OUTPUT_INDEX_RANGE: RangeInclusive<u16> = 0..=OUTPUT_INDEX_MAX; // [0..127]

#[derive(Copy, Clone)]
pub enum OutputBuilderAmount {
    Amount(u64),
    MinimumAmount(StorageScoreParameters),
}

/// Contains the generic [`Output`] with associated [`OutputMetadata`].
#[derive(Clone, Debug)]
pub struct OutputWithMetadata {
    pub(crate) output: Output,
    pub(crate) metadata: OutputMetadata,
}

impl OutputWithMetadata {
    /// Creates a new [`OutputWithMetadata`].
    pub fn new(output: Output, metadata: OutputMetadata) -> Self {
        Self { output, metadata }
    }

    /// Returns the [`Output`].
    pub fn output(&self) -> &Output {
        &self.output
    }

    /// Consumes self and returns the [`Output`].
    pub fn into_output(self) -> Output {
        self.output
    }

    /// Returns the [`OutputMetadata`].
    pub fn metadata(&self) -> &OutputMetadata {
        &self.metadata
    }

    /// Consumes self and returns the [`OutputMetadata`].
    pub fn into_metadata(self) -> OutputMetadata {
        self.metadata
    }
}

/// A generic output that can represent different types defining the deposit of funds.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = Error::InvalidOutputKind)]
pub enum Output {
    /// A basic output.
    #[packable(tag = BasicOutput::KIND)]
    Basic(BasicOutput),
    /// An account output.
    #[packable(tag = AccountOutput::KIND)]
    Account(AccountOutput),
    /// An anchor output.
    #[packable(tag = AnchorOutput::KIND)]
    Anchor(AnchorOutput),
    /// A foundry output.
    #[packable(tag = FoundryOutput::KIND)]
    Foundry(FoundryOutput),
    /// An NFT output.
    #[packable(tag = NftOutput::KIND)]
    Nft(NftOutput),
    /// A delegation output.
    #[packable(tag = DelegationOutput::KIND)]
    Delegation(DelegationOutput),
}

impl core::fmt::Debug for Output {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Basic(output) => output.fmt(f),
            Self::Account(output) => output.fmt(f),
            Self::Anchor(output) => output.fmt(f),
            Self::Foundry(output) => output.fmt(f),
            Self::Nft(output) => output.fmt(f),
            Self::Delegation(output) => output.fmt(f),
        }
    }
}

impl Output {
    /// Return the output kind of an [`Output`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Basic(_) => BasicOutput::KIND,
            Self::Account(_) => AccountOutput::KIND,
            Self::Anchor(_) => AnchorOutput::KIND,
            Self::Foundry(_) => FoundryOutput::KIND,
            Self::Nft(_) => NftOutput::KIND,
            Self::Delegation(_) => DelegationOutput::KIND,
        }
    }

    /// Returns the output kind of an [`Output`] as a string.
    pub fn kind_str(&self) -> &str {
        match self {
            Self::Basic(_) => "Basic",
            Self::Account(_) => "Account",
            Self::Anchor(_) => "Anchor",
            Self::Foundry(_) => "Foundry",
            Self::Nft(_) => "Nft",
            Self::Delegation(_) => "Delegation",
        }
    }

    /// Returns the amount of an [`Output`].
    pub fn amount(&self) -> u64 {
        match self {
            Self::Basic(output) => output.amount(),
            Self::Account(output) => output.amount(),
            Self::Anchor(output) => output.amount(),
            Self::Foundry(output) => output.amount(),
            Self::Nft(output) => output.amount(),
            Self::Delegation(output) => output.amount(),
        }
    }

    /// Returns the mana of an [`Output`].
    pub fn mana(&self) -> u64 {
        match self {
            Self::Basic(output) => output.mana(),
            Self::Account(output) => output.mana(),
            Self::Anchor(output) => output.mana(),
            Self::Foundry(_) => 0,
            Self::Nft(output) => output.mana(),
            Self::Delegation(_) => 0,
        }
    }

    /// Returns the native tokens of an [`Output`], if any.
    pub fn native_tokens(&self) -> Option<&NativeTokens> {
        match self {
            Self::Basic(output) => Some(output.native_tokens()),
            Self::Account(output) => Some(output.native_tokens()),
            Self::Anchor(output) => Some(output.native_tokens()),
            Self::Foundry(output) => Some(output.native_tokens()),
            Self::Nft(output) => Some(output.native_tokens()),
            Self::Delegation(_) => None,
        }
    }

    /// Returns the unlock conditions of an [`Output`], if any.
    pub fn unlock_conditions(&self) -> Option<&UnlockConditions> {
        match self {
            Self::Basic(output) => Some(output.unlock_conditions()),
            Self::Account(output) => Some(output.unlock_conditions()),
            Self::Anchor(output) => Some(output.unlock_conditions()),
            Self::Foundry(output) => Some(output.unlock_conditions()),
            Self::Nft(output) => Some(output.unlock_conditions()),
            Self::Delegation(output) => Some(output.unlock_conditions()),
        }
    }

    /// Returns the features of an [`Output`], if any.
    pub fn features(&self) -> Option<&Features> {
        match self {
            Self::Basic(output) => Some(output.features()),
            Self::Account(output) => Some(output.features()),
            Self::Anchor(output) => Some(output.features()),
            Self::Foundry(output) => Some(output.features()),
            Self::Nft(output) => Some(output.features()),
            Self::Delegation(_) => None,
        }
    }

    /// Returns the immutable features of an [`Output`], if any.
    pub fn immutable_features(&self) -> Option<&Features> {
        match self {
            Self::Basic(_) => None,
            Self::Account(output) => Some(output.immutable_features()),
            Self::Anchor(output) => Some(output.immutable_features()),
            Self::Foundry(output) => Some(output.immutable_features()),
            Self::Nft(output) => Some(output.immutable_features()),
            Self::Delegation(_) => None,
        }
    }

    /// Returns the chain identifier of an [`Output`], if any.
    pub fn chain_id(&self) -> Option<ChainId> {
        match self {
            Self::Basic(_) => None,
            Self::Account(output) => Some(output.chain_id()),
            Self::Anchor(output) => Some(output.chain_id()),
            Self::Foundry(output) => Some(output.chain_id()),
            Self::Nft(output) => Some(output.chain_id()),
            Self::Delegation(_) => None,
        }
    }

    /// Checks whether the output is an implicit account.
    pub fn is_implicit_account(&self) -> bool {
        if let Output::Basic(output) = self {
            output.is_implicit_account()
        } else {
            false
        }
    }

    crate::def_is_as_opt!(Output: Basic, Account, Foundry, Nft, Delegation, Anchor);

    /// Returns the address that is required to unlock this [`Output`] and the account or nft address that gets
    /// unlocked by it, if it's an account or nft.
    /// If no `account_transition` has been provided, assumes a state transition.
    pub fn required_and_unlocked_address(
        &self,
        slot_index: SlotIndex,
        output_id: &OutputId,
    ) -> Result<(Address, Option<Address>), Error> {
        match self {
            Self::Basic(output) => Ok((
                output
                    .unlock_conditions()
                    .locked_address(output.address(), slot_index)
                    .clone(),
                None,
            )),
            Self::Account(output) => Ok((
                output
                    .unlock_conditions()
                    .locked_address(output.address(), slot_index)
                    .clone(),
                Some(Address::Account(output.account_address(output_id))),
            )),
            Self::Anchor(_) => Err(Error::UnsupportedOutputKind(AnchorOutput::KIND)),
            Self::Foundry(output) => Ok((Address::Account(*output.account_address()), None)),
            Self::Nft(output) => Ok((
                output
                    .unlock_conditions()
                    .locked_address(output.address(), slot_index)
                    .clone(),
                Some(Address::Nft(output.nft_address(output_id))),
            )),
            Self::Delegation(output) => Ok((
                output
                    .unlock_conditions()
                    .locked_address(output.address(), slot_index)
                    .clone(),
                None,
            )),
        }
    }

    ///
    pub fn verify_state_transition(
        current_state: Option<&Self>,
        next_state: Option<&Self>,
        context: &SemanticValidationContext<'_>,
    ) -> Result<(), StateTransitionError> {
        match (current_state, next_state) {
            // Creations.
            (None, Some(Self::Account(next_state))) => AccountOutput::creation(next_state, context),
            (None, Some(Self::Foundry(next_state))) => FoundryOutput::creation(next_state, context),
            (None, Some(Self::Nft(next_state))) => NftOutput::creation(next_state, context),
            (None, Some(Self::Delegation(next_state))) => DelegationOutput::creation(next_state, context),

            // Transitions.
            (Some(Self::Account(current_state)), Some(Self::Account(next_state))) => {
                AccountOutput::transition(current_state, next_state, context)
            }
            (Some(Self::Foundry(current_state)), Some(Self::Foundry(next_state))) => {
                FoundryOutput::transition(current_state, next_state, context)
            }
            (Some(Self::Nft(current_state)), Some(Self::Nft(next_state))) => {
                NftOutput::transition(current_state, next_state, context)
            }
            (Some(Self::Delegation(current_state)), Some(Self::Delegation(next_state))) => {
                DelegationOutput::transition(current_state, next_state, context)
            }

            // Destructions.
            (Some(Self::Account(current_state)), None) => AccountOutput::destruction(current_state, context),
            (Some(Self::Foundry(current_state)), None) => FoundryOutput::destruction(current_state, context),
            (Some(Self::Nft(current_state)), None) => NftOutput::destruction(current_state, context),
            (Some(Self::Delegation(current_state)), None) => DelegationOutput::destruction(current_state, context),

            // Unsupported.
            _ => Err(StateTransitionError::UnsupportedStateTransition),
        }
    }

    /// Verifies if a valid storage deposit was made. Each [`Output`] has to have an amount that covers its associated
    /// byte cost, given by [`StorageScoreParameters`].
    /// If there is a [`StorageDepositReturnUnlockCondition`](unlock_condition::StorageDepositReturnUnlockCondition),
    /// its amount is also checked.
    pub fn verify_storage_deposit(&self, params: StorageScoreParameters) -> Result<(), Error> {
        let required_output_amount = self.minimum_amount(params);

        if self.amount() < required_output_amount {
            return Err(Error::InsufficientStorageDepositAmount {
                amount: self.amount(),
                required: required_output_amount,
            });
        }

        if let Some(return_condition) = self
            .unlock_conditions()
            .and_then(UnlockConditions::storage_deposit_return)
        {
            // We can't return more tokens than were originally contained in the output.
            // `Return Amount` ≤ `Amount`.
            if return_condition.amount() > self.amount() {
                return Err(Error::StorageDepositReturnExceedsOutputAmount {
                    deposit: return_condition.amount(),
                    amount: self.amount(),
                });
            }

            let minimum_deposit = BasicOutput::minimum_amount(return_condition.return_address(), params);

            // `Minimum Storage Deposit` ≤ `Return Amount`
            if return_condition.amount() < minimum_deposit {
                return Err(Error::InsufficientStorageDepositReturnAmount {
                    deposit: return_condition.amount(),
                    required: minimum_deposit,
                });
            }
        }

        Ok(())
    }
}

impl StorageScore for Output {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Self::Basic(o) => o.storage_score(params),
            Self::Account(o) => o.storage_score(params),
            Self::Anchor(o) => o.storage_score(params),
            Self::Foundry(o) => o.storage_score(params),
            Self::Nft(o) => o.storage_score(params),
            Self::Delegation(o) => o.storage_score(params),
        }
    }
}

impl MinimumOutputAmount for Output {}

/// A trait that is shared by all output types, which is used to calculate the minimum amount the output
/// must contain to satisfy its storage cost.
pub trait MinimumOutputAmount: StorageScore {
    /// Computes the minimum amount of this output given [`StorageScoreParameters`].
    fn minimum_amount(&self, params: StorageScoreParameters) -> u64 {
        params.storage_cost() * self.storage_score(params)
    }
}

#[cfg(feature = "serde")]
pub mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize, Serializer};
    use serde_json::Value;

    use super::*;
    pub use super::{
        account::dto::AccountOutputDto, anchor::dto::AnchorOutputDto, basic::dto::BasicOutputDto,
        delegation::dto::DelegationOutputDto, foundry::dto::FoundryOutputDto, nft::dto::NftOutputDto,
    };

    /// Describes all the different output types.
    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum OutputDto {
        Basic(BasicOutputDto),
        Account(AccountOutputDto),
        Anchor(AnchorOutputDto),
        Foundry(FoundryOutputDto),
        Nft(NftOutputDto),
        Delegation(DelegationOutputDto),
    }

    impl From<&Output> for OutputDto {
        fn from(value: &Output) -> Self {
            match value {
                Output::Basic(o) => Self::Basic(o.into()),
                Output::Account(o) => Self::Account(o.into()),
                Output::Anchor(o) => Self::Anchor(o.into()),
                Output::Foundry(o) => Self::Foundry(o.into()),
                Output::Nft(o) => Self::Nft(o.into()),
                Output::Delegation(o) => Self::Delegation(o.into()),
            }
        }
    }

    impl TryFrom<OutputDto> for Output {
        type Error = Error;

        fn try_from(dto: OutputDto) -> Result<Self, Self::Error> {
            Ok(match dto {
                OutputDto::Basic(o) => Self::Basic(BasicOutput::try_from(o)?),
                OutputDto::Account(o) => Self::Account(AccountOutput::try_from(o)?),
                OutputDto::Anchor(o) => Self::Anchor(AnchorOutput::try_from(o)?),
                OutputDto::Foundry(o) => Self::Foundry(FoundryOutput::try_from(o)?),
                OutputDto::Nft(o) => Self::Nft(NftOutput::try_from(o)?),
                OutputDto::Delegation(o) => Self::Delegation(DelegationOutput::try_from(o)?),
            })
        }
    }

    impl<'de> Deserialize<'de> for OutputDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid output type"))? as u8
                {
                    BasicOutput::KIND => Self::Basic(
                        BasicOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize basic output: {e}")))?,
                    ),
                    AccountOutput::KIND => Self::Account(
                        AccountOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize account output: {e}")))?,
                    ),
                    AnchorOutput::KIND => Self::Anchor(
                        AnchorOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize anchor output: {e}")))?,
                    ),
                    FoundryOutput::KIND => Self::Foundry(
                        FoundryOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize foundry output: {e}")))?,
                    ),
                    NftOutput::KIND => Self::Nft(
                        NftOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize NFT output: {e}")))?,
                    ),
                    DelegationOutput::KIND => {
                        Self::Delegation(DelegationOutputDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize delegation output: {e}"))
                        })?)
                    }
                    _ => return Err(serde::de::Error::custom("invalid output type")),
                },
            )
        }
    }

    impl Serialize for OutputDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum OutputDto_<'a> {
                T0(&'a BasicOutputDto),
                T1(&'a AccountOutputDto),
                T2(&'a AnchorOutputDto),
                T3(&'a FoundryOutputDto),
                T4(&'a NftOutputDto),
                T5(&'a DelegationOutputDto),
            }
            #[derive(Serialize)]
            struct TypedOutput<'a> {
                #[serde(flatten)]
                output: OutputDto_<'a>,
            }
            let output = match self {
                Self::Basic(o) => TypedOutput {
                    output: OutputDto_::T0(o),
                },
                Self::Account(o) => TypedOutput {
                    output: OutputDto_::T1(o),
                },
                Self::Anchor(o) => TypedOutput {
                    output: OutputDto_::T2(o),
                },
                Self::Foundry(o) => TypedOutput {
                    output: OutputDto_::T3(o),
                },
                Self::Nft(o) => TypedOutput {
                    output: OutputDto_::T4(o),
                },
                Self::Delegation(o) => TypedOutput {
                    output: OutputDto_::T5(o),
                },
            };
            output.serialize(serializer)
        }
    }
}
