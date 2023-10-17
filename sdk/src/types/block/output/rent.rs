// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;

use super::{
    feature::{BlockIssuerFeature, BlockIssuerKey, Ed25519BlockIssuerKey},
    unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
    AccountId, AccountOutputBuilder, AddressUnlockCondition, BasicOutputBuilder,
};
use crate::types::block::address::Ed25519Address;

const DEFAULT_STORAGE_COST: u64 = 500;
const DEFAULT_STORAGE_SCORE_FACTOR_DATA: u8 = 1;
const DEFAULT_STORAGE_SCORE_OFFSET_OUTPUT: u64 = 10;
const DEFAULT_STORAGE_SCORE_OFFSET_ED25519_BLOCK_ISSUER_KEY: u64 = 50;
const DEFAULT_STORAGE_SCORE_OFFSET_STAKING_FEATURE: u64 = 100;
const DEFAULT_STORAGE_SCORE_OFFSET_DELEGATION: u64 = 100;

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
    storage_score_factor_data: u8,
    /// Defines the offset to be used for key/lookup generating fields.
    storage_score_offset_output: u64,
    /// Defines the offset to be used for block issuer feature public keys.
    storage_score_offset_ed25519_block_issuer_key: u64,
    /// Defines the offset to be used for staking feature.
    storage_score_offset_staking_feature: u64,
    /// Defines the offset to be used for delegation output.
    storage_score_offset_delegation: u64,
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
        storage_score_factor_data: u8,
        storage_score_offset_output: u64,
        storage_score_offset_ed25519_block_issuer_key: u64,
        storage_score_offset_staking_feature: u64,
        storage_score_offset_delegation: u64,
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
    pub fn with_storage_score_factor_data(mut self, storage_score_factor_data: u8) -> Self {
        self.storage_score_factor_data = storage_score_factor_data;
        self
    }

    /// Sets the storage score offset per output.
    pub fn with_storage_score_offset_output(mut self, storage_score_offset_output: u64) -> Self {
        self.storage_score_offset_output = storage_score_offset_output;
        self
    }

    /// Sets the storage score offset for Ed25519 block issuer key fields.
    pub fn with_storage_score_offset_ed25519_block_issuer_key(
        mut self,
        storage_score_offset_ed25519_block_issuer_key: u64,
    ) -> Self {
        self.storage_score_offset_ed25519_block_issuer_key = storage_score_offset_ed25519_block_issuer_key;
        self
    }

    /// Sets the storage score offset for staking fields.
    pub fn with_storage_score_offset_staking_feature(mut self, storage_score_offset_staking_feature: u64) -> Self {
        self.storage_score_offset_staking_feature = storage_score_offset_staking_feature;
        self
    }

    /// Sets the storage score offset for delegation fields.
    pub fn with_storage_score_offset_delegation(mut self, storage_score_offset_delegation: u64) -> Self {
        self.storage_score_offset_delegation = storage_score_offset_delegation;
        self
    }

    /// Returns the storage cost per unit of storage score.
    pub fn storage_cost(&self) -> u64 {
        self.storage_cost
    }

    /// Returns the storage score factor for data fields.
    pub fn storage_score_factor_data(&self) -> u8 {
        self.storage_score_factor_data
    }

    /// Returns the storage score offset per output.
    pub fn storage_score_offset_output(&self) -> u64 {
        self.storage_score_offset_output
    }

    /// Returns the storage score offset for Ed25519 block issuer key fields.
    pub fn storage_score_offset_ed25519_block_issuer_key(&self) -> u64 {
        self.storage_score_offset_ed25519_block_issuer_key
    }

    /// Returns the storage score offset for staking fields.
    pub fn storage_score_offset_staking_feature(&self) -> u64 {
        self.storage_score_offset_staking_feature
    }

    /// Returns the storage score offset for delegation fields.
    pub fn storage_score_offset_delegation(&self) -> u64 {
        self.storage_score_offset_delegation
    }

    pub fn storage_score_offset_implicit_account_creation_address(&self) -> u64 {
        let null_address = Ed25519Address::null();
        let basic_output_score = BasicOutputBuilder::new_with_amount(0)
            .add_unlock_condition(AddressUnlockCondition::new(null_address))
            .storage_score(*self);
        let ed25519_address_score = null_address.storage_score(*self);
        let basic_score_without_address = basic_output_score - ed25519_address_score;
        let account_output_score = AccountOutputBuilder::new_with_amount(0, AccountId::null())
            .add_unlock_condition(GovernorAddressUnlockCondition::new(null_address))
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(null_address))
            .add_feature(
                BlockIssuerFeature::new(0, vec![BlockIssuerKey::Ed25519(Ed25519BlockIssuerKey::null())]).unwrap(),
            )
            .storage_score(*self);
        account_output_score - basic_score_without_address
    }
}

/// A trait to facilitate the computation of the byte cost of block outputs, which is central to dust protection.
pub trait StorageScore {
    fn storage_score(&self, _params: RentParameters) -> u64 {
        0
    }

    /// Computes the minimum deposit of this instance given [`RentParameters`].
    fn min_deposit(&self, params: RentParameters) -> u64 {
        params.storage_cost() * self.storage_score(params)
    }
}
