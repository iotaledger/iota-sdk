// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::slot::SlotIndex;

impl_id!(pub SlotCommitmentId, 40, "Identifier of a slot commitment.");

impl SlotCommitmentId {
    pub fn index(&self) -> SlotIndex {
        // PANIC: taking the last 8 bytes of 40 bytes is safe.
        u32::from_le_bytes(
            self.0[Self::LENGTH - core::mem::size_of::<SlotIndex>()..]
                .try_into()
                .unwrap(),
        )
        .into()
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(SlotCommitmentId);
