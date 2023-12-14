// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod anchor;
mod bech32;
mod ed25519;
mod implicit_account_creation;
mod multi;
mod nft;
mod restricted;

use alloc::boxed::Box;

use derive_more::{Display, From};
use packable::Packable;

pub(crate) use self::multi::WeightedAddressCount;
pub use self::{
    account::AccountAddress,
    anchor::AnchorAddress,
    bech32::{Bech32Address, Hrp},
    ed25519::Ed25519Address,
    implicit_account_creation::ImplicitAccountCreationAddress,
    multi::{MultiAddress, WeightedAddress},
    nft::NftAddress,
    restricted::{AddressCapabilities, AddressCapabilityFlag, RestrictedAddress},
};
use crate::{
    types::block::{
        output::{StorageScore, StorageScoreParameters},
        Error,
    },
    utils::ConvertTo,
};

/// A generic address supporting different address kinds.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, Display, Packable)]
#[packable(tag_type = u8, with_error = Error::InvalidAddressKind)]
#[packable(unpack_error = Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Address {
    /// An Ed25519 address.
    #[packable(tag = Ed25519Address::KIND)]
    Ed25519(Ed25519Address),
    /// An account address.
    #[packable(tag = AccountAddress::KIND)]
    Account(AccountAddress),
    /// An NFT address.
    #[packable(tag = NftAddress::KIND)]
    Nft(NftAddress),
    /// An anchor address.
    #[packable(tag = AnchorAddress::KIND)]
    Anchor(AnchorAddress),
    /// An implicit account creation address.
    #[packable(tag = ImplicitAccountCreationAddress::KIND)]
    ImplicitAccountCreation(ImplicitAccountCreationAddress),
    /// A multi address.
    #[packable(tag = MultiAddress::KIND)]
    Multi(MultiAddress),
    /// An address with restricted capabilities.
    #[packable(tag = RestrictedAddress::KIND)]
    #[from(ignore)]
    Restricted(Box<RestrictedAddress>),
}

impl From<RestrictedAddress> for Address {
    fn from(value: RestrictedAddress) -> Self {
        Self::Restricted(value.into())
    }
}

impl core::fmt::Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(address) => address.fmt(f),
            Self::Account(address) => address.fmt(f),
            Self::Nft(address) => address.fmt(f),
            Self::Anchor(address) => address.fmt(f),
            Self::ImplicitAccountCreation(address) => address.fmt(f),
            Self::Multi(address) => address.fmt(f),
            Self::Restricted(address) => address.fmt(f),
        }
    }
}

impl Address {
    /// Returns the address kind of an [`Address`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519Address::KIND,
            Self::Account(_) => AccountAddress::KIND,
            Self::Nft(_) => NftAddress::KIND,
            Self::Anchor(_) => AnchorAddress::KIND,
            Self::ImplicitAccountCreation(_) => ImplicitAccountCreationAddress::KIND,
            Self::Multi(_) => MultiAddress::KIND,
            Self::Restricted(_) => RestrictedAddress::KIND,
        }
    }

    /// Returns the address kind of an [`Address`] as a string.
    pub fn kind_str(&self) -> &str {
        match self {
            Self::Ed25519(_) => "Ed25519",
            Self::Account(_) => "Account",
            Self::Nft(_) => "Nft",
            Self::Anchor(_) => "Anchor",
            Self::ImplicitAccountCreation(_) => "ImplicitAccountCreation",
            Self::Multi(_) => "Multi",
            Self::Restricted(_) => "Restricted",
        }
    }

    crate::def_is_as_opt!(Address: Ed25519, Account, Nft, Anchor, ImplicitAccountCreation, Multi, Restricted);

    /// Checks whether the address is backed by an [`Ed25519Address`].
    pub fn is_ed25519_backed(&self) -> bool {
        match self {
            Self::Ed25519(_) | Self::ImplicitAccountCreation(_) => true,
            Self::Restricted(restricted) => restricted.address().is_ed25519(),
            _ => false,
        }
    }

    /// Returns the backing [`Ed25519Address`], if any.
    pub fn backing_ed25519(&self) -> Option<&Ed25519Address> {
        match self {
            Self::Ed25519(ed25519) => Some(ed25519),
            Self::ImplicitAccountCreation(implicit) => Some(implicit.ed25519_address()),
            Self::Restricted(restricted) => restricted.address().as_ed25519_opt(),
            _ => None,
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
}

impl StorageScore for Address {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Self::Ed25519(address) => address.storage_score(params),
            Self::Account(address) => address.storage_score(params),
            Self::Nft(address) => address.storage_score(params),
            Self::Anchor(address) => address.storage_score(params),
            Self::ImplicitAccountCreation(address) => address.storage_score(params),
            Self::Multi(address) => address.storage_score(params),
            Self::Restricted(address) => address.storage_score(params),
        }
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
    fn try_to_bech32(self, hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address, Error> {
        Bech32Address::try_new(hrp, self)
    }

    fn to_bech32(self, hrp: Hrp) -> Bech32Address {
        Bech32Address::new(hrp, self)
    }

    fn to_bech32_unchecked(self, hrp: impl ConvertTo<Hrp>) -> Bech32Address {
        Bech32Address::new(hrp.convert_unchecked(), self)
    }
}
