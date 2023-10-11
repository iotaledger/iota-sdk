// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Add, AddAssign, Deref, Display, From, FromStr, Sub, SubAssign};

use super::SlotIndex;

/// The tangle timeline is divided into epochs, and each epoch has a corresponding epoch index. Epochs are further
/// subdivided into slots, each with a [`SlotIndex`].
/// To calculate the epoch index of a timestamp, `slotsPerEpochExponent` and `slotDurationInSeconds` are needed.
/// An epoch consists of `2^slotsPerEpochExponent` slots.
///
/// # Examples
///
/// Given `slotDurationInSeconds == 10` and `slotsPerEpochExponent == 3`
///
/// ## Slots
///
/// | slot<br>index | start timestamp<br>(inclusive) | end timestamp<br>(exclusive) |
/// | :- | :------------ | :------------ |
/// | 0  | -infinity     | genesis       |
/// | 1  | genesis       | genesis + 10s |
/// | 2  | genesis + 10s | genesis + 20s |
///
/// ## Epochs
///
/// | epoch<br>index | start slot<br>(inclusive) | end slot<br>(exclusive) |
/// | :- | :-- | :-- |
/// | 0  | 0   | 8   |
/// | 1  | 8   | 16  |
/// | 2  | 16  | 24  |
// ...
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    From,
    Deref,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Display,
    FromStr,
    packable::Packable,
)]
#[repr(transparent)]
pub struct EpochIndex(pub u32);

impl EpochIndex {
    /// Gets the range of slots this epoch contains.
    pub fn slot_index_range(&self, slots_per_epoch_exponent: u8) -> core::ops::RangeInclusive<SlotIndex> {
        self.first_slot_index(slots_per_epoch_exponent)..=self.last_slot_index(slots_per_epoch_exponent)
    }

    /// Gets the epoch index given a [`SlotIndex`].
    pub fn from_slot_index(slot_index: SlotIndex, slots_per_epoch_exponent: u8) -> Self {
        Self(*slot_index >> slots_per_epoch_exponent)
    }

    /// Gets the first [`SlotIndex`] of this epoch.
    pub fn first_slot_index(self, slots_per_epoch_exponent: u8) -> SlotIndex {
        SlotIndex::from_epoch_index(self, slots_per_epoch_exponent)
    }

    /// Gets the last [`SlotIndex`] of this epoch.
    pub fn last_slot_index(self, slots_per_epoch_exponent: u8) -> SlotIndex {
        SlotIndex::from_epoch_index(self + 1, slots_per_epoch_exponent) - 1
    }
}

impl From<EpochIndex> for u32 {
    fn from(epoch_index: EpochIndex) -> Self {
        *epoch_index
    }
}

impl PartialEq<u32> for EpochIndex {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl core::ops::Add<u32> for EpochIndex {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        Self(self.0 + other)
    }
}

impl core::ops::AddAssign<u32> for EpochIndex {
    fn add_assign(&mut self, other: u32) {
        self.0 += other;
    }
}

impl core::ops::Sub<u32> for EpochIndex {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        Self(self.0 - other)
    }
}

impl core::ops::SubAssign<u32> for EpochIndex {
    fn sub_assign(&mut self, other: u32) {
        self.0 -= other;
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(EpochIndex);

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::block::protocol::ProtocolParameters;

    #[test]
    fn epoch_index_to_from_slot() {
        let params = ProtocolParameters {
            version: 3,
            slots_per_epoch_exponent: 10,
            ..Default::default()
        };
        let slot_index = SlotIndex(3000);
        let epoch_index = EpochIndex::from_slot_index(slot_index, params.slots_per_epoch_exponent());
        assert_eq!(epoch_index, EpochIndex(2));
        assert_eq!(
            epoch_index.slot_index_range(params.slots_per_epoch_exponent()),
            SlotIndex(2048)..=SlotIndex(3071)
        );

        let slot_index = SlotIndex(10 * params.slots_per_epoch() + 2000);
        let epoch_index = EpochIndex::from_slot_index(slot_index, params.slots_per_epoch_exponent());
        assert_eq!(epoch_index, EpochIndex(11));
        assert_eq!(
            epoch_index.slot_index_range(params.slots_per_epoch_exponent()),
            SlotIndex(11 * params.slots_per_epoch())..=SlotIndex(12 * params.slots_per_epoch() - 1)
        );
    }
}
