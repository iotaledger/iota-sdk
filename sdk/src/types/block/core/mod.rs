// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod basic;
mod parent;
mod signed_block;
pub mod validation;

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
    basic::{BasicBlockBody, BasicBlockBodyBuilder},
    parent::Parents,
    signed_block::{BlockHeader, SignedBlock, UnsignedBlock},
    validation::{ValidationBlockBody, ValidationBlockBodyBuilder},
};
use crate::types::block::{
    protocol::{ProtocolParameters, ProtocolParametersHash},
    Error,
};

#[derive(Clone, Debug, Eq, PartialEq, From)]
pub enum Block {
    Basic(Box<BasicBlockBody>),
    Validation(Box<ValidationBlockBody>),
}

impl From<BasicBlockBody> for Block {
    fn from(value: BasicBlockBody) -> Self {
        Self::Basic(value.into())
    }
}

impl From<ValidationBlockBody> for Block {
    fn from(value: ValidationBlockBody) -> Self {
        Self::Validation(value.into())
    }
}

impl TryFrom<Block> for BasicBlockBodyBuilder {
    type Error = Error;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        if let Block::Basic(block) = value {
            Ok((*block).into())
        } else {
            Err(Error::InvalidBlockKind(value.kind()))
        }
    }
}

impl TryFrom<Block> for ValidationBlockBodyBuilder {
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
            Self::Basic(_) => BasicBlockBody::KIND,
            Self::Validation(_) => ValidationBlockBody::KIND,
        }
    }

    /// Creates a new [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn build_basic(strong_parents: self::basic::StrongParents, max_burned_mana: u64) -> BasicBlockBodyBuilder {
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

impl Packable for Block {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Basic(block_body) => {
                BasicBlockBody::KIND.pack(packer)?;
                block_body.pack(packer)
            }
            Self::Validation(block_body) => {
                ValidationBlockBody::KIND.pack(packer)?;
                block_body.pack(packer)
            }
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            BasicBlockBody::KIND => Self::from(BasicBlockBody::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            ValidationBlockBody::KIND => {
                Self::from(ValidationBlockBody::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            }
            k => return Err(UnpackError::Packable(Error::InvalidBlockKind(k))),
        })
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
        block::core::{basic::dto::BasicBlockBodyDto, validation::dto::ValidationBlockBodyDto},
        TryFromDto, ValidationParams,
    };

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum BlockDto {
        Basic(BasicBlockBodyDto),
        Validation(ValidationBlockBodyDto),
    }

    impl From<&BasicBlockBody> for BlockDto {
        fn from(value: &BasicBlockBody) -> Self {
            Self::Basic(value.into())
        }
    }

    impl From<&ValidationBlockBody> for BlockDto {
        fn from(value: &ValidationBlockBody) -> Self {
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
                    .ok_or_else(|| serde::de::Error::custom("invalid block body type"))? as u8
                {
                    BasicBlockBody::KIND => Self::Basic(
                        BasicBlockBodyDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize basic block body: {e}")))?,
                    ),
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

    impl Serialize for BlockDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum BlockTypeDto_<'a> {
                T0(&'a BasicBlockBodyDto),
                T1(&'a ValidationBlockBodyDto),
            }
            #[derive(Serialize)]
            struct TypedBlock<'a> {
                #[serde(flatten)]
                kind: BlockTypeDto_<'a>,
            }
            let block = match self {
                Self::Basic(basic_block_body) => TypedBlock {
                    kind: BlockTypeDto_::T0(basic_block_body),
                },
                Self::Validation(validation_block_body) => TypedBlock {
                    kind: BlockTypeDto_::T1(validation_block_body),
                },
            };
            block.serialize(serializer)
        }
    }

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            match value {
                Block::Basic(basic_block_body) => BasicBlockBodyDto::from(&**basic_block_body).into(),
                Block::Validation(validation_block_body) => ValidationBlockBodyDto::from(validation_block_body.as_ref()).into(),
            }
        }
    }

    impl TryFromDto for Block {
        type Dto = BlockDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            match dto {
                Self::Dto::Basic(basic_block_body) => Ok(BasicBlockBody::try_from_dto_with_params_inner(basic_block_body, params)?.into()),
                Self::Dto::Validation(validation_block_body) => {
                    Ok(ValidationBlockBody::try_from_dto_with_params_inner(validation_block_body, params)?.into())
                }
            }
        }
    }
}
