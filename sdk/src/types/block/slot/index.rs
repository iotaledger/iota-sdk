// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From};

/// Timeline is divided into slots, and each slot has a corresponding slot index.
/// To calculate the slot index of a timestamp, `genesisTimestamp` and the duration of a slot are needed.
/// The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, Display, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    type Output = SlotIndex;

    fn add(self, other: u64) -> Self {
        SlotIndex(self.0 + other)
    }
}

impl core::ops::AddAssign<u64> for SlotIndex {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}

impl core::ops::Sub<u64> for SlotIndex {
    type Output = SlotIndex;

    fn sub(self, other: u64) -> Self {
        SlotIndex(self.0 - other)
    }
}

impl core::ops::SubAssign<u64> for SlotIndex {
    fn sub_assign(&mut self, other: u64) {
        self.0 -= other;
    }
}
