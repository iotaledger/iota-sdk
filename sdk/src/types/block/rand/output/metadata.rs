// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    output::OutputMetadata,
    rand::{
        block::rand_block_id, bool::rand_bool, option::rand_option, output::rand_output_id,
        slot::rand_slot_commitment_id,
    },
};

/// Generates a random [`OutputMetadata`].
pub fn rand_output_metadata() -> OutputMetadata {
    OutputMetadata::new(
        rand_block_id(),
        rand_output_id(),
        rand_bool(),
        rand_option(rand_slot_commitment_id()),
        rand_slot_commitment_id(),
    )
}
