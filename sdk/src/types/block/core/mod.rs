// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod basic;
mod validation;
mod wrapper;

use alloc::boxed::Box;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

pub use self::{
    basic::{BasicBlock, BasicBlockBuilder},
    validation::{ValidationBlock, ValidationBlockBuilder},
    wrapper::BlockWrapper,
};
use crate::types::block::{
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    protocol::ProtocolParameters,
    Error,
};

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

impl Block {
    /// Creates a new [`BasicBlockBuilder`].
    #[inline(always)]
    pub fn build_basic(strong_parents: StrongParents) -> BasicBlockBuilder {
        BasicBlockBuilder::new(strong_parents)
    }

    /// Creates a new [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn build_validation(
        strong_parents: StrongParents,
        highest_supported_version: u8,
        protocol_parameters: &ProtocolParameters,
    ) -> ValidationBlockBuilder {
        ValidationBlockBuilder::new(strong_parents, highest_supported_version, protocol_parameters)
    }

    /// Returns the strong parents of a [`Block`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        match self {
            Self::Basic(b) => b.strong_parents(),
            Self::Validation(b) => b.strong_parents(),
        }
    }

    /// Returns the weak parents of a [`Block`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        match self {
            Self::Basic(block) => block.weak_parents(),
            Self::Validation(block) => block.weak_parents(),
        }
    }

    /// Returns the shallow like parents of a [`Block`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        match self {
            Self::Basic(block) => block.shallow_like_parents(),
            Self::Validation(block) => block.shallow_like_parents(),
        }
    }

    /// Checks whether the block is a [`BasicBlock`].
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic(_))
    }

    /// Gets the block as an actual [`BasicBlock`].
    /// NOTE: Will panic if the block is not a [`BasicBlock`].
    pub fn as_basic(&self) -> &BasicBlock {
        if let Self::Basic(block) = self {
            block
        } else {
            panic!("invalid downcast of non-BasicBlock");
        }
    }

    /// Checks whether the block is a [`ValidationBlock`].
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Gets the block as an actual [`ValidationBlock`].
    /// NOTE: Will panic if the block is not a [`BasicBlock`].
    pub fn as_validation(&self) -> &ValidationBlock {
        if let Self::Validation(block) = self {
            block
        } else {
            panic!("invalid downcast of non-ValidationBlock");
        }
    }

    pub(crate) fn hash(&self) -> [u8; 32] {
        Blake2b256::digest(self.pack_to_vec()).into()
    }
}

impl Packable for Block {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Basic(block) => {
                BasicBlock::KIND.pack(packer)?;
                block.pack(packer)
            }
            Self::Validation(block) => {
                ValidationBlock::KIND.pack(packer)?;
                block.pack(packer)
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
            k => return Err(Error::InvalidBlockKind(k)).map_err(UnpackError::Packable),
        })
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
            let block = match self {
                Self::Basic(b) => TypedBlock {
                    kind: BlockTypeDto_::T0(b),
                },
                Self::Validation(b) => TypedBlock {
                    kind: BlockTypeDto_::T1(b),
                },
            };
            block.serialize(serializer)
        }
    }
}
