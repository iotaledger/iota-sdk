// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use core::mem::size_of;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::{
    error::{UnexpectedEOF, UnpackError, UnpackErrorExt},
    packer::{Packer, SlicePacker},
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use super::{
    basic::{BasicBlock, BasicBlockData},
    block_id::BlockHash,
    signature::{Ed25519Signature, Signature},
    slot::{SlotCommitmentId, SlotIndex},
    validation::{ValidationBlock, ValidationBlockData},
    IssuerId,
};
use crate::types::block::{
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    payload::Payload,
    protocol::ProtocolParameters,
    BlockId, Error,
};

/// A builder to build a [`Block`].
#[derive(Clone)]
#[must_use]
pub struct BlockBuilder<B> {
    /// Protocol parameters of the block.
    pub(crate) protocol_params: ProtocolParameters,
    /// The time at which the block was issued. It is a Unix timestamp in nanoseconds.
    pub(crate) issuing_time: u64,
    /// The identifier of the slot to which this block commits.
    pub(crate) slot_commitment_id: SlotCommitmentId,
    /// The slot index of the latest finalized slot.
    pub(crate) latest_finalized_slot: SlotIndex,
    /// The identifier of the account that issued this block.
    pub(crate) issuer_id: IssuerId,
    /// The inner block data, either [`BasicBlock`] or [`ValidationBlock`].
    pub(crate) data: B,
}

impl<B> BlockBuilder<B> {
    pub fn from_block_data(
        protocol_params: ProtocolParameters,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: IssuerId,
        data: B,
    ) -> Self {
        Self {
            protocol_params,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            data,
        }
    }
}

impl<B> BlockBuilder<B>
where
    B: Packable,
    Block: From<BlockWrapper<B>>,
{
    fn _finish(self, signature: Ed25519Signature) -> Result<(Block, Vec<u8>), Error> {
        let wrapper = BlockWrapper {
            protocol_params: self.protocol_params,
            issuing_time: self.issuing_time,
            slot_commitment_id: self.slot_commitment_id,
            latest_finalized_slot: self.latest_finalized_slot,
            issuer_id: self.issuer_id,
            data: self.data,
            signature,
        };
        let block = Block::from(wrapper);

        verify_parents(
            block.strong_parents(),
            block.weak_parents(),
            block.shallow_like_parents(),
        )?;

        let block_bytes = block.pack_to_vec();

        if block_bytes.len() > Block::LENGTH_MAX {
            return Err(Error::InvalidBlockLength(block_bytes.len()));
        }

        Ok((block, block_bytes))
    }

    /// Finishes the [`BlockBuilder`] into a [`Block`].
    pub fn finish(self, signature: Ed25519Signature) -> Result<Block, Error> {
        self._finish(signature).map(|res| res.0)
    }
}

impl<B: Packable> BlockBuilder<B> {
    pub(crate) fn pack_header<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.protocol_params.protocol_version().pack(packer)?;
        self.protocol_params.network_id().pack(packer)?;
        self.issuing_time.pack(packer)?;
        self.slot_commitment_id.pack(packer)?;
        self.latest_finalized_slot.pack(packer)?;
        self.issuer_id.pack(packer)?;
        Ok(())
    }

    pub(crate) fn header_hash(&self) -> [u8; 32] {
        let mut bytes = [0u8; Block::HEADER_LENGTH];
        self.pack_header(&mut SlicePacker::new(&mut bytes)).unwrap();
        Blake2b256::digest(bytes).into()
    }
}

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
            Self::Basic(block) => block.pack(packer),
            Self::Validation(block) => block.pack(packer),
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        protocol_params: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let start_opt = unpacker.read_bytes();

        let protocol_version = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        if VERIFY && protocol_version != protocol_params.version() {
            return Err(UnpackError::Packable(Error::ProtocolVersionMismatch {
                expected: protocol_params.version(),
                actual: protocol_version,
            }));
        }

        let network_id = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        if VERIFY && network_id != protocol_params.network_id() {
            return Err(UnpackError::Packable(Error::NetworkIdMismatch {
                expected: protocol_params.network_id(),
                actual: network_id,
            }));
        }

        let issuing_time = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let slot_commitment_id = SlotCommitmentId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let latest_finalized_slot = SlotIndex::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let issuer_id = IssuerId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let kind = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let block = match kind {
            BasicBlock::KIND => {
                let data = BasicBlockData::unpack::<_, VERIFY>(unpacker, protocol_params)?;
                let Signature::Ed25519(signature) = Signature::unpack::<_, VERIFY>(unpacker, &())?;

                Self::from(BlockWrapper {
                    protocol_params: protocol_params.clone(),
                    issuing_time,
                    slot_commitment_id,
                    latest_finalized_slot,
                    issuer_id,
                    data,
                    signature,
                })
            }
            ValidationBlock::KIND => {
                let data = ValidationBlockData::unpack::<_, VERIFY>(unpacker, protocol_params)?;
                let Signature::Ed25519(signature) = Signature::unpack::<_, VERIFY>(unpacker, &())?;

                Self::from(BlockWrapper {
                    protocol_params: protocol_params.clone(),
                    issuing_time,
                    slot_commitment_id,
                    latest_finalized_slot,
                    issuer_id,
                    data,
                    signature,
                })
            }
            _ => return Err(Error::InvalidBlockKind(kind)).map_err(UnpackError::Packable),
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

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockWrapper<B> {
    /// Protocol parameters of the network to which this block belongs.
    pub(crate) protocol_params: ProtocolParameters,
    /// The time at which the block was issued. It is a Unix timestamp in nanoseconds.
    pub(crate) issuing_time: u64,
    /// The identifier of the slot to which this block commits.
    pub(crate) slot_commitment_id: SlotCommitmentId,
    /// The slot index of the latest finalized slot.
    pub(crate) latest_finalized_slot: SlotIndex,
    /// The identifier of the account that issued this block.
    pub(crate) issuer_id: IssuerId,
    /// The inner block data, either [`BasicBlock`] or [`ValidationBlock`].
    pub(crate) data: B,
    /// The block signature, used to validate issuance capabilities.
    pub(crate) signature: Ed25519Signature,
}

impl<B> BlockWrapper<B> {
    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.protocol_params.version()
    }

    /// Returns the protocol parameters of a [`Block`].
    #[inline(always)]
    pub fn protocol_parameters(&self) -> &ProtocolParameters {
        &self.protocol_params
    }

    /// Returns the network id of a [`Block`].
    #[inline(always)]
    pub fn network_id(&self) -> u64 {
        self.protocol_params.network_id()
    }

    /// Returns the issuing time of a [`Block`].
    #[inline(always)]
    pub fn issuing_time(&self) -> u64 {
        self.issuing_time
    }

    /// Returns the slot commitment ID of a [`Block`].
    #[inline(always)]
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        self.slot_commitment_id
    }

    /// Returns the latest finalized slot of a [`Block`].
    #[inline(always)]
    pub fn latest_finalized_slot(&self) -> SlotIndex {
        self.latest_finalized_slot
    }

    /// Returns the issuer ID of a [`Block`].
    #[inline(always)]
    pub fn issuer_id(&self) -> IssuerId {
        self.issuer_id
    }

    /// Returns the signature of a [`Block`].
    #[inline(always)]
    pub fn signature(&self) -> &Ed25519Signature {
        &self.signature
    }
}

impl<B: Packable> BlockWrapper<B> {
    pub(crate) fn pack_header<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.protocol_version().pack(packer)?;
        self.network_id().pack(packer)?;
        self.issuing_time.pack(packer)?;
        self.slot_commitment_id.pack(packer)?;
        self.latest_finalized_slot.pack(packer)?;
        self.issuer_id.pack(packer)?;
        Ok(())
    }

    pub(crate) fn header_hash(&self) -> [u8; 32] {
        let mut bytes = [0u8; Block::HEADER_LENGTH];
        self.pack_header(&mut SlicePacker::new(&mut bytes)).unwrap();
        Blake2b256::digest(bytes).into()
    }
}

impl Block {
    /// The minimum number of bytes in a block.
    pub const LENGTH_MIN: usize = 46;
    /// The maximum number of bytes in a block.
    pub const LENGTH_MAX: usize = 32768;
    /// The length of the block header.
    pub const HEADER_LENGTH: usize = size_of::<u8>()
        + 2 * size_of::<u64>()
        + size_of::<SlotCommitmentId>()
        + size_of::<SlotIndex>()
        + size_of::<IssuerId>();
    /// The length of the block signature.
    pub const SIGNATURE_LENGTH: usize =
        size_of::<u8>() + Ed25519Signature::PUBLIC_KEY_LENGTH + Ed25519Signature::SIGNATURE_LENGTH;

    /// Creates a new [`BlockBuilder`] to construct an instance of a [`BasicBlock`].
    #[inline(always)]
    pub fn build_basic(
        protocol_params: ProtocolParameters,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: IssuerId,
        strong_parents: StrongParents,
    ) -> BlockBuilder<BasicBlockData> {
        BlockBuilder::<BasicBlockData>::new(
            protocol_params,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            strong_parents,
        )
    }

    /// Creates a new [`BlockBuilder`] to construct an instance of a [`ValidationBlock`].
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub fn build_validation(
        protocol_params: ProtocolParameters,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: IssuerId,
        strong_parents: StrongParents,
        highest_supported_version: u8,
        protocol_parameters: &ProtocolParameters,
    ) -> BlockBuilder<ValidationBlockData> {
        BlockBuilder::<ValidationBlockData>::new(
            protocol_params,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            strong_parents,
            highest_supported_version,
            protocol_parameters,
        )
    }

    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        match self {
            Self::Basic(b) => b.protocol_version(),
            Self::Validation(b) => b.protocol_version(),
        }
    }

    /// Returns the protocol parameters of a [`Block`].
    #[inline(always)]
    pub fn protocol_parameters(&self) -> &ProtocolParameters {
        match self {
            Self::Basic(b) => b.protocol_parameters(),
            Self::Validation(b) => b.protocol_parameters(),
        }
    }

    /// Returns the network id of a [`Block`].
    #[inline(always)]
    pub fn network_id(&self) -> u64 {
        match self {
            Self::Basic(b) => b.network_id(),
            Self::Validation(b) => b.network_id(),
        }
    }

    /// Returns the issuing time of a [`Block`].
    #[inline(always)]
    pub fn issuing_time(&self) -> u64 {
        match self {
            Self::Basic(b) => b.issuing_time(),
            Self::Validation(b) => b.issuing_time(),
        }
    }

    /// Returns the slot commitment ID of a [`Block`].
    #[inline(always)]
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        match self {
            Self::Basic(b) => b.slot_commitment_id(),
            Self::Validation(b) => b.slot_commitment_id(),
        }
    }

    /// Returns the latest finalized slot of a [`Block`].
    #[inline(always)]
    pub fn latest_finalized_slot(&self) -> SlotIndex {
        match self {
            Self::Basic(b) => b.latest_finalized_slot(),
            Self::Validation(b) => b.latest_finalized_slot(),
        }
    }

    /// Returns the issuer ID of a [`Block`].
    #[inline(always)]
    pub fn issuer_id(&self) -> IssuerId {
        match self {
            Self::Basic(b) => b.issuer_id(),
            Self::Validation(b) => b.issuer_id(),
        }
    }

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

    /// Returns the signature of a [`Block`].
    #[inline(always)]
    pub fn signature(&self) -> &Ed25519Signature {
        match self {
            Self::Basic(b) => b.signature(),
            Self::Validation(b) => b.signature(),
        }
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

    /// Checks whether the block is a [`BasicBlock`].
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic(_))
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

    /// Checks whether the block is a [`ValidationBlock`].
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Computes the block hash.
    pub fn hash(&self) -> BlockHash {
        let id = [
            &self.header_hash()[..],
            &self.block_hash()[..],
            &self.signature_bytes()[..],
        ]
        .concat();
        BlockHash::new(Blake2b256::digest(id).into())
    }

    /// Computes the identifier of the block.
    pub fn id(&self) -> BlockId {
        self.hash()
            .with_slot_index(self.protocol_parameters().slot_index(self.issuing_time()))
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

    pub(crate) fn header_hash(&self) -> [u8; 32] {
        match self {
            Self::Basic(b) => b.header_hash(),
            Self::Validation(b) => b.header_hash(),
        }
    }

    pub(crate) fn block_hash(&self) -> [u8; 32] {
        match self {
            Self::Basic(b) => b.block_hash(),
            Self::Validation(b) => b.block_hash(),
        }
    }

    pub(crate) fn signature_bytes(&self) -> [u8; Self::SIGNATURE_LENGTH] {
        let mut bytes = [0u8; Self::SIGNATURE_LENGTH];
        let mut packer = SlicePacker::new(&mut bytes);
        Ed25519Signature::KIND.pack(&mut packer).unwrap();
        self.signature().pack(&mut packer).unwrap();
        bytes
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
    use crate::{
        types::{
            block::{basic::dto::BasicBlockDataDto, validation::dto::ValidationBlockDataDto, Error},
            TryFromDto,
        },
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq, From)]
    pub enum BlockDataDto {
        Basic(BasicBlockDataDto),
        Validation(ValidationBlockDataDto),
    }

    impl From<&BasicBlockData> for BlockDataDto {
        fn from(value: &BasicBlockData) -> Self {
            Self::Basic(value.into())
        }
    }

    impl From<&ValidationBlockData> for BlockDataDto {
        fn from(value: &ValidationBlockData) -> Self {
            Self::Validation(value.into())
        }
    }

    impl<'de> Deserialize<'de> for BlockDataDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid block type"))? as u8
                {
                    BasicBlock::KIND => Self::Basic(
                        BasicBlockDataDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize basic block: {e}")))?,
                    ),
                    ValidationBlock::KIND => {
                        Self::Validation(ValidationBlockDataDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize validation block: {e}"))
                        })?)
                    }
                    _ => return Err(serde::de::Error::custom("invalid block type")),
                },
            )
        }
    }

    impl Serialize for BlockDataDto {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #[derive(Serialize)]
            #[serde(untagged)]
            enum BlockTypeDto_<'a> {
                T0(&'a BasicBlockDataDto),
                T1(&'a ValidationBlockDataDto),
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
        #[serde(with = "string")]
        pub network_id: u64,
        #[serde(with = "string")]
        pub issuing_time: u64,
        pub slot_commitment: SlotCommitmentId,
        pub latest_finalized_slot: SlotIndex,
        pub issuer_id: IssuerId,
        pub block: BlockDataDto,
        pub signature: Signature,
    }

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            match value {
                Block::Basic(b) => Self {
                    protocol_version: b.protocol_version(),
                    network_id: b.network_id(),
                    issuing_time: b.issuing_time(),
                    slot_commitment: b.slot_commitment_id(),
                    latest_finalized_slot: b.latest_finalized_slot(),
                    issuer_id: b.issuer_id(),
                    block: (&b.data).into(),
                    signature: b.signature.into(),
                },
                Block::Validation(b) => Self {
                    protocol_version: b.protocol_version(),
                    network_id: b.network_id(),
                    issuing_time: b.issuing_time(),
                    slot_commitment: b.slot_commitment_id(),
                    latest_finalized_slot: b.latest_finalized_slot(),
                    issuer_id: b.issuer_id(),
                    block: (&b.data).into(),
                    signature: b.signature.into(),
                },
            }
        }
    }

    impl Block {
        pub fn try_from_dto(dto: BlockDto, protocol_params: ProtocolParameters) -> Result<Self, Error> {
            if dto.protocol_version != protocol_params.version() {
                return Err(Error::ProtocolVersionMismatch {
                    expected: protocol_params.version(),
                    actual: dto.protocol_version,
                });
            }

            if dto.network_id != protocol_params.network_id() {
                return Err(Error::NetworkIdMismatch {
                    expected: protocol_params.network_id(),
                    actual: dto.network_id,
                });
            }

            match dto.block {
                BlockDataDto::Basic(b) => {
                    let data = BasicBlockData::try_from_dto_with_params(b, &protocol_params)?;
                    BlockBuilder::from_block_data(
                        protocol_params,
                        dto.issuing_time,
                        dto.slot_commitment,
                        dto.latest_finalized_slot,
                        dto.issuer_id,
                        data,
                    )
                    .finish(*dto.signature.as_ed25519())
                }
                BlockDataDto::Validation(b) => {
                    let data = ValidationBlockData::try_from_dto_with_params(b, &protocol_params)?;
                    BlockBuilder::from_block_data(
                        protocol_params,
                        dto.issuing_time,
                        dto.slot_commitment,
                        dto.latest_finalized_slot,
                        dto.issuer_id,
                        data,
                    )
                    .finish(*dto.signature.as_ed25519())
                }
            }
        }
    }
}
