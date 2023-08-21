// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

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
}

impl PartialEq<u64> for SlotIndex {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl core::ops::Add<u64> for SlotIndex {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        Self(self.0 + other)
    }
}

impl core::ops::AddAssign<u64> for SlotIndex {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}

impl core::ops::Sub<u64> for SlotIndex {
    type Output = Self;

    fn sub(self, other: u64) -> Self {
        Self(self.0 - other)
    }
}

impl core::ops::SubAssign<u64> for SlotIndex {
    fn sub_assign(&mut self, other: u64) {
        self.0 -= other;
    }
}

impl From<SlotIndex> for u64 {
    fn from(slot_index: SlotIndex) -> Self {
        *slot_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(SlotIndex);
