// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An extension to the address format to make them configurable.
//! This enables an address to opt-in or -out of certain functionality, like disabling the receipt of Native Tokens, NFT
//! Outputs or Timelock Unlock Conditions.
//! [TIP-50: Configurable Addresses](https://github.com/iotaledger/tips/blob/tip50/tips/TIP-0050/tip-0050.md).

use getset::Getters;
use packable::{Packable, PackableExt};

use crate::types::block::{
    address::{Address, AddressError},
    capabilities::{Capabilities, CapabilityFlag},
    output::{StorageScore, StorageScoreParameters},
};

/// An [`Address`] that contains another address and allows for configuring its capabilities.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Getters, Packable)]
#[getset(get = "pub")]
pub struct RestrictedAddress {
    #[packable(verify_with = verify_address)]
    address: Address,
    allowed_capabilities: AddressCapabilities,
}

impl RestrictedAddress {
    /// The [`Address`] kind of a [`RestrictedAddress`].
    pub const KIND: u8 = 48;

    /// Creates a new [`RestrictedAddress`] address from an [`Address`] with default allowed capabilities.
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Result<Self, AddressError> {
        let address = address.into();

        verify_address(&address)?;

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

    /// Returns whether a given [`AddressCapabilityFlag`] is enabled.
    pub fn has_capability(&self, flag: AddressCapabilityFlag) -> bool {
        self.allowed_capabilities.has_capability(flag)
    }
}

impl StorageScore for RestrictedAddress {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.address.storage_score(params)
    }
}

impl TryFrom<Address> for RestrictedAddress {
    type Error = AddressError;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl core::fmt::Display for RestrictedAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.pack_to_vec()))
    }
}

fn verify_address(address: &Address) -> Result<(), AddressError> {
    if !matches!(
        address,
        Address::Ed25519(_) | Address::Account(_) | Address::Nft(_) | Address::Anchor(_) | Address::Multi(_)
    ) {
        Err(AddressError::Kind(address.kind()))
    } else {
        Ok(())
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
    /// Can receive Anchor Outputs.
    AnchorOutputs,
    /// Can receive NFT Outputs.
    NftOutputs,
    /// Can receive Delegation Outputs.
    DelegationOutputs,
}

impl AddressCapabilityFlag {
    // Byte 0
    const OUTPUTS_WITH_NATIVE_TOKENS: u8 = 0b00000001;
    const OUTPUTS_WITH_MANA: u8 = 0b00000010;
    const OUTPUTS_WITH_TIMELOCK: u8 = 0b00000100;
    const OUTPUTS_WITH_EXPIRATION: u8 = 0b00001000;
    const OUTPUTS_WITH_STORAGE_DEPOSIT_RETURN: u8 = 0b00010000;
    const ACCOUNT_OUTPUTS: u8 = 0b00100000;
    const ANCHOR_OUTPUTS: u8 = 0b01000000;
    const NFT_OUTPUTS: u8 = 0b10000000;
    // Byte 1
    const DELEGATION_OUTPUTS: u8 = 0b00000001;
}

impl CapabilityFlag for AddressCapabilityFlag {
    type Iterator = core::array::IntoIter<Self, 9>;

    fn as_byte(&self) -> u8 {
        match self {
            Self::OutputsWithNativeTokens => Self::OUTPUTS_WITH_NATIVE_TOKENS,
            Self::OutputsWithMana => Self::OUTPUTS_WITH_MANA,
            Self::OutputsWithTimelock => Self::OUTPUTS_WITH_TIMELOCK,
            Self::OutputsWithExpiration => Self::OUTPUTS_WITH_EXPIRATION,
            Self::OutputsWithStorageDepositReturn => Self::OUTPUTS_WITH_STORAGE_DEPOSIT_RETURN,
            Self::AccountOutputs => Self::ACCOUNT_OUTPUTS,
            Self::AnchorOutputs => Self::ANCHOR_OUTPUTS,
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
            | Self::AnchorOutputs
            | Self::NftOutputs => 0,
            Self::DelegationOutputs => 1,
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
            Self::AnchorOutputs,
            Self::NftOutputs,
            Self::DelegationOutputs,
        ]
        .into_iter()
    }
}

pub type AddressCapabilities = Capabilities<AddressCapabilityFlag>;

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RestrictedAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        pub address: Address,
        #[serde(default, skip_serializing_if = "AddressCapabilities::is_none")]
        pub allowed_capabilities: AddressCapabilities,
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
                allowed_capabilities: value.allowed_capabilities.clone(),
            }
        }
    }

    impl TryFrom<RestrictedAddressDto> for RestrictedAddress {
        type Error = AddressError;

        fn try_from(value: RestrictedAddressDto) -> Result<Self, Self::Error> {
            Ok(Self::new(value.address)?.with_allowed_capabilities(value.allowed_capabilities))
        }
    }

    crate::impl_serde_typed_dto!(RestrictedAddress, RestrictedAddressDto, "restricted address");
}
