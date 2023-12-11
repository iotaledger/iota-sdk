// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{AsRef, Deref, Display, From, FromStr};
use packable::Packable;

use crate::types::block::{
    address::Ed25519Address,
    output::{StorageScore, StorageScoreParameters},
};

/// An implicit account creation address that can be used to convert a
/// [`BasicOutput`](crate::types::block::output::BasicOutput) to an
/// [`AccountOutput`](crate::types::block::output::AccountOutput).
#[derive(Copy, Clone, Display, Eq, PartialEq, Ord, PartialOrd, Hash, FromStr, AsRef, Deref, From, Packable)]
#[as_ref(forward)]
pub struct ImplicitAccountCreationAddress(Ed25519Address);

impl ImplicitAccountCreationAddress {
    /// The [`Address`](super::Address) kind of an [`ImplicitAccountCreationAddress`].
    pub const KIND: u8 = 32;
    /// The length of an [`ImplicitAccountCreationAddress`].
    pub const LENGTH: usize = Ed25519Address::LENGTH;

    /// Creates a new [`ImplicitAccountCreationAddress`].
    #[inline(always)]
    pub fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(Ed25519Address::new(address))
    }

    /// Returns the inner [`Ed25519Address`] of the [`ImplicitAccountCreationAddress`].
    pub fn ed25519_address(&self) -> &Ed25519Address {
        &self.0
    }
}

impl StorageScore for ImplicitAccountCreationAddress {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.implicit_account_creation_address_offset()
    }
}

impl core::fmt::Debug for ImplicitAccountCreationAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ImplicitAccountCreationAddress({self})")
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ImplicitAccountCreationAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "prefix_hex_bytes")]
        pub_key_hash: [u8; Ed25519Address::LENGTH],
    }

    impl From<&ImplicitAccountCreationAddress> for ImplicitAccountCreationAddressDto {
        fn from(value: &ImplicitAccountCreationAddress) -> Self {
            Self {
                kind: ImplicitAccountCreationAddress::KIND,
                pub_key_hash: *value.0,
            }
        }
    }

    impl From<ImplicitAccountCreationAddressDto> for ImplicitAccountCreationAddress {
        fn from(value: ImplicitAccountCreationAddressDto) -> Self {
            Self::new(value.pub_key_hash)
        }
    }

    crate::impl_serde_typed_dto!(
        ImplicitAccountCreationAddress,
        ImplicitAccountCreationAddressDto,
        "implicit account creation address"
    );
}
