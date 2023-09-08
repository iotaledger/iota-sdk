// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{
    packer::{Packer, SlicePacker},
    Packable,
};

use crate::types::block::{
    protocol::ProtocolParameters,
    signature::Ed25519Signature,
    slot::{SlotCommitmentId, SlotIndex},
    Block, IssuerId,
};

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockWrapper<B> {
    /// Protocol parameters of the network to which this block belongs.
    pub(crate) protocol_params: ProtocolParameters,
    /// The time at which the block was issued. It is a Unix timestamp in nanoseconds.
    pub(crate) issuing_time: u64,
    /// The identifier of the slot to which this block commits.
    pub(crate) slot_commitment_id: SlotCommitmentId,
    /// The slot index of the latest finalized slot.
    pub(crate) latest_finalized_slot: SlotIndex,
    /// The identifier of the account that issued this block.
    pub(crate) issuer_id: IssuerId,
    /// The inner block data, either [`BasicBlock`] or [`ValidationBlock`].
    pub(crate) data: B,
    /// The block signature, used to validate issuance capabilities.
    pub(crate) signature: Ed25519Signature,
}

impl<B> BlockWrapper<B> {
    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.protocol_params.version()
    }

    /// Returns the protocol parameters of a [`Block`].
    #[inline(always)]
    pub fn protocol_parameters(&self) -> &ProtocolParameters {
        &self.protocol_params
    }

    /// Returns the network id of a [`Block`].
    #[inline(always)]
    pub fn network_id(&self) -> u64 {
        self.protocol_params.network_id()
    }

    /// Returns the issuing time of a [`Block`].
    #[inline(always)]
    pub fn issuing_time(&self) -> u64 {
        self.issuing_time
    }

    /// Returns the slot commitment ID of a [`Block`].
    #[inline(always)]
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        self.slot_commitment_id
    }

    /// Returns the latest finalized slot of a [`Block`].
    #[inline(always)]
    pub fn latest_finalized_slot(&self) -> SlotIndex {
        self.latest_finalized_slot
    }

    /// Returns the issuer ID of a [`Block`].
    #[inline(always)]
    pub fn issuer_id(&self) -> IssuerId {
        self.issuer_id
    }

    /// Returns the signature of a [`Block`].
    #[inline(always)]
    pub fn signature(&self) -> &Ed25519Signature {
        &self.signature
    }

    pub(crate) fn pack_header<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.protocol_version().pack(packer)?;
        self.network_id().pack(packer)?;
        self.issuing_time.pack(packer)?;
        self.slot_commitment_id.pack(packer)?;
        self.latest_finalized_slot.pack(packer)?;
        self.issuer_id.pack(packer)?;
        Ok(())
    }

    pub(crate) fn header_hash(&self) -> [u8; 32] {
        let mut bytes = [0u8; Block::HEADER_LENGTH];
        self.pack_header(&mut SlicePacker::new(&mut bytes)).unwrap();
        Blake2b256::digest(bytes).into()
    }
}
