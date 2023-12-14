// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::mem::size_of;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use getset::{CopyGetters, Getters};
use packable::{
    error::{UnexpectedEOF, UnpackError},
    packer::{Packer, SlicePacker},
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use crate::types::block::{
    block_id::{BlockHash, BlockId},
    core::{BasicBlockBody, ValidationBlockBody},
    output::AccountId,
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    signature::Signature,
    slot::{SlotCommitmentId, SlotIndex},
    BlockBody, Error,
};

/// Block without a signature. Can be finished into a [`Block`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsignedBlock {
    /// The block header.
    pub(crate) header: BlockHeader,
    /// The block body.
    pub(crate) body: BlockBody,
}

impl UnsignedBlock {
    pub fn new(header: BlockHeader, body: BlockBody) -> Self {
        Self { header, body }
    }

    /// Updates the block header.
    #[inline(always)]
    pub fn with_block_header(mut self, header: BlockHeader) -> Self {
        self.header = header;
        self
    }

    /// Updates the block body.
    #[inline(always)]
    pub fn with_block_body(mut self, body: BlockBody) -> Self {
        self.body = body;
        self
    }

    /// Get the signing input that can be used to generate a [`Signature`] for the resulting block.
    pub fn signing_input(&self) -> Vec<u8> {
        [self.header.hash(), self.body.hash()].concat()
    }

    pub fn finish(self, signature: impl Into<Signature>) -> Result<Block, Error> {
        Ok(Block::new(self.header, self.body, signature))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, CopyGetters, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
#[getset(get_copy = "pub")]
pub struct BlockHeader {
    /// Protocol version of the network to which this block belongs.
    #[packable(verify_with = verify_protocol_version)]
    protocol_version: u8,
    /// The identifier of the network to which this block belongs.
    #[packable(verify_with = verify_network_id)]
    network_id: u64,
    /// The time at which the block was issued. It is a Unix timestamp in nanoseconds.
    issuing_time: u64,
    /// The identifier of the slot to which this block commits.
    slot_commitment_id: SlotCommitmentId,
    /// The slot index of the latest finalized slot.
    latest_finalized_slot: SlotIndex,
    /// The identifier of the account that issued this block.
    issuer_id: AccountId,
}

impl BlockHeader {
    /// The length of the block header.
    pub const LENGTH: usize =
        size_of::<u8>() + 2 * size_of::<u64>() + SlotCommitmentId::LENGTH + size_of::<SlotIndex>() + AccountId::LENGTH;

    pub fn new(
        protocol_version: u8,
        network_id: u64,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: AccountId,
    ) -> Self {
        Self {
            protocol_version,
            network_id,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
        }
    }

    pub(crate) fn hash(&self) -> [u8; 32] {
        let mut bytes = [0u8; Self::LENGTH];

        self.pack(&mut SlicePacker::new(&mut bytes)).unwrap();
        Blake2b256::digest(bytes).into()
    }
}

impl WorkScore for BlockHeader {}

fn verify_protocol_version<const VERIFY: bool>(
    protocol_version: &u8,
    params: &ProtocolParameters,
) -> Result<(), Error> {
    if VERIFY && *protocol_version != params.version() {
        return Err(Error::ProtocolVersionMismatch {
            expected: params.version(),
            actual: *protocol_version,
        });
    }

    Ok(())
}

fn verify_network_id<const VERIFY: bool>(network_id: &u64, params: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY && *network_id != params.network_id() {
        return Err(Error::NetworkIdMismatch {
            expected: params.network_id(),
            actual: *network_id,
        });
    }

    Ok(())
}

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq, Getters, CopyGetters)]
pub struct Block {
    #[getset(skip)]
    header: BlockHeader,
    /// The block body.
    #[getset(get = "pub")]
    body: BlockBody,
    /// The block signature, used to validate issuance capabilities.
    #[getset(get_copy = "pub")]
    signature: Signature,
}

impl Block {
    /// The minimum number of bytes in a block.
    pub const LENGTH_MIN: usize = 46;
    /// The maximum number of bytes in a block.
    pub const LENGTH_MAX: usize = 32768;

    /// Creates a new [`Block`].
    #[inline(always)]
    pub fn new(header: BlockHeader, body: BlockBody, signature: impl Into<Signature>) -> Self {
        let signature = signature.into();

        Self {
            header,
            body,
            signature,
        }
    }

    /// Creates a new [`UnsignedBlock`].
    #[inline(always)]
    pub fn build(header: BlockHeader, body: BlockBody) -> UnsignedBlock {
        UnsignedBlock::new(header, body)
    }

    /// Returns the protocol version of a [`Block`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.header.protocol_version()
    }

    /// Returns the network id of a [`Block`].
    #[inline(always)]
    pub fn network_id(&self) -> u64 {
        self.header.network_id()
    }

    /// Returns the issuing time of a [`Block`].
    #[inline(always)]
    pub fn issuing_time(&self) -> u64 {
        self.header.issuing_time()
    }

    /// Returns the slot commitment ID of a [`Block`].
    #[inline(always)]
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        self.header.slot_commitment_id()
    }

    /// Returns the latest finalized slot of a [`Block`].
    #[inline(always)]
    pub fn latest_finalized_slot(&self) -> SlotIndex {
        self.header.latest_finalized_slot()
    }

    /// Returns the issuer ID of a [`Block`].
    #[inline(always)]
    pub fn issuer_id(&self) -> AccountId {
        self.header.issuer_id()
    }

    /// Computes the block identifier.
    pub fn id(&self, protocol_params: &ProtocolParameters) -> BlockId {
        let id = [
            &self.header.hash()[..],
            &self.body.hash()[..],
            &self.signature.pack_to_vec(),
        ]
        .concat();
        let block_hash = BlockHash::new(Blake2b256::digest(id).into());
        block_hash.into_block_id(protocol_params.slot_index(self.header.issuing_time() / 1_000_000_000))
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

    /// Checks whether the block body is a [`BasicBlockBody`].
    pub fn is_basic(&self) -> bool {
        self.body.is_basic()
    }

    /// Gets the block body as an actual [`BasicBlockBody`].
    /// NOTE: Will panic if the block body is not a [`BasicBlockBody`].
    pub fn as_basic(&self) -> &BasicBlockBody {
        self.body.as_basic()
    }

    /// Checks whether the block body is a [`ValidationBlockBody`].
    pub fn is_validation(&self) -> bool {
        self.body.is_validation()
    }

    /// Gets the block body as an actual [`ValidationBlockBody`].
    /// NOTE: Will panic if the block body is not a [`ValidationBlockBody`].
    pub fn as_validation(&self) -> &ValidationBlockBody {
        self.body.as_validation()
    }
}

impl WorkScore for Block {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        self.header.work_score(params) + self.body.work_score(params) + self.signature.work_score(params)
    }
}

impl Packable for Block {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.header.pack(packer)?;
        self.body.pack(packer)?;
        self.signature.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        protocol_params: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let start_opt = unpacker.read_bytes();

        let header = BlockHeader::unpack::<_, VERIFY>(unpacker, protocol_params)?;
        let body = BlockBody::unpack::<_, VERIFY>(unpacker, protocol_params)?;
        let signature = Signature::unpack::<_, VERIFY>(unpacker, &())?;

        let block = Self {
            header,
            body,
            signature,
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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{block::core::dto::BlockBodyDto, TryFromDto},
        utils::serde::string,
    };

    /// The block object that nodes gossip around in the network.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockDto {
        #[serde(flatten)]
        pub inner: UnsignedBlockDto,
        pub signature: Signature,
    }

    impl core::ops::Deref for BlockDto {
        type Target = UnsignedBlockDto;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl From<&Block> for BlockDto {
        fn from(value: &Block) -> Self {
            Self {
                inner: UnsignedBlockDto {
                    header: BlockHeaderDto::from(&value.header),
                    body: BlockBodyDto::from(&value.body),
                },
                signature: value.signature,
            }
        }
    }

    impl TryFromDto<BlockDto> for Block {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: BlockDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            if let Some(protocol_params) = params {
                if dto.inner.header.protocol_version != protocol_params.version() {
                    return Err(Error::ProtocolVersionMismatch {
                        expected: protocol_params.version(),
                        actual: dto.inner.header.protocol_version,
                    });
                }

                if dto.inner.header.network_id != protocol_params.network_id() {
                    return Err(Error::NetworkIdMismatch {
                        expected: protocol_params.network_id(),
                        actual: dto.inner.header.network_id,
                    });
                }
            }

            Ok(Self::new(
                BlockHeader::try_from_dto_with_params_inner(dto.inner.header, params)?,
                BlockBody::try_from_dto_with_params_inner(dto.inner.body, params)?,
                dto.signature,
            ))
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockHeaderDto {
        pub protocol_version: u8,
        #[serde(with = "string")]
        pub network_id: u64,
        #[serde(with = "string")]
        pub issuing_time: u64,
        pub slot_commitment_id: SlotCommitmentId,
        pub latest_finalized_slot: SlotIndex,
        pub issuer_id: AccountId,
    }

    impl From<&BlockHeader> for BlockHeaderDto {
        fn from(value: &BlockHeader) -> Self {
            Self {
                protocol_version: value.protocol_version(),
                network_id: value.network_id(),
                issuing_time: value.issuing_time(),
                slot_commitment_id: value.slot_commitment_id(),
                latest_finalized_slot: value.latest_finalized_slot(),
                issuer_id: value.issuer_id(),
            }
        }
    }

    impl TryFromDto<BlockHeaderDto> for BlockHeader {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: BlockHeaderDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            if let Some(protocol_params) = params {
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
            }

            Ok(Self::new(
                dto.protocol_version,
                dto.network_id,
                dto.issuing_time,
                dto.slot_commitment_id,
                dto.latest_finalized_slot,
                dto.issuer_id,
            ))
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UnsignedBlockDto {
        pub header: BlockHeaderDto,
        pub body: BlockBodyDto,
    }

    impl From<&UnsignedBlock> for UnsignedBlockDto {
        fn from(value: &UnsignedBlock) -> Self {
            Self {
                header: BlockHeaderDto::from(&value.header),
                body: BlockBodyDto::from(&value.body),
            }
        }
    }

    impl TryFromDto<UnsignedBlockDto> for UnsignedBlock {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: UnsignedBlockDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            Ok(Self::new(
                BlockHeader::try_from_dto_with_params_inner(dto.header, params)?,
                BlockBody::try_from_dto_with_params_inner(dto.body, params)?,
            ))
        }
    }
}
