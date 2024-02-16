// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

use crypto::hashes::blake2b::Blake2b256;
#[cfg(feature = "serde")]
use {crate::utils::serde::prefix_hex_bytes, alloc::format, serde::de::Deserialize, serde_json::Value};

use crate::{
    types::block::{output::OutputId, slot::SlotIndex},
    utils::merkle_hasher::{largest_power_of_two, MerkleHasher},
};

/// The proof of the output identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputIdProof {
    pub slot: SlotIndex,
    pub output_index: u16,
    #[cfg_attr(feature = "serde", serde(with = "prefix_hex_bytes"))]
    pub transaction_commitment: [u8; 32],
    pub output_commitment_proof: OutputCommitmentProof,
}

impl OutputCommitmentProof {
    pub fn new(output_ids: &[OutputId], index: u16) -> Self {
        let n = output_ids.len();
        debug_assert!(n > 0 && index < n as u16, "n={n}, index={index}");

        // Handle the special case where the "value" makes up the whole Merkle Tree.
        if n == 1 {
            return OutputCommitmentProof::ValueHash(ValueHash::new(&output_ids[0]));
        }

        // Select a `pivot` element to split `data` into two slices `left` and `right`.
        let pivot = largest_power_of_two(n as _) as u16;
        let (left, right) = output_ids.split_at(pivot as _);

        // Produces the Merkle hash of a sub tree not containing the `value`.
        let subtree_hash = |output_ids: &[OutputId]| {
            let values = output_ids.into_iter().map(OutputId::to_bytes).collect::<Vec<_>>();
            OutputCommitmentProof::LeafHash(LeafHash::new(MerkleHasher::digest::<Blake2b256>(&values).into()))
        };

        OutputCommitmentProof::HashableNode(if index < pivot {
            // `value` is contained in the left subtree, and the `right` subtree can be hashed together.
            HashableNode::new(Self::new(left, index), subtree_hash(right))
        } else {
            // `value` is contained in the right subtree, and the `left` subtree can be hashed together.
            HashableNode::new(subtree_hash(left), Self::new(right, index - pivot))
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum OutputCommitmentProof {
    HashableNode(HashableNode),
    LeafHash(LeafHash),
    ValueHash(ValueHash),
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for OutputCommitmentProof {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        Ok(
            match value
                .get("type")
                .and_then(Value::as_u64)
                .ok_or_else(|| serde::de::Error::custom("invalid output commitment proof type"))?
                as u8
            {
                0 => Self::HashableNode(
                    serde_json::from_value::<HashableNode>(value)
                        .map_err(|e| serde::de::Error::custom(format!("cannot deserialize hashable node: {e}")))?,
                ),
                1 => Self::LeafHash(
                    serde_json::from_value::<LeafHash>(value)
                        .map_err(|e| serde::de::Error::custom(format!("cannot deserialize leaf hash: {e}")))?,
                ),
                2 => Self::ValueHash(
                    serde_json::from_value::<ValueHash>(value)
                        .map_err(|e| serde::de::Error::custom(format!("cannot deserialize value hash: {e}")))?,
                ),
                _ => return Err(serde::de::Error::custom("invalid output commitment proof")),
            },
        )
    }
}

/// Node contains the hashes of the left and right children of a node in the tree.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HashableNode {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: u8,
    pub l: Box<OutputCommitmentProof>,
    pub r: Box<OutputCommitmentProof>,
}

impl HashableNode {
    const KIND: u8 = 0;

    fn new(left: OutputCommitmentProof, right: OutputCommitmentProof) -> Self {
        Self {
            kind: Self::KIND,
            l: left.into(),
            r: right.into(),
        }
    }
}

/// Leaf Hash contains the hash of a leaf in the tree.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeafHash {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: u8,
    #[cfg_attr(feature = "serde", serde(with = "prefix_hex_bytes"))]
    pub hash: [u8; 32],
}

impl LeafHash {
    const KIND: u8 = 1;

    fn new(hash: [u8; 32]) -> Self {
        Self { kind: Self::KIND, hash }
    }
}

/// Value Hash contains the hash of the value for which the proof is being computed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValueHash {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: u8,
    #[cfg_attr(feature = "serde", serde(with = "prefix_hex_bytes"))]
    pub hash: [u8; 32],
}

impl ValueHash {
    const KIND: u8 = 2;

    fn new(output_id: &OutputId) -> Self {
        Self {
            kind: Self::KIND,
            hash: output_id.hash(),
        }
    }
}
