// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, From};
use packable::{error::UnpackErrorExt, Packable};

use crate::types::block::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, getset::Getters)]
#[getset(get = "pub")]
pub struct RestrictedAddress<A> {
    address: A,
    allowed_capabilities: Option<Capabilities>,
}

impl<A> RestrictedAddress<A> {
    /// Creates a new [`RestrictedAddress`] address from the underlying type.
    #[inline(always)]
    pub fn new(address: A) -> Self {
        Self {
            address,
            allowed_capabilities: Default::default(),
        }
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn with_allowed_capabilities(mut self, allowed_capabilities: impl Into<Capabilities>) -> Self {
        self.allowed_capabilities.replace(allowed_capabilities.into());
        self
    }

    /// Sets the allowed capabilities flags.
    #[inline(always)]
    pub fn set_allowed_capabilities(&mut self, allowed_capabilities: impl Into<Capabilities>) -> &mut Self {
        self.allowed_capabilities.replace(allowed_capabilities.into());
        self
    }
}

impl<A> From<A> for RestrictedAddress<A> {
    fn from(value: A) -> Self {
        Self::new(value)
    }
}

impl<A: 'static + Packable> Packable for RestrictedAddress<A>
where
    Error: From<A::UnpackError>,
{
    type UnpackError = Error;
    type UnpackVisitor = A::UnpackVisitor;

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.address.pack(packer)?;
        if self
            .allowed_capabilities
            .map(|c| c.0 != CapabilityFlag::NONE)
            .unwrap_or_default()
        {
            self.allowed_capabilities.pack(packer)?;
        } else {
            0_u8.pack(packer)?;
        }
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let address = A::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let allowed_capabilities_set = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? != 0;
        let allowed_capabilities = allowed_capabilities_set
            .then(|| Capabilities::unpack::<_, VERIFY>(unpacker, &()).coerce())
            .transpose()?;
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

    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RestrictedAddressDto<A> {
        #[serde(flatten)]
        pub address: A,
        // TODO: is this format right?
        #[serde(with = "prefix_hex_bytes")]
        pub allowed_capabilities: Vec<u8>,
    }

    impl<A> core::ops::Deref for RestrictedAddressDto<A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.address
        }
    }
}
