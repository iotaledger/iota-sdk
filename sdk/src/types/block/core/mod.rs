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
    protocol::{ProtocolParameters, ProtocolParametersHash},
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
    /// Return the block kind of a [`Block`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Basic(_) => BasicBlock::KIND,
            Self::Validation(_) => ValidationBlock::KIND,
        }
    }

    /// Creates a new [`BasicBlockBuilder`].
    #[inline(always)]
    pub fn build_basic(strong_parents: StrongParents, burned_mana: u64) -> BasicBlockBuilder {
        BasicBlockBuilder::new(strong_parents, burned_mana)
    }

    /// Creates a new [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn build_validation(
        strong_parents: StrongParents,
        highest_supported_version: u8,
        protocol_parameters_hash: ProtocolParametersHash,
    ) -> ValidationBlockBuilder {
        ValidationBlockBuilder::new(strong_parents, highest_supported_version, protocol_parameters_hash)
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
    /// NOTE: Will panic if the block is not a [`ValidationBlock`].
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
        strong_parents.as_set().clone(),
        weak_parents.as_set().clone(),
        shallow_like_parents.as_set().clone(),
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
    pub use crate::types::block::core::wrapper::dto::BlockWrapperDto;
    use crate::types::{
        block::core::{basic::dto::BasicBlockDto, validation::dto::ValidationBlockDto},
        TryFromDto, ValidationParams,
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

    impl TryFromDto for Block {
        type Dto = BlockDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            match dto {
                Self::Dto::Basic(basic) => Ok(BasicBlock::try_from_dto_with_params_inner(basic, params)?.into()),
                Self::Dto::Validation(validation) => {
                    Ok(ValidationBlock::try_from_dto_with_params_inner(validation, params)?.into())
                }
            }
        }
    }
}
