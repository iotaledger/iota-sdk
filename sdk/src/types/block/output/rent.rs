// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem::size_of;

use packable::{Packable, PackableExt};

const DEFAULT_BYTE_COST: u32 = 100;
const DEFAULT_BYTE_COST_FACTOR_KEY: u8 = 10;
const DEFAULT_BYTE_COST_FACTOR_DATA: u8 = 1;
// TODO: fill in the real values
const DEFAULT_BYTE_COST_FACTOR_DELEGATION: u8 = 1;
const DEFAULT_BYTE_COST_FACTOR_STAKING_FEATURE: u8 = 1;
const DEFAULT_BYTE_COST_FACTOR_BLOCK_ISSUER_KEY: u8 = 1;

/// Specifies the current parameters for the byte cost computation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct RentStructure {
    /// Cost in tokens per virtual byte.
    v_byte_cost: u32,
    /// The weight factor used for data fields in the outputs.
    v_byte_factor_data: u8,
    /// The weight factor used for key fields in the outputs.
    v_byte_factor_key: u8,
    /// The weight factor used for block issuer key fields in the outputs.
    v_byte_factor_block_issuer_key: u8,
    /// The weight factor used for staking fields in the outputs.
    v_byte_factor_staking_feature: u8,
    /// The weight factor used for delegation fields in the outputs.
    v_byte_factor_delegation: u8,
}

impl Default for RentStructure {
    fn default() -> Self {
        Self {
            v_byte_cost: DEFAULT_BYTE_COST,
            v_byte_factor_data: DEFAULT_BYTE_COST_FACTOR_DATA,
            v_byte_factor_key: DEFAULT_BYTE_COST_FACTOR_KEY,
            v_byte_factor_block_issuer_key: DEFAULT_BYTE_COST_FACTOR_BLOCK_ISSUER_KEY,
            v_byte_factor_staking_feature: DEFAULT_BYTE_COST_FACTOR_STAKING_FEATURE,
            v_byte_factor_delegation: DEFAULT_BYTE_COST_FACTOR_DELEGATION,
        }
    }
}

impl RentStructure {
    /// Creates a new [`RentStructure`].
    pub fn new(
        byte_cost: u32,
        byte_factor_data: u8,
        byte_factor_key: u8,
        byte_factor_block_issuer_key: u8,
        byte_factor_staking_feature: u8,
        byte_factor_delegation: u8,
    ) -> Self {
        Self {
            v_byte_cost: byte_cost,
            v_byte_factor_data: byte_factor_data,
            v_byte_factor_key: byte_factor_key,
            v_byte_factor_block_issuer_key: byte_factor_block_issuer_key,
            v_byte_factor_staking_feature: byte_factor_staking_feature,
            v_byte_factor_delegation: byte_factor_delegation,
        }
    }

    /// Sets the byte cost for the storage deposit.
    pub fn with_byte_cost(mut self, byte_cost: u32) -> Self {
        self.v_byte_cost = byte_cost;
        self
    }

    /// Sets the virtual byte weight for the data fields.
    pub fn with_byte_factor_data(mut self, byte_factor_data: u8) -> Self {
        self.v_byte_factor_data = byte_factor_data;
        self
    }

    /// Sets the virtual byte weight for the key fields.
    pub fn with_byte_factor_key(mut self, byte_factor_key: u8) -> Self {
        self.v_byte_factor_key = byte_factor_key;
        self
    }

    /// Sets the virtual byte weight for the block issuer key fields.
    pub fn with_byte_factor_block_issuer_key(mut self, byte_factor_block_issuer_key: u8) -> Self {
        self.v_byte_factor_block_issuer_key = byte_factor_block_issuer_key;
        self
    }

    /// Sets the virtual byte weight for the staking fields.
    pub fn with_byte_factor_staking_feature(mut self, byte_factor_staking_feature: u8) -> Self {
        self.v_byte_factor_staking_feature = byte_factor_staking_feature;
        self
    }

    /// Sets the virtual byte weight for the delegation fields.
    pub fn with_byte_factor_delegation(mut self, byte_factor_delegation: u8) -> Self {
        self.v_byte_factor_delegation = byte_factor_delegation;
        self
    }

    /// Returns the byte cost of the [`RentStructure`].
    pub const fn byte_cost(&self) -> u32 {
        self.v_byte_cost
    }

    /// Returns the byte factor data of the [`RentStructure`].
    pub const fn byte_factor_data(&self) -> u8 {
        self.v_byte_factor_data
    }

    /// Returns the byte factor key of the [`RentStructure`].
    pub const fn byte_factor_key(&self) -> u8 {
        self.v_byte_factor_key
    }

    /// Returns the block issuer key byte factor of the [`RentStructure`].
    pub const fn byte_factor_block_issuer_key(&self) -> u8 {
        self.v_byte_factor_block_issuer_key
    }

    /// Returns the staking byte factor of the [`RentStructure`].
    pub const fn byte_factor_staking_feature(&self) -> u8 {
        self.v_byte_factor_staking_feature
    }

    /// Returns the delegation byte factor of the [`RentStructure`].
    pub const fn byte_factor_delegation(&self) -> u8 {
        self.v_byte_factor_delegation
    }
}

/// A trait to facilitate the computation of the byte cost of block outputs, which is central to dust protection.
pub trait Rent {
    /// Different fields in a type lead to different storage requirements for the ledger state.
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder;

    fn weighted_bytes(&self, config: RentStructure) -> u64 {
        self.build_weighted_bytes(RentBuilder::new(config)).bytes
    }

    /// Computes the rent cost of this instance given a [`RentStructure`].
    fn rent_cost(&self, rent_structure: RentStructure) -> u64 {
        rent_structure.byte_cost() as u64 * self.weighted_bytes(rent_structure)
    }
}

pub struct RentBuilder {
    config: RentStructure,
    bytes: u64,
}

impl RentBuilder {
    pub const fn new(config: RentStructure) -> Self {
        Self { config, bytes: 0 }
    }

    pub const fn bytes(mut self, bytes: u64) -> Self {
        self.bytes += bytes;
        self
    }

    pub const fn key_field<T>(mut self) -> Self {
        self.bytes += size_of::<T>() as u64 * self.config.byte_factor_key() as u64;
        self
    }

    pub const fn data_field<T>(mut self) -> Self {
        self.bytes += size_of::<T>() as u64 * self.config.byte_factor_data() as u64;
        self
    }

    pub const fn delegation_field<T>(mut self) -> Self {
        self.bytes += size_of::<T>() as u64 * self.config.byte_factor_delegation() as u64;
        self
    }

    pub const fn staking_field<T>(mut self) -> Self {
        self.bytes += size_of::<T>() as u64 * self.config.byte_factor_staking_feature() as u64;
        self
    }

    pub const fn issuer_keys_field<T>(mut self) -> Self {
        self.bytes += size_of::<T>() as u64 * self.config.byte_factor_issuer_keys() as u64;
        self
    }

    pub fn weighted_field<T: Rent>(self, field: T) -> Self {
        field.build_weighted_bytes(self)
    }

    pub fn iter_field<'a, T: 'a + Rent>(mut self, field: impl IntoIterator<Item = &'a T>) -> Self {
        for elem in field {
            self = elem.build_weighted_bytes(self);
        }
        self
    }

    pub fn packable_key_field<T: Packable>(mut self, field: &T) -> Self {
        self.bytes += field.pack_to_vec().len() as u64 * self.config.byte_factor_key() as u64;
        self
    }

    pub fn packable_data_field<T: Packable>(mut self, field: &T) -> Self {
        self.bytes += field.pack_to_vec().len() as u64 * self.config.byte_factor_data() as u64;
        self
    }

    pub fn packable_delegation_field<T: Packable>(mut self, field: &T) -> Self {
        self.bytes += field.pack_to_vec().len() as u64 * self.config.byte_factor_delegation() as u64;
        self
    }

    pub fn packable_staking_field<T: Packable>(mut self, field: &T) -> Self {
        self.bytes += field.pack_to_vec().len() as u64 * self.config.byte_factor_staking_feature() as u64;
        self
    }

    pub fn packable_issuer_keys_field<T: Packable>(mut self, field: &T) -> Self {
        self.bytes += field.pack_to_vec().len() as u64 * self.config.byte_factor_issuer_keys() as u64;
        self
    }

    pub const fn finish(self) -> u64 {
        self.bytes
    }
}
