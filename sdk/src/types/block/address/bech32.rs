// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::hash::Hash;

use crate::types::block::address::Address;

/// An address and its network type.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Bech32Address {
    pub(crate) inner: Address,
    pub(crate) hrp: String,
}

impl AsRef<Address> for Bech32Address {
    fn as_ref(&self) -> &Address {
        &self.inner
    }
}

impl FromStr for Bech32Address {
    type Err = crate::types::block::Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        let (address, hrp) = Address::try_from_bech32_with_hrp(address)?;

        Ok(Self::new(address, hrp))
    }
}

impl Bech32Address {
    /// Create a new address wrapper.
    pub fn new(address: Address, hrp: String) -> Self {
        Self { inner: address, hrp }
    }

    /// Get the address part
    pub fn address(&self) -> &Address {
        &self.inner
    }

    /// Get the bech32 human readable part
    pub fn hrp(&self) -> &str {
        &self.hrp
    }

    /// Parses a bech32 address string.
    pub fn try_from_bech32<A: AsRef<str>>(address: A) -> Result<Self, crate::types::block::Error> {
        let (address, hrp) = Address::try_from_bech32_with_hrp(address)?;

        Ok(Self::new(address, hrp))
    }
}

/// Encodes the address as bech32.
impl core::fmt::Display for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner.to_bech32(&self.hrp))
    }
}

impl core::fmt::Debug for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bech32Address({self})")
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(Bech32Address);
