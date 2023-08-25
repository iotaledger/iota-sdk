// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

use super::SlotIndex;
use crate::types::block::Error;

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
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, Display, FromStr, packable::Packable,
)]
#[repr(transparent)]
pub struct EpochIndex(u64);

impl EpochIndex {
    /// Creates a new [`EpochIndex`].
    pub fn new(index: u64) -> Self {
        Self::from(index)
    }

    /// Gets the epoch index given a [`SlotIndex`].
    pub fn from_slot_index(
        slot_index: SlotIndex,
        slots_per_epoch_exponent_iter: impl Iterator<Item = (EpochIndex, u32)>,
    ) -> Result<Self, Error> {
        let mut slot_index = *slot_index;
        let mut res = 0;
        let mut last = None;
        for (start_epoch, exponent) in slots_per_epoch_exponent_iter {
            if let Some((last_start_epoch, last_exponent)) = last {
                if *start_epoch <= last_start_epoch {
                    return Err(Error::InvalidStartEpoch(start_epoch));
                }
                // Get the number of slots this range of epochs represents
                let slots_in_range = (*start_epoch - last_start_epoch) << last_exponent;
                // Check whether the slot index is contained in this range
                if slot_index > slots_in_range {
                    // Update the slot index so it is in the context of the next epoch
                    slot_index -= slots_in_range;
                } else {
                    break;
                }
            }
            if *start_epoch > res {
                // We can't calculate the epoch if we don't have the exponent for the containing range
                if slot_index > 0 {
                    return Err(Error::InvalidStartEpoch(start_epoch));
                } else {
                    break;
                }
            }
            res = *start_epoch + (slot_index >> exponent);
            last = Some((*start_epoch, exponent));
        }
        Ok(Self(res))
    }
}

impl From<EpochIndex> for u64 {
    fn from(epoch_index: EpochIndex) -> Self {
        *epoch_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(EpochIndex);

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::block::protocol::ProtocolParameters;

    #[test]
    fn epoch_index_from_slot() {
        let v3_params = ProtocolParameters {
            version: 3,
            slots_per_epoch_exponent: 10,
            ..Default::default()
        };
        let v4_params = ProtocolParameters {
            version: 4,
            slots_per_epoch_exponent: 11,
            ..Default::default()
        };
        let params = [(EpochIndex(0), v3_params.clone()), (EpochIndex(10), v4_params)];
        let slots_per_epoch_exponent_iter = params
            .iter()
            .map(|(start_index, params)| (*start_index, params.slots_per_epoch_exponent()));

        let slot_index = SlotIndex::new(3000);
        let epoch_index = EpochIndex::from_slot_index(slot_index, slots_per_epoch_exponent_iter.clone());
        assert_eq!(epoch_index, Ok(EpochIndex(2)));

        let slot_index = SlotIndex::new(10 * v3_params.slots_per_epoch() + 3000);
        let epoch_index = EpochIndex::from_slot_index(slot_index, slots_per_epoch_exponent_iter.clone());
        assert_eq!(epoch_index, Ok(EpochIndex(11)));
    }

    #[test]
    fn invalid_params() {
        let v3_params = ProtocolParameters {
            version: 3,
            slots_per_epoch_exponent: 10,
            ..Default::default()
        };
        let v4_params = ProtocolParameters {
            version: 4,
            slots_per_epoch_exponent: 11,
            ..Default::default()
        };
        let v5_params = ProtocolParameters {
            version: 5,
            slots_per_epoch_exponent: 12,
            ..Default::default()
        };
        let slot_index = SlotIndex::new(100000);

        // Params must cover the entire history starting at epoch 0
        let params = [(EpochIndex(10), v4_params.clone()), (EpochIndex(20), v5_params.clone())];
        let slots_per_epoch_exponent_iter = params
            .iter()
            .map(|(start_index, params)| (*start_index, params.slots_per_epoch_exponent()));
        let epoch_index = EpochIndex::from_slot_index(slot_index, slots_per_epoch_exponent_iter);
        assert_eq!(epoch_index, Err(Error::InvalidStartEpoch(EpochIndex(10))));

        // Params must not contain duplicate start epochs
        let params = [
            (EpochIndex(0), v3_params.clone()),
            (EpochIndex(10), v4_params.clone()),
            (EpochIndex(10), v5_params.clone()),
        ];
        let slots_per_epoch_exponent_iter = params
            .iter()
            .map(|(start_index, params)| (*start_index, params.slots_per_epoch_exponent()));
        let epoch_index = EpochIndex::from_slot_index(slot_index, slots_per_epoch_exponent_iter);
        assert_eq!(epoch_index, Err(Error::InvalidStartEpoch(EpochIndex(10))));

        // Params must be properly ordered
        let params = [
            (EpochIndex(10), v4_params),
            (EpochIndex(0), v3_params),
            (EpochIndex(20), v5_params),
        ];
        let slots_per_epoch_exponent_iter = params
            .iter()
            .map(|(start_index, params)| (*start_index, params.slots_per_epoch_exponent()));
        let epoch_index = EpochIndex::from_slot_index(slot_index, slots_per_epoch_exponent_iter);
        assert_eq!(epoch_index, Err(Error::InvalidStartEpoch(EpochIndex(10))));
    }
}
