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

    impl_serde_typed_dto!(AccountAddress, AccountAddressDto, "account address");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::utils::json::{FromJson, ToJson};

    impl ToJson for AccountAddress {
        fn to_json(&self) -> ::json::JsonValue {
            crate::json! ({
                "type": AccountAddress::KIND,
                "accountId": self.0
            })
        }
    }

    impl FromJson for AccountAddress {
        type Error = Error;

        fn from_non_null_json(mut value: ::json::JsonValue) -> Result<Self, crate::utils::json::JsonError<Self::Error>>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(::json::Error::WrongType(alloc::format!(
                    "invalid account address type: expected {}, found {}",
                    Self::KIND,
                    value["type"]
                ))
                .into());
            }
            Ok(Self::new(AccountId::from_json(value["accountId"].take())?))
        }
    }
}
