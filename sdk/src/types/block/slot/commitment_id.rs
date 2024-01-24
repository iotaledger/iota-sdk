// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::SlotIndex;

crate::impl_id!(
    /// The hash of a [`SlotCommitment`](crate::types::block::slot::SlotCommitment).
    pub SlotCommitmentHash {
        pub const LENGTH: usize = 32;
    }
    /// A [`SlotCommitment`](crate::types::block::slot::SlotCommitment) identifier.
    pub SlotCommitmentId;
);

impl SlotCommitmentId {
    /// Calculates the past bounded slot for the given slot of the SlotCommitment.
    /// Given any slot index of a commitment input, the result of this function is a slot index
    /// that is at least equal to the slot of the block in which it was issued, or higher.
    /// That means no commitment input can be chosen such that the index lies behind the slot index of the block,
    /// hence the past is bounded.
    pub fn past_bounded_slot(self, max_commitable_age: u32) -> SlotIndex {
        SlotIndex(*self.slot_index() + max_commitable_age)
    }
    /// Calculates the future bounded slot for the given slot of the SlotCommitment.
    /// Given any slot index of a commitment input, the result of this function is a slot index
    /// that is at most equal to the slot of the block in which it was issued, or lower.
    /// That means no commitment input can be chosen such that the index lies ahead of the slot index of the block,
    /// hence the future is bounded.
    pub fn future_bounded_slot(self, min_commitable_age: u32) -> SlotIndex {
        SlotIndex(*self.slot_index() + min_commitable_age)
    }
}
