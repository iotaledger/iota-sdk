// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{
    error::{UnexpectedEOF, UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use crate::types::block::{
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    payload::{OptionalPayload, Payload},
    protocol::ProtocolParameters,
    BlockId, Error, PROTOCOL_VERSION,
};

/// A builder to build a [`Block`].
#[derive(Clone)]
#[must_use]
pub struct BlockBuilder {
    protocol_version: Option<u8>,
    strong_parents: StrongParents,
    weak_parents: WeakParents,
    shallow_like_parents: ShallowLikeParents,
    payload: OptionalPayload,
    burned_mana: u64,
}

impl BlockBuilder {
    /// Creates a new [`BlockBuilder`].
    #[inline(always)]
    pub fn new(strong_parents: StrongParents) -> Self {
        Self {
            protocol_version: None,
            strong_parents,
            weak_parents: Default::default(),
            shallow_like_parents: Default::default(),
            payload: OptionalPayload::default(),
            burned_mana: Default::default(),
        }
    }

    /// Adds a protocol version to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_protocol_version(mut self, protocol_version: impl Into<Option<u8>>) -> Self {
        self.protocol_version = protocol_version.into();
        self
    }

    /// Adds weak parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.shallow_like_parents = shallow_like_parents.into();
        self
    }

    /// Adds a payload to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Adds burned mana to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_burned_mana(mut self, burned_mana: u64) -> Self {
        self.burned_mana = burned_mana;
        self
    }

    pub fn finish(self) -> Result<Block, Error> {
        verify_parents(&self.strong_parents, &self.weak_parents, &self.shallow_like_parents)?;

        let block = Block {
            protocol_version: self.protocol_version.unwrap_or(PROTOCOL_VERSION),
            strong_parents: self.strong_parents,
            weak_parents: self.weak_parents,
            shallow_like_parents: self.shallow_like_parents,
            payload: self.payload,
            burned_mana: self.burned_mana,
        };

        let block_bytes = block.pack_to_vec();

        if block_bytes.len() > Block::LENGTH_MAX {
            return Err(Error::InvalidBlockLength(block_bytes.len()));
        }

        Ok(block)
    }
}

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    /// Protocol version of the block.
    protocol_version: u8,
    /// Blocks that are strongly directly approved.
    strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    shallow_like_parents: ShallowLikeParents,
    /// The optional [Payload] of the block.
    payload: OptionalPayload,
    /// The amount of mana the Account identified by [`IssuerId`](super::IssuerId) is at most
    /// willing to burn for this block.
    burned_mana: u64,
}

impl Block {
    /// The minimum number of bytes in a block.
    pub const LENGTH_MIN: usize = 46;
    /// The maximum number of bytes in a block.
    pub const LENGTH_MAX: usize = 32768;

    /// Creates a new [`BlockBuilder`] to construct an instance of a [`Block`].
    #[inline(always)]
    pub fn build(strong_parents: StrongParents) -> BlockBuilder {
        BlockBuilder::new(strong_parents)
    }

    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the strong parents of a [`Block`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        &self.strong_parents
    }

    /// Returns the weak parents of a [`Block`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        &self.weak_parents
    }

    /// Returns the shallow like parents of a [`Block`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        &self.shallow_like_parents
    }

    /// Returns the optional payload of a [`Block`].
    #[inline(always)]
    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }

    /// Returns the burned mana of a [`Block`].
    #[inline(always)]
    pub fn burned_mana(&self) -> u64 {
        self.burned_mana
    }

    /// Computes the identifier of the block.
    #[inline(always)]
    pub fn id(&self) -> BlockId {
        BlockId::new(Blake2b256::digest(self.pack_to_vec()).into())
    }

    /// Consumes the [`Block`], and returns ownership over its [`StrongParents`].
    #[inline(always)]
    pub fn into_strong_parents(self) -> StrongParents {
        self.strong_parents
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
        self.strong_parents.pack(packer)?;
        self.weak_parents.pack(packer)?;
        self.shallow_like_parents.pack(packer)?;
        self.payload.pack(packer)?;
        self.burned_mana.pack(packer)?;

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

        let strong_parents = StrongParents::unpack::<_, VERIFY>(unpacker, &())?;
        let weak_parents = WeakParents::unpack::<_, VERIFY>(unpacker, &())?;
        let shallow_like_parents = ShallowLikeParents::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_parents(&strong_parents, &weak_parents, &shallow_like_parents).map_err(UnpackError::Packable)?;
        }

        let payload = OptionalPayload::unpack::<_, VERIFY>(unpacker, visitor)?;

        let burned_mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let block = Self {
            protocol_version,
            strong_parents,
            weak_parents,
            shallow_like_parents,
            payload,
            burned_mana,
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

fn verify_parents(
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
    use alloc::collections::BTreeSet;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{
        block::{payload::dto::PayloadDto, Error},
        TryFromDto, ValidationParams,
    };

    /// The block object that nodes gossip around in the network.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockDto {
        ///
        pub protocol_version: u8,
        ///
        pub strong_parents: BTreeSet<BlockId>,
        pub weak_parents: BTreeSet<BlockId>,
        pub shallow_like_parents: BTreeSet<BlockId>,
        ///
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
        #[serde(with = "crate::utils::serde::string")]
        pub burned_mana: u64,
    }

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            Self {
                protocol_version: value.protocol_version(),
                strong_parents: value.strong_parents().to_set(),
                weak_parents: value.weak_parents().to_set(),
                shallow_like_parents: value.shallow_like_parents().to_set(),
                payload: value.payload().map(Into::into),
                burned_mana: value.burned_mana(),
            }
        }
    }

    impl TryFromDto for Block {
        type Dto = BlockDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let strong_parents = StrongParents::from_set(dto.strong_parents)?;

            let mut builder = BlockBuilder::new(strong_parents)
                .with_weak_parents(WeakParents::from_set(dto.weak_parents)?)
                .with_shallow_like_parents(ShallowLikeParents::from_set(dto.shallow_like_parents)?)
                .with_protocol_version(dto.protocol_version)
                .with_burned_mana(dto.burned_mana);

            if let Some(p) = dto.payload {
                builder = builder.with_payload(Payload::try_from_dto_with_params_inner(p, params)?);
            }

            builder.finish()
        }
    }
}
