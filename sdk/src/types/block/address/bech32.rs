// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::str::FromStr;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::{AsRef, Deref, Display};
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

use crate::{
    types::block::address::{Address, AddressError, MultiAddress},
    utils::{ConversionError, ConvertTo},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Display)]
#[repr(transparent)]
pub struct Hrp(bech32::Hrp);

impl core::fmt::Debug for Hrp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Hrp")
            .field("display", &self.0.to_string())
            .field("bytes", &prefix_hex::encode(self.0.byte_iter().collect::<Vec<_>>()))
            .field("len", &self.0.len())
            .finish()
    }
}

impl Hrp {
    /// Convert a string to an Hrp without checking validity.
    pub const fn from_str_unchecked(hrp: &str) -> Self {
        Self(bech32::Hrp::parse_unchecked(hrp))
    }
}

impl FromStr for Hrp {
    type Err = AddressError;

    fn from_str(hrp: &str) -> Result<Self, Self::Err> {
        Ok(Self(bech32::Hrp::parse(hrp)?))
    }
}

impl Packable for Hrp {
    type UnpackError = AddressError;
    type UnpackVisitor = ();

    #[inline]
    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        (self.0.len() as u8).pack(packer)?;
        packer.pack_bytes(self.0.as_bytes())?;

        Ok(())
    }

    #[inline]
    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let len = u8::unpack(unpacker, visitor).coerce()? as usize;

        let mut bytes = alloc::vec![0u8; len];
        unpacker.unpack_bytes(&mut bytes)?;

        Ok(Self(
            bech32::Hrp::parse(&String::from_utf8_lossy(&bytes))
                .map_err(|e| UnpackError::Packable(AddressError::Bech32Hrp(e)))?,
        ))
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
crate::string_serde_impl!(Hrp);

impl<T: AsRef<str> + Send> ConvertTo<Hrp> for T {
    fn convert(self) -> Result<Hrp, ConversionError> {
        Hrp::from_str(self.as_ref()).map_err(ConversionError::new)
    }

    fn convert_unchecked(self) -> Hrp {
        Hrp::from_str_unchecked(self.as_ref())
    }
}

/// An address and its network type.
#[derive(Clone, Eq, PartialEq, Hash, AsRef, Deref, Ord, PartialOrd)]
pub struct Bech32Address {
    pub(crate) hrp: Hrp,
    #[as_ref]
    #[deref]
    pub(crate) inner: Address,
}

impl FromStr for Bech32Address {
    type Err = AddressError;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        match bech32::decode(address) {
            Ok((hrp, bytes)) => Address::unpack_bytes_verified(bytes.as_slice(), &())
                .map_err(|e| match e {
                    UnpackError::Packable(e) => e,
                    UnpackError::Unpacker(_) => AddressError::Length(bytes.len()),
                })
                .map(|address| Self {
                    hrp: Hrp(hrp),
                    inner: address,
                }),
            Err(e) => Err(AddressError::Bech32Encoding(e)),
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
    pub fn try_new(hrp: impl ConvertTo<Hrp>, inner: impl Into<Address>) -> Result<Self, AddressError> {
        Ok(Self {
            hrp: hrp.convert()?,
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
    pub fn try_from_str(address: impl AsRef<str>) -> Result<Self, AddressError> {
        Self::from_str(address.as_ref())
    }
}

impl core::fmt::Display for Bech32Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = if self.inner.is_multi() {
            core::iter::once(MultiAddress::KIND)
                .chain(Blake2b256::digest(self.inner.pack_to_vec()))
                .collect()
        } else {
            self.inner.pack_to_vec()
        };

        // PANIC: unwrap is fine as the Bech32Address has been validated at construction.
        write!(f, "{}", bech32::encode::<bech32::Bech32>(self.hrp.0, &bytes).unwrap())
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
        value.borrow().inner.clone()
    }
}

#[cfg(feature = "serde")]
crate::string_serde_impl!(Bech32Address);

impl<T: AsRef<str> + Send> ConvertTo<Bech32Address> for T {
    fn convert(self) -> Result<Bech32Address, ConversionError> {
        Bech32Address::try_from_str(self).map_err(ConversionError::new)
    }
}
