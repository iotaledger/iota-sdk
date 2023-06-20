// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Timeline is divided into slots, and each slot has a corresponding slot index.
/// To calculate the slot index of a timestamp, `genesisTimestamp` and the duration of a slot are needed.
/// The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, derive_more::From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotIndex(u64);

impl SlotIndex {
    /// Creates a new [`SlotIndex`].
    pub fn new(index: u64) -> Self {
        Self::from(index)
    }
}
