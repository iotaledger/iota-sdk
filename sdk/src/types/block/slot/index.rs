// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

use super::EpochIndex;

/// Timeline is divided into slots, and each slot has a corresponding slot index.
/// To calculate the slot index of a timestamp, `genesisTimestamp` and the duration of a slot are needed.
/// The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, Display, FromStr, packable::Packable,
)]
#[repr(transparent)]
pub struct SlotIndex(u64);

impl SlotIndex {
    /// Creates a new [`SlotIndex`].
    pub fn new(index: u64) -> Self {
        Self::from(index)
    }

    pub fn to_epoch_index(self, slots_per_epoch_exponent: u32) -> EpochIndex {
        EpochIndex::new((self.0 >> slots_per_epoch_exponent) + 1)
    }

    pub fn as_timestamp(self, genesis_unix_timestamp: u32, slot_duration_in_seconds: u8) -> u32 {
        (((self.0 - 1) * slot_duration_in_seconds as u64) + genesis_unix_timestamp as u64) as _
    }
}

impl From<SlotIndex> for u64 {
    fn from(slot_index: SlotIndex) -> Self {
        *slot_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(SlotIndex);
