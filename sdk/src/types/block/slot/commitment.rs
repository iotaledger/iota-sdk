// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{Packable, PackableExt};

use crate::types::block::slot::{commitment_id::SlotCommitmentHash, RootsId, SlotCommitmentId, SlotIndex};

/// Contains a summary of a slot.
/// It is linked to the commitment of the previous slot, which forms a commitment chain.
#[derive(Clone, Debug, Eq, PartialEq, Hash, derive_more::From, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct SlotCommitment {
    /// The protocol version.
    protocol_version: u8,
    /// The slot index of this commitment.
    /// It is calculated based on genesis timestamp and the duration of a slot.
    slot: SlotIndex,
    /// The commitment ID of the previous slot.
    previous_commitment_id: SlotCommitmentId,
    /// The digest of multiple sparse merkle tree roots of this slot.
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
        slot: SlotIndex,
        previous_commitment_id: SlotCommitmentId,
        roots_id: RootsId,
        cumulative_weight: u64,
        reference_mana_cost: u64,
    ) -> Self {
        Self {
            protocol_version,
            slot,
            previous_commitment_id,
            roots_id,
            cumulative_weight,
            reference_mana_cost,
        }
    }

    /// Returns the protocol version of the [`SlotCommitment`].
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the slot index of the [`SlotCommitment`].
    pub fn slot(&self) -> SlotIndex {
        self.slot
    }

    /// Returns the previous slot commitment ID of the [`SlotCommitment`].
    pub fn previous_commitment_id(&self) -> &SlotCommitmentId {
        &self.previous_commitment_id
    }

    /// Returns the roots ID of the [`SlotCommitment`].
    pub fn roots_id(&self) -> &RootsId {
        &self.roots_id
    }

    /// Returns the cumulative weight of the [`SlotCommitment`].
    pub fn cumulative_weight(&self) -> u64 {
        self.cumulative_weight
    }

    /// Returns the reference mana cost of the [`SlotCommitment`].
    pub fn reference_mana_cost(&self) -> u64 {
        self.reference_mana_cost
    }

    /// Computes the [`SlotCommitmentId`] of the [`SlotCommitment`].
    pub fn id(&self) -> SlotCommitmentId {
        SlotCommitmentHash::new(Blake2b256::digest(self.pack_to_vec()).into()).into_slot_commitment_id(self.slot)
    }
}
