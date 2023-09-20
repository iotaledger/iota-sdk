// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem::size_of;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use getset::{CopyGetters, Getters};
use packable::{
    error::{UnexpectedEOF, UnpackError, UnpackErrorExt},
    packer::{Packer, SlicePacker},
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use crate::types::block::{
    block_id::{BlockHash, BlockId},
    core::{BasicBlock, ValidationBlock},
    protocol::{ProtocolParameters, WorkScore, WorkScoreStructure},
    signature::Signature,
    slot::{SlotCommitmentId, SlotIndex},
    Block, Error, IssuerId,
};

#[derive(Clone, Debug, Eq, PartialEq, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct BlockHeader {
    /// Protocol version of the network to which this block belongs.
    protocol_version: u8,
    /// The identifier of the network to which this block belongs.
    network_id: u64,
    /// The time at which the block was issued. It is a Unix timestamp in nanoseconds.
    issuing_time: u64,
    /// The identifier of the slot to which this block commits.
    slot_commitment_id: SlotCommitmentId,
    /// The slot index of the latest finalized slot.
    latest_finalized_slot: SlotIndex,
    /// The identifier of the account that issued this block.
    issuer_id: IssuerId,
}

impl BlockHeader {
    /// The length of the block header.
    pub const LENGTH: usize =
        size_of::<u8>() + 2 * size_of::<u64>() + SlotCommitmentId::LENGTH + size_of::<SlotIndex>() + IssuerId::LENGTH;

    pub(crate) fn hash(&self) -> [u8; 32] {
        let mut bytes = [0u8; Self::LENGTH];

        self.pack(&mut SlicePacker::new(&mut bytes)).unwrap();
        Blake2b256::digest(bytes).into()
    }
}

impl WorkScore for BlockHeader {
    fn workscore(&self, workscore_structure: WorkScoreStructure) -> u32 {
        // The work score of a block header is `0`.
        0
    }
}

impl Packable for BlockHeader {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.protocol_version.pack(packer)?;
        self.network_id.pack(packer)?;
        self.issuing_time.pack(packer)?;
        self.slot_commitment_id.pack(packer)?;
        self.latest_finalized_slot.pack(packer)?;
        self.issuer_id.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        protocol_params: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
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

        Ok(Self {
            protocol_version,
            network_id,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
        })
    }
}

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq, Getters, CopyGetters)]
pub struct BlockWrapper {
    #[getset(skip)]
    header: BlockHeader,
    /// The inner block.
    #[getset(get = "pub")]
    block: Block,
    /// The block signature, used to validate issuance capabilities.
    #[getset(get_copy = "pub")]
    signature: Signature,
    /// The identifier of the block.
    #[getset(get_copy = "pub")]
    id: BlockId,
}

impl BlockWrapper {
    /// The minimum number of bytes in a block.
    pub const LENGTH_MIN: usize = 46;
    /// The maximum number of bytes in a block.
    pub const LENGTH_MAX: usize = 32768;

    /// Creates a new [`BlockWrapper`].
    #[inline(always)]
    pub fn new(
        protocol_params: &ProtocolParameters,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: IssuerId,
        block: impl Into<Block>,
        signature: impl Into<Signature>,
    ) -> Self {
        let block = block.into();
        let signature = signature.into();
        let header = BlockHeader {
            protocol_version: protocol_params.version(),
            network_id: protocol_params.network_id(),
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
        };
        Self {
            id: Self::block_id(&header, &block, &signature, protocol_params),
            header,
            block,
            signature,
        }
    }

    /// Returns the protocol version of a [`BlockWrapper`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.header.protocol_version()
    }

    /// Returns the network id of a [`BlockWrapper`].
    #[inline(always)]
    pub fn network_id(&self) -> u64 {
        self.header.network_id()
    }

    /// Returns the issuing time of a [`BlockWrapper`].
    #[inline(always)]
    pub fn issuing_time(&self) -> u64 {
        self.header.issuing_time()
    }

    /// Returns the slot commitment ID of a [`BlockWrapper`].
    #[inline(always)]
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        self.header.slot_commitment_id()
    }

    /// Returns the latest finalized slot of a [`BlockWrapper`].
    #[inline(always)]
    pub fn latest_finalized_slot(&self) -> SlotIndex {
        self.header.latest_finalized_slot()
    }

    /// Returns the issuer ID of a [`BlockWrapper`].
    #[inline(always)]
    pub fn issuer_id(&self) -> IssuerId {
        self.header.issuer_id()
    }

    /// Computes the block identifier.
    pub fn block_id(
        block_header: &BlockHeader,
        block: &Block,
        signature: &Signature,
        protocol_params: &ProtocolParameters,
    ) -> BlockId {
        let id = [&block_header.hash()[..], &block.hash()[..], &signature.pack_to_vec()].concat();
        let block_hash = BlockHash::new(Blake2b256::digest(id).into());
        block_hash.with_slot_index(protocol_params.slot_index(block_header.issuing_time()))
    }

    /// Unpacks a [`BlockWrapper`] from a sequence of bytes doing syntactical checks and verifying that
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

    /// Checks whether the inner block is a [`BasicBlock`].
    pub fn is_basic(&self) -> bool {
        self.block.is_basic()
    }

    /// Gets the inner block as an actual [`BasicBlock`].
    /// NOTE: Will panic if the inner block is not a [`BasicBlock`].
    pub fn as_basic(&self) -> &BasicBlock {
        self.block.as_basic()
    }

    /// Checks whether the inner block is a [`ValidationBlock`].
    pub fn is_validation(&self) -> bool {
        self.block.is_validation()
    }

    /// Gets the inner block as an actual [`ValidationBlock`].
    /// NOTE: Will panic if the inner block is not a [`ValidationBlock`].
    pub fn as_validation(&self) -> &ValidationBlock {
        self.block.as_validation()
    }
}

impl WorkScore for BlockWrapper {
    fn workscore(&self, workscore_structure: WorkScoreStructure) -> u32 {
        let mut score = workscore_structure.data_kilobyte * self.packed_len() as u32 / 1024;
        score += self.header.workscore(workscore_structure);
        score += self.block.workscore(workscore_structure);
        score += self.signature.workscore(workscore_structure);
        score
    }
}

impl Packable for BlockWrapper {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.header.pack(packer)?;
        self.block.pack(packer)?;
        self.signature.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        protocol_params: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let start_opt = unpacker.read_bytes();

        let header = BlockHeader::unpack::<_, VERIFY>(unpacker, protocol_params)?;

        let block = Block::unpack::<_, VERIFY>(unpacker, protocol_params)?;

        let signature = Signature::unpack::<_, VERIFY>(unpacker, &())?;

        let wrapper = Self {
            id: Self::block_id(&header, &block, &signature, protocol_params),
            header,
            block,
            signature,
        };

        if VERIFY {
            let wrapper_len = if let (Some(start), Some(end)) = (start_opt, unpacker.read_bytes()) {
                end - start
            } else {
                wrapper.packed_len()
            };

            if wrapper_len > Self::LENGTH_MAX {
                return Err(UnpackError::Packable(Error::InvalidBlockWrapperLength(wrapper_len)));
            }
        }

        Ok(wrapper)
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{block::core::dto::BlockDto, TryFromDto},
        utils::serde::string,
    };

    /// The block object that nodes gossip around in the network.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockWrapperDto {
        pub protocol_version: u8,
        #[serde(with = "string")]
        pub network_id: u64,
        #[serde(with = "string")]
        pub issuing_time: u64,
        pub slot_commitment_id: SlotCommitmentId,
        pub latest_finalized_slot: SlotIndex,
        pub issuer_id: IssuerId,
        pub block: BlockDto,
        pub signature: Signature,
    }

    impl From<&BlockWrapper> for BlockWrapperDto {
        fn from(value: &BlockWrapper) -> Self {
            Self {
                protocol_version: value.protocol_version(),
                network_id: value.network_id(),
                issuing_time: value.issuing_time(),
                slot_commitment_id: value.slot_commitment_id(),
                latest_finalized_slot: value.latest_finalized_slot(),
                issuer_id: value.issuer_id(),
                block: BlockDto::from(&value.block),
                signature: value.signature,
            }
        }
    }

    impl BlockWrapper {
        pub fn try_from_dto(dto: BlockWrapperDto, protocol_params: ProtocolParameters) -> Result<Self, Error> {
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

            Ok(Self::new(
                &protocol_params,
                dto.issuing_time,
                dto.slot_commitment_id,
                dto.latest_finalized_slot,
                dto.issuer_id,
                Block::try_from_dto_with_params(dto.block, &protocol_params)?,
                dto.signature,
            ))
        }
    }
}
