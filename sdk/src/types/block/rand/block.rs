// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use super::signature::rand_ed25519_signature;
use crate::types::block::{
    basic::BasicBlock,
    core::Block,
    parent::StrongParents,
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array, number::rand_number, parents::rand_strong_parents, payload::rand_payload_for_block,
    },
    BlockBuilder, BlockId,
};

/// Generates a random block id.
pub fn rand_block_id() -> BlockId {
    BlockId::new(rand_bytes_array())
}

/// Generates a vector of random block ids of a given length.
pub fn rand_block_ids(len: usize) -> Vec<BlockId> {
    let mut parents = (0..len).map(|_| rand_block_id()).collect::<Vec<_>>();
    parents.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));
    parents
}

/// Generates a random basic block with given parents.
pub fn rand_basic_block_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: StrongParents,
) -> Block {
    rand_basic_block_builder_with_strong_parents(protocol_params, strong_parents)
        .with_payload(rand_payload_for_block())
        .finish()
        .unwrap()
}

/// Generates a random basic block builder with given parents.
pub fn rand_basic_block_builder_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: StrongParents,
) -> BlockBuilder<BasicBlock> {
    Block::build_basic(
        protocol_params,
        rand_number(),
        rand_bytes_array().into(),
        rand_number::<u64>().into(),
        rand_bytes_array().into(),
        strong_parents,
        rand_ed25519_signature(),
    )
}

/// Generates a random block.
pub fn rand_block(protocol_params: ProtocolParameters) -> Block {
    rand_basic_block_with_strong_parents(protocol_params, rand_strong_parents())
}
