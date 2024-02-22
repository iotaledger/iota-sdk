// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Storage deposit is a concept that creates a monetary incentive to keep the ledger state small.
//! This is achieved by enforcing a minimum IOTA coin deposit in every output based on the disk space that will actually
//! be used to store it.
//! [TIP-47: Storage Deposit Dust Protection](https://github.com/iotaledger/tips/blob/tip47/tips/TIP-0047/tip-0047.md).

use packable::Packable;

use crate::types::block::{
    address::Ed25519Address,
    output::{
        feature::{BlockIssuerFeature, BlockIssuerKey, Ed25519PublicKeyHashBlockIssuerKey},
        AccountId, AccountOutputBuilder, AddressUnlockCondition, BasicOutputBuilder, OutputId,
    },
    slot::SlotIndex,
    BlockId,
};

// Parameters of storage score calculations on objects which take node resources.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct StorageScoreParameters {
    /// Number of IOTA tokens required per unit of storage score.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) storage_cost: u64,
    /// Factor to be used for data only fields.
    pub(crate) factor_data: u8,
    /// Offset to be applied to all outputs for the overhead of handling them in storage.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) offset_output_overhead: u64,
    /// Offset to be used for Ed25519-based block issuer keys.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) offset_ed25519_block_issuer_key: u64,
    /// Offset to be used for staking feature.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) offset_staking_feature: u64,
    /// Offset to be used for delegation.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) offset_delegation: u64,
}

impl StorageScoreParameters {
    /// Creates a new [`StorageScoreParameters`].
    pub fn new(
        storage_cost: u64,
        data_factor: u8,
        output_overhead_offset: u64,
        ed25519_block_issuer_key_offset: u64,
        staking_feature_offset: u64,
        delegation_offset: u64,
    ) -> Self {
        Self {
            storage_cost,
            factor_data: data_factor,
            offset_output_overhead: output_overhead_offset,
            offset_ed25519_block_issuer_key: ed25519_block_issuer_key_offset,
            offset_staking_feature: staking_feature_offset,
            offset_delegation: delegation_offset,
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

    /// Sets the storage score offset overhead per output.
    pub fn with_output_overhead_offset(mut self, offset: u64) -> Self {
        self.offset_output_overhead = offset;
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

    /// Returns the storage score offset overhead per output.
    pub fn output_overhead_offset(&self) -> u64 {
        self.offset_output_overhead
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

    /// Returns the storage score offset per output.
    pub fn output_offset(&self) -> u64 {
        self.output_overhead_offset()
            + (self.data_factor() as usize * (OutputId::LENGTH + BlockId::LENGTH + core::mem::size_of::<SlotIndex>()))
                as u64
    }

    /// Returns the storage score offset for implicit account creation address fields.
    pub fn implicit_account_creation_address_offset(&self) -> u64 {
        let null_address = Ed25519Address::null();
        let basic_output_score = BasicOutputBuilder::new_with_amount(0)
            .add_unlock_condition(AddressUnlockCondition::new(null_address))
            .finish()
            .unwrap()
            .storage_score(*self);
        let account_output_score = AccountOutputBuilder::new_with_amount(0, AccountId::null())
            .add_unlock_condition(AddressUnlockCondition::new(null_address))
            .add_feature(
                BlockIssuerFeature::new(
                    0,
                    [BlockIssuerKey::Ed25519PublicKeyHash(
                        Ed25519PublicKeyHashBlockIssuerKey::from([0; 32]),
                    )],
                )
                .unwrap(),
            )
            .finish()
            .unwrap()
            .storage_score(*self);
        account_output_score - basic_output_score + null_address.storage_score(*self)
    }
}

/// A trait to facilitate the computation of the byte cost of block outputs, which is central to dust protection.
pub trait StorageScore {
    fn storage_score(&self, _params: StorageScoreParameters) -> u64 {
        0
    }
}
