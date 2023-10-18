// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;

use crate::types::block::{
    address::Ed25519Address,
    output::{
        feature::{BlockIssuerFeature, BlockIssuerKey, Ed25519BlockIssuerKey},
        unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
        AccountId, AccountOutputBuilder, AddressUnlockCondition, BasicOutputBuilder,
    },
};

const DEFAULT_STORAGE_COST: u64 = 500;
const DEFAULT_FACTOR_DATA: u8 = 1;
const DEFAULT_OFFSET_OUTPUT: u64 = 10;
const DEFAULT_OFFSET_ED25519_BLOCK_ISSUER_KEY: u64 = 50;
const DEFAULT_OFFSET_STAKING_FEATURE: u64 = 100;
const DEFAULT_OFFSET_DELEGATION: u64 = 100;

// Defines the parameters of storage score calculations on objects which take node resources.
// This structure defines the minimum base token deposit required on an object. This deposit does not
// generate Mana, which serves as a payment in Mana for storing the object.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct StorageScoreParameters {
    /// Defines the number of IOTA tokens required per unit of storage score.
    storage_cost: u64,
    /// Defines the factor to be used for data only fields.
    factor_data: u8,
    /// Defines the offset to be used for key/lookup generating fields.
    offset_output: u64,
    /// Defines the offset to be used for block issuer feature public keys.
    offset_ed25519_block_issuer_key: u64,
    /// Defines the offset to be used for staking feature.
    offset_staking_feature: u64,
    /// Defines the offset to be used for delegation output.
    offset_delegation: u64,
}

impl Default for StorageScoreParameters {
    fn default() -> Self {
        Self {
            storage_cost: DEFAULT_STORAGE_COST,
            factor_data: DEFAULT_FACTOR_DATA,
            offset_output: DEFAULT_OFFSET_OUTPUT,
            offset_ed25519_block_issuer_key: DEFAULT_OFFSET_ED25519_BLOCK_ISSUER_KEY,
            offset_staking_feature: DEFAULT_OFFSET_STAKING_FEATURE,
            offset_delegation: DEFAULT_OFFSET_DELEGATION,
        }
    }
}

impl StorageScoreParameters {
    /// Creates a new [`StorageScoreParameters`].
    pub fn new(
        storage_cost: u64,
        factor_data: u8,
        offset_output: u64,
        offset_ed25519_block_issuer_key: u64,
        offset_staking_feature: u64,
        offset_delegation: u64,
    ) -> Self {
        Self {
            storage_cost,
            factor_data,
            offset_output,
            offset_ed25519_block_issuer_key,
            offset_staking_feature,
            offset_delegation,
        }
    }

    /// Sets the storage cost per unit of storage score.
    pub fn with_storage_cost(mut self, storage_cost: u64) -> Self {
        self.storage_cost = storage_cost;
        self
    }

    /// Sets the storage score factor for data fields.
    pub fn with_data_factor(mut self, factor: u8) -> Self {
        self.factor_data = factor;
        self
    }

    /// Sets the storage score offset per output.
    pub fn with_output_offset(mut self, offset: u64) -> Self {
        self.offset_output = offset;
        self
    }

    /// Sets the storage score offset for Ed25519 block issuer key fields.
    pub fn with_ed25519_block_issuer_key_offset(mut self, offset: u64) -> Self {
        self.offset_ed25519_block_issuer_key = offset;
        self
    }

    /// Sets the storage score offset for staking fields.
    pub fn with_staking_feature_offset(mut self, offset: u64) -> Self {
        self.offset_staking_feature = offset;
        self
    }

    /// Sets the storage score offset for delegation fields.
    pub fn with_delegation_offset(mut self, offset: u64) -> Self {
        self.offset_delegation = offset;
        self
    }

    /// Returns the storage cost per unit of storage score.
    pub fn storage_cost(&self) -> u64 {
        self.storage_cost
    }

    /// Returns the storage score factor for data fields.
    pub fn data_factor(&self) -> u8 {
        self.factor_data
    }

    /// Returns the storage score offset per output.
    pub fn output_offset(&self) -> u64 {
        self.offset_output
    }

    /// Returns the storage score offset for Ed25519 block issuer key fields.
    pub fn ed25519_block_issuer_key_offset(&self) -> u64 {
        self.offset_ed25519_block_issuer_key
    }

    /// Returns the storage score offset for staking fields.
    pub fn staking_feature_offset(&self) -> u64 {
        self.offset_staking_feature
    }

    /// Returns the storage score offset for delegation fields.
    pub fn delegation_offset(&self) -> u64 {
        self.offset_delegation
    }

    pub fn implicit_account_creation_address_offset(&self) -> u64 {
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
    fn storage_score(&self, _params: StorageScoreParameters) -> u64 {
        0
    }

    /// Computes the storage cost of this instance given [`StorageScoreParameters`].
    fn storage_cost(&self, params: StorageScoreParameters) -> u64 {
        params.storage_cost() * self.storage_score(params)
    }
}
