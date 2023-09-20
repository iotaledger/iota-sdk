// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use super::{BlockBuilder, BlockWrapper};
use crate::types::block::{
    core::{verify_parents, Block},
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    protocol::{ProtocolParameters, ProtocolParametersHash},
    signature::Ed25519Signature,
    Error,
};

/// A builder for a [`ValidationBlock`].
pub struct ValidationBlockBuilder {
    strong_parents: StrongParents,
    weak_parents: WeakParents,
    shallow_like_parents: ShallowLikeParents,
    highest_supported_version: u8,
    protocol_parameters_hash: ProtocolParametersHash,
}

impl ValidationBlockBuilder {
    /// Creates a new [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn new(
        strong_parents: StrongParents,
        highest_supported_version: u8,
        protocol_parameters_hash: ProtocolParametersHash,
    ) -> Self {
        Self {
            strong_parents,
            weak_parents: WeakParents::default(),
            shallow_like_parents: ShallowLikeParents::default(),
            highest_supported_version,
            protocol_parameters_hash,
        }
    }

    /// Adds strong parents to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_strong_parents(mut self, strong_parents: impl Into<StrongParents>) -> Self {
        self.strong_parents = strong_parents.into();
        self
    }

    /// Adds weak parents to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.shallow_like_parents = shallow_like_parents.into();
        self
    }

    /// Adds a highest supported version to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_highest_supported_version(mut self, highest_supported_version: u8) -> Self {
        self.highest_supported_version = highest_supported_version;
        self
    }

    /// Adds a protocol parameter hash to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_protocol_parameters_hash(mut self, protocol_parameters_hash: ProtocolParametersHash) -> Self {
        self.protocol_parameters_hash = protocol_parameters_hash;
        self
    }

    // TODO: It's bothersome that this is duplicated code
    pub(crate) fn pack_block<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        ValidationBlock::KIND.pack(packer)?;
        self.strong_parents.pack(packer)?;
        self.weak_parents.pack(packer)?;
        self.shallow_like_parents.pack(packer)?;
        self.highest_supported_version.pack(packer)?;
        self.protocol_parameters_hash.pack(packer)?;

        Ok(())
    }

    pub(crate) fn hash(&self) -> [u8; 32] {
        let mut bytes = Vec::new();
        self.pack_block(&mut bytes).unwrap();
        Blake2b256::digest(bytes).into()
    }

    /// Finishes the builder into a [`ValidationBlock`].
    pub fn finish(self) -> Result<ValidationBlock, Error> {
        verify_parents(&self.strong_parents, &self.weak_parents, &self.shallow_like_parents)?;

        Ok(ValidationBlock {
            strong_parents: self.strong_parents,
            weak_parents: self.weak_parents,
            shallow_like_parents: self.shallow_like_parents,
            highest_supported_version: self.highest_supported_version,
            protocol_parameters_hash: self.protocol_parameters_hash,
        })
    }

    /// Finishes the builder into a [`Block`].
    pub fn finish_block(self) -> Result<Block, Error> {
        Ok(Block::from(self.finish()?))
    }
}

impl BlockBuilder<ValidationBlockBuilder> {
    /// Adds strong parents to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_strong_parents(mut self, strong_parents: impl Into<StrongParents>) -> Self {
        self.block = self.block.with_strong_parents(strong_parents);
        self
    }

    /// Adds weak parents to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.block = self.block.with_weak_parents(weak_parents);
        self
    }

    /// Adds shallow like parents to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.block = self.block.with_shallow_like_parents(shallow_like_parents);
        self
    }

    /// Adds a highest supported version to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_highest_supported_version(mut self, highest_supported_version: u8) -> Self {
        self.block = self.block.with_highest_supported_version(highest_supported_version);
        self
    }

    /// Adds a protocol parameter hash to a [`ValidationBlockBuilder`].
    #[inline(always)]
    pub fn with_protocol_parameters_hash(mut self, protocol_parameters_hash: ProtocolParametersHash) -> Self {
        self.block = self.block.with_protocol_parameters_hash(protocol_parameters_hash);
        self
    }

    /// Get the signing input that can be used to generate an
    /// [`Ed25519Signature`](crate::types::block::signature::Ed25519Signature) for the resulting block.
    pub fn signing_input(&self) -> Vec<u8> {
        [self.header_hash(), self.block.hash()].concat()
    }

    pub fn finish(self, signature: Ed25519Signature) -> Result<BlockWrapper, Error> {
        Ok(BlockWrapper::new(
            &self.protocol_parameters,
            // TODO provide a sensible default
            self.issuing_time.ok_or(Error::InvalidField("issuing time"))?,
            self.slot_commitment_id,
            self.latest_finalized_slot,
            self.issuer_id,
            self.block.finish_block()?,
            signature,
        ))
    }
}

impl From<ValidationBlock> for ValidationBlockBuilder {
    fn from(value: ValidationBlock) -> Self {
        Self {
            strong_parents: value.strong_parents,
            weak_parents: value.weak_parents,
            shallow_like_parents: value.shallow_like_parents,
            highest_supported_version: value.highest_supported_version,
            protocol_parameters_hash: value.protocol_parameters_hash,
        }
    }
}

/// A Validation Block is a special type of block used by validators to secure the network. It is recognized by the
/// Congestion Control of the IOTA 2.0 protocol and can be issued without burning Mana within the constraints of the
/// allowed validator throughput. It is allowed to reference more parent blocks than a normal Basic Block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationBlock {
    /// Blocks that are strongly directly approved.
    strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    shallow_like_parents: ShallowLikeParents,
    /// The highest supported protocol version the issuer of this block supports.
    highest_supported_version: u8,
    /// The hash of the protocol parameters for the Highest Supported Version.
    protocol_parameters_hash: ProtocolParametersHash,
}

impl ValidationBlock {
    pub const KIND: u8 = 1;

    /// Returns the strong parents of a [`ValidationBlock`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        &self.strong_parents
    }

    /// Returns the weak parents of a [`ValidationBlock`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        &self.weak_parents
    }

    /// Returns the shallow like parents of a [`ValidationBlock`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        &self.shallow_like_parents
    }

    /// Returns the highest supported protocol version of a [`ValidationBlock`].
    #[inline(always)]
    pub fn highest_supported_version(&self) -> u8 {
        self.highest_supported_version
    }

    /// Returns the protocol parameters hash of a [`ValidationBlock`].
    #[inline(always)]
    pub fn protocol_parameters_hash(&self) -> ProtocolParametersHash {
        self.protocol_parameters_hash
    }
}

impl Packable for ValidationBlock {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.strong_parents.pack(packer)?;
        self.weak_parents.pack(packer)?;
        self.shallow_like_parents.pack(packer)?;
        self.highest_supported_version.pack(packer)?;
        self.protocol_parameters_hash.pack(packer)?;

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

        let highest_supported_version = u8::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        let protocol_parameters_hash = ProtocolParametersHash::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        if VERIFY {
            validate_protocol_params_hash(&protocol_parameters_hash, visitor).map_err(UnpackError::Packable)?;
        }

        Ok(Self {
            strong_parents,
            weak_parents,
            shallow_like_parents,
            highest_supported_version,
            protocol_parameters_hash,
        })
    }
}

fn validate_protocol_params_hash(hash: &ProtocolParametersHash, params: &ProtocolParameters) -> Result<(), Error> {
    let params_hash = params.hash();

    if hash != &params_hash {
        return Err(Error::InvalidProtocolParametersHash {
            expected: params_hash,
            actual: *hash,
        });
    }

    Ok(())
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::collections::BTreeSet;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{
        block::{BlockId, Error},
        TryFromDto, ValidationParams,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ValidationBlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub strong_parents: BTreeSet<BlockId>,
        pub weak_parents: BTreeSet<BlockId>,
        pub shallow_like_parents: BTreeSet<BlockId>,
        pub highest_supported_version: u8,
        pub protocol_parameters_hash: ProtocolParametersHash,
    }

    impl From<&ValidationBlock> for ValidationBlockDto {
        fn from(value: &ValidationBlock) -> Self {
            Self {
                kind: ValidationBlock::KIND,
                strong_parents: value.strong_parents.to_set(),
                weak_parents: value.weak_parents.to_set(),
                shallow_like_parents: value.shallow_like_parents.to_set(),
                highest_supported_version: value.highest_supported_version,
                protocol_parameters_hash: value.protocol_parameters_hash,
            }
        }
    }

    impl TryFromDto for ValidationBlock {
        type Dto = ValidationBlockDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            if let Some(protocol_params) = params.protocol_parameters() {
                validate_protocol_params_hash(&dto.protocol_parameters_hash, protocol_params)?;
            }

            ValidationBlockBuilder::new(
                StrongParents::from_set(dto.strong_parents)?,
                dto.highest_supported_version,
                dto.protocol_parameters_hash,
            )
            .with_weak_parents(WeakParents::from_set(dto.weak_parents)?)
            .with_shallow_like_parents(ShallowLikeParents::from_set(dto.shallow_like_parents)?)
            .finish()
        }
    }
}
