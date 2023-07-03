// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, From};

use crate::types::block::{output::AccountId, Error};

/// An account address.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, packable::Packable)]
#[as_ref(forward)]
pub struct AccountAddress(AccountId);

impl AccountAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of an [`AccountAddress`].
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

#[cfg(feature = "serde")]
string_serde_impl!(AccountAddress);

impl FromStr for AccountAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(AccountId::from_str(s)?))
    }
}

impl core::fmt::Display for AccountAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for AccountAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccountAddress({self})")
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Describes an account address.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountAddressDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub account_id: String,
    }

    impl From<&AccountAddress> for AccountAddressDto {
        fn from(value: &AccountAddress) -> Self {
            Self {
                kind: AccountAddress::KIND,
                account_id: value.to_string(),
            }
        }
    }

    impl TryFrom<AccountAddressDto> for AccountAddress {
        type Error = Error;

        fn try_from(value: AccountAddressDto) -> Result<Self, Self::Error> {
            value
                .account_id
                .parse::<Self>()
                .map_err(|_| Error::InvalidField("accountId"))
        }
    }
}
