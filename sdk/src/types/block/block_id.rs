// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{slot::SlotIndex, ConvertTo};
use crate::types::block::Error;

impl_id!(pub BlockHash, 32, "The hash of a [`Block`].");

impl BlockHash {
    #[cfg(target_endian = "little")]
    pub fn with_slot_index(self, slot_index: SlotIndex) -> BlockId {
        BlockId { hash: self, slot_index }
    }

    #[cfg(target_endian = "big")]
    pub fn with_slot_index(self, slot_index: SlotIndex) -> BlockId {
        BlockId {
            hash: self,
            slot_index: slot_index.to_le().into(),
        }
    }
}

/// A block identifier.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug, packable::Packable)]
#[packable(unpack_error = Error)]
#[repr(C)]
pub struct BlockId {
    pub(crate) hash: BlockHash,
    // IMPORTANT: On big-endian systems this value is misrepresented because it is transmuted directly
    // from bytes, so the getter below handles that conversion. Do not access it directly.
    slot_index: SlotIndex,
}

impl BlockId {
    /// The length of a [`BlockId`]
    pub const LENGTH: usize = 40;

    pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }

    /// Returns the [`BlockId`]'s hash part.
    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    /// Returns the [`BlockId`]'s slot index part.
    #[cfg(target_endian = "little")]
    pub fn slot_index(&self) -> SlotIndex {
        self.slot_index
    }

    /// Returns the [`BlockId`]'s slot index part.
    #[cfg(target_endian = "big")]
    pub fn slot_index(&self) -> SlotIndex {
        self.slot_index.to_le().into()
    }
}

impl AsRef<[u8]> for BlockId {
    fn as_ref(&self) -> &[u8] {
        unsafe { core::mem::transmute::<_, &[u8; Self::LENGTH]>(self) }
    }
}

impl core::str::FromStr for BlockId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(prefix_hex::decode(s).map_err(Error::Hex)?))
    }
}

impl core::fmt::Display for BlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        prefix_hex::encode(self.as_ref()).fmt(f)
    }
}

impl TryFrom<&alloc::string::String> for BlockId {
    type Error = Error;

    fn try_from(s: &alloc::string::String) -> Result<Self, Self::Error> {
        core::str::FromStr::from_str(s.as_str())
    }
}

impl TryFrom<&str> for BlockId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        core::str::FromStr::from_str(s)
    }
}

impl ConvertTo<BlockId> for &alloc::string::String {
    fn convert(self) -> Result<BlockId, Error> {
        self.try_into()
    }
}

impl ConvertTo<BlockId> for &str {
    fn convert(self) -> Result<BlockId, Error> {
        self.try_into()
    }
}

impl core::ops::Deref for BlockId {
    type Target = [u8; Self::LENGTH];

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute::<_, &[u8; Self::LENGTH]>(self) }
    }
}
#[cfg(feature = "serde_types")]
string_serde_impl!(BlockId);

#[cfg(feature = "json")]
string_json_impl!(BlockId);
