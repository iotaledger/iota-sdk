// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

use crate::utils::serde::string;

/// Timeline is divided into slots, and each slot has a corresponding slot index.
/// To calculate the slot index of a timestamp, `genesisTimestamp` and the duration of a slot are needed.
/// The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, From, Deref, Display, FromStr, packable::Packable)]
pub struct SlotIndex(u64);

impl SlotIndex {
    /// Creates a new [`SlotIndex`].
    pub fn new(index: u64) -> Self {
        Self::from(index)
    }
}

impl From<SlotIndex> for u64 {
    fn from(slot_index: SlotIndex) -> Self {
        *slot_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(SlotIndex);
