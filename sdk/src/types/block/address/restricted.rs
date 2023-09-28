// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

use derive_more::{Deref, From};
use packable::{bounded::BoundedU8, error::UnpackErrorExt, prefix::BoxedSlicePrefix, Packable};

use super::Address;
use crate::types::block::Error;

pub type CapabilitiesCount = BoundedU8<0, 1>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, getset::Getters)]
pub struct RestrictedAddress {
    #[getset(get = "pub")]
    address: Address,
    allowed_capabilities: BoxedSlicePrefix<Capabilities, CapabilitiesCount>,
}

impl RestrictedAddress {
    /// Type of a Restricted Address.
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
    pub fn with_allowed_capabilities(
        mut self,
        allowed_capabilities: impl IntoIterator<Item = impl Into<Capabilities>>,
    ) -> Result<Self, Error> {
        self.allowed_capabilities = allowed_capabilities
            .into_iter()
            .map(Into::into)
            .collect::<Box<[_]>>()
            .try_into()
            .map_err(Error::InvalidCapabilitiesCount)?;
        Ok(self)
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn set_allowed_capabilities(
        &mut self,
        allowed_capabilities: impl IntoIterator<Item = impl Into<Capabilities>>,
    ) -> Result<&mut Self, Error> {
        self.allowed_capabilities = allowed_capabilities
            .into_iter()
            .map(Into::into)
            .collect::<Box<[_]>>()
            .try_into()
            .map_err(Error::InvalidCapabilitiesCount)?;
        Ok(self)
    }

    /// Gets the allowed capabilities.
    pub fn allowed_capabilities(&self) -> &[Capabilities] {
        &self.allowed_capabilities
    }
}

impl TryFrom<Address> for RestrictedAddress {
    type Error = Error;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Packable for RestrictedAddress {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.address.pack(packer)?;
        if self.allowed_capabilities.iter().any(|c| c.0 != CapabilityFlag::NONE) {
            self.allowed_capabilities.pack(packer)?;
        } else {
            0_u8.pack(packer)?;
        }
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        _visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let address = Address::unpack::<_, VERIFY>(unpacker, &())?;
        let allowed_capabilities = BoxedSlicePrefix::<_, CapabilitiesCount>::unpack::<_, VERIFY>(unpacker, &())
            .map_packable_err(|e| Error::InvalidCapabilitiesCount(e.into_prefix_err().into()))
            .coerce()?;
        Ok(Self {
            address,
            allowed_capabilities,
        })
    }
}

pub struct CapabilityFlag;

impl CapabilityFlag {
    pub const NATIVE_TOKENS: u8 = 0b00000001;
    pub const MANA: u8 = 0b00000010;
    pub const TIMELOCKED_OUTPUTS: u8 = 0b00000100;
    pub const EXPIRING_OUTPUTS: u8 = 0b00001000;
    pub const STORAGE_DEPOSIT_OUTPUTS: u8 = 0b00010000;
    pub const ACCOUNT_OUTPUTS: u8 = 0b00100000;
    pub const NFT_OUTPUTS: u8 = 0b01000000;
    pub const DELEGATION_OUTPUTS: u8 = 0b10000000;
    pub const NONE: u8 = 0;
    pub const ALL: u8 = u8::MAX;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, Packable)]
#[repr(transparent)]
pub struct Capabilities(u8);

impl Capabilities {
    pub fn with_capabilities(mut self, flags: impl Into<u8>) -> Self {
        self.0 |= flags.into();
        self
    }

    pub fn add_capabilities(&mut self, flags: impl Into<u8>) -> &mut Self {
        self.0 |= flags.into();
        self
    }

    pub fn set_capabilities(&mut self, flags: impl Into<u8>) -> &mut Self {
        self.0 = flags.into();
        self
    }

    pub fn has_capabilities(&self, flags: impl Into<u8>) -> bool {
        self.0 & flags.into() != 0
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::vec::Vec;

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
        pub allowed_capabilities: Vec<u8>,
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
                allowed_capabilities: value.allowed_capabilities.iter().map(|c| **c).collect(),
            }
        }
    }

    impl TryFrom<RestrictedAddressDto> for RestrictedAddress {
        type Error = Error;

        fn try_from(value: RestrictedAddressDto) -> Result<Self, Self::Error> {
            Self::new(value.address)?.with_allowed_capabilities(value.allowed_capabilities)
        }
    }

    impl_serde_typed_dto!(RestrictedAddress, RestrictedAddressDto, "restricted address");
}
