// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod basic;
mod block;
mod parent;
pub mod validation;

use alloc::boxed::Box;
use core::convert::Infallible;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::{Packable, PackableExt};

pub use self::{
    basic::{BasicBlockBody, BasicBlockBodyBuilder},
    block::{Block, BlockHeader, UnsignedBlock},
    parent::Parents,
    validation::{ValidationBlockBody, ValidationBlockBodyBuilder},
};
use crate::types::block::{
    context_input::ContextInputError,
    core::basic::MaxBurnedManaAmount,
    input::InputError,
    mana::ManaError,
    output::{
        feature::FeatureError, unlock_condition::UnlockConditionError, NativeTokenError, OutputError, TokenSchemeError,
    },
    payload::PayloadError,
    protocol::{ProtocolParameters, ProtocolParametersHash},
    semantic::SemanticError,
    signature::SignatureError,
    unlock::UnlockError,
    IdentifierError,
};

#[derive(Debug, PartialEq, Eq, strum::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum BlockError {
    #[strum(to_string = "invalid block body kind: {0}")]
    InvalidBlockBodyKind(u8),
    #[strum(to_string = "invalid block length {length}")]
    InvalidBlockLength(usize),
    #[strum(to_string = "remaining bytes after block")]
    RemainingBytesAfterBlock,
    #[strum(to_string = "invalid parent count")]
    InvalidParentCount,
    #[strum(to_string = "weak parents are not disjoint to strong or shallow like parents")]
    NonDisjointParents,
    #[strum(to_string = "parents are not unique and/or sorted")]
    ParentsNotUniqueSorted,
    #[strum(to_string = "network ID mismatch: expected {expected} but got {actual}")]
    NetworkIdMismatch { expected: u64, actual: u64 },
    #[strum(to_string = "protocol version mismatch: expected {expected} but got {actual}")]
    ProtocolVersionMismatch { expected: u8, actual: u8 },
    #[strum(to_string = "invalid protocol parameters hash: expected {expected} but got {actual}")]
    InvalidProtocolParametersHash {
        expected: ProtocolParametersHash,
        actual: ProtocolParametersHash,
    },
    #[strum(to_string = "unsupported address kind: {0}")]
    UnsupportedAddressKind(u8),
    #[from]
    #[strum(to_string = "{0}")]
    Payload(PayloadError),
    #[from]
    #[strum(to_string = "{0}")]
    Signature(SignatureError),
    #[from]
    #[strum(to_string = "{0}")]
    Identifier(IdentifierError),
    #[from]
    #[strum(to_string = "{0}")]
    Semantic(SemanticError),
}

#[cfg(feature = "std")]
impl std::error::Error for BlockError {}

macro_rules! impl_from_error_via {
    ($via:ident: $($err:ident),+$(,)?) => {
        $(
        impl From<$err> for BlockError {
            fn from(error: $err) -> Self {
                Self::from($via::from(error))
            }
        }
        )+
    };
}
impl_from_error_via!(PayloadError:
    UnlockError,
    ContextInputError,
    NativeTokenError,
    ManaError,
    UnlockConditionError,
    FeatureError,
    TokenSchemeError,
    InputError,
    OutputError
);

impl From<Infallible> for BlockError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, From, Packable)]
#[packable(unpack_error = BlockError)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = BlockError::InvalidBlockBodyKind)]
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
            Err(BlockError::InvalidBlockBodyKind(value.kind()))
        }
    }
}

impl TryFrom<BlockBody> for ValidationBlockBodyBuilder {
    type Error = BlockError;

    fn try_from(value: BlockBody) -> Result<Self, Self::Error> {
        if let BlockBody::Validation(block) = value {
            Ok((*block).into())
        } else {
            Err(BlockError::InvalidBlockBodyKind(value.kind()))
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
