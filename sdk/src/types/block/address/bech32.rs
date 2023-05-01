// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref};

use crate::types::block::{address::Address, Error};

/// An address and its network type.
#[derive(Clone, Eq, PartialEq, Hash, AsRef, Deref)]
pub struct Bech32Address {
    pub(crate) hrp: String,
    #[as_ref]
    #[deref]
    pub(crate) inner: Address,
}

impl FromStr for Bech32Address {
    type Err = Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        let (hrp, inner) = Address::try_from_bech32_with_hrp(address)?;
        Ok(Self { hrp, inner })
    }
}

impl Bech32Address {
    /// Creates a new address wrapper.
    pub fn new(hrp: impl Into<String>, inner: impl Into<Address>) -> Result<Self, Error> {
        // TODO validate HRP
        Ok(Self {
            hrp: hrp.into(),
            inner: inner.into(),
        })
    }

    /// Gets the human readable part.
    pub fn hrp(&self) -> &str {
        &self.hrp
    }

    /// Gets the address part.
    pub fn inner(&self) -> &Address {
        &self.inner
    }

    /// Parses a bech32 address string.
    pub fn try_from_str(address: impl AsRef<str>) -> Result<Self, Error> {
        Self::from_str(address.as_ref())
    }
}

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

impl<T: std::borrow::Borrow<Bech32Address>> From<T> for Address {
    fn from(value: T) -> Self {
        value.borrow().inner
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(Bech32Address);
