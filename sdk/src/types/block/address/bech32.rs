// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::hash::Hash;

use crate::types::block::{address::Address, Error};

/// An address and its network type.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Bech32Address {
    pub(crate) hrp: String,
    pub(crate) inner: Address,
}

impl AsRef<Address> for Bech32Address {
    fn as_ref(&self) -> &Address {
        &self.inner
    }
}

impl FromStr for Bech32Address {
    type Err = Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        let (hrp, inner) = Address::try_from_bech32_with_hrp(address)?;
        Ok(Self { hrp, inner })
    }
}

impl Bech32Address {
    /// Create a new address wrapper.
    pub fn new(address: Address, hrp: String) -> Result<Self, Error> {
        Ok(Self { inner: address, hrp })
    }

    /// Get the bech32 human readable part
    pub fn hrp(&self) -> &str {
        &self.hrp
    }

    /// Get the address part
    pub fn inner(&self) -> &Address {
        &self.inner
    }

    /// Parses a bech32 address string.
    pub fn try_from_bech32<A: AsRef<str>>(address: A) -> Result<Self, Error> {
        let (hrp, inner) = Address::try_from_bech32_with_hrp(address)?;

        Ok(Self { hrp, inner })
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
