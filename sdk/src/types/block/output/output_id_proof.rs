// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::convert::Infallible;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{Packable, PackableExt};

#[cfg(feature = "serde")]
use crate::utils::serde::prefix_hex_bytes;
use crate::{
    types::block::{output::Output, slot::SlotIndex},
    utils::merkle_hasher::{largest_power_of_two, LEAF_HASH_PREFIX, NODE_HASH_PREFIX},
};

#[derive(Debug, derive_more::Display)]
pub enum ProofError {
    #[display(fmt = "invalid output ID proof kind: {_0}")]
    Kind(u8),
    #[display(fmt = "index {index} is out of range, outputs length {len}")]
    IndexOutOfRange { index: u16, len: u16 },
    #[display(fmt = "no outputs provided")]
    NoOutputs,
}

#[cfg(feature = "std")]
impl std::error::Error for ProofError {}

impl From<Infallible> for ProofError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

/// The proof of the output identifier.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = ProofError)]
pub struct OutputIdProof {
    pub slot: SlotIndex,
    pub output_index: u16,
    #[cfg_attr(feature = "serde", serde(with = "prefix_hex_bytes"))]
    pub transaction_commitment: [u8; 32],
    #[packable(verify_with = verify_output_commitment_type)]
    pub output_commitment_proof: OutputCommitmentProof,
}

impl OutputCommitmentProof {
    pub fn new(outputs: &[Output], index: u16) -> Result<Self, ProofError> {
        let num_outputs = outputs.len() as u16;
        if num_outputs == 0 {
            Err(ProofError::NoOutputs)
        } else if index >= num_outputs {
            Err(ProofError::IndexOutOfRange {
                index,
                len: num_outputs,
            })
        } else {
            Ok(Self::proof(outputs, index))
        }
    }

    fn proof(outputs: &[Output], index: u16) -> Self {
        if let [output] = outputs {
            Self::value(&output.pack_to_vec())
        } else {
            let num_outputs = outputs.len() as u16;

            // Select a `pivot` element to split `data` into two slices `left` and `right`.
            let pivot = largest_power_of_two(num_outputs as _) as u16;
            let (left, right) = outputs.split_at(pivot as _);

            if index < pivot {
                // `value` is contained in the left subtree, and the `right` subtree can be hashed together.
                Self::node(Self::proof(left, index), Self::hash(right))
            } else {
                // `value` is contained in the right subtree, and the `left` subtree can be hashed together.
                Self::node(Self::hash(left), Self::proof(right, index - pivot))
            }
        }
    }

    /// Get the merkle tree hash for a list of outputs
    fn hash(outputs: &[Output]) -> LeafHash {
        match outputs {
            [] => LeafHash::empty(),
            [output] => LeafHash::new(&output.pack_to_vec()),
            _ => {
                let pivot = largest_power_of_two(outputs.len() as _);
                let (left, right) = outputs.split_at(pivot as _);
                let left = Self::hash(left).0;
                let right = Self::hash(right).0;

                let mut hasher = Blake2b256::default();

                hasher.update([NODE_HASH_PREFIX]);
                hasher.update(left);
                hasher.update(right);
                LeafHash(hasher.finalize().into())
            }
        }
    }

    fn node(left: impl Into<Self>, right: impl Into<Self>) -> Self {
        Self::Node(HashableNode::new(left, right))
    }

    fn value(bytes: &[u8]) -> Self {
        Self::Value(ValueHash::new(bytes))
    }
}

fn verify_output_commitment_type(proof: &OutputCommitmentProof) -> Result<(), ProofError> {
    match proof {
        OutputCommitmentProof::Leaf(_) => Err(ProofError::Kind(LeafHash::KIND)),
        _ => Ok(()),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[packable(tag_type = u8, with_error = ProofError::Kind)]
#[packable(unpack_error = ProofError)]
pub enum OutputCommitmentProof {
    #[packable(tag = HashableNode::KIND)]
    Node(HashableNode),
    #[packable(tag = LeafHash::KIND)]
    Leaf(LeafHash),
    #[packable(tag = ValueHash::KIND)]
    Value(ValueHash),
}

/// Contains the hashes of the left and right children of a node in the OutputCommitmentProof tree.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_visitor = ())]
pub struct HashableNode {
    pub l: Box<OutputCommitmentProof>,
    pub r: Box<OutputCommitmentProof>,
}

impl HashableNode {
    const KIND: u8 = 0;

    pub fn new(left: impl Into<OutputCommitmentProof>, right: impl Into<OutputCommitmentProof>) -> Self {
        Self {
            l: Box::new(left.into()),
            r: Box::new(right.into()),
        }
    }
}

/// Contains the hash of a leaf in the OutputCommitmentProof tree.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LeafHash(pub [u8; 32]);

impl LeafHash {
    const KIND: u8 = 1;

    pub fn new(bytes: &[u8]) -> Self {
        let mut hasher = Blake2b256::default();

        hasher.update([LEAF_HASH_PREFIX]);
        hasher.update(bytes);
        Self(hasher.finalize().into())
    }

    pub fn empty() -> Self {
        Self(Blake2b256::digest([]).into())
    }
}

impl Packable for LeafHash {
    type UnpackError = Infallible;
    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        32_u8.pack(packer)?;
        self.0.pack(packer)?;
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let _len = u8::unpack(unpacker, visitor)?;
        let bytes = Packable::unpack(unpacker, visitor)?;
        Ok(Self(bytes))
    }
}

/// Contains the hash of the value for which the OutputCommitmentProof is being computed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ValueHash(pub [u8; 32]);

impl ValueHash {
    const KIND: u8 = 2;

    pub fn new(bytes: &[u8]) -> Self {
        let mut hasher = Blake2b256::default();

        hasher.update([LEAF_HASH_PREFIX]);
        hasher.update(bytes);
        Self(hasher.finalize().into())
    }
}

impl Packable for ValueHash {
    type UnpackError = Infallible;
    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        32_u8.pack(packer)?;
        self.0.pack(packer)?;
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        let _len = u8::unpack(unpacker, visitor)?;
        let bytes = Packable::unpack(unpacker, visitor)?;
        Ok(Self(bytes))
    }
}

#[cfg(feature = "serde")]
mod serialization {
    use alloc::format;

    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use super::*;
    use crate::{impl_serde_typed_dto, utils::serde::prefix_hex_bytes};

    #[derive(Serialize, Deserialize)]
    struct NodeDto {
        #[serde(rename = "type")]
        kind: u8,
        l: Box<OutputCommitmentProof>,
        r: Box<OutputCommitmentProof>,
    }

    #[derive(Serialize, Deserialize)]
    struct HashDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "prefix_hex_bytes")]
        hash: [u8; 32],
    }

    impl From<NodeDto> for HashableNode {
        fn from(value: NodeDto) -> Self {
            Self { l: value.l, r: value.r }
        }
    }

    impl From<&HashableNode> for NodeDto {
        fn from(value: &HashableNode) -> Self {
            Self {
                kind: HashableNode::KIND,
                l: value.l.clone(),
                r: value.r.clone(),
            }
        }
    }

    impl From<HashDto> for ValueHash {
        fn from(value: HashDto) -> Self {
            Self(value.hash)
        }
    }

    impl From<&ValueHash> for HashDto {
        fn from(value: &ValueHash) -> Self {
            Self {
                kind: ValueHash::KIND,
                hash: value.0,
            }
        }
    }

    impl From<HashDto> for LeafHash {
        fn from(value: HashDto) -> Self {
            Self(value.hash)
        }
    }

    impl From<&LeafHash> for HashDto {
        fn from(value: &LeafHash) -> Self {
            Self {
                kind: LeafHash::KIND,
                hash: value.0,
            }
        }
    }

    impl_serde_typed_dto!(HashableNode, NodeDto, "hashable node");
    impl_serde_typed_dto!(ValueHash, HashDto, "value hash");
    impl_serde_typed_dto!(LeafHash, HashDto, "leaf hash");

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
                    HashableNode::KIND => Self::Node(
                        serde_json::from_value::<HashableNode>(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize hashable node: {e}")))?,
                    ),
                    LeafHash::KIND => Self::Leaf(
                        serde_json::from_value::<LeafHash>(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize leaf hash: {e}")))?,
                    ),
                    ValueHash::KIND => Self::Value(
                        serde_json::from_value::<ValueHash>(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize value hash: {e}")))?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid output commitment proof")),
                },
            )
        }
    }
}
