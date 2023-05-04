// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{string::String, vec::Vec};
use core::str::FromStr;

use bech32::{FromBase32, ToBase32, Variant};
use derive_more::{AsRef, Deref};
use packable::PackableExt;

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
        match ::bech32::decode(address) {
            Ok((hrp, data, _)) => {
                let bytes = Vec::<u8>::from_base32(&data).map_err(|_| Error::InvalidAddress)?;
                Address::unpack_verified(bytes.as_slice(), &())
                    .map_err(|_| Error::InvalidAddress)
                    .map(|address| Self { hrp, inner: address })
            }
            Err(_) => Err(Error::InvalidAddress),
        }
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

    /// Discard the hrp and get the address.
    pub fn into_inner(self) -> Address {
        self.inner
    }

    /// Parses a bech32 address string.
    pub fn try_from_str(address: impl AsRef<str>) -> Result<Self, Error> {
        Self::from_str(address.as_ref())
    }
}

impl core::fmt::Display for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            ::bech32::encode(&self.hrp, self.inner.pack_to_vec().to_base32(), Variant::Bech32).unwrap()
        )
    }
}

impl core::fmt::Debug for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bech32Address({self})")
    }
}

impl<T: core::borrow::Borrow<Bech32Address>> From<T> for Address {
    fn from(value: T) -> Self {
        value.borrow().inner
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(Bech32Address);
