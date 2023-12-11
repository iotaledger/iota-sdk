// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, string::String};

#[cfg(feature = "serde")]
use {crate::utils::serde::prefix_hex_bytes, alloc::format, serde::de::Deserialize, serde_json::Value};

use crate::types::block::slot::SlotIndex;

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
    pub transaction_commitment: String,
    pub output_commitment_proof: OutputCommitmentProof,
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

/// Leaf Hash contains the hash of a leaf in the tree.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeafHash {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: u8,
    #[cfg_attr(feature = "serde", serde(with = "prefix_hex_bytes"))]
    pub hash: [u8; 32],
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
