// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use super::signature::rand_sign_ed25519;
use crate::types::block::{
    core::{
        basic::{self, BasicBlockBuilder},
        BlockWrapper, BlockWrapperBuilder,
    },
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array,
        issuer_id::rand_issuer_id,
        number::rand_number,
        parents::rand_strong_parents,
        payload::rand_payload_for_block,
        slot::{rand_slot_commitment_id, rand_slot_index},
    },
    Block, BlockId,
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
pub fn rand_basic_block_with_strong_parents(strong_parents: basic::StrongParents) -> Block {
    rand_basic_block_builder_with_strong_parents(strong_parents)
        .with_payload(rand_payload_for_block())
        .finish_block()
        .unwrap()
}

/// Generates a random basic block builder with given strong parents.
pub fn rand_basic_block_builder_with_strong_parents(strong_parents: basic::StrongParents) -> BasicBlockBuilder {
    Block::build_basic(strong_parents, rand_number())
}

/// Generates a random block wrapper with given block.
pub fn rand_block_wrapper_with_block(protocol_params: ProtocolParameters, block: Block) -> BlockWrapper {
    BlockWrapper::build(
        protocol_params.version(),
        protocol_params.network_id(),
        rand_slot_commitment_id(),
        rand_slot_index(),
        rand_issuer_id(),
        block,
    )
    .with_issuing_time(rand_number::<u64>())
    .sign_random()
}

/// Generates a random block wrapper with given strong parents.
pub fn rand_block_wrapper_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: basic::StrongParents,
) -> BlockWrapper {
    rand_block_wrapper_with_block(protocol_params, rand_basic_block_with_strong_parents(strong_parents))
}

/// Generates a random block wrapper.
pub fn rand_block_wrapper(protocol_params: ProtocolParameters) -> BlockWrapper {
    rand_block_wrapper_with_strong_parents(protocol_params, rand_strong_parents())
}

pub trait SignBlockRandom {
    fn sign_random(self) -> BlockWrapper;
}

impl SignBlockRandom for BlockWrapperBuilder {
    fn sign_random(self) -> BlockWrapper {
        let signing_input = self.signing_input();
        self.finish(rand_sign_ed25519(&signing_input)).unwrap()
    }
}
