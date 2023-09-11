// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem::size_of;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{
    error::{UnexpectedEOF, UnpackError, UnpackErrorExt},
    packer::{Packer, SlicePacker},
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

use crate::types::block::{
    block_id::{BlockHash, BlockId},
    protocol::ProtocolParameters,
    signature::{Ed25519Signature, Signature},
    slot::{SlotCommitmentId, SlotIndex},
    Block, Error, IssuerId,
};

// /// A builder to build a [`Block`].
// #[derive(Clone)]
// #[must_use]
// pub struct BlockBuilder<B>(pub(crate) B);

// impl<B> BlockBuilder<BlockWrapper<B>> {
//     pub fn from_block_data(
//         protocol_params: ProtocolParameters,
//         issuing_time: u64,
//         slot_commitment_id: SlotCommitmentId,
//         latest_finalized_slot: SlotIndex,
//         issuer_id: IssuerId,
//         data: B,
//         signature: Ed25519Signature,
//     ) -> Self { Self(BlockWrapper { protocol_params, issuing_time, slot_commitment_id, latest_finalized_slot,
//       issuer_id, data, signature, })
//     }
// }

// impl<B> BlockBuilder<B>
// where
//     B: Packable,
//     Block: From<B>,
// {
//     fn _finish(self) -> Result<(Block, Vec<u8>), Error> {
//         let block = Block::from(self.0);

//         verify_parents(
//             block.strong_parents(),
//             block.weak_parents(),
//             block.shallow_like_parents(),
//         )?;

//         let block_bytes = block.pack_to_vec();

//         if block_bytes.len() > Block::LENGTH_MAX {
//             return Err(Error::InvalidBlockLength(block_bytes.len()));
//         }

//         Ok((block, block_bytes))
//     }

//     /// Finishes the [`BlockBuilder`] into a [`Block`].
//     pub fn finish(self) -> Result<Block, Error> {
//         self._finish().map(|res| res.0)
//     }
// }

/// Represent the object that nodes gossip around the network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockWrapper {
    /// Protocol parameters of the network to which this block belongs.
    // TODO remove
    protocol_params: ProtocolParameters,
    /// The time at which the block was issued. It is a Unix timestamp in nanoseconds.
    issuing_time: u64,
    /// The identifier of the slot to which this block commits.
    slot_commitment_id: SlotCommitmentId,
    /// The slot index of the latest finalized slot.
    latest_finalized_slot: SlotIndex,
    /// The identifier of the account that issued this block.
    issuer_id: IssuerId,
    /// The inner block.
    block: Block,
    /// The block signature, used to validate issuance capabilities.
    signature: Signature,
}

impl BlockWrapper {
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

    // TODO builder?
    /// Creates a new [`BlockWrapper`].
    #[inline(always)]
    pub fn new(
        protocol_params: ProtocolParameters,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: IssuerId,
        block: impl Into<Block>,
        signature: impl Into<Signature>,
    ) -> Self {
        Self {
            protocol_params,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            block: block.into(),
            signature: signature.into(),
        }
    }

    /// Returns the protocol version of a [`BlockWrapper`].
    #[inline(always)]
    pub fn protocol_version(&self) -> u8 {
        self.protocol_params.version()
    }

    /// Returns the protocol parameters of a [`BlockWrapper`].
    #[inline(always)]
    pub fn protocol_parameters(&self) -> &ProtocolParameters {
        &self.protocol_params
    }

    /// Returns the network id of a [`BlockWrapper`].
    #[inline(always)]
    pub fn network_id(&self) -> u64 {
        self.protocol_params.network_id()
    }

    /// Returns the issuing time of a [`BlockWrapper`].
    #[inline(always)]
    pub fn issuing_time(&self) -> u64 {
        self.issuing_time
    }

    /// Returns the slot commitment ID of a [`BlockWrapper`].
    #[inline(always)]
    pub fn slot_commitment_id(&self) -> SlotCommitmentId {
        self.slot_commitment_id
    }

    /// Returns the latest finalized slot of a [`BlockWrapper`].
    #[inline(always)]
    pub fn latest_finalized_slot(&self) -> SlotIndex {
        self.latest_finalized_slot
    }

    /// Returns the issuer ID of a [`BlockWrapper`].
    #[inline(always)]
    pub fn issuer_id(&self) -> IssuerId {
        self.issuer_id
    }

    /// Returns the [`Block`] of a [`BlockWrapper`].
    #[inline(always)]
    pub fn block(&self) -> &Block {
        &self.block
    }

    /// Returns the signature of a [`BlockWrapper`].
    #[inline(always)]
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

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
        let mut bytes = [0u8; Self::HEADER_LENGTH];

        self.pack_header(&mut SlicePacker::new(&mut bytes)).unwrap();
        Blake2b256::digest(bytes).into()
    }

    /// Computes the block hash.
    pub fn hash(&self) -> BlockHash {
        let id = [
            &self.header_hash()[..],
            &self.block.hash()[..],
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

    pub(crate) fn signature_bytes(&self) -> [u8; Self::SIGNATURE_LENGTH] {
        let mut bytes = [0u8; Self::SIGNATURE_LENGTH];
        let mut packer = SlicePacker::new(&mut bytes);
        self.signature().pack(&mut packer).unwrap();

        bytes
    }
}

impl Packable for BlockWrapper {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.pack_header(packer)?;
        self.block.pack(packer)?;
        self.signature.pack(packer)?;

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

        let block = Block::unpack::<_, VERIFY>(unpacker, protocol_params)?;

        let signature = Signature::unpack::<_, VERIFY>(unpacker, &())?;

        let block_wrapper = Self {
            protocol_params: protocol_params.clone(),
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            block,
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

        Ok(block_wrapper)
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
                issuing_time: value.issuing_time,
                slot_commitment_id: value.slot_commitment_id,
                latest_finalized_slot: value.latest_finalized_slot,
                issuer_id: value.issuer_id,
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

            Ok(BlockWrapper::new(
                protocol_params,
                dto.issuing_time,
                dto.slot_commitment_id,
                dto.latest_finalized_slot,
                dto.issuer_id,
                Block::try_from_dto_with_params(dto.block, protocol_params)?,
                dto.signature,
            ))
        }
    }
}
