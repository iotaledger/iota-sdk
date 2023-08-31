// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{packer::SlicePacker, Packable, PackableExt};

use crate::types::block::slot::{RootsId, SlotCommitmentId, SlotIndex};

/// Contains a summary of a slot.
/// It is linked to the commitment of the previous slot, which forms a commitment chain.
#[derive(Clone, Debug, Eq, PartialEq, Hash, derive_more::From, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct SlotCommitment {
    // The version of the protocol running.
    #[cfg_attr(feature = "serde", serde(rename = "version"))]
    protocol_version: u8,
    /// The slot index of this commitment.
    /// It is calculated based on genesis timestamp and the duration of a slot.
    index: SlotIndex,
    /// The commitment ID of the previous slot.
    #[cfg_attr(feature = "serde", serde(rename = "prevId"))]
    previous_slot_commitment_id: SlotCommitmentId,
    /// A BLAKE2b-256 hash of concatenating multiple sparse merkle tree roots of a slot.
    roots_id: RootsId,
    /// The sum of previous slot commitment cumulative weight and weight of issuers of accepted blocks within this
    /// slot. It is just an indication of "committed into" this slot, and can not strictly be used for evaluating
    /// the switching of a chain.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    cumulative_weight: u64,
    /// Reference Mana Cost (RMC) to be used in the slot with index at `index + Max Committable Age`.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    reference_mana_cost: u64,
}

impl SlotCommitment {
    /// Creates a new [`SlotCommitment`].
    pub fn new(
        protocol_version: u8,
        index: SlotIndex,
        previous_slot_commitment_id: SlotCommitmentId,
        roots_id: RootsId,
        cumulative_weight: u64,
        reference_mana_cost: u64,
    ) -> Self {
        Self {
            protocol_version,
            index,
            previous_slot_commitment_id,
            roots_id,
            cumulative_weight,
            reference_mana_cost,
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

    /// Returns the [`RootsId`] of the [`SlotCommitment`].
    pub fn roots_id(&self) -> &RootsId {
        &self.roots_id
    }

    /// Returns the cumulative weight of the [`SlotCommitment`].
    pub fn cumulative_weight(&self) -> u64 {
        self.cumulative_weight
    }

    /// Derives the [`SlotCommitmentId`] of the [`SlotCommitment`].
    pub fn id(&self) -> SlotCommitmentId {
        let mut bytes = [0u8; SlotCommitmentId::LENGTH];
        let mut packer = SlicePacker::new(&mut bytes);
        let hash: [u8; 32] = Blake2b256::digest(self.pack_to_vec()).into();

        // PANIC: packing to an array of bytes can't fail.
        hash.pack(&mut packer).unwrap();
        self.index.pack(&mut packer).unwrap();

        SlotCommitmentId::from(bytes)
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::SlotCommitment;
    use crate::types::block::{
        slot::{RootsId, SlotCommitmentId, SlotIndex},
        PROTOCOL_VERSION,
    };

    #[test]
    fn test() {
        let commitment = SlotCommitment::new(
            PROTOCOL_VERSION,
            SlotIndex::new(10),
            SlotCommitmentId::from_str(
                "0x20e07a0ea344707d69a08b90be7ad14eec8326cf2b8b86c8ec23720fab8dcf8ec43a30e4a8cc3f1f",
            )
            .unwrap(),
            RootsId::from_str("0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f").unwrap(),
            5,
            10,
        );
        // TODO: Independently verify this value
        assert_eq!(
            &commitment.id().to_string(),
            "0x2f3ad38aa65d20ede9dcd6a045dccdd3332cf38192c4875308bb77116e8650880a00000000000000"
        )
    }
}
