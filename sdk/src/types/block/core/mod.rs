// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod basic;
mod validation;
mod wrapper;

use alloc::boxed::Box;

use derive_more::From;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

pub use self::{basic::BasicBlock, validation::ValidationBlock, wrapper::BlockWrapper};
use crate::types::block::{
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    protocol::ProtocolParameters,
    Error,
};

// /// A builder to build a [`Block`].
// #[derive(Clone)]
// #[must_use]
// pub struct BlockBuilder<B>(pub(crate) B);

// impl<B> BlockBuilder<BlockWrapper<B>> {
//     pub fn from_block_data(
//         protocol_params: ProtocolParameters,
//         issuing_time: u64,
//         slot_commitment_id: SlotCommitmentId,
//         latest_finalized_slot: SlotIndex,
//         issuer_id: IssuerId,
//         data: B,
//         signature: Ed25519Signature,
//     ) -> Self { Self(BlockWrapper { protocol_params, issuing_time, slot_commitment_id, latest_finalized_slot,
//       issuer_id, data, signature, })
//     }
// }

// impl<B> BlockBuilder<B>
// where
//     B: Packable,
//     Block: From<B>,
// {
//     fn _finish(self) -> Result<(Block, Vec<u8>), Error> {
//         let block = Block::from(self.0);

//         verify_parents(
//             block.strong_parents(),
//             block.weak_parents(),
//             block.shallow_like_parents(),
//         )?;

//         let block_bytes = block.pack_to_vec();

//         if block_bytes.len() > Block::LENGTH_MAX {
//             return Err(Error::InvalidBlockLength(block_bytes.len()));
//         }

//         Ok((block, block_bytes))
//     }

//     /// Finishes the [`BlockBuilder`] into a [`Block`].
//     pub fn finish(self) -> Result<Block, Error> {
//         self._finish().map(|res| res.0)
//     }
// }

#[derive(Clone, Debug, Eq, PartialEq, From)]
pub enum Block {
    Basic(Box<BasicBlock>),
    Validation(Box<ValidationBlock>),
}

impl From<BasicBlock> for Block {
    fn from(value: BasicBlock) -> Self {
        Self::Basic(value.into())
    }
}

impl From<ValidationBlock> for Block {
    fn from(value: ValidationBlock) -> Self {
        Self::Validation(value.into())
    }
}

impl Packable for Block {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Basic(output) => {
                BasicBlock::KIND.pack(packer)?;
                output.pack(packer)
            }
            Self::Validation(output) => {
                ValidationBlock::KIND.pack(packer)?;
                output.pack(packer)
            }
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            BasicBlock::KIND => Self::from(BasicBlock::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            ValidationBlock::KIND => Self::from(ValidationBlock::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            // TODO Block error
            k => return Err(Error::InvalidOutputKind(k)).map_err(UnpackError::Packable),
        })
    }
}

// impl Packable for Block {
//     type UnpackError = Error;
//     type UnpackVisitor = ProtocolParameters;

//     fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
//         match self {
//             Self::Basic(block) => block.pack(packer),
//             Self::Validation(block) => block.pack(packer),
//         }?;

//         Ok(())
//     }

//     fn unpack<U: Unpacker, const VERIFY: bool>(
//         unpacker: &mut U,
//         protocol_params: &Self::UnpackVisitor,
//     ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> { let start_opt = unpacker.read_bytes();

//         let protocol_version = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         if VERIFY && protocol_version != protocol_params.version() {
//             return Err(UnpackError::Packable(Error::ProtocolVersionMismatch {
//                 expected: protocol_params.version(),
//                 actual: protocol_version,
//             }));
//         }

//         let network_id = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         if VERIFY && network_id != protocol_params.network_id() {
//             return Err(UnpackError::Packable(Error::NetworkIdMismatch {
//                 expected: protocol_params.network_id(),
//                 actual: network_id,
//             }));
//         }

//         let issuing_time = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         let slot_commitment_id = SlotCommitmentId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         let latest_finalized_slot = SlotIndex::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         let issuer_id = IssuerId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         let kind = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

//         let block = match kind {
//             BasicBlock::KIND => {
//                 let data = BasicBlockData::unpack::<_, VERIFY>(unpacker, protocol_params)?;
//                 let Signature::Ed25519(signature) = Signature::unpack::<_, VERIFY>(unpacker, &())?;

//                 Self::from(BlockWrapper {
//                     protocol_params: protocol_params.clone(),
//                     issuing_time,
//                     slot_commitment_id,
//                     latest_finalized_slot,
//                     issuer_id,
//                     data,
//                     signature,
//                 })
//             }
//             ValidationBlock::KIND => {
//                 let data = ValidationBlockData::unpack::<_, VERIFY>(unpacker, protocol_params)?;
//                 let Signature::Ed25519(signature) = Signature::unpack::<_, VERIFY>(unpacker, &())?;

//                 Self::from(BlockWrapper {
//                     protocol_params: protocol_params.clone(),
//                     issuing_time,
//                     slot_commitment_id,
//                     latest_finalized_slot,
//                     issuer_id,
//                     data,
//                     signature,
//                 })
//             }
//             _ => return Err(Error::InvalidBlockKind(kind)).map_err(UnpackError::Packable),
//         };

//         if VERIFY {
//             let block_len = if let (Some(start), Some(end)) = (start_opt, unpacker.read_bytes()) {
//                 end - start
//             } else {
//                 block.packed_len()
//             };

//             if block_len > Self::LENGTH_MAX {
//                 return Err(UnpackError::Packable(Error::InvalidBlockLength(block_len)));
//             }
//         }

//         Ok(block)
//     }
// }

impl Block {
    // /// Creates a new [`BlockBuilder`] to construct an instance of a [`BasicBlock`].
    // #[inline(always)]
    // pub fn build_basic(
    //     protocol_params: ProtocolParameters,
    //     issuing_time: u64,
    //     slot_commitment_id: SlotCommitmentId,
    //     latest_finalized_slot: SlotIndex,
    //     issuer_id: IssuerId,
    //     strong_parents: StrongParents,
    //     signature: Ed25519Signature,
    // ) -> BlockBuilder<BasicBlock> { BlockBuilder::<BasicBlock>::new( protocol_params, issuing_time,
    //   slot_commitment_id, latest_finalized_slot, issuer_id, strong_parents, signature, )
    // }

    // /// Creates a new [`BlockBuilder`] to construct an instance of a [`ValidationBlock`].
    // #[inline(always)]
    // #[allow(clippy::too_many_arguments)]
    // pub fn build_validation(
    //     protocol_params: ProtocolParameters,
    //     issuing_time: u64,
    //     slot_commitment_id: SlotCommitmentId,
    //     latest_finalized_slot: SlotIndex,
    //     issuer_id: IssuerId,
    //     strong_parents: StrongParents,
    //     highest_supported_version: u8,
    //     protocol_parameters: &ProtocolParameters,
    //     signature: Ed25519Signature,
    // ) -> BlockBuilder<ValidationBlock> { BlockBuilder::<ValidationBlock>::new( protocol_params, issuing_time,
    //   slot_commitment_id, latest_finalized_slot, issuer_id, strong_parents, highest_supported_version,
    //   protocol_parameters, signature, )
    // }

    /// Returns the strong parents of a [`BlockType`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        match self {
            Self::Basic(b) => b.strong_parents(),
            Self::Validation(b) => b.strong_parents(),
        }
    }

    /// Returns the weak parents of a [`BlockType`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        match self {
            Self::Basic(b) => b.weak_parents(),
            Self::Validation(b) => b.weak_parents(),
        }
    }

    /// Returns the shallow like parents of a [`BlockType`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        match self {
            Self::Basic(b) => b.shallow_like_parents(),
            Self::Validation(b) => b.shallow_like_parents(),
        }
    }

    /// Checks whether the block is a [`BasicBlock`].
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic(_))
    }

    /// Gets the block as an actual [`BasicBlock`].
    /// PANIC: do not call on a non-basic block.
    pub fn as_basic(&self) -> &BasicBlock {
        if let Self::Basic(block) = self {
            block
        } else {
            panic!("as_basic called on a non-basic block");
        }
    }

    /// Checks whether the block is a [`ValidationBlock`].
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Gets the block as an actual [`ValidationBlock`].
    /// PANIC: do not call on a non-validation block.
    pub fn as_validation(&self) -> &ValidationBlock {
        if let Self::Validation(block) = self {
            block
        } else {
            panic!("as_validation called on a non-validation block");
        }
    }
}

pub(crate) fn verify_parents(
    strong_parents: &StrongParents,
    weak_parents: &WeakParents,
    shallow_like_parents: &ShallowLikeParents,
) -> Result<(), Error> {
    let (strong_parents, weak_parents, shallow_like_parents) = (
        strong_parents.to_set(),
        weak_parents.to_set(),
        shallow_like_parents.to_set(),
    );
    if !weak_parents.is_disjoint(&strong_parents) || !weak_parents.is_disjoint(&shallow_like_parents) {
        return Err(Error::NonDisjointParents);
    }
    Ok(())
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use super::*;
    use crate::types::block::core::{basic::dto::BasicBlockDto, validation::dto::ValidationBlockDto};

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum BlockDto {
        Basic(BasicBlockDto),
        Validation(ValidationBlockDto),
    }

    impl From<&BasicBlock> for BlockDto {
        fn from(value: &BasicBlock) -> Self {
            Self::Basic(value.into())
        }
    }

    impl From<&ValidationBlock> for BlockDto {
        fn from(value: &ValidationBlock) -> Self {
            Self::Validation(value.into())
        }
    }

    impl<'de> Deserialize<'de> for BlockDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid block type"))? as u8
                {
                    BasicBlock::KIND => Self::Basic(
                        BasicBlockDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize basic block: {e}")))?,
                    ),
                    ValidationBlock::KIND => {
                        Self::Validation(ValidationBlockDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize validation block: {e}"))
                        })?)
                    }
                    _ => return Err(serde::de::Error::custom("invalid block type")),
                },
            )
        }
    }

    impl Serialize for BlockDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum BlockTypeDto_<'a> {
                T0(&'a BasicBlockDto),
                T1(&'a ValidationBlockDto),
            }
            #[derive(Serialize)]
            struct TypedBlock<'a> {
                #[serde(flatten)]
                kind: BlockTypeDto_<'a>,
            }
            let output = match self {
                Self::Basic(b) => TypedBlock {
                    kind: BlockTypeDto_::T0(b),
                },
                Self::Validation(b) => TypedBlock {
                    kind: BlockTypeDto_::T1(b),
                },
            };
            output.serialize(serializer)
        }
    }

    // impl Block {
    //     pub fn try_from_dto(dto: BlockDto, protocol_params: ProtocolParameters) -> Result<Self, Error> {
    //         if dto.protocol_version != protocol_params.version() {
    //             return Err(Error::ProtocolVersionMismatch {
    //                 expected: protocol_params.version(),
    //                 actual: dto.protocol_version,
    //             });
    //         }

    //         if dto.network_id != protocol_params.network_id() {
    //             return Err(Error::NetworkIdMismatch {
    //                 expected: protocol_params.network_id(),
    //                 actual: dto.network_id,
    //             });
    //         }

    //         match dto.block {
    //             BlockDto::Basic(b) => {
    //                 let data = BasicBlock::try_from_dto_with_params(b, &protocol_params)?;
    //                 BlockBuilder::from_block_data(
    //                     protocol_params,
    //                     dto.issuing_time,
    //                     dto.slot_commitment,
    //                     dto.latest_finalized_slot,
    //                     dto.issuer_id,
    //                     data,
    //                     *dto.signature.as_ed25519(),
    //                 )
    //                 .finish()
    //             }
    //             BlockDto::Validation(b) => {
    //                 let data = ValidationBlock::try_from_dto_with_params(b, &protocol_params)?;
    //                 BlockBuilder::from_block_data(
    //                     protocol_params,
    //                     dto.issuing_time,
    //                     dto.slot_commitment,
    //                     dto.latest_finalized_slot,
    //                     dto.issuer_id,
    //                     data,
    //                     *dto.signature.as_ed25519(),
    //                 )
    //                 .finish()
    //             }
    //         }
    //     }
    // }
}
