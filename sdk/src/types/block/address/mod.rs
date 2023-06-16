// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias;
mod bech32;
mod ed25519;
mod nft;

use derive_more::From;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use self::{
    alias::AliasAddress,
    bech32::{Bech32Address, Hrp},
    ed25519::Ed25519Address,
    nft::NftAddress,
};
use crate::types::block::{
    output::{Output, OutputId},
    semantic::{ConflictReason, ValidationContext},
    signature::Signature,
    unlock::Unlock,
    ConvertTo, Error,
};

/// A generic address supporting different address kinds.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[packable(tag_type = u8, with_error = Error::InvalidAddressKind)]
#[packable(unpack_error = Error)]
pub enum Address {
    /// An Ed25519 address.
    #[packable(tag = Ed25519Address::KIND)]
    Ed25519(Ed25519Address),
    /// An alias address.
    #[packable(tag = AliasAddress::KIND)]
    Alias(AliasAddress),
    /// An NFT address.
    #[packable(tag = NftAddress::KIND)]
    Nft(NftAddress),
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(address) => address.fmt(f),
            Self::Alias(address) => address.fmt(f),
            Self::Nft(address) => address.fmt(f),
        }
    }
}

impl Address {
    /// Returns the address kind of an [`Address`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519Address::KIND,
            Self::Alias(_) => AliasAddress::KIND,
            Self::Nft(_) => NftAddress::KIND,
        }
    }

    /// Checks whether the address is an [`Ed25519Address`].
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Gets the address as an actual [`Ed25519Address`].
    /// PANIC: do not call on a non-ed25519 address.
    pub fn as_ed25519(&self) -> &Ed25519Address {
        if let Self::Ed25519(address) = self {
            address
        } else {
            panic!("as_ed25519 called on a non-ed25519 address");
        }
    }

    /// Checks whether the address is an [`AliasAddress`].
    pub fn is_alias(&self) -> bool {
        matches!(self, Self::Alias(_))
    }

    /// Gets the address as an actual [`AliasAddress`].
    /// PANIC: do not call on a non-alias address.
    pub fn as_alias(&self) -> &AliasAddress {
        if let Self::Alias(address) = self {
            address
        } else {
            panic!("as_alias called on a non-alias address");
        }
    }

    /// Checks whether the address is an [`NftAddress`].
    pub fn is_nft(&self) -> bool {
        matches!(self, Self::Nft(_))
    }

    /// Gets the address as an actual [`NftAddress`].
    /// PANIC: do not call on a non-nft address.
    pub fn as_nft(&self) -> &NftAddress {
        if let Self::Nft(address) = self {
            address
        } else {
            panic!("as_nft called on a non-nft address");
        }
    }

    /// Tries to create an [`Address`] from a bech32 encoded string.
    pub fn try_from_bech32(address: impl AsRef<str>) -> Result<Self, Error> {
        Bech32Address::try_from_str(address).map(|res| res.inner)
    }

    /// Checks if an string is a valid bech32 encoded address.
    #[must_use]
    pub fn is_valid_bech32(address: &str) -> bool {
        Self::try_from_bech32(address).is_ok()
    }

    ///
    pub fn unlock(
        &self,
        unlock: &Unlock,
        inputs: &[(OutputId, &Output)],
        context: &mut ValidationContext<'_>,
    ) -> Result<(), ConflictReason> {
        match (self, unlock) {
            (Self::Ed25519(ed25519_address), Unlock::Signature(unlock)) => {
                if context.unlocked_addresses.contains(self) {
                    return Err(ConflictReason::InvalidUnlock);
                }

                let Signature::Ed25519(signature) = unlock.signature();

                if signature.is_valid(&context.essence_hash, ed25519_address).is_err() {
                    return Err(ConflictReason::InvalidSignature);
                }

                context.unlocked_addresses.insert(*self);
            }
            (Self::Ed25519(_ed25519_address), Unlock::Reference(_unlock)) => {
                // TODO actually check that it was unlocked by the same signature.
                if !context.unlocked_addresses.contains(self) {
                    return Err(ConflictReason::InvalidUnlock);
                }
            }
            (Self::Alias(alias_address), Unlock::Alias(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Alias(alias_output)) = inputs[unlock.index() as usize] {
                    if &alias_output.alias_id_non_null(&output_id) != alias_address.alias_id() {
                        return Err(ConflictReason::InvalidUnlock);
                    }
                    if !context.unlocked_addresses.contains(self) {
                        return Err(ConflictReason::InvalidUnlock);
                    }
                } else {
                    return Err(ConflictReason::InvalidUnlock);
                }
            }
            (Self::Nft(nft_address), Unlock::Nft(unlock)) => {
                // PANIC: indexing is fine as it is already syntactically verified that indexes reference below.
                if let (output_id, Output::Nft(nft_output)) = inputs[unlock.index() as usize] {
                    if &nft_output.nft_id_non_null(&output_id) != nft_address.nft_id() {
                        return Err(ConflictReason::InvalidUnlock);
                    }
                    if !context.unlocked_addresses.contains(self) {
                        return Err(ConflictReason::InvalidUnlock);
                    }
                } else {
                    return Err(ConflictReason::InvalidUnlock);
                }
            }
            _ => return Err(ConflictReason::InvalidUnlock),
        }

        Ok(())
    }
}

pub trait ToBech32Ext: Sized {
    /// Try to encode this address to a bech32 string with the given Human Readable Part as prefix.
    fn try_to_bech32(self, hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address, Error>;

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix.
    fn to_bech32(self, hrp: Hrp) -> Bech32Address;

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix without checking
    /// validity.
    fn to_bech32_unchecked(self, hrp: impl ConvertTo<Hrp>) -> Bech32Address;
}
impl<T: Into<Address>> ToBech32Ext for T {
    /// Try to encode this address to a bech32 string with the given Human Readable Part as prefix.
    fn try_to_bech32(self, hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address, Error> {
        Bech32Address::try_new(hrp, self)
    }

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix.
    fn to_bech32(self, hrp: Hrp) -> Bech32Address {
        Bech32Address::new(hrp, self)
    }

    /// Encodes this address to a bech32 string with the given Human Readable Part as prefix without checking
    /// validity.
    fn to_bech32_unchecked(self, hrp: impl ConvertTo<Hrp>) -> Bech32Address {
        Bech32Address::new(hrp.convert_unchecked(), self)
    }
}

impl From<&Self> for Address {
    fn from(value: &Self) -> Self {
        *value
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Address {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct TypedAddress {
            #[serde(rename = "type")]
            kind: u8,
            data: serde_json::Value,
        }

        let value = TypedAddress::deserialize(d)?;
        Ok(match value.kind {
            Ed25519Address::KIND => Ed25519Address::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize ed25519 address: {e}")))?
                .into(),
            AliasAddress::KIND => AliasAddress::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize alias address: {e}")))?
                .into(),
            NftAddress::KIND => NftAddress::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize NFT address: {e}")))?
                .into(),
            _ => {
                return Err(serde::de::Error::custom("invalid address type"));
            }
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum AddressDto<'a> {
            T1(&'a Ed25519Address),
            T2(&'a AliasAddress),
            T3(&'a NftAddress),
        }
        #[derive(Serialize)]
        struct TypedAddress<'a> {
            #[serde(rename = "type")]
            kind: u8,
            data: AddressDto<'a>,
        }
        let data = match self {
            Self::Ed25519(data) => AddressDto::T1(data),
            Self::Alias(data) => AddressDto::T2(data),
            Self::Nft(data) => AddressDto::T3(data),
        };
        TypedAddress {
            kind: self.kind(),
            data,
        }
        .serialize(serializer)
    }
}
