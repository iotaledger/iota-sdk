// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use super::signature::rand_sign_ed25519;
use crate::types::block::{
    basic::BasicBlockData,
    core::Block,
    parent::StrongParents,
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
pub fn rand_basic_block_with_strong_parents(strong_parents: StrongParents) -> Block {
    rand_basic_block_builder_with_strong_parents(strong_parents)
        .with_payload(rand_payload_for_block())
        .sign_random()
}

/// Generates a random basic block builder with given parents.
pub fn rand_basic_block_builder_with_strong_parents(strong_parents: StrongParents) -> BlockBuilder<BasicBlockData> {
    Block::build_basic(
        rand_number(),
        rand_number(),
        rand_bytes_array().into(),
        rand_number::<u64>().into(),
        rand_bytes_array().into(),
        strong_parents,
    )
}

/// Generates a random block.
pub fn rand_block() -> Block {
    rand_basic_block_with_strong_parents(rand_strong_parents())
}

pub trait SignBlockRandom {
    fn sign_random(self) -> Block;
}

impl SignBlockRandom for BlockBuilder<BasicBlockData> {
    fn sign_random(self) -> Block {
        let signing_input = self.signing_input();
        self.finish(rand_sign_ed25519(&signing_input)).unwrap()
    }
}
