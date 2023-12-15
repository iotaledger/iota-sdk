// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Add, AddAssign, Deref, Display, From, FromStr, Sub, SubAssign};

use super::EpochIndex;

/// The tangle timeline is divided into epochs, and each epoch has a corresponding [`EpochIndex`]. Epochs are further
/// subdivided into slots, each with a slot index.
/// To calculate the slot index of a timestamp, `genesisUnixTimestamp` and the `slotDurationInSeconds` are needed.
/// The slot index of timestamp `ts` is `(ts - genesisTimestamp)/duration + 1`.
///
/// # Examples
///
/// Given `slotDurationInSeconds == 10`
///
/// ## Slots
///
/// | slot<br>index | start timestamp<br>(inclusive) | end timestamp<br>(exclusive) |
/// | :- | :------------ | :------------ |
/// | 0  | -infinity     | genesis       |
/// | 1  | genesis       | genesis + 10s |
/// | 2  | genesis + 10s | genesis + 20s |
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    From,
    Deref,
    Display,
    FromStr,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    packable::Packable,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct SlotIndex(pub u32);

impl SlotIndex {
    /// Gets the [`EpochIndex`] of this slot.
    pub fn to_epoch_index(self, slots_per_epoch_exponent: u8) -> EpochIndex {
        EpochIndex::from_slot_index(self, slots_per_epoch_exponent)
    }

    pub fn from_epoch_index(epoch_index: EpochIndex, slots_per_epoch_exponent: u8) -> Self {
        Self(*epoch_index << slots_per_epoch_exponent)
    }

    /// Gets the slot index of a unix timestamp in seconds.
    /// Slots are counted starting from `1` with `0` being reserved for times before the genesis.
    pub fn from_timestamp(
        timestamp: u64,
        genesis_slot: u32,
        genesis_unix_timestamp: u64,
        slot_duration_in_seconds: u8,
    ) -> Self {
        timestamp
            .checked_sub(genesis_unix_timestamp)
            .map(|elapsed| (genesis_slot as u64 + (elapsed / slot_duration_in_seconds as u64) + 1) as u32)
            .unwrap_or(genesis_slot)
            .into()
    }

    /// Converts the slot index into the unix timestamp representing the beginning of the slot.
    /// Slot `0` will return the unix epoch.
    pub fn to_timestamp(self, genesis_unix_timestamp: u64, slot_duration_in_seconds: u8) -> u64 {
        self.0
            .checked_sub(1)
            .map(|adjusted_slot| (adjusted_slot as u64 * slot_duration_in_seconds as u64) + genesis_unix_timestamp)
            .unwrap_or_default()
    }
}

impl PartialEq<u32> for SlotIndex {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl core::ops::Add<u32> for SlotIndex {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        Self(self.0 + other)
    }
}

impl core::ops::AddAssign<u32> for SlotIndex {
    fn add_assign(&mut self, other: u32) {
        self.0 += other;
    }
}

impl core::ops::Sub<u32> for SlotIndex {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        Self(self.0 - other)
    }
}

impl core::ops::SubAssign<u32> for SlotIndex {
    fn sub_assign(&mut self, other: u32) {
        self.0 -= other;
    }
}

impl From<SlotIndex> for u32 {
    fn from(slot_index: SlotIndex) -> Self {
        *slot_index
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::types::block::protocol::ProtocolParameters;

    #[test]
    fn to_from_timestamp() {
        let protocol_params = ProtocolParameters::default();

        // Timestamp before the genesis
        let timestamp = protocol_params.genesis_unix_timestamp() - 100;
        let slot_index = protocol_params.slot_index(timestamp);
        assert_eq!(*slot_index, 0);
        assert_eq!(
            slot_index.to_timestamp(
                protocol_params.genesis_unix_timestamp(),
                protocol_params.slot_duration_in_seconds()
            ),
            0
        );

        // Genesis timestamp
        let timestamp = protocol_params.genesis_unix_timestamp();
        let slot_index = protocol_params.slot_index(timestamp);
        assert_eq!(*slot_index, 1);
        assert_eq!(
            slot_index.to_timestamp(
                protocol_params.genesis_unix_timestamp(),
                protocol_params.slot_duration_in_seconds()
            ),
            timestamp
        );

        // Timestamp 5 seconds after slot 100 starts
        let timestamp = protocol_params.genesis_unix_timestamp()
            + (99 * protocol_params.slot_duration_in_seconds() as u64) // Add 99 because the slots are 1-indexed
            + 5;
        let slot_index = protocol_params.slot_index(timestamp);
        assert_eq!(*slot_index, 100);
        assert_eq!(
            slot_index.to_timestamp(
                protocol_params.genesis_unix_timestamp(),
                protocol_params.slot_duration_in_seconds()
            ),
            timestamp - 5
        );
    }
}
