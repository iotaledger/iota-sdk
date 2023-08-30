// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

use super::EpochIndex;
use crate::types::block::Error;

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
        slots_per_epoch_exponent_iter: impl Iterator<Item = (EpochIndex, u8)>,
    ) -> Result<EpochIndex, Error> {
        EpochIndex::from_slot_index(self, slots_per_epoch_exponent_iter)
    }

    /// Gets the slot index of a unix timestamp.
    /// Slots are counted starting from `1` with `0` being reserved for times before the genesis.
    pub fn from_timestamp(timestamp: u64, genesis_unix_timestamp: u64, slot_duration_in_seconds: u8) -> Self {
        timestamp
            .checked_sub(genesis_unix_timestamp)
            .map(|diff| (diff / slot_duration_in_seconds as u64) + 1)
            .unwrap_or_default()
            .into()
    }

    /// Converts the slot index into the unix timestamp representing the beginning of the slot.
    /// Slot `0` will return the unix epoch.
    pub fn to_timestamp(self, genesis_unix_timestamp: u64, slot_duration_in_seconds: u8) -> u64 {
        self.0
            .checked_sub(1)
            .map(|adjusted_slot| (adjusted_slot * slot_duration_in_seconds as u64) + genesis_unix_timestamp)
            .unwrap_or_default()
    }
}

impl From<SlotIndex> for u64 {
    fn from(slot_index: SlotIndex) -> Self {
        *slot_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(SlotIndex);

#[cfg(test)]
mod test {
    use crate::types::block::protocol::ProtocolParameters;

    #[test]
    fn to_from_timestamp() {
        let protocol_params = ProtocolParameters::default();

        // Timestamp before the genesis
        let timestamp = protocol_params.genesis_unix_timestamp() as u64 - 100;
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
        let timestamp = protocol_params.genesis_unix_timestamp() as u64;
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
        let timestamp = protocol_params.genesis_unix_timestamp() as u64
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
