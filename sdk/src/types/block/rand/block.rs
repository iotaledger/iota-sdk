// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::types::block::{
    core::{
        basic::{self, BasicBlockBodyBuilder},
        BlockHeader, SignedBlock, UnsignedBlock,
    },
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array,
        issuer_id::rand_issuer_id,
        number::rand_number,
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
    BlockBody::build_basic(strong_parents, rand_number())
}

/// Generates a random block with given block body.
pub fn rand_block_with_block_body(protocol_params: ProtocolParameters, block: BlockBody) -> SignedBlock {
    SignedBlock::build(
        BlockHeader::new(
            protocol_params.version(),
            protocol_params.network_id(),
            rand_number(),
            rand_slot_commitment_id(),
            rand_slot_index(),
            rand_issuer_id(),
        ),
        block,
    )
    .sign_random()
}

/// Generates a random signed block with given strong parents.
pub fn rand_signed_block_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: basic::StrongParents,
) -> SignedBlock {
    rand_block_with_block_body(
        protocol_params,
        rand_basic_block_body_with_strong_parents(strong_parents),
    )
}

/// Generates a random signed block.
pub fn rand_signed_block(protocol_params: ProtocolParameters) -> SignedBlock {
    rand_signed_block_with_strong_parents(protocol_params, rand_strong_parents())
}

pub trait SignBlockRandom {
    fn sign_random(self) -> SignedBlock;
}

impl SignBlockRandom for UnsignedBlock {
    fn sign_random(self) -> SignedBlock {
        let signing_input = self.signing_input();
        self.finish(rand_sign_ed25519(&signing_input)).unwrap()
    }
}
