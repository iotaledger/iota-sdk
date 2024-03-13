// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod anchor;
mod chain_id;
mod delegation;
mod error;
mod metadata;
mod native_token;
mod output_id;
mod output_id_proof;
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
use getset::Getters;
use packable::Packable;

pub(crate) use self::unlock_condition::AddressUnlockCondition;
pub use self::{
    account::{AccountId, AccountOutput, AccountOutputBuilder},
    anchor::{AnchorId, AnchorOutput, AnchorOutputBuilder, AnchorTransition},
    basic::{BasicOutput, BasicOutputBuilder},
    chain_id::ChainId,
    delegation::{DelegationId, DelegationOutput, DelegationOutputBuilder},
    error::OutputError,
    feature::{Feature, Features},
    foundry::{FoundryId, FoundryOutput, FoundryOutputBuilder},
    metadata::{OutputConsumptionMetadata, OutputInclusionMetadata, OutputMetadata},
    native_token::{NativeToken, NativeTokenError, NativeTokens, NativeTokensBuilder, TokenId},
    nft::{NftId, NftOutput, NftOutputBuilder},
    output_id::OutputId,
    output_id_proof::{HashableNode, LeafHash, OutputCommitmentProof, OutputIdProof, ProofError, ValueHash},
    storage_score::{StorageScore, StorageScoreParameters},
    token_scheme::{SimpleTokenScheme, TokenScheme, TokenSchemeError},
    unlock_condition::{UnlockCondition, UnlockConditions},
};
use crate::types::block::{
    address::Address,
    protocol::{CommittableAgeRange, ProtocolParameters, WorkScore, WorkScoreParameters},
    slot::SlotIndex,
};
#[cfg(feature = "serde")]
use crate::utils::serde::string;

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
    AmountOrMinimum(u64, StorageScoreParameters),
    MinimumAmount(StorageScoreParameters),
}

/// Contains the generic [`Output`] and the associated [`OutputMetadata`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputWithMetadata {
    pub output: Output,
    pub metadata: OutputMetadata,
}

/// A generic output that can represent different types defining the deposit of funds.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[packable(unpack_error = OutputError)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = OutputError::Kind)]
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
        match self {
            Self::Basic(output) => output.decayed_mana(protocol_parameters, creation_index, target_index),
            Self::Account(output) => output.decayed_mana(protocol_parameters, creation_index, target_index),
            Self::Anchor(output) => output.decayed_mana(protocol_parameters, creation_index, target_index),
            Self::Foundry(output) => output.decayed_mana(protocol_parameters, creation_index, target_index),
            Self::Nft(output) => output.decayed_mana(protocol_parameters, creation_index, target_index),
            Self::Delegation(output) => output.decayed_mana(protocol_parameters, creation_index, target_index),
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

    /// Returns the native token of an [`Output`], if any.
    pub fn native_token(&self) -> Option<&NativeToken> {
        match self {
            Self::Basic(output) => output.native_token(),
            Self::Account(_) => None,
            Self::Anchor(_) => None,
            Self::Foundry(output) => output.native_token(),
            Self::Nft(_) => None,
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
            Self::Delegation(output) => Some(output.chain_id()),
        }
    }

    /// Checks whether the output is an implicit account.
    pub fn is_implicit_account(&self) -> bool {
        if let Self::Basic(output) = self {
            output.is_implicit_account()
        } else {
            false
        }
    }

    /// Returns whether the output can claim rewards based on its current and next state in a transaction.
    pub fn can_claim_rewards(&self, next_state: Option<&Self>) -> bool {
        match self {
            // Validator Rewards
            Self::Account(account_input) => account_input.can_claim_rewards(next_state.and_then(Self::as_account_opt)),
            // Delegator Rewards
            Self::Delegation(delegation_input) => {
                delegation_input.can_claim_rewards(next_state.and_then(Self::as_delegation_opt))
            }
            _ => false,
        }
    }

    crate::def_is_as_opt!(Output: Basic, Account, Foundry, Nft, Delegation, Anchor);

    /// Returns the address that is required to unlock this [`Output`].
    pub fn required_address(
        &self,
        commitment_slot_index: impl Into<Option<SlotIndex>>,
        committable_age_range: CommittableAgeRange,
    ) -> Result<Option<Address>, OutputError> {
        Ok(match self {
            Self::Basic(output) => output
                .unlock_conditions()
                .locked_address(output.address(), commitment_slot_index, committable_age_range)?
                .cloned(),
            Self::Account(output) => Some(output.address().clone()),
            Self::Anchor(_) => return Err(OutputError::Kind(AnchorOutput::KIND)),
            Self::Foundry(output) => Some(Address::Account(*output.account_address())),
            Self::Nft(output) => output
                .unlock_conditions()
                .locked_address(output.address(), commitment_slot_index, committable_age_range)?
                .cloned(),
            Self::Delegation(output) => Some(output.address().clone()),
        })
    }

    /// Verifies if a valid storage deposit was made.
    /// Each [`Output`] has to have an amount that covers its associated byte cost, given by [`StorageScoreParameters`].
    /// If there is a [`StorageDepositReturnUnlockCondition`](unlock_condition::StorageDepositReturnUnlockCondition),
    /// its amount is also checked.
    pub fn verify_storage_deposit(&self, params: StorageScoreParameters) -> Result<(), OutputError> {
        let minimum_storage_deposit = self.minimum_amount(params);

        // For any created `Output` in a transaction, it must hold that `Output::Amount >= Minimum Storage Deposit`.
        if self.amount() < minimum_storage_deposit {
            return Err(OutputError::AmountLessThanMinimum {
                amount: self.amount(),
                required: minimum_storage_deposit,
            });
        }

        if let Some(return_condition) = self
            .unlock_conditions()
            .and_then(UnlockConditions::storage_deposit_return)
        {
            // We can't return more tokens than were originally contained in the output.
            // `Return Amount` ≤ `Amount`.
            if return_condition.amount() > self.amount() {
                return Err(OutputError::StorageDepositReturnExceedsOutputAmount {
                    deposit: return_condition.amount(),
                    amount: self.amount(),
                });
            }

            let minimum_storage_deposit = BasicOutput::minimum_amount(return_condition.return_address(), params);

            // `Minimum Storage Deposit` ≤ `Return Amount`
            if return_condition.amount() < minimum_storage_deposit {
                return Err(OutputError::InsufficientStorageDepositReturnAmount {
                    deposit: return_condition.amount(),
                    required: minimum_storage_deposit,
                });
            }
        }

        Ok(())
    }
}

impl StorageScore for Output {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Self::Basic(basic) => basic.storage_score(params),
            Self::Account(account) => account.storage_score(params),
            Self::Anchor(anchor) => anchor.storage_score(params),
            Self::Foundry(foundry) => foundry.storage_score(params),
            Self::Nft(nft) => nft.storage_score(params),
            Self::Delegation(delegation) => delegation.storage_score(params),
        }
    }
}

impl WorkScore for Output {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Basic(basic) => basic.work_score(params),
            Self::Account(account) => account.work_score(params),
            Self::Anchor(anchor) => anchor.work_score(params),
            Self::Foundry(foundry) => foundry.work_score(params),
            Self::Nft(nft) => nft.work_score(params),
            Self::Delegation(delegation) => delegation.work_score(params),
        }
    }
}

impl MinimumOutputAmount for Output {}

/// A trait that is shared by all output types, which is used to calculate the minimum amount the output
/// must contain to satisfy its storage cost.
pub trait MinimumOutputAmount: StorageScore {
    /// Computes the minimum amount of this output given [`StorageScoreParameters`].
    fn minimum_amount(&self, params: StorageScoreParameters) -> u64 {
        self.storage_score(params) * params.storage_cost()
    }
}

/// Decayed stored and potential Mana of an output.
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Getters, derive_more::AddAssign)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecayedMana {
    /// Decayed stored mana.
    #[cfg_attr(feature = "serde", serde(with = "string"))]
    pub(crate) stored: u64,
    /// Decayed potential mana.
    #[cfg_attr(feature = "serde", serde(with = "string"))]
    pub(crate) potential: u64,
}

#[cfg(feature = "serde")]
crate::impl_deserialize_untagged!(Output: Basic, Account, Anchor, Foundry, Nft, Delegation);
