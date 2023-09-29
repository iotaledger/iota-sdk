// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use super::signature::rand_sign_ed25519;
use crate::types::block::{
    core::{basic, BasicBlockBuilder, BlockBuilder, BlockWrapper},
    protocol::ProtocolParameters,
    rand::{
        bytes::rand_bytes_array,
        issuer_id::rand_issuer_id,
        number::rand_number,
        parents::rand_strong_parents,
        payload::rand_payload_for_block,
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
pub fn rand_basic_block_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: basic::StrongParents,
) -> BlockWrapper {
    rand_basic_block_builder_with_strong_parents(protocol_params, strong_parents).sign_random()
}

/// Generates a random basic block builder with given strong parents.
pub fn rand_basic_block_builder_with_strong_parents(
    protocol_params: ProtocolParameters,
    strong_parents: basic::StrongParents,
) -> BlockBuilder<BasicBlockBuilder> {
    BlockWrapper::build_basic(
        protocol_params.version(),
        protocol_params.network_id(),
        rand_slot_commitment_id(),
        rand_slot_index(),
        rand_issuer_id(),
        strong_parents,
        rand_number(),
    )
    .with_issuing_time(rand_number::<u64>())
    .with_payload(rand_payload_for_block())
}

/// Generates a random block wrapper.
pub fn rand_block_wrapper(protocol_params: ProtocolParameters) -> BlockWrapper {
    rand_basic_block_with_strong_parents(protocol_params, rand_strong_parents())
}

pub trait SignBlockRandom {
    fn sign_random(self) -> BlockWrapper;
}

impl SignBlockRandom for BlockBuilder<BasicBlockBuilder> {
    fn sign_random(self) -> BlockWrapper {
        let signing_input = self.signing_input();
        self.finish(rand_sign_ed25519(&signing_input)).unwrap()
    }
}
