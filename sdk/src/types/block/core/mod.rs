// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod basic;
mod parent;
mod signed_block;
pub mod validation;

use alloc::boxed::Box;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::{Packable, PackableExt};

pub use self::{
    basic::{BasicBlock, BasicBlockBuilder},
    parent::Parents,
    signed_block::{BlockHeader, SignedBlock, UnsignedBlock},
    validation::{ValidationBlock, ValidationBlockBuilder},
};
use crate::types::block::{
    protocol::{ProtocolParameters, ProtocolParametersHash},
    Error,
};

#[derive(Clone, Debug, Eq, PartialEq, From, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = Error::InvalidBlockKind)]
pub enum Block {
    #[packable(tag = BasicBlock::KIND)]
    Basic(Box<BasicBlock>),
    #[packable(tag = ValidationBlock::KIND)]
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

impl TryFrom<Block> for BasicBlockBuilder {
    type Error = Error;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        if let Block::Basic(block) = value {
            Ok((*block).into())
        } else {
            Err(Error::InvalidBlockKind(value.kind()))
        }
    }
}

impl TryFrom<Block> for ValidationBlockBuilder {
    type Error = Error;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        if let Block::Validation(block) = value {
            Ok((*block).into())
        } else {
            Err(Error::InvalidBlockKind(value.kind()))
        }
    }
}

impl Block {
    /// Return the block kind of a [`Block`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Basic(_) => BasicBlock::KIND,
            Self::Validation(_) => ValidationBlock::KIND,
        }
    }

    /// Creates a new [`BasicBlockBuilder`].
    #[inline(always)]
    pub fn build_basic(strong_parents: self::basic::StrongParents, max_burned_mana: u64) -> BasicBlockBuilder {
        BasicBlockBuilder::new(strong_parents, max_burned_mana)
    }

    /// Creates a new [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn build_validation(
        strong_parents: self::validation::StrongParents,
        highest_supported_version: u8,
        protocol_parameters_hash: ProtocolParametersHash,
    ) -> ValidationBlockBuilder {
        ValidationBlockBuilder::new(strong_parents, highest_supported_version, protocol_parameters_hash)
    }

    crate::def_is_as_opt!(Block: Basic, Validation);

    pub(crate) fn hash(&self) -> [u8; 32] {
        Blake2b256::digest(self.pack_to_vec()).into()
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use super::*;
    pub use crate::types::block::core::signed_block::dto::{SignedBlockDto, UnsignedBlockDto};
    use crate::types::{
        block::core::{basic::dto::BasicBlockDto, validation::dto::ValidationBlockDto},
        TryFromDto,
    };

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

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            match value {
                Block::Basic(basic) => BasicBlockDto::from(&**basic).into(),
                Block::Validation(validation) => ValidationBlockDto::from(validation.as_ref()).into(),
            }
        }
    }

    impl TryFromDto<BlockDto> for Block {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: BlockDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            match dto {
                BlockDto::Basic(basic) => Ok(BasicBlock::try_from_dto_with_params_inner(basic, params)?.into()),
                BlockDto::Validation(validation) => {
                    Ok(ValidationBlock::try_from_dto_with_params_inner(validation, params)?.into())
                }
            }
        }
    }
}
