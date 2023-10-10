// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::{mem::size_of, ops::Deref};

use packable::Packable;

use crate::types::block::{
    address::{Address, Ed25519Address},
    output::{
        feature::{BlockIssuerFeature, BlockIssuerKey, Ed25519BlockIssuerKey},
        unlock_condition::{
            AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
            StateControllerAddressUnlockCondition, StorageDepositReturnUnlockCondition,
        },
        AccountId, AccountOutput, AccountOutputBuilder, BasicOutput, BasicOutputBuilder, NativeTokens, Output,
        OutputId,
    },
    slot::SlotIndex,
    BlockId, Error,
};

// TODO: fill in the real values and/or verify
const DEFAULT_STORAGE_COST: u64 = 500;
const DEFAULT_STORAGE_SCORE_FACTOR_DATA: StorageScoreFactor = 1;
const DEFAULT_STORAGE_SCORE_OFFSET_OUTPUT: StorageScoreOffset = 10;
const DEFAULT_STORAGE_SCORE_OFFSET_ED25519_BLOCK_ISSUER_KEY: StorageScoreOffset = 50;
const DEFAULT_STORAGE_SCORE_OFFSET_STAKING_FEATURE: StorageScoreOffset = 100;
const DEFAULT_STORAGE_SCORE_OFFSET_DELEGATION: StorageScoreOffset = 100;

type StorageScoreFactor = u8;
type StorageScoreOffset = u64;

// Includes the rent parameters and the additional factors/offsets computed from these parameters.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RentStructure {
    rent_parameters: RentParameters,
    storage_score_offset_implicit_account_creation_address: u64,
}

impl RentStructure {
    /// Creates a new [`RentStructure`]. Computes the score offset for implicit account creation addresses.
    pub fn new(rent_parameters: RentParameters) -> Self {
        let mut rent_structure = Self {
            rent_parameters,
            storage_score_offset_implicit_account_creation_address: 0,
        };

        // TODO: check TIP for changes!
        // Set the storage score offset for implicit account creation addresses as
        // the difference between the storage score of the dummy account and the storage
        // score of the dummy basic output minus the storage score of the dummy address.

        // Unwrap: cannot fail for provided dummy data.
        let basic_output_score = BasicOutputBuilder::new_with_amount(0)
            .add_unlock_condition(AddressUnlockCondition::new(Ed25519Address::null()))
            .finish()
            .unwrap()
            .storage_score(rent_structure);
        let ed25519_address_score = Ed25519Address::null().storage_score(rent_structure);
        // Unwrap: should never underflow.
        let basic_score_without_address = basic_output_score
            .checked_sub(ed25519_address_score)
            .expect("underflow");
        // Unwrap: cannot fail for provided dummy data.
        let account_output_score = AccountOutputBuilder::new_with_amount(0, AccountId::null())
            .add_unlock_condition(GovernorAddressUnlockCondition::new(Ed25519Address::null()))
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(Ed25519Address::null()))
            .add_feature(
                BlockIssuerFeature::new(0, vec![BlockIssuerKey::Ed25519(Ed25519BlockIssuerKey::null())]).unwrap(),
            )
            .finish()
            .unwrap()
            .storage_score(rent_structure);
        // Unwrap: should never underflow.
        rent_structure.storage_score_offset_implicit_account_creation_address = account_output_score
            .checked_sub(basic_score_without_address)
            .expect("underflow");
        rent_structure
    }
}

impl Deref for RentStructure {
    type Target = RentParameters;

    fn deref(&self) -> &Self::Target {
        &self.rent_parameters
    }
}

impl From<RentParameters> for RentStructure {
    fn from(rent_parameters: RentParameters) -> Self {
        Self::new(rent_parameters)
    }
}

// Defines the parameters of storage score calculations on objects which take node resources.
// This structure defines the minimum base token deposit required on an object. This deposit does not
// generate Mana, which serves as a rent payment in Mana for storing the object.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct RentParameters {
    /// Defines the number of IOTA tokens required per unit of storage score.
    storage_cost: u64,
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

impl Default for RentParameters {
    fn default() -> Self {
        Self {
            storage_cost: DEFAULT_STORAGE_COST,
            storage_score_factor_data: DEFAULT_STORAGE_SCORE_FACTOR_DATA,
            storage_score_offset_output: DEFAULT_STORAGE_SCORE_OFFSET_OUTPUT,
            storage_score_offset_ed25519_block_issuer_key: DEFAULT_STORAGE_SCORE_OFFSET_ED25519_BLOCK_ISSUER_KEY,
            storage_score_offset_staking_feature: DEFAULT_STORAGE_SCORE_OFFSET_STAKING_FEATURE,
            storage_score_offset_delegation: DEFAULT_STORAGE_SCORE_OFFSET_DELEGATION,
        }
    }
}

impl RentParameters {
    /// Creates a new [`RentParameters`].
    pub fn new(
        storage_cost: u64,
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

    /// Sets the storage cost per unit of storage score.
    pub fn with_storage_cost(mut self, storage_cost: u64) -> Self {
        self.storage_cost = storage_cost;
        self
    }

    /// Sets the storage score factor for data fields.
    pub fn with_storage_score_factor_data(mut self, storage_score_factor_data: StorageScoreFactor) -> Self {
        self.storage_score_factor_data = storage_score_factor_data;
        self
    }

    /// Sets the storage score offset per output.
    pub fn with_storage_score_offset_output(mut self, storage_score_offset_output: StorageScoreOffset) -> Self {
        self.storage_score_offset_output = storage_score_offset_output;
        self
    }

    /// Sets the storage score offset for Ed25519 block issuer key fields.
    pub fn with_storage_score_offset_ed25519_block_issuer_key(
        mut self,
        storage_score_offset_ed25519_block_issuer_key: StorageScoreOffset,
    ) -> Self {
        self.storage_score_offset_ed25519_block_issuer_key = storage_score_offset_ed25519_block_issuer_key;
        self
    }

    /// Sets the storage score offset for staking fields.
    pub fn with_storage_score_offset_staking_feature(
        mut self,
        storage_score_offset_staking_feature: StorageScoreOffset,
    ) -> Self {
        self.storage_score_offset_staking_feature = storage_score_offset_staking_feature;
        self
    }

    /// Sets the storage score offset for delegation fields.
    pub fn with_storage_score_offset_delegation(mut self, storage_score_offset_delegation: StorageScoreOffset) -> Self {
        self.storage_score_offset_delegation = storage_score_offset_delegation;
        self
    }

    /// Returns the storage cost per unit of storage score.
    pub const fn storage_cost(&self) -> u64 {
        self.storage_cost
    }

    /// Returns the storage score factor for data fields.
    pub const fn storage_score_factor_data(&self) -> StorageScoreFactor {
        self.storage_score_factor_data
    }

    /// Returns the storage score offset per output.
    pub const fn storage_score_offset_output(&self) -> StorageScoreOffset {
        self.storage_score_offset_output
    }

    /// Returns the storage score offset for Ed25519 block issuer key fields.
    pub const fn storage_score_offset_ed25519_block_issuer_key(&self) -> StorageScoreOffset {
        self.storage_score_offset_ed25519_block_issuer_key
    }

    /// Returns the storage score offset for staking fields.
    pub const fn storage_score_offset_staking_feature(&self) -> StorageScoreOffset {
        self.storage_score_offset_staking_feature
    }

    /// Returns the storage score offset for delegation fields.
    pub const fn storage_score_offset_delegation(&self) -> StorageScoreOffset {
        self.storage_score_offset_delegation
    }
}

/// A trait to facilitate the rent cost computation for implementing types, which is central to dust protection.
pub trait StorageScore {
    /// Computes the storage score given a [`RentStructure`]. Different fields in a type lead to different storage
    /// requirements for the ledger state.
    fn storage_score(&self, rent_structure: RentStructure) -> u64;

    /// Computes the rent cost given a [`RentStructure`].
    fn rent_cost(&self, rent_structure: RentStructure) -> u64 {
        rent_structure.storage_cost as u64 * self.storage_score(rent_structure)
    }
}

impl<T: StorageScore, const N: usize> StorageScore for [T; N] {
    fn storage_score(&self, rent_structure: RentStructure) -> u64 {
        self.iter().map(|elem| elem.storage_score(rent_structure)).sum()
    }
}
