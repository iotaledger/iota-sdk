// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::types::block::{
    core::{BasicBlockBuilder, Block, BlockWrapper},
    parent::StrongParents,
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array,
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
pub fn rand_block_ids(len: usize) -> Vec<BlockId> {
    let mut parents = (0..len).map(|_| rand_block_id()).collect::<Vec<_>>();
    parents.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));
    parents
}

/// Generates a random basic block with given strong parents.
pub fn rand_basic_block_builder_with_strong_parents(strong_parents: StrongParents) -> BasicBlockBuilder {
    Block::build_basic(strong_parents).with_payload(rand_payload_for_block())
}

/// Generates a random block wrapper.
pub fn rand_block_wrapper_with_block(protocol_params: ProtocolParameters, block: impl Into<Block>) -> BlockWrapper {
    BlockWrapper::new(
        protocol_params,
        rand_number(),
        rand_slot_commitment_id(),
        rand_slot_index(),
        // TODO rand_issuer_id
        rand_bytes_array().into(),
        block,
        rand_signature(),
    )
}

/// Generates a random block wrapper.
pub fn rand_block_wrapper(protocol_params: ProtocolParameters) -> BlockWrapper {
    rand_block_wrapper_with_block(
        protocol_params,
        rand_basic_block_builder_with_strong_parents(rand_strong_parents())
            .finish_block()
            .unwrap(),
    )
}
