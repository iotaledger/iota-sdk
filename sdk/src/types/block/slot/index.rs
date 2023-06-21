// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, From};

/// Timeline is divided into slots, and each slot has a corresponding slot index.
/// To calculate the slot index of a timestamp, `genesisTimestamp` and the duration of a slot are needed.
/// The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, From, Deref, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotIndex(u64);

impl core::fmt::Display for SlotIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SlotIndex {
    /// Creates a new [`SlotIndex`].
    pub fn new(index: u64) -> Self {
        Self::from(index)
    }
}
