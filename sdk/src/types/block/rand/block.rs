// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::types::block::{
    core::{
        basic::{self, BasicBlockBodyBuilder},
        Block, BlockHeader, UnsignedBlock,
    },
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array,
        number::rand_number,
        output::rand_account_id,
        parents::rand_strong_parents,
        payload::rand_payload_for_block,
        signature::rand_sign_ed25519,
        slot::{rand_slot_commitment_id, rand_slot_index},
    },
    BlockBody, BlockId,
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

/// Generates a random basic block body with given strong parents.
pub fn rand_basic_block_body_with_strong_parents(strong_parents: basic::StrongParents) -> BlockBody {
    rand_basic_block_body_builder_with_strong_parents(strong_parents)
        .with_payload(rand_payload_for_block())
        .finish_block_body()
        .unwrap()
}

/// Generates a random basic block body builder with given strong parents.
pub fn rand_basic_block_body_builder_with_strong_parents(
    strong_parents: basic::StrongParents,
) -> BasicBlockBodyBuilder {
    BlockBody::build_basic(strong_parents, basic::MaxBurnedManaAmount::Amount(0))
}

/// Generates a random block with given block body.
pub fn rand_block_with_block_body(protocol_params: ProtocolParameters, block: BlockBody) -> Block {
    Block::build(
        BlockHeader::new(
            protocol_params.version(),
            protocol_params.network_id(),
            rand_number(),
            rand_slot_commitment_id(),
            rand_slot_index(),
            rand_account_id(),
        ),
        block,
    )
    .sign_random()
}

/// Generates a random block with given strong parents.
pub fn rand_block_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: basic::StrongParents,
) -> Block {
    rand_block_with_block_body(
        protocol_params,
        rand_basic_block_body_with_strong_parents(strong_parents),
    )
}

/// Generates a random block.
pub fn rand_block(protocol_params: ProtocolParameters) -> Block {
    rand_block_with_strong_parents(protocol_params, rand_strong_parents())
}

pub trait SignBlockRandom {
    fn sign_random(self) -> Block;
}

impl SignBlockRandom for UnsignedBlock {
    fn sign_random(self) -> Block {
        let signing_input = self.signing_input();
        self.finish(rand_sign_ed25519(&signing_input)).unwrap()
    }
}
