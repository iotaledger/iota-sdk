// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{packer::SlicePacker, Packable, PackableExt};

use crate::types::block::slot::{RootsId, SlotCommitmentId, SlotIndex};

/// Contains a summary of a slot.
/// It is linked to the commitment of the previous slot, which forms a commitment chain.
#[derive(Clone, Debug, Eq, PartialEq, Hash, derive_more::From, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotCommitment {
    /// The slot index of this commitment.
    /// It is calculated based on genesis timestamp and the duration of a slot.
    index: SlotIndex,
    /// The commitment ID of the previous slot.
    previous_slot_commitment_id: SlotCommitmentId,
    /// A BLAKE2b-256 hash of concatenating multiple sparse merkle tree roots of a slot.
    roots_id: RootsId,
    /// The sum of previous slot commitment cumulative weight and weight of issuers of accepted blocks within this
    /// slot. It is just an indication of "committed into" this slot, and can not strictly be used for evaluating
    /// the switching of a chain.
    cumulative_weight: u64,
}

impl SlotCommitment {
    /// Creates a new [`SlotCommitment`].
    pub fn new(
        index: SlotIndex,
        previous_slot_commitment_id: SlotCommitmentId,
        roots_id: RootsId,
        cumulative_weight: u64,
    ) -> Self {
        Self {
            index,
            previous_slot_commitment_id,
            roots_id,
            cumulative_weight,
        }
    }

    /// Returns the index of the [`SlotCommitment`].
    pub fn index(&self) -> SlotIndex {
        self.index
    }

    /// Returns the previous slot commitment ID of the [`SlotCommitment`].
    pub fn previous_slot_commitment_id(&self) -> &SlotCommitmentId {
        &self.previous_slot_commitment_id
    }

    /// Returns the roots ID of the [`SlotCommitment`].
    pub fn roots_id(&self) -> &RootsId {
        &self.roots_id
    }

    /// Returns the cumulative weight of the [`SlotCommitment`].
    pub fn cumulative_weight(&self) -> u64 {
        self.cumulative_weight
    }

    /// Derives a [`SlotCommitmentId`] from the [`SlotCommitment`] and a [`SlotIndex`].
    pub fn id(&self, index: SlotIndex) -> SlotCommitmentId {
        let mut bytes = [0u8; SlotCommitmentId::LENGTH];
        let mut packer = SlicePacker::new(&mut bytes);
        let hash: [u8; 32] = Blake2b256::digest(self.pack_to_vec()).into();

        // PANIC: packing to an array of bytes can't fail.
        hash.pack(&mut packer).unwrap();
        index.pack(&mut packer).unwrap();

        SlotCommitmentId::from(bytes)
    }
}
