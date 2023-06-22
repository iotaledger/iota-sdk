// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{rand::bytes::rand_bytes_array, slot::SlotCommitmentId};

/// Generates a random slot commitment id.
pub fn rand_slot_commitment_id() -> SlotCommitmentId {
    SlotCommitmentId::new(rand_bytes_array())
}
