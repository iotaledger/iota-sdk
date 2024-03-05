// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::{
    types::block::{
        address::{AddressCapabilityFlag, WeightedAddressCount},
        capabilities::CapabilityError,
        IdentifierError,
    },
    utils::ConversionError,
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum AddressError {
    #[display(fmt = "invalid address kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid address weight: {_0}")]
    Weight(u8),
    #[display(fmt = "invalid address length: {_0}")]
    Length(usize),
    #[display(fmt = "invalid multi address threshold: {_0}")]
    MultiAddressThreshold(u16),
    #[display(fmt = "invalid multi address cumulative weight {cumulative_weight} < threshold {threshold}")]
    MultiAddressCumulativeWeight { cumulative_weight: u16, threshold: u16 },
    #[display(fmt = "invalid weighted address count: {_0}")]
    WeightedAddressCount(<WeightedAddressCount as TryFrom<usize>>::Error),
    #[display(fmt = "weighted addresses are not unique and/or sorted")]
    WeightedAddressesNotUniqueSorted,
    #[display(fmt = "restricted address capability: {_0:?}")]
    RestrictedAddressCapability(AddressCapabilityFlag),
    #[from]
    Bech32Encoding(::bech32::DecodeError),
    #[from]
    Bech32Hrp(::bech32::primitives::hrp::Error),
    #[from]
    Hex(prefix_hex::Error),
    #[from]
    Identifier(IdentifierError),
    #[from]
    Convert(ConversionError),
    #[from]
    Capability(CapabilityError),
}

#[cfg(feature = "std")]
impl std::error::Error for AddressError {}

impl From<Infallible> for AddressError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
