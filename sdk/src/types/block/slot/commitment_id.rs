// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

crate::impl_id!(
    /// The hash of a [`SlotCommitment`](crate::types::block::slot::SlotCommitment).
    pub SlotCommitmentHash {
        pub const LENGTH: usize = 32;
    }
    /// A [`SlotCommitment`](crate::types::block::slot::SlotCommitment) identifier.
    pub SlotCommitmentId;
);
