// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    rand::{bytes::rand_bytes_array, number::rand_number},
    slot::{SlotCommitmentId, SlotIndex},
};

/// Generates a random slot commitment id.
pub fn rand_slot_commitment_id() -> SlotCommitmentId {
    SlotCommitmentId::new(rand_bytes_array())
}

/// Generates a random slot index.
pub fn rand_slot_index() -> SlotIndex {
    SlotIndex::new(rand_number())
}
