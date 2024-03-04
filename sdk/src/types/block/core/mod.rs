// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod basic;
mod block;
mod error;
mod parent;
pub mod validation;

use alloc::boxed::Box;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::{Packable, PackableExt};

pub use self::{
    basic::{BasicBlockBody, BasicBlockBodyBuilder},
    block::{Block, BlockHeader, UnsignedBlock},
    error::BlockError,
    parent::Parents,
    validation::{ValidationBlockBody, ValidationBlockBodyBuilder},
};
use crate::types::block::{
    core::basic::MaxBurnedManaAmount,
    protocol::{ProtocolParameters, ProtocolParametersHash},
};

#[derive(Clone, Debug, Eq, PartialEq, From, Packable)]
#[packable(unpack_error = BlockError)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = BlockError::Kind)]
pub enum BlockBody {
    #[packable(tag = BasicBlockBody::KIND)]
    Basic(Box<BasicBlockBody>),
    #[packable(tag = ValidationBlockBody::KIND)]
    Validation(Box<ValidationBlockBody>),
}

impl From<BasicBlockBody> for BlockBody {
    fn from(value: BasicBlockBody) -> Self {
        Self::Basic(value.into())
    }
}

impl From<ValidationBlockBody> for BlockBody {
    fn from(value: ValidationBlockBody) -> Self {
        Self::Validation(value.into())
    }
}

impl TryFrom<BlockBody> for BasicBlockBodyBuilder {
    type Error = BlockError;

    fn try_from(value: BlockBody) -> Result<Self, Self::Error> {
        if let BlockBody::Basic(block) = value {
            Ok((*block).into())
        } else {
            Err(BlockError::Kind(value.kind()))
        }
    }
}

impl TryFrom<BlockBody> for ValidationBlockBodyBuilder {
    type Error = BlockError;

    fn try_from(value: BlockBody) -> Result<Self, Self::Error> {
        if let BlockBody::Validation(block) = value {
            Ok((*block).into())
        } else {
            Err(BlockError::Kind(value.kind()))
        }
    }
}

impl BlockBody {
    /// Return the block body kind of a [`BlockBody`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Basic(_) => BasicBlockBody::KIND,
            Self::Validation(_) => ValidationBlockBody::KIND,
        }
    }

    /// Creates a new [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn build_basic(
        strong_parents: self::basic::StrongParents,
        max_burned_mana: impl Into<MaxBurnedManaAmount>,
    ) -> BasicBlockBodyBuilder {
        BasicBlockBodyBuilder::new(strong_parents, max_burned_mana)
    }

    /// Creates a new [`ValidationBlockBodyBuilder`].
    #[inline(always)]
    pub fn build_validation(
        strong_parents: self::validation::StrongParents,
        highest_supported_version: u8,
        protocol_parameters_hash: ProtocolParametersHash,
    ) -> ValidationBlockBodyBuilder {
        ValidationBlockBodyBuilder::new(strong_parents, highest_supported_version, protocol_parameters_hash)
    }

    crate::def_is_as_opt!(BlockBody: Basic, Validation);

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
    pub use crate::types::block::core::block::dto::{BlockDto, UnsignedBlockDto};
    use crate::types::{
        block::core::{basic::dto::BasicBlockBodyDto, validation::dto::ValidationBlockBodyDto},
        TryFromDto,
    };

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum BlockBodyDto {
        Basic(BasicBlockBodyDto),
        Validation(ValidationBlockBodyDto),
    }

    impl From<&BasicBlockBody> for BlockBodyDto {
        fn from(value: &BasicBlockBody) -> Self {
            Self::Basic(value.into())
        }
    }

    impl From<&ValidationBlockBody> for BlockBodyDto {
        fn from(value: &ValidationBlockBody) -> Self {
            Self::Validation(value.into())
        }
    }

    impl<'de> Deserialize<'de> for BlockBodyDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid block body type"))? as u8
                {
                    BasicBlockBody::KIND => {
                        Self::Basic(BasicBlockBodyDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize basic block body: {e}"))
                        })?)
                    }
                    ValidationBlockBody::KIND => {
                        Self::Validation(ValidationBlockBodyDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize validation block body: {e}"))
                        })?)
                    }
                    _ => return Err(serde::de::Error::custom("invalid block body type")),
                },
            )
        }
    }

    impl Serialize for BlockBodyDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum BlockBodyTypeDto_<'a> {
                T0(&'a BasicBlockBodyDto),
                T1(&'a ValidationBlockBodyDto),
            }
            #[derive(Serialize)]
            struct TypedBlockBody<'a> {
                #[serde(flatten)]
                kind: BlockBodyTypeDto_<'a>,
            }
            let block_body = match self {
                Self::Basic(basic_block_body) => TypedBlockBody {
                    kind: BlockBodyTypeDto_::T0(basic_block_body),
                },
                Self::Validation(validation_block_body) => TypedBlockBody {
                    kind: BlockBodyTypeDto_::T1(validation_block_body),
                },
            };
            block_body.serialize(serializer)
        }
    }

    impl From<&BlockBody> for BlockBodyDto {
        fn from(value: &BlockBody) -> Self {
            match value {
                BlockBody::Basic(basic_block_body) => BasicBlockBodyDto::from(&**basic_block_body).into(),
                BlockBody::Validation(validation_block_body) => {
                    ValidationBlockBodyDto::from(validation_block_body.as_ref()).into()
                }
            }
        }
    }

    impl TryFromDto<BlockBodyDto> for BlockBody {
        type Error = BlockError;

        fn try_from_dto_with_params_inner(
            dto: BlockBodyDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            match dto {
                BlockBodyDto::Basic(dto) => Ok(BasicBlockBody::try_from_dto_with_params_inner(dto, params)?.into()),
                BlockBodyDto::Validation(dto) => {
                    Ok(ValidationBlockBody::try_from_dto_with_params_inner(dto, params)?.into())
                }
            }
        }
    }
}
