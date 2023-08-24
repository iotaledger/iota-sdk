// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

use super::EpochIndex;
use crate::types::block::Error;

/// The tangle timeline is divided into epochs, and each epoch has a corresponding [`EpochIndex`]. Epochs are further
/// subdivided into slots, each with a slot index.
/// To calculate the slot index of a timestamp, `genesisUnixTimestamp` and the `slotDurationInSeconds` are needed.
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

    /// Gets the [`EpochIndex`] of this slot.
    pub fn to_epoch_index(
        self,
        slots_per_epoch_exponent_iter: impl Iterator<Item = (EpochIndex, u32)>,
    ) -> Result<EpochIndex, Error> {
        EpochIndex::from_slot_index(self, slots_per_epoch_exponent_iter)
    }

    /// Gets the slot index of a unix timestamp.
    pub fn from_timestamp(timestamp: u64, genesis_unix_timestamp: u32, slot_duration_in_seconds: u8) -> SlotIndex {
        (1 + (timestamp - genesis_unix_timestamp as u64) / slot_duration_in_seconds as u64).into()
    }

    /// Converts the slot index into the corresponding unix timestamp.
    pub fn to_timestamp(self, genesis_unix_timestamp: u32, slot_duration_in_seconds: u8) -> u64 {
        ((self.0 - 1) * slot_duration_in_seconds as u64) + genesis_unix_timestamp as u64
    }
}

impl From<SlotIndex> for u64 {
    fn from(slot_index: SlotIndex) -> Self {
        *slot_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(SlotIndex);
