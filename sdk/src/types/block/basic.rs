// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

use super::{
    core::{verify_parents, BlockWrapper},
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    payload::{OptionalPayload, Payload},
    protocol::{ProtocolParameters, WorkScore, WorkScoreStructure},
    signature::{Ed25519Signature, Signature},
    slot::{SlotCommitmentId, SlotIndex},
    Block, BlockBuilder, Error, IssuerId,
};

pub type BasicBlock = BlockWrapper<BasicBlockData>;

impl BlockBuilder<BasicBlock> {
    /// Creates a new [`BlockBuilder`] for a [`BasicBlock`].
    #[inline(always)]
    pub fn new(
        protocol_params: ProtocolParameters,
        issuing_time: u64,
        slot_commitment_id: SlotCommitmentId,
        latest_finalized_slot: SlotIndex,
        issuer_id: IssuerId,
        strong_parents: StrongParents,
        signature: Ed25519Signature,
    ) -> Self {
        Self(BlockWrapper {
            protocol_params,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            data: BasicBlockData {
                strong_parents,
                weak_parents: Default::default(),
                shallow_like_parents: Default::default(),
                payload: OptionalPayload::default(),
                burned_mana: Default::default(),
            },
            signature,
        })
    }

    /// Adds weak parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.0.data.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.0.data.shallow_like_parents = shallow_like_parents.into();
        self
    }

    /// Adds a payload to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.0.data.payload = payload.into();
        self
    }

    /// Adds burned mana to a [`BlockBuilder`].
    #[inline(always)]
    pub fn with_burned_mana(mut self, burned_mana: u64) -> Self {
        self.0.data.burned_mana = burned_mana;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicBlockData {
    /// Blocks that are strongly directly approved.
    pub(crate) strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    pub(crate) weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    pub(crate) shallow_like_parents: ShallowLikeParents,
    /// The optional [Payload] of the block.
    pub(crate) payload: OptionalPayload,
    /// The amount of mana the Account identified by [`IssuerId`](super::IssuerId) is at most
    /// willing to burn for this block.
    pub(crate) burned_mana: u64,
}

impl BasicBlock {
    pub const KIND: u8 = 0;

    /// Returns the strong parents of a [`BasicBlock`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        &self.data.strong_parents
    }

    /// Returns the weak parents of a [`BasicBlock`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        &self.data.weak_parents
    }

    /// Returns the shallow like parents of a [`BasicBlock`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        &self.data.shallow_like_parents
    }

    /// Returns the optional payload of a [`BasicBlock`].
    #[inline(always)]
    pub fn payload(&self) -> Option<&Payload> {
        self.data.payload.as_ref()
    }

    /// Returns the burned mana of a [`BasicBlock`].
    #[inline(always)]
    pub fn burned_mana(&self) -> u64 {
        self.data.burned_mana
    }

    /// Returns the work score of a [`BasicBlock`].
    pub fn workscore(&self) -> u32 {
        let workscore_structure = self.protocol_parameters().work_score_structure;
        let workscore_bytes = workscore_structure.data_kilobyte * self.packed_len() as u32;
        let mut workscore_kilobytes = workscore_bytes / 1024;

        workscore_kilobytes += self.workscore_header(workscore_structure);
        workscore_kilobytes += self.data.workscore(workscore_structure);
        workscore_kilobytes += self.workscore_signature(workscore_structure);
        workscore_kilobytes
    }
}

impl WorkScore for BasicBlockData {
    fn workscore(&self, workscore_structure: WorkScoreStructure) -> u32 {
        // offset for block, plus missing parents, plus payload.
        let mut score = workscore_structure.block;

        let min_strong_parents_threshold = workscore_structure.min_strong_parents_threshold as usize;
        if self.strong_parents.len() < min_strong_parents_threshold {
            let missing_parents_count = min_strong_parents_threshold - self.strong_parents.len();
            score += workscore_structure.missing_parent * missing_parents_count as u32;
        }

        if let Some(payload) = &*self.payload {
            score += payload.workscore(workscore_structure);
        }

        score
    }
}

impl Packable for BasicBlockData {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
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
        let strong_parents = StrongParents::unpack::<_, VERIFY>(unpacker, &())?;
        let weak_parents = WeakParents::unpack::<_, VERIFY>(unpacker, &())?;
        let shallow_like_parents = ShallowLikeParents::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_parents(&strong_parents, &weak_parents, &shallow_like_parents).map_err(UnpackError::Packable)?;
        }

        let payload = OptionalPayload::unpack::<_, VERIFY>(unpacker, visitor)?;

        let burned_mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        Ok(Self {
            strong_parents,
            weak_parents,
            shallow_like_parents,
            payload,
            burned_mana,
        })
    }
}

impl Packable for BasicBlock {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.pack_header(packer)?;
        Self::KIND.pack(packer)?;
        self.data.pack(packer)?;
        Signature::Ed25519(self.signature).pack(packer)?;

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

        if kind != Self::KIND {
            return Err(Error::InvalidBlockKind(kind)).map_err(UnpackError::Packable);
        }

        let data = BasicBlockData::unpack::<_, VERIFY>(unpacker, protocol_params)?;

        let Signature::Ed25519(signature) = Signature::unpack::<_, VERIFY>(unpacker, &())?;

        let block = Self {
            protocol_params: protocol_params.clone(),
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            data,
            signature,
        };

        if VERIFY {
            let block_len = if let (Some(start), Some(end)) = (start_opt, unpacker.read_bytes()) {
                end - start
            } else {
                block.packed_len()
            };

            if block_len > Block::LENGTH_MAX {
                return Err(UnpackError::Packable(Error::InvalidBlockLength(block_len)));
            }
        }

        Ok(block)
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::collections::BTreeSet;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{
        block::{payload::dto::PayloadDto, BlockId, Error},
        TryFromDto, ValidationParams,
    };

    /// A basic block.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BasicBlockDataDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub strong_parents: BTreeSet<BlockId>,
        pub weak_parents: BTreeSet<BlockId>,
        pub shallow_like_parents: BTreeSet<BlockId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
        #[serde(with = "crate::utils::serde::string")]
        pub burned_mana: u64,
    }

    impl From<&BasicBlockData> for BasicBlockDataDto {
        fn from(value: &BasicBlockData) -> Self {
            Self {
                kind: BasicBlock::KIND,
                strong_parents: value.strong_parents.to_set(),
                weak_parents: value.weak_parents.to_set(),
                shallow_like_parents: value.shallow_like_parents.to_set(),
                payload: value.payload.as_ref().map(Into::into),
                burned_mana: value.burned_mana,
            }
        }
    }

    impl TryFromDto for BasicBlockData {
        type Dto = BasicBlockDataDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            Ok(Self {
                strong_parents: StrongParents::from_set(dto.strong_parents)?,
                weak_parents: WeakParents::from_set(dto.weak_parents)?,
                shallow_like_parents: ShallowLikeParents::from_set(dto.shallow_like_parents)?,
                payload: dto
                    .payload
                    .map(|payload| Payload::try_from_dto_with_params_inner(payload, params))
                    .transpose()?
                    .into(),
                burned_mana: dto.burned_mana,
            })
        }
    }
}
