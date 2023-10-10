// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{output::OutputId, slot::SlotIndex, ConvertTo, Error};

impl_id!(pub TransactionHash, 32, "The hash of a [`TransactionPayload`].");

impl TransactionHash {
    pub fn with_slot_index(self, slot_index: impl Into<SlotIndex>) -> TransactionId {
        TransactionId {
            hash: self,
            slot_index: slot_index.into().to_le_bytes(),
        }
    }
}

/// A transaction identifier.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, packable::Packable)]
#[packable(unpack_error = Error)]
#[repr(C)]
pub struct TransactionId {
    pub(crate) hash: TransactionHash,
    slot_index: [u8; core::mem::size_of::<SlotIndex>()],
}

impl TransactionId {
    /// The length of a [`TransactionId`]
    pub const LENGTH: usize = TransactionHash::LENGTH + core::mem::size_of::<SlotIndex>();

    pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }

    /// Returns the [`TransactionId`]'s hash part.
    pub fn hash(&self) -> &TransactionHash {
        &self.hash
    }

    /// Returns the [`TransactionId`]'s slot index part.
    pub fn slot_index(&self) -> SlotIndex {
        unsafe {
            #[cfg(target_endian = "little")]
            {
                core::mem::transmute(self.slot_index)
            }

            #[cfg(target_endian = "big")]
            {
                core::mem::transmute(self.slot_index.to_le())
            }
        }
    }

    /// Creates an [`OutputId`] from this [`TransactionId`] and an output index.
    pub fn with_output_index(self, index: u16) -> Result<OutputId, Error> {
        OutputId::new(self, index)
    }
}

impl AsRef<[u8]> for TransactionId {
    fn as_ref(&self) -> &[u8] {
        unsafe { core::mem::transmute::<_, &[u8; Self::LENGTH]>(self) }
    }
}

impl core::str::FromStr for TransactionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(prefix_hex::decode(s).map_err(Error::Hex)?))
    }
}

impl core::fmt::Debug for TransactionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TransactionId")
            .field("hash", &self.hash)
            .field("slot_index", &self.slot_index())
            .finish()
    }
}

impl core::fmt::Display for TransactionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        prefix_hex::encode(self.as_ref()).fmt(f)
    }
}

impl TryFrom<&alloc::string::String> for TransactionId {
    type Error = Error;

    fn try_from(s: &alloc::string::String) -> Result<Self, Self::Error> {
        core::str::FromStr::from_str(s.as_str())
    }
}

impl TryFrom<&str> for TransactionId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        core::str::FromStr::from_str(s)
    }
}

impl ConvertTo<TransactionId> for &alloc::string::String {
    fn convert(self) -> Result<TransactionId, Error> {
        self.try_into()
    }
}

impl ConvertTo<TransactionId> for &str {
    fn convert(self) -> Result<TransactionId, Error> {
        self.try_into()
    }
}

impl core::ops::Deref for TransactionId {
    type Target = [u8; Self::LENGTH];

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute::<_, &[u8; Self::LENGTH]>(self) }
    }
}
#[cfg(feature = "serde")]
string_serde_impl!(TransactionId);
