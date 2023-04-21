// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem::size_of;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{output::OutputId, payload::milestone::MilestoneIndex, BlockId, Error};

const DEFAULT_BYTE_COST: u32 = 100;
const DEFAULT_BYTE_COST_FACTOR_KEY: u8 = 10;
const DEFAULT_BYTE_COST_FACTOR_DATA: u8 = 1;

type ConfirmationUnixTimestamp = u32;

/// Builder for a [`RentStructure`].
#[derive(Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[must_use]
pub struct RentStructureBuilder {
    #[cfg_attr(feature = "serde", serde(alias = "vByteCost"))]
    v_byte_cost: Option<u32>,
    #[cfg_attr(feature = "serde", serde(alias = "vByteFactorKey"))]
    v_byte_factor_key: Option<u8>,
    #[cfg_attr(feature = "serde", serde(alias = "vByteFactorData"))]
    v_byte_factor_data: Option<u8>,
}

impl RentStructureBuilder {
    /// Returns a new [`RentStructureBuilder`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the byte cost for the storage deposit.
    pub fn byte_cost(mut self, byte_cost: impl Into<Option<u32>>) -> Self {
        self.v_byte_cost = byte_cost.into();
        self
    }

    /// Sets the virtual byte weight for the key fields.
    pub fn byte_factor_key(mut self, weight: impl Into<Option<u8>>) -> Self {
        self.v_byte_factor_key = weight.into();
        self
    }

    /// Sets the virtual byte weight for the data fields.
    pub fn byte_factor_data(mut self, weight: impl Into<Option<u8>>) -> Self {
        self.v_byte_factor_data = weight.into();
        self
    }

    /// Returns the built [`RentStructure`].
    pub fn finish(self) -> RentStructure {
        let v_byte_factor_key = self.v_byte_factor_key.unwrap_or(DEFAULT_BYTE_COST_FACTOR_KEY);
        let v_byte_factor_data = self.v_byte_factor_data.unwrap_or(DEFAULT_BYTE_COST_FACTOR_DATA);
        let v_byte_offset = v_byte_offset(v_byte_factor_key, v_byte_factor_data);

        RentStructure {
            v_byte_cost: self.v_byte_cost.unwrap_or(DEFAULT_BYTE_COST),
            v_byte_factor_key,
            v_byte_factor_data,
            v_byte_offset,
        }
    }
}

/// Specifies the current parameters for the byte cost computation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RentStructure {
    /// Cost in tokens per virtual byte.
    #[cfg_attr(feature = "serde", serde(alias = "vByteCost"))]
    v_byte_cost: u32,
    /// The weight factor used for key fields in the outputs.
    #[cfg_attr(feature = "serde", serde(alias = "vByteFactorKey"))]
    v_byte_factor_key: u8,
    /// The weight factor used for data fields in the outputs.
    #[cfg_attr(feature = "serde", serde(alias = "vByteFactorData"))]
    v_byte_factor_data: u8,
    /// The offset in addition to the other fields.
    #[cfg_attr(feature = "serde", serde(alias = "vByteOffset"))]
    v_byte_offset: u32,
}

impl Default for RentStructure {
    fn default() -> Self {
        RentStructureBuilder::new().finish()
    }
}

impl RentStructure {
    /// Creates a new [`RentStructure`].
    pub fn new(byte_cost: u32, byte_factor_key: u8, byte_factor_data: u8) -> Self {
        Self::build()
            .byte_cost(byte_cost)
            .byte_factor_key(byte_factor_key)
            .byte_factor_data(byte_factor_data)
            .finish()
    }

    /// Returns a builder for a [`RentStructure`].
    pub fn build() -> RentStructureBuilder {
        RentStructureBuilder::new()
    }

    /// Returns the byte cost of the [`RentStructure`].
    pub fn byte_cost(&self) -> u32 {
        self.v_byte_cost
    }

    /// Returns the byte factor key of the [`RentStructure`].
    pub fn byte_factor_key(&self) -> u8 {
        self.v_byte_factor_key
    }

    /// Returns the byte factor data of the [`RentStructure`].
    pub fn byte_factor_data(&self) -> u8 {
        self.v_byte_factor_data
    }

    /// Returns the byte offset of the [`RentStructure`].
    pub fn byte_offset(&self) -> u32 {
        self.v_byte_offset
    }
}

impl Packable for RentStructure {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.v_byte_cost.pack(packer)?;
        self.v_byte_factor_data.pack(packer)?;
        self.v_byte_factor_key.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let v_byte_cost = u32::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let v_byte_factor_data = u8::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let v_byte_factor_key = u8::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let v_byte_offset = v_byte_offset(v_byte_factor_key, v_byte_factor_data);

        Ok(Self {
            v_byte_cost,
            v_byte_factor_key,
            v_byte_factor_data,
            v_byte_offset,
        })
    }
}

/// A trait to facilitate the computation of the byte cost of block outputs, which is central to dust protection.
pub trait Rent {
    /// Different fields in a type lead to different storage requirements for the ledger state.
    fn weighted_bytes(&self, config: &RentStructure) -> u64;

    /// Computes the rent cost given a [`RentStructure`].
    fn rent_cost(&self, config: &RentStructure) -> u64 {
        config.v_byte_cost as u64 * (self.weighted_bytes(config) + config.v_byte_offset as u64)
    }
}

impl<T: Rent, const N: usize> Rent for [T; N] {
    fn weighted_bytes(&self, config: &RentStructure) -> u64 {
        self.iter().map(|elem| elem.weighted_bytes(config)).sum()
    }
}

fn v_byte_offset(v_byte_factor_key: u8, v_byte_factor_data: u8) -> u32 {
    size_of::<OutputId>() as u32 * v_byte_factor_key as u32
        + size_of::<BlockId>() as u32 * v_byte_factor_data as u32
        + size_of::<MilestoneIndex>() as u32 * v_byte_factor_data as u32
        + size_of::<ConfirmationUnixTimestamp>() as u32 * v_byte_factor_data as u32
}

#[allow(missing_docs)]
pub mod dto {

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(rename_all = "camelCase")
    )]
    pub struct RentStructureDto {
        pub v_byte_cost: u32,
        pub v_byte_factor_key: u8,
        pub v_byte_factor_data: u8,
    }

    impl From<RentStructureDto> for RentStructure {
        fn from(value: RentStructureDto) -> Self {
            Self::new(value.v_byte_cost, value.v_byte_factor_key, value.v_byte_factor_data)
        }
    }
}
