// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use packable::{Packable, PackableExt};

use super::Address;
use crate::types::block::{
    capabilities::{Capabilities, CapabilityFlag},
    output::StorageScore,
    Error,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Getters, Packable)]
#[getset(get = "pub")]
pub struct RestrictedAddress {
    address: Address,
    allowed_capabilities: AddressCapabilities,
}

impl RestrictedAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of a [`RestrictedAddress`].
    pub const KIND: u8 = 40;

    /// Creates a new [`RestrictedAddress`] address from an [`Address`] with default allowed capabilities.
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Result<Self, Error> {
        let address = address.into();
        if matches!(address, Address::Restricted(_)) {
            return Err(Error::InvalidAddressKind(Self::KIND));
        }
        Ok(Self {
            address,
            allowed_capabilities: Default::default(),
        })
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn with_allowed_capabilities(mut self, allowed_capabilities: impl Into<AddressCapabilities>) -> Self {
        self.allowed_capabilities = allowed_capabilities.into();
        self
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn set_allowed_capabilities(&mut self, allowed_capabilities: impl Into<AddressCapabilities>) -> &mut Self {
        self.allowed_capabilities = allowed_capabilities.into();
        self
    }
}

impl StorageScore for RestrictedAddress {}

impl TryFrom<Address> for RestrictedAddress {
    type Error = Error;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl core::fmt::Display for RestrictedAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.pack_to_vec()))
    }
}

/// All possible capabilities that an [`Address`] can have.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum AddressCapabilityFlag {
    /// Can receive Outputs with Native Tokens.
    OutputsWithNativeTokens,
    /// Can receive Outputs with Mana.
    OutputsWithMana,
    /// Can receive Outputs with a Timelock Unlock Condition.
    OutputsWithTimelock,
    /// Can receive Outputs with an Expiration Unlock Condition.
    OutputsWithExpiration,
    /// Can receive Outputs with a Storage Deposit Return Unlock Condition.
    OutputsWithStorageDepositReturn,
    /// Can receive Account Outputs.
    AccountOutputs,
    /// Can receive NFT Outputs.
    NftOutputs,
    /// Can receive Delegation Outputs.
    DelegationOutputs,
}

impl AddressCapabilityFlag {
    const OUTPUTS_WITH_NATIVE_TOKENS: u8 = 0b00000001;
    const OUTPUTS_WITH_MANA: u8 = 0b00000010;
    const OUTPUTS_WITH_TIMELOCK: u8 = 0b00000100;
    const OUTPUTS_WITH_EXPIRATION: u8 = 0b00001000;
    const OUTPUTS_WITH_STORAGE_DEPOSIT_RETURN: u8 = 0b00010000;
    const ACCOUNT_OUTPUTS: u8 = 0b00100000;
    const NFT_OUTPUTS: u8 = 0b01000000;
    const DELEGATION_OUTPUTS: u8 = 0b10000000;
}

impl CapabilityFlag for AddressCapabilityFlag {
    type Iterator = core::array::IntoIter<Self, 8>;

    fn as_byte(&self) -> u8 {
        match self {
            Self::OutputsWithNativeTokens => Self::OUTPUTS_WITH_NATIVE_TOKENS,
            Self::OutputsWithMana => Self::OUTPUTS_WITH_MANA,
            Self::OutputsWithTimelock => Self::OUTPUTS_WITH_TIMELOCK,
            Self::OutputsWithExpiration => Self::OUTPUTS_WITH_EXPIRATION,
            Self::OutputsWithStorageDepositReturn => Self::OUTPUTS_WITH_STORAGE_DEPOSIT_RETURN,
            Self::AccountOutputs => Self::ACCOUNT_OUTPUTS,
            Self::NftOutputs => Self::NFT_OUTPUTS,
            Self::DelegationOutputs => Self::DELEGATION_OUTPUTS,
        }
    }

    fn index(&self) -> usize {
        match self {
            Self::OutputsWithNativeTokens
            | Self::OutputsWithMana
            | Self::OutputsWithTimelock
            | Self::OutputsWithExpiration
            | Self::OutputsWithStorageDepositReturn
            | Self::AccountOutputs
            | Self::NftOutputs
            | Self::DelegationOutputs => 0,
        }
    }

    fn all() -> Self::Iterator {
        [
            Self::OutputsWithNativeTokens,
            Self::OutputsWithMana,
            Self::OutputsWithTimelock,
            Self::OutputsWithExpiration,
            Self::OutputsWithStorageDepositReturn,
            Self::AccountOutputs,
            Self::NftOutputs,
            Self::DelegationOutputs,
        ]
        .into_iter()
    }
}

pub type AddressCapabilities = Capabilities<AddressCapabilityFlag>;

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::boxed::Box;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RestrictedAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        pub address: Address,
        #[serde(with = "prefix_hex_bytes")]
        pub allowed_capabilities: Box<[u8]>,
    }

    impl core::ops::Deref for RestrictedAddressDto {
        type Target = Address;

        fn deref(&self) -> &Self::Target {
            &self.address
        }
    }

    impl From<&RestrictedAddress> for RestrictedAddressDto {
        fn from(value: &RestrictedAddress) -> Self {
            Self {
                kind: RestrictedAddress::KIND,
                address: value.address.clone(),
                allowed_capabilities: value.allowed_capabilities.iter().copied().collect(),
            }
        }
    }

    impl TryFrom<RestrictedAddressDto> for RestrictedAddress {
        type Error = Error;

        fn try_from(value: RestrictedAddressDto) -> Result<Self, Self::Error> {
            Ok(
                Self::new(value.address)?.with_allowed_capabilities(AddressCapabilities::from_bytes(
                    value
                        .allowed_capabilities
                        .try_into()
                        .map_err(Error::InvalidCapabilitiesCount)?,
                )),
            )
        }
    }

    impl_serde_typed_dto!(RestrictedAddress, RestrictedAddressDto, "restricted address");
}
