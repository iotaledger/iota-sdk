// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    output::{OutputConsumptionMetadata, OutputId, OutputInclusionMetadata, OutputMetadata},
    rand::{
        block::rand_block_id,
        option::rand_option,
        output::rand_output_id,
        slot::{rand_slot_commitment_id, rand_slot_index},
        transaction::rand_transaction_id,
    },
};

/// Generates a random [`OutputInclusionMetadata`].
pub fn rand_output_inclusion_metadata() -> OutputInclusionMetadata {
    OutputInclusionMetadata::new(
        rand_slot_index(),
        rand_transaction_id(),
        rand_option(rand_slot_commitment_id()),
    )
}

/// Generates a random [`OutputConsumptionMetadata`].
pub fn rand_output_consumption_metadata() -> OutputConsumptionMetadata {
    OutputConsumptionMetadata::new(
        rand_slot_index(),
        rand_transaction_id(),
        rand_option(rand_slot_commitment_id()),
    )
}

/// Generates a random [`OutputMetadata`] with a specific [`OutputId`].
pub fn rand_output_metadata_with_id(output_id: OutputId) -> OutputMetadata {
    OutputMetadata::new(
        output_id,
        rand_block_id(),
        rand_output_inclusion_metadata(),
        rand_option(rand_output_consumption_metadata()),
        rand_slot_commitment_id(),
    )
}

/// Generates a random [`OutputMetadata`].
pub fn rand_output_metadata() -> OutputMetadata {
    rand_output_metadata_with_id(rand_output_id())
}
