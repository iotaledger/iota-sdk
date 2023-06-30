// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::{AccountAddress, Address},
    Error,
};

/// Defines the permanent [`AccountAddress`] that owns this output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImmutableAccountAddressUnlockCondition(#[packable(verify_with = verify_account_address)] Address);

impl ImmutableAccountAddressUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an
    /// [`ImmutableAccountAddressUnlockCondition`].
    pub const KIND: u8 = 6;

    /// Creates a new [`ImmutableAccountAddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: impl Into<AccountAddress>) -> Self {
        Self(Address::Account(address.into()))
    }

    /// Returns the address of an [`ImmutableAccountAddressUnlockCondition`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        // An ImmutableAccountAddressUnlockCondition must have an AccountAddress.
        // It has already been validated at construction that the address is an `AccountAddress`.
        debug_assert!(&self.0.is_account());
        &self.0
    }

    /// Returns the account address of an [`ImmutableAccountAddressUnlockCondition`].
    pub fn account_address(&self) -> &AccountAddress {
        // It has already been validated at construction that the address is an `AccountAddress`.
        self.0.as_account()
    }
}

fn verify_account_address<const VERIFY: bool>(address: &Address, _: &()) -> Result<(), Error> {
    if VERIFY && !address.is_account() {
        Err(Error::InvalidAddressKind(address.kind()))
    } else {
        Ok(())
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::{address::dto::AddressDto, Error};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct ImmutableAccountAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    impl From<&ImmutableAccountAddressUnlockCondition> for ImmutableAccountAddressUnlockConditionDto {
        fn from(value: &ImmutableAccountAddressUnlockCondition) -> Self {
            Self {
                kind: ImmutableAccountAddressUnlockCondition::KIND,
                address: value.address().into(),
            }
        }
    }

    impl TryFrom<ImmutableAccountAddressUnlockConditionDto> for ImmutableAccountAddressUnlockCondition {
        type Error = Error;

        fn try_from(value: ImmutableAccountAddressUnlockConditionDto) -> Result<Self, Error> {
            let address: Address = value
                .address
                .try_into()
                .map_err(|_e| Error::InvalidField("immutableAccountAddressUnlockCondition"))?;

            // An ImmutableAccountAddressUnlockCondition must have an AccountAddress.
            if let Address::Account(account_address) = address {
                Ok(Self::new(account_address))
            } else {
                Err(Error::InvalidField("immutableAccountAddressUnlockCondition"))
            }
        }
    }
}
