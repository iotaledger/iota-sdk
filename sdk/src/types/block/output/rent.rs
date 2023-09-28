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
    /// Computes the byte offset given a [`RentStructure`].
    fn byte_offset(&self, rent_structure: RentStructure) -> u32 {
        // The ID of the output.
        size_of::<OutputId>() as u32 * rent_structure.v_byte_factor_key as u32
        // The ID of the block in which the transaction payload that created this output was included.
            + size_of::<BlockId>() as u32 * rent_structure.v_byte_factor_data as u32
            // The index of the slot in which the transaction that created it was booked.
            + size_of::<SlotIndex>() as u32 * rent_structure.v_byte_factor_data as u32
            // The index of the slot in which the transaction was created.
            + size_of::<SlotIndex>() as u32 * rent_structure.v_byte_factor_data as u32
    }

    /// Different fields in a type lead to different storage requirements for the ledger state.
    fn weighted_bytes(&self, config: RentStructure) -> u64;

    /// Computes the rent cost given a [`RentStructure`].
    fn rent_cost(&self, rent_structure: RentStructure) -> u64 {
        rent_structure.v_byte_cost as u64
            * (self.weighted_bytes(rent_structure) + self.byte_offset(rent_structure) as u64)
    }
}

impl<T: Rent, const N: usize> Rent for [T; N] {
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
        Ok(self.builder.finish_output(self.token_supply)?.rent_cost(self.config))
    }
}
