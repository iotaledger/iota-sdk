// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, Display, From, FromStr};

use super::SlotIndex;
use crate::types::block::Error;

/// The tangle timeline is divided into epochs, and each epoch has a corresponding epoch index. Epochs are further
/// subdivided into slots, each with a [`SlotIndex`].
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

    /// Gets the epoch index given a [`SlotIndex`].
    pub fn from_slot_index(
        slot_index: SlotIndex,
        slots_per_epoch_exponent_iter: impl Iterator<Item = (EpochIndex, u32)>,
    ) -> Result<Self, Error> {
        let mut slot_index = *slot_index;
        // TODO: Should this start at 1?
        let mut res = 0;
        let mut last = None;
        for (start_epoch, exponent) in slots_per_epoch_exponent_iter {
            if *start_epoch > res {
                // We can't calculate the epoch if we don't have the exponent for the containing range
                if slot_index > 0 {
                    return Err(Error::InvalidStartEpoch(start_epoch));
                } else {
                    break;
                }
            }
            if let Some((last_start_epoch, last_exponent)) = last {
                // Get the number of slots this range of epochs represents
                let slots_in_range = (*start_epoch - last_start_epoch) << last_exponent;
                // Check whether the slot index is contained in this range
                if slot_index > slots_in_range {
                    // Update the slot index so it is in the context of the next epoch
                    slot_index -= slots_in_range;
                }
            }
            res = *start_epoch;
            last = Some((*start_epoch, exponent));
        }
        if let Some((_, exponent)) = last {
            res += slot_index >> exponent;
        }
        // TODO: Do we need to add one?
        Ok(Self(res + 1))
    }
}

impl From<EpochIndex> for u64 {
    fn from(epoch_index: EpochIndex) -> Self {
        *epoch_index
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(EpochIndex);
