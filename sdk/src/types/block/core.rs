// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::{
    error::{UnexpectedEOF, UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use super::{basic::BasicBlock, validation::ValidationBlock};
use crate::types::block::{
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    payload::{OptionalPayload, Payload},
    protocol::ProtocolParameters,
    BlockId, Error, PROTOCOL_VERSION,
};

/// A builder to build a [`Block`].
#[derive(Clone)]
#[must_use]
pub struct BlockBuilder<B> {
    protocol_version: Option<u8>,
    inner: B,
}

impl<B> BlockBuilder<B> {
    /// Adds a protocol version to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_protocol_version(mut self, protocol_version: impl Into<Option<u8>>) -> Self {
        self.protocol_version = protocol_version.into();
        self
    }
}

impl BlockBuilder<BasicBlock> {
    /// Creates a new [`BlockBuilder`] for a [`BasicBlock`].
    #[inline(always)]
    pub fn new(strong_parents: StrongParents) -> Self {
        Self {
            protocol_version: None,
            inner: BasicBlock {
                strong_parents,
                weak_parents: Default::default(),
                shallow_like_parents: Default::default(),
                payload: OptionalPayload::default(),
                burned_mana: Default::default(),
            },
        }
    }

    /// Adds weak parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.inner.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.inner.shallow_like_parents = shallow_like_parents.into();
        self
    }

    /// Adds a payload to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.inner.payload = payload.into();
        self
    }

    /// Adds burned mana to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_burned_mana(mut self, burned_mana: u64) -> Self {
        self.inner.burned_mana = burned_mana;
        self
    }
}

impl BlockBuilder<ValidationBlock> {
    /// Creates a new [`BlockBuilder`] for a [`ValidationBlock`].
    #[inline(always)]
    pub fn new(
        strong_parents: StrongParents,
        highest_supported_version: u8,
        protocol_parameters: &ProtocolParameters,
    ) -> Self {
        Self {
            protocol_version: None,
            inner: ValidationBlock {
                strong_parents,
                weak_parents: Default::default(),
                shallow_like_parents: Default::default(),
                highest_supported_version,
                protocol_parameters_hash: protocol_parameters.hash(),
            },
        }
    }

    /// Adds weak parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.inner.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.inner.shallow_like_parents = shallow_like_parents.into();
        self
    }
}

impl<B: Into<BlockType>> From<B> for BlockBuilder<B> {
    fn from(inner: B) -> Self {
        Self {
            protocol_version: None,
            inner,
        }
    }
}

impl<B: Into<BlockType>> BlockBuilder<B> {
    fn _finish(self) -> Result<(Block, Vec<u8>), Error> {
        let inner = self.inner.into();
        verify_parents(
            inner.strong_parents(),
            inner.weak_parents(),
            inner.shallow_like_parents(),
        )?;

        let block = Block {
            protocol_version: self.protocol_version.unwrap_or(PROTOCOL_VERSION),
            inner,
        };

        let block_bytes = block.pack_to_vec();

        if block_bytes.len() > Block::LENGTH_MAX {
            return Err(Error::InvalidBlockLength(block_bytes.len()));
        }

        Ok((block, block_bytes))
    }

    /// Finishes the [`BlockBuilder`] into a [`Block`].
    pub fn finish(self) -> Result<Block, Error> {
        self._finish().map(|res| res.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, From)]
pub enum BlockType {
    Basic(BasicBlock),
    Validation(ValidationBlock),
}

impl Packable for BlockType {
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
            k => return Err(Error::InvalidOutputKind(k)).map_err(UnpackError::Packable),
        })
    }
}

impl BlockType {
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

    /// Returns the optional payload of a [`Block`].
    #[inline(always)]
    pub fn payload(&self) -> Option<&Payload> {
        match self {
            Self::Basic(b) => b.payload(),
            Self::Validation(_) => None,
        }
    }
}

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    /// Protocol version of the block.
    protocol_version: u8,
    pub(crate) inner: BlockType,
}

impl Block {
    /// The minimum number of bytes in a block.
    pub const LENGTH_MIN: usize = 46;
    /// The maximum number of bytes in a block.
    pub const LENGTH_MAX: usize = 32768;

    /// Creates a new [`BlockBuilder`] to construct an instance of a [`BasicBlock`].
    #[inline(always)]
    pub fn build_basic(strong_parents: StrongParents) -> BlockBuilder<BasicBlock> {
        BlockBuilder::<BasicBlock>::new(strong_parents)
    }

    /// Creates a new [`BlockBuilder`] to construct an instance of a [`ValidationBlock`].
    #[inline(always)]
    pub fn build_validation(
        strong_parents: StrongParents,
        highest_supported_version: u8,
        protocol_parameters: &ProtocolParameters,
    ) -> BlockBuilder<ValidationBlock> {
        BlockBuilder::<ValidationBlock>::new(strong_parents, highest_supported_version, protocol_parameters)
    }

    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the strong parents of a [`Block`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        self.inner.strong_parents()
    }

    /// Returns the weak parents of a [`Block`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        self.inner.weak_parents()
    }

    /// Returns the shallow like parents of a [`Block`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        self.inner.shallow_like_parents()
    }

    /// Returns the optional payload of a [`Block`].
    #[inline(always)]
    pub fn payload(&self) -> Option<&Payload> {
        self.inner.payload()
    }

    /// Returns the inner block type of a [`Block`].
    #[inline(always)]
    pub fn inner(&self) -> &BlockType {
        &self.inner
    }

    /// Computes the identifier of the block.
    #[inline(always)]
    pub fn id(&self) -> BlockId {
        BlockId::new(Blake2b256::digest(self.pack_to_vec()).into())
    }

    /// Unpacks a [`Block`] from a sequence of bytes doing syntactical checks and verifying that
    /// there are no trailing bytes in the sequence.
    pub fn unpack_strict<T: AsRef<[u8]>>(
        bytes: T,
        visitor: &<Self as Packable>::UnpackVisitor,
    ) -> Result<Self, UnpackError<<Self as Packable>::UnpackError, UnexpectedEOF>> {
        let mut unpacker = CounterUnpacker::new(SliceUnpacker::new(bytes.as_ref()));
        let block = Self::unpack::<_, true>(&mut unpacker, visitor)?;

        // When parsing the block is complete, there should not be any trailing bytes left that were not parsed.
        if u8::unpack::<_, true>(&mut unpacker, &()).is_ok() {
            return Err(UnpackError::Packable(Error::RemainingBytesAfterBlock));
        }

        Ok(block)
    }
}

impl Packable for Block {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.protocol_version.pack(packer)?;
        self.inner.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let start_opt = unpacker.read_bytes();

        let protocol_version = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        if VERIFY && protocol_version != visitor.protocol_version() {
            return Err(UnpackError::Packable(Error::ProtocolVersionMismatch {
                expected: visitor.protocol_version(),
                actual: protocol_version,
            }));
        }

        let inner = BlockType::unpack::<_, VERIFY>(unpacker, visitor)?;

        let block = Self {
            protocol_version,
            inner,
        };

        if VERIFY {
            let block_len = if let (Some(start), Some(end)) = (start_opt, unpacker.read_bytes()) {
                end - start
            } else {
                block.packed_len()
            };

            if block_len > Self::LENGTH_MAX {
                return Err(UnpackError::Packable(Error::InvalidBlockLength(block_len)));
            }
        }

        Ok(block)
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

pub(crate) mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use super::*;
    use crate::types::{
        block::{basic::dto::BasicBlockDto, validation::dto::ValidationBlockDto, Error},
        TryFromDto, ValidationParams,
    };

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum BlockTypeDto {
        Basic(BasicBlockDto),
        Validation(ValidationBlockDto),
    }

    impl From<&BlockType> for BlockTypeDto {
        fn from(value: &BlockType) -> Self {
            match value {
                BlockType::Basic(b) => Self::Basic(b.into()),
                BlockType::Validation(b) => Self::Validation(b.into()),
            }
        }
    }

    impl<'de> Deserialize<'de> for BlockTypeDto {
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

    impl Serialize for BlockTypeDto {
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

    /// The block object that nodes gossip around in the network.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockDto {
        pub protocol_version: u8,
        pub inner: BlockTypeDto,
    }

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            Self {
                protocol_version: value.protocol_version(),
                inner: value.inner().into(),
            }
        }
    }

    impl TryFromDto for Block {
        type Dto = BlockDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let inner = BlockType::try_from_dto_with_params_inner(dto.inner, params)?;
            BlockBuilder::from(inner)
                .with_protocol_version(dto.protocol_version)
                .finish()
        }
    }

    impl TryFromDto for BlockType {
        type Dto = BlockTypeDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            Ok(match dto {
                BlockTypeDto::Basic(b) => Self::Basic(BasicBlock::try_from_dto_with_params(b, &params)?),
                BlockTypeDto::Validation(b) => Self::Validation(ValidationBlock::try_from_dto_with_params(b, &params)?),
            })
        }
    }
}
