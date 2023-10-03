// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::Deref;
use getset::Getters;
use packable::{error::UnpackErrorExt, prefix::BoxedSlicePrefix, Packable, PackableExt};

use super::Address;
use crate::types::block::Error;

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
    pub fn new(address: impl Into<Address> + Send) -> Result<Self, Error> {
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum AddressCapabilityFlag {
    NativeTokens,
    Mana,
    TimelockedOutputs,
    ExpiringOutputs,
    StorageDepositOutputs,
    AccountOutputs,
    NftOutputs,
    DelegationOutputs,
}

impl AddressCapabilityFlag {
    const NATIVE_TOKENS: u8 = 0b00000001;
    const MANA: u8 = 0b00000010;
    const TIMELOCKED_OUTPUTS: u8 = 0b00000100;
    const EXPIRING_OUTPUTS: u8 = 0b00001000;
    const STORAGE_DEPOSIT_OUTPUTS: u8 = 0b00010000;
    const ACCOUNT_OUTPUTS: u8 = 0b00100000;
    const NFT_OUTPUTS: u8 = 0b01000000;
    const DELEGATION_OUTPUTS: u8 = 0b10000000;

    pub fn as_byte(&self) -> u8 {
        match self {
            AddressCapabilityFlag::NativeTokens => AddressCapabilityFlag::NATIVE_TOKENS,
            AddressCapabilityFlag::Mana => AddressCapabilityFlag::MANA,
            AddressCapabilityFlag::TimelockedOutputs => AddressCapabilityFlag::TIMELOCKED_OUTPUTS,
            AddressCapabilityFlag::ExpiringOutputs => AddressCapabilityFlag::EXPIRING_OUTPUTS,
            AddressCapabilityFlag::StorageDepositOutputs => AddressCapabilityFlag::STORAGE_DEPOSIT_OUTPUTS,
            AddressCapabilityFlag::AccountOutputs => AddressCapabilityFlag::ACCOUNT_OUTPUTS,
            AddressCapabilityFlag::NftOutputs => AddressCapabilityFlag::NFT_OUTPUTS,
            AddressCapabilityFlag::DelegationOutputs => AddressCapabilityFlag::DELEGATION_OUTPUTS,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            AddressCapabilityFlag::NativeTokens
            | AddressCapabilityFlag::Mana
            | AddressCapabilityFlag::TimelockedOutputs
            | AddressCapabilityFlag::ExpiringOutputs
            | AddressCapabilityFlag::StorageDepositOutputs
            | AddressCapabilityFlag::AccountOutputs
            | AddressCapabilityFlag::NftOutputs
            | AddressCapabilityFlag::DelegationOutputs => 0,
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::NativeTokens,
            Self::Mana,
            Self::TimelockedOutputs,
            Self::ExpiringOutputs,
            Self::StorageDepositOutputs,
            Self::AccountOutputs,
            Self::NftOutputs,
            Self::DelegationOutputs,
        ]
        .into_iter()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Deref)]
#[repr(transparent)]
pub struct AddressCapabilities(BoxedSlicePrefix<u8, u8>);

impl AddressCapabilities {
    pub fn all() -> Self {
        let mut res = Self::default();
        res.set_all();
        res
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn is_all(&self) -> bool {
        AddressCapabilityFlag::all().all(|flag| self.has_capability(flag))
    }

    pub fn is_none(&self) -> bool {
        self.0.iter().all(|b| 0.eq(b))
    }

    pub fn set_all(&mut self) -> &mut Self {
        for flag in AddressCapabilityFlag::all() {
            self.add_capability(flag);
        }
        self
    }

    pub fn set_none(&mut self) -> &mut Self {
        *self = Default::default();
        self
    }

    pub fn add_capability(&mut self, flag: AddressCapabilityFlag) -> &mut Self {
        if self.0.len() <= flag.index() {
            let mut v = Box::<[_]>::from(self.0.clone()).into_vec();
            v.resize(flag.index() + 1, 0);
            // Unwrap: safe because the indexes are within u8 bounds
            self.0 = v.into_boxed_slice().try_into().unwrap();
        }
        self.0[flag.index()] |= flag.as_byte();
        self
    }

    pub fn add_capabilities(&mut self, flags: impl IntoIterator<Item = AddressCapabilityFlag>) -> &mut Self {
        for flag in flags {
            self.add_capability(flag);
        }
        self
    }

    pub fn with_capabilities(mut self, flags: impl IntoIterator<Item = AddressCapabilityFlag>) -> Self {
        self.add_capabilities(flags);
        self
    }

    pub fn set_capabilities(&mut self, flags: impl IntoIterator<Item = AddressCapabilityFlag>) -> &mut Self {
        *self = Self::default().with_capabilities(flags);
        self
    }

    pub fn has_capability(&self, flag: AddressCapabilityFlag) -> bool {
        self.0
            .get(flag.index())
            .map(|byte| byte & flag.as_byte() == flag.as_byte())
            .unwrap_or_default()
    }

    pub fn has_capabilities(&self, flags: impl IntoIterator<Item = AddressCapabilityFlag>) -> bool {
        flags.into_iter().all(|flag| self.has_capability(flag))
    }

    pub fn split<'a>(&'a self) -> impl Iterator<Item = AddressCapabilityFlag> + 'a {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, byte)| {
                AddressCapabilityFlag::all()
                    .filter_map(move |f| (idx == f.index() && byte & f.as_byte() == f.as_byte()).then_some(f))
            })
            .flatten()
    }
}

impl<I: IntoIterator<Item = AddressCapabilityFlag>> From<I> for AddressCapabilities {
    fn from(value: I) -> Self {
        Self::default().with_capabilities(value)
    }
}

impl Packable for AddressCapabilities {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        if !self.is_none() {
            self.0.pack(packer)?;
        } else {
            0_u8.pack(packer)?;
        }
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        use packable::prefix::UnpackPrefixError;
        Ok(Self(
            BoxedSlicePrefix::unpack::<_, VERIFY>(unpacker, visitor)
                // TODO: not sure if this is the best way to do this
                .map_packable_err(|e| match e {
                    UnpackPrefixError::Item(i) | UnpackPrefixError::Prefix(i) => i,
                })
                .coerce()?,
        ))
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RestrictedAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        pub address: Address,
        // TODO: is this format right?
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
            Ok(Self::new(value.address)?.with_allowed_capabilities(AddressCapabilities(
                value
                    .allowed_capabilities
                    .try_into()
                    .map_err(Error::InvalidAddressCapabilitiesCount)?,
            )))
        }
    }

    impl_serde_typed_dto!(RestrictedAddress, RestrictedAddressDto, "restricted address");
}
