// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{string::String, vec::Vec};
use core::str::FromStr;

use bech32::{FromBase32, ToBase32, Variant};
use derive_more::{AsRef, Deref};
use packable::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable, PackableExt};

use crate::types::block::{address::Address, Error};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Hrp {
    inner: [u8; 83],
    len: u8,
}

impl Hrp {
    /// Convert a string to an Hrp without checking validity.
    pub const fn from_str_unchecked(hrp: &str) -> Self {
        let len = hrp.len();
        let mut bytes = [0; 83];
        let hrp = hrp.as_bytes();
        let mut i = 0;
        while i < len {
            bytes[i] = hrp[i];
            i += 1;
        }
        Self {
            inner: bytes,
            len: len as _,
        }
    }
}

impl FromStr for Hrp {
    type Err = Error;

    fn from_str(hrp: &str) -> Result<Self, Self::Err> {
        let len = hrp.len();
        if hrp.is_ascii() && len <= 83 {
            let mut bytes = [0; 83];
            bytes[..len].copy_from_slice(hrp.as_bytes());
            Ok(Self {
                inner: bytes,
                len: len as _,
            })
        } else {
            Err(Error::InvalidBech32Hrp(hrp.to_owned()))
        }
    }
}

impl core::fmt::Display for Hrp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let hrp_str = self.inner[..self.len as usize]
            .iter()
            .map(|b| *b as char)
            .collect::<String>();
        f.write_str(&hrp_str)
    }
}

impl Packable for Hrp {
    type UnpackError = Error;
    type UnpackVisitor = ();

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.len.pack(packer)?;
        packer.pack_bytes(self.inner)?;

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let Ok(len) = u8::unpack::<_, VERIFY>(unpacker, visitor) else { unreachable!() };

        if len > 83 {
            return Err(UnpackError::Packable(Error::InvalidBech32Hrp(
                "hrp len above 83".to_owned(),
            )));
        }

        let mut bytes = alloc::vec![0u8; len as usize];
        unpacker.unpack_bytes(&mut bytes)?;

        let mut inner = [0; 83];
        inner[..len as usize].copy_from_slice(&bytes);

        Ok(Self { inner, len })
    }
}

impl PartialEq<String> for Hrp {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<&str> for Hrp {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<str> for Hrp {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(Hrp);

/// An address and its network type.
#[derive(Copy, Clone, Eq, PartialEq, Hash, AsRef, Deref)]
pub struct Bech32Address {
    pub(crate) hrp: Hrp,
    #[as_ref]
    #[deref]
    pub(crate) inner: Address,
}

impl FromStr for Bech32Address {
    type Err = Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        match ::bech32::decode(address) {
            Ok((hrp, data, _)) => {
                let hrp = hrp.parse()?;
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
    pub fn new(hrp: Hrp, inner: impl Into<Address>) -> Self {
        Self {
            hrp,
            inner: inner.into(),
        }
    }

    /// Creates a new address wrapper by parsing a string HRP.
    pub fn try_new(hrp: impl Into<String>, inner: impl Into<Address>) -> Result<Self, Error> {
        Ok(Self {
            hrp: hrp.into().parse()?,
            inner: inner.into(),
        })
    }

    /// Gets the human readable part.
    pub fn hrp(&self) -> &Hrp {
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
            ::bech32::encode(
                &self.hrp.to_string(),
                self.inner.pack_to_vec().to_base32(),
                Variant::Bech32
            )
            .unwrap()
        )
    }
}

impl core::fmt::Debug for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bech32Address({self})")
    }
}

impl PartialEq<String> for Bech32Address {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<&str> for Bech32Address {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<str> for Bech32Address {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}

impl<T: core::borrow::Borrow<Bech32Address>> From<T> for Address {
    fn from(value: T) -> Self {
        value.borrow().inner
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(Bech32Address);
