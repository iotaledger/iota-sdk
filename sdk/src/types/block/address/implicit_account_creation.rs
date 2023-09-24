// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{AsRef, Deref, Display, From, FromStr};
use packable::Packable;

use crate::types::block::address::Ed25519Address;

/// An implicit account creation address that can be used to transition an account.
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Ord, PartialOrd, Hash, FromStr, AsRef, Deref, From, Packable)]
#[as_ref(forward)]
pub struct ImplicitAccountCreationAddress(Ed25519Address);

impl ImplicitAccountCreationAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of an [`ImplicitAccountCreationAddress`].
    pub const KIND: u8 = 24;
    /// The length of an [`ImplicitAccountCreationAddress`].
    pub const LENGTH: usize = Ed25519Address::LENGTH;

    /// Creates a new [`ImplicitAccountCreationAddress`].
    #[inline(always)]
    pub fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(Ed25519Address::new(address))
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

    impl_serde_typed_dto!(
        ImplicitAccountCreationAddress,
        ImplicitAccountCreationAddressDto,
        "implicit account creation address"
    );
}
