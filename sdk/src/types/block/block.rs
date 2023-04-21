// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ops::Deref;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{
    error::{UnexpectedEOF, UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use crate::types::block::{
    parent::Parents,
    payload::{OptionalPayload, Payload},
    protocol::ProtocolParameters,
    BlockId, Error, PROTOCOL_VERSION,
};

/// A builder to build a [`Block`].
#[derive(Clone)]
#[must_use]
pub struct BlockBuilder {
    protocol_version: Option<u8>,
    parents: Parents,
    payload: OptionalPayload,
    nonce: Option<u64>,
}

impl BlockBuilder {
    const DEFAULT_NONCE: u64 = 0;

    /// Creates a new [`BlockBuilder`].
    #[inline(always)]
    pub fn new(parents: Parents) -> Self {
        Self {
            protocol_version: None,
            parents,
            payload: OptionalPayload::none(),
            nonce: None,
        }
    }

    /// Adds a protocol version to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_protocol_version(mut self, protocol_version: impl Into<Option<u8>>) -> Self {
        self.protocol_version = protocol_version.into();
        self
    }

    /// Adds a payload to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Adds a nonce to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_nonce(mut self, nonce: impl Into<Option<u64>>) -> Self {
        self.nonce = nonce.into();
        self
    }

    fn _finish(self) -> Result<(Block, Vec<u8>), Error> {
        verify_payload(self.payload.as_ref())?;

        let block = Block {
            protocol_version: self.protocol_version.unwrap_or(PROTOCOL_VERSION),
            parents: self.parents,
            payload: self.payload,
            nonce: self.nonce.unwrap_or(Self::DEFAULT_NONCE),
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

    /// Finishes the [`BlockBuilder`] into a [`Block`], computing the nonce with a given provider.
    pub fn finish_nonce<F: Fn(&[u8]) -> Option<u64>>(self, nonce_provider: F) -> Result<Block, Error> {
        let (mut block, block_bytes) = self._finish()?;

        block.nonce = nonce_provider(&block_bytes[..block_bytes.len() - core::mem::size_of::<u64>()])
            .ok_or(Error::NonceNotFound)?;

        Ok(block)
    }
}

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// Protocol version of the block.
    protocol_version: u8,
    /// The [`BlockId`]s that this block directly approves.
    parents: Parents,
    /// The optional [Payload] of the block.
    payload: OptionalPayload,
    /// The result of the Proof of Work in order for the block to be accepted into the tangle.
    nonce: u64,
}

impl Block {
    /// The minimum number of bytes in a block.
    pub const LENGTH_MIN: usize = 46;
    /// The maximum number of bytes in a block.
    pub const LENGTH_MAX: usize = 32768;

    /// Creates a new [`BlockBuilder`] to construct an instance of a [`Block`].
    #[inline(always)]
    pub fn build(parents: Parents) -> BlockBuilder {
        BlockBuilder::new(parents)
    }

    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the parents of a [`Block`].
    #[inline(always)]
    pub fn parents(&self) -> &Parents {
        &self.parents
    }

    /// Returns the optional payload of a [`Block`].
    #[inline(always)]
    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }

    /// Returns the nonce of a [`Block`].
    #[inline(always)]
    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    /// Computes the identifier of the block.
    #[inline(always)]
    pub fn id(&self) -> BlockId {
        BlockId::new(Blake2b256::digest(self.pack_to_vec()).into())
    }

    /// Consumes the [`Block`], and returns ownership over its [`Parents`].
    #[inline(always)]
    pub fn into_parents(self) -> Parents {
        self.parents
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
        self.parents.pack(packer)?;
        self.payload.pack(packer)?;
        self.nonce.pack(packer)?;

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

        let parents = Parents::unpack::<_, VERIFY>(unpacker, &())?;
        let payload = OptionalPayload::unpack::<_, VERIFY>(unpacker, visitor)?;

        if VERIFY {
            verify_payload(payload.deref().as_ref()).map_err(UnpackError::Packable)?;
        }

        let nonce = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let block = Self {
            protocol_version,
            parents,
            payload,
            nonce,
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

fn verify_payload(payload: Option<&Payload>) -> Result<(), Error> {
    if !matches!(
        payload,
        None | Some(Payload::Transaction(_)) | Some(Payload::Milestone(_)) | Some(Payload::TaggedData(_))
    ) {
        // Safe to unwrap since it's known not to be None.
        Err(Error::InvalidPayloadKind(payload.unwrap().kind()))
    } else {
        Ok(())
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::{error::dto::DtoError, payload::dto::PayloadDto, protocol::ProtocolParameters};

    /// The block object that nodes gossip around in the network.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockDto {
        ///
        pub protocol_version: u8,
        ///
        pub parents: Vec<String>,
        ///
        #[serde(skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
        ///
        pub nonce: String,
    }

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            Self {
                protocol_version: value.protocol_version(),
                parents: value.parents().iter().map(BlockId::to_string).collect(),
                payload: value.payload().map(Into::into),
                nonce: value.nonce().to_string(),
            }
        }
    }

    impl Block {
        fn _try_from_dto(value: &BlockDto) -> Result<BlockBuilder, DtoError> {
            let parents = Parents::new(
                value
                    .parents
                    .iter()
                    .map(|m| m.parse::<BlockId>().map_err(|_| DtoError::InvalidField("parents")))
                    .collect::<Result<Vec<BlockId>, DtoError>>()?,
            )?;

            let builder = BlockBuilder::new(parents)
                .with_protocol_version(value.protocol_version)
                .with_nonce(
                    value
                        .nonce
                        .parse::<u64>()
                        .map_err(|_| DtoError::InvalidField("nonce"))?,
                );

            Ok(builder)
        }

        pub fn try_from_dto(value: &BlockDto, protocol_parameters: &ProtocolParameters) -> Result<Self, DtoError> {
            let mut builder = Self::_try_from_dto(value)?;

            if let Some(p) = value.payload.as_ref() {
                builder = builder.with_payload(Payload::try_from_dto(p, protocol_parameters)?);
            }

            Ok(builder.finish()?)
        }

        pub fn try_from_dto_unverified(value: &BlockDto) -> Result<Self, DtoError> {
            let mut builder = Self::_try_from_dto(value)?;

            if let Some(p) = value.payload.as_ref() {
                builder = builder.with_payload(Payload::try_from_dto_unverified(p)?);
            }

            Ok(builder.finish()?)
        }
    }
}
