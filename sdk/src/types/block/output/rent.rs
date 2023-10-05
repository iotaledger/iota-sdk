// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem::size_of;

use packable::Packable;

use crate::types::block::{
    address::{Address, Ed25519Address},
    output::{
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition},
        BasicOutputBuilder, NativeTokens, Output, OutputId,
    },
    slot::SlotIndex,
    BlockId, Error,
};

// TODO: fill in the real values
const DEFAULT_STORAGE_COST: u32 = 100;
const DEFAULT_STORAGE_SCORE_FACTOR_DATA: StorageScoreFactor = 1;
const DEFAULT_STORAGE_SCORE_OFFSET_OUTPUT: StorageScoreOffset = 1;
const DEFAULT_STORAGE_SCORE_OFFSET_ED25519_BLOCK_ISSUER_KEY: StorageScoreOffset = 1;
const DEFAULT_STORAGE_SCORE_STAKING_FEATURE: StorageScoreOffset = 1;
const DEFAULT_STORAGE_SCORE_OFFSET_DELEGATION: StorageScoreOffset = 1;

// Defines the type of the storage score factor.
type StorageScoreFactor = u8;
// Defines the type of storage score.
type StorageScoreOffset = u64;

/// Specifies the current parameters for the byte cost computation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct RentStructure {
    // TODO: what actual primitive type is their `BaseToken` type def?
    /// Defines the number of IOTA tokens required per unit of storage score.
    storage_cost: u32,
    /// Defines the factor to be used for data only fields.
    storage_score_factor_data: StorageScoreFactor,
    /// Defines the offset to be used for key/lookup generating fields.
    storage_score_offset_output: StorageScoreOffset,
    /// Defines the offset to be used for block issuer feature public keys.
    storage_score_offset_ed25519_block_issuer_key: StorageScoreOffset,
    /// Defines the offset to be used for staking feature.
    storage_score_offset_staking_feature: StorageScoreOffset,
    /// Defines the offset to be used for delegation output.
    storage_score_offset_delegation: StorageScoreOffset,
}

impl Default for RentStructure {
    fn default() -> Self {
        Self {
            storage_cost: DEFAULT_STORAGE_COST,
            storage_score_factor_data: DEFAULT_STORAGE_SCORE_FACTOR_DATA,
            storage_score_offset_output: DEFAULT_STORAGE_SCORE_OFFSET_OUTPUT,
            storage_score_offset_ed25519_block_issuer_key: DEFAULT_STORAGE_SCORE_OFFSET_ED25519_BLOCK_ISSUER_KEY,
            storage_score_offset_staking_feature: DEFAULT_STORAGE_SCORE_STAKING_FEATURE,
            storage_score_offset_delegation: DEFAULT_STORAGE_SCORE_OFFSET_DELEGATION,
        }
    }
}

impl RentStructure {
    /// Creates a new [`RentStructure`].
    pub fn new(
        storage_cost: u32,
        storage_score_factor_data: StorageScoreFactor,
        storage_score_offset_output: StorageScoreOffset,
        storage_score_offset_ed25519_block_issuer_key: StorageScoreOffset,
        storage_score_offset_staking_feature: StorageScoreOffset,
        storage_score_offset_delegation: StorageScoreOffset,
    ) -> Self {
        Self {
            storage_cost,
            storage_score_factor_data,
            storage_score_offset_output: storage_score_offset_output,
            storage_score_offset_ed25519_block_issuer_key: storage_score_offset_ed25519_block_issuer_key,
            storage_score_offset_staking_feature: storage_score_offset_staking_feature,
            storage_score_offset_delegation: storage_score_offset_delegation,
        }
    }

    /// Sets the storage cost for the storage deposit.
    pub fn with_storage_cost(mut self, storage_cost: u32) -> Self {
        self.storage_cost = storage_cost;
        self
    }

    /// Sets the storage score factor for the data fields.
    pub fn with_storage_score_factor_data(mut self, storage_score_factor_data: StorageScoreFactor) -> Self {
        self.storage_score_factor_data = storage_score_factor_data;
        self
    }

    /// Sets the TODO.
    pub fn with_storage_score_offset_output(mut self, storage_score_offset_output: StorageScoreOffset) -> Self {
        self.storage_score_offset_output = storage_score_offset_output;
        self
    }

    /// Sets the TODO.
    pub fn with_storage_score_offset_ed25519_block_issuer_key(
        mut self,
        storage_score_offset_ed25519_block_issuer_key: StorageScoreOffset,
    ) -> Self {
        self.storage_score_offset_ed25519_block_issuer_key = storage_score_offset_ed25519_block_issuer_key;
        self
    }

    /// Sets the TODO for the staking fields.
    pub fn with_storage_score_offset_staking_feature(
        mut self,
        storage_score_offset_staking_feature: StorageScoreOffset,
    ) -> Self {
        self.storage_score_offset_staking_feature = storage_score_offset_staking_feature;
        self
    }

    /// Sets the TODO for the delegation fields.
    pub fn with_storage_score_offset_delegation(mut self, storage_score_offset_delegation: StorageScoreOffset) -> Self {
        self.storage_score_offset_delegation = storage_score_offset_delegation;
        self
    }

    /// Returns the TODO of the [`RentStructure`].
    pub const fn storage_cost(&self) -> u32 {
        self.storage_cost
    }

    /// Returns the TODO of the [`RentStructure`].
    pub const fn storage_score_factor_data(&self) -> StorageScoreFactor {
        self.storage_score_factor_data
    }

    /// Returns the TODO of the [`RentStructure`].
    pub const fn storage_score_offset_output(&self) -> StorageScoreOffset {
        self.storage_score_offset_output
    }

    /// Returns the TODO the [`RentStructure`].
    pub const fn storage_score_offset_ed25519_block_issuer_key(&self) -> StorageScoreOffset {
        self.storage_score_offset_ed25519_block_issuer_key
    }

    /// Returns the TODO the [`RentStructure`].
    pub const fn storage_score_offset_staking_feature(&self) -> StorageScoreOffset {
        self.storage_score_offset_staking_feature
    }

    /// Returns the TODO the [`RentStructure`].
    pub const fn storage_score_offset_delegation(&self) -> StorageScoreOffset {
        self.storage_score_offset_delegation
    }
}

/// A trait to facilitate the computation of the storage score of block outputs, which is central to dust protection.
pub trait StorageScore {
    /// Computes the byte offset given a [`RentStructure`].
    fn byte_offset(&self, rent_structure: RentStructure) -> u32 {
        // TODO: verify this
        // The ID of the output.
        size_of::<OutputId>() as u32 * rent_structure.storage_score_offset_output as u32
        // The ID of the block in which the transaction payload that created this output was included.
            + size_of::<BlockId>() as u32 * rent_structure.storage_score_factor_data as u32
            // The index of the slot in which the transaction that created it was booked.
            + size_of::<SlotIndex>() as u32 * rent_structure.storage_score_factor_data as u32
            // The index of the slot in which the transaction was created.
            + size_of::<SlotIndex>() as u32 * rent_structure.storage_score_factor_data as u32
    }

    /// Different fields in a type lead to different storage requirements for the ledger state.
    fn weighted_bytes(&self, config: RentStructure) -> u64;

    /// Computes the storage score given a [`RentStructure`].
    fn storage_score(&self, rent_structure: RentStructure) -> u64 {
        rent_structure.storage_cost as u64
            * (self.weighted_bytes(rent_structure) + self.byte_offset(rent_structure) as u64)
    }
}

impl<T: StorageScore, const N: usize> StorageScore for [T; N] {
    fn weighted_bytes(&self, config: RentStructure) -> u64 {
        self.iter().map(|elem| elem.weighted_bytes(config)).sum()
    }
}

pub struct MinimumStorageDepositBasicOutput {
    config: RentStructure,
    token_supply: u64,
    builder: BasicOutputBuilder,
}

impl MinimumStorageDepositBasicOutput {
    pub fn new(config: RentStructure, token_supply: u64) -> Self {
        Self {
            config,
            token_supply,
            builder: BasicOutputBuilder::new_with_amount(Output::AMOUNT_MIN).add_unlock_condition(
                AddressUnlockCondition::new(Address::from(Ed25519Address::from([0; Ed25519Address::LENGTH]))),
            ),
        }
    }

    pub fn with_native_tokens(mut self, native_tokens: impl Into<Option<NativeTokens>>) -> Self {
        if let Some(native_tokens) = native_tokens.into() {
            self.builder = self.builder.with_native_tokens(native_tokens);
        }
        self
    }

    pub fn with_storage_deposit_return(mut self) -> Result<Self, Error> {
        self.builder = self
            .builder
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                Address::from(Ed25519Address::from([0; Ed25519Address::LENGTH])),
                Output::AMOUNT_MIN,
                self.token_supply,
            )?);
        Ok(self)
    }

    pub fn with_expiration(mut self) -> Result<Self, Error> {
        self.builder = self.builder.add_unlock_condition(ExpirationUnlockCondition::new(
            Address::from(Ed25519Address::from([0; Ed25519Address::LENGTH])),
            1,
        )?);
        Ok(self)
    }

    pub fn finish(self) -> Result<u64, Error> {
        Ok(self
            .builder
            .finish_output(self.token_supply)?
            .storage_score(self.config))
    }
}
