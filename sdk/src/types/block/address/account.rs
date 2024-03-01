// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, Display, From};

use crate::types::block::{
    address::AddressError,
    output::{AccountId, OutputId, StorageScore},
};

/// An [`Address`](super::Address) derived from an account ID which can be unlocked by unlocking the corresponding
/// account.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, Display, packable::Packable)]
#[as_ref(forward)]
pub struct AccountAddress(
    /// BLAKE2b-256 hash of the Output ID that created the account.
    AccountId,
);

impl AccountAddress {
    /// The [`Address`](super::Address) kind of an [`AccountAddress`].
    pub const KIND: u8 = 8;
    /// The length of an [`AccountAddress`].
    pub const LENGTH: usize = AccountId::LENGTH;

    /// Creates a new [`AccountAddress`].
    #[inline(always)]
    pub fn new(id: AccountId) -> Self {
        Self::from(id)
    }

    /// Returns the [`AccountId`] of an [`AccountAddress`].
    #[inline(always)]
    pub fn account_id(&self) -> &AccountId {
        &self.0
    }

    /// Consumes an [`AccountAddress`] and returns its [`AccountId`].
    #[inline(always)]
    pub fn into_account_id(self) -> AccountId {
        self.0
    }
}

impl StorageScore for AccountAddress {}

impl FromStr for AccountAddress {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(AccountId::from_str(s)?))
    }
}

impl From<&OutputId> for AccountAddress {
    fn from(output_id: &OutputId) -> Self {
        Self(AccountId::from(output_id))
    }
}

impl core::fmt::Debug for AccountAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccountAddress({self})")
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// Describes an account address.
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AccountAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        account_id: AccountId,
    }

    impl From<&AccountAddress> for AccountAddressDto {
        fn from(value: &AccountAddress) -> Self {
            Self {
                kind: AccountAddress::KIND,
                account_id: value.0,
            }
        }
    }

    impl From<AccountAddressDto> for AccountAddress {
        fn from(value: AccountAddressDto) -> Self {
            Self(value.account_id)
        }
    }

    crate::impl_serde_typed_dto!(AccountAddress, AccountAddressDto, "account address");
}
