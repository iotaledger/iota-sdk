// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{packer::SlicePacker, Packable, PackableExt};

use crate::types::block::slot::{SlotCommitmentId, SlotIndex};

/// Contains a summary of a slot.
/// It is linked to the commitment of the previous slot, which forms a commitment chain.
#[derive(Clone, Debug, Eq, PartialEq, Hash, derive_more::From, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotCommitment {}

impl SlotCommitment {
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
