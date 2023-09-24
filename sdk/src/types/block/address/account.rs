// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, From};

use super::RestrictedAddress;
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

impl RestrictedAddress<AccountAddress> {
    /// The [`Address`](crate::types::block::address::Address) kind of a
    /// [`RestrictedAccountAddress`](Restricted<AccountAddress>).
    pub const KIND: u8 = 9;
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
    use crate::types::block::address::restricted::dto::RestrictedAddressDto;

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

    impl From<&RestrictedAddress<AccountAddress>> for RestrictedAddressDto<AccountAddressDto> {
        fn from(value: &RestrictedAddress<AccountAddress>) -> Self {
            Self {
                address: AccountAddressDto {
                    kind: RestrictedAddress::<AccountAddress>::KIND,
                    account_id: **value.address(),
                },
                allowed_capabilities: value.allowed_capabilities().into_iter().map(|c| **c).collect(),
            }
        }
    }

    impl From<RestrictedAddressDto<AccountAddressDto>> for RestrictedAddress<AccountAddress> {
        fn from(value: RestrictedAddressDto<AccountAddressDto>) -> Self {
            let mut res = Self::new(AccountAddress::from(value.address));
            if let Some(allowed_capabilities) = value.allowed_capabilities.first() {
                res = res.with_allowed_capabilities(*allowed_capabilities);
            }
            res
        }
    }

    impl_serde_typed_dto!(
        RestrictedAddress<AccountAddress>,
        RestrictedAddressDto<AccountAddressDto>,
        "restricted account address"
    );
}
