// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

/// Timeline is divided into slots, and each epoch has a corresponding epoch index.
/// To calculate the epoch index of a timestamp, `slotsPerEpochExponent` and `slotDurationInSeconds` are needed.
/// An epoch consists of `2^slotsPerEpochExponent` slots.
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, Display, FromStr, packable::Packable,
)]
#[repr(transparent)]
pub struct EpochIndex(u64);

impl EpochIndex {
    /// Creates a new [`EpochIndex`].
    pub fn new(index: u64) -> Self {
        Self::from(index)
    }
}

impl From<EpochIndex> for u64 {
    fn from(epoch_index: EpochIndex) -> Self {
        *epoch_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(EpochIndex);
