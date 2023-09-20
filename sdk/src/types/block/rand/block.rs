// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeSet;

use crate::types::block::{
    core::{BasicBlockBuilder, Block, BlockWrapper},
    parent::StrongParents,
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array,
        issuer_id::rand_issuer_id,
        number::rand_number,
        parents::rand_strong_parents,
        payload::rand_payload_for_block,
        signature::rand_signature,
        slot::{rand_slot_commitment_id, rand_slot_index},
    },
    BlockId,
};

/// Generates a random block id.
pub fn rand_block_id() -> BlockId {
    BlockId::new(rand_bytes_array())
}

/// Generates a vector of random block ids of a given length.
pub fn rand_block_ids(len: usize) -> BTreeSet<BlockId> {
    let mut res = BTreeSet::new();
    while res.len() < len {
        res.insert(rand_block_id());
    }
    res
}

/// Generates a random basic block with given strong parents.
pub fn rand_basic_block_builder_with_strong_parents(strong_parents: StrongParents) -> BasicBlockBuilder {
    Block::build_basic(strong_parents, rand_number()).with_payload(rand_payload_for_block())
}

/// Generates a random block wrapper.
pub fn rand_block_wrapper_with_block(protocol_params: &ProtocolParameters, block: impl Into<Block>) -> BlockWrapper {
    BlockWrapper::new(
        protocol_params,
        rand_number(),
        rand_slot_commitment_id(),
        rand_slot_index(),
        rand_issuer_id(),
        block,
        rand_signature(),
    )
}

/// Generates a random block wrapper.
pub fn rand_block_wrapper(protocol_params: &ProtocolParameters) -> BlockWrapper {
    rand_block_wrapper_with_block(
        protocol_params,
        rand_basic_block_builder_with_strong_parents(rand_strong_parents())
            .finish_block()
            .unwrap(),
    )
}
