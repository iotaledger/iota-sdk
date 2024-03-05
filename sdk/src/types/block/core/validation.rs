// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;

use crate::types::block::{
    core::{parent::verify_parents_sets, BlockBody, BlockError, Parents},
    protocol::{ProtocolParameters, ProtocolParametersError, ProtocolParametersHash},
};

pub type StrongParents = Parents<1, 50>;
pub type WeakParents = Parents<0, 50>;
pub type ShallowLikeParents = Parents<0, 50>;

/// A builder for a [`ValidationBlockBody`].
pub struct ValidationBlockBodyBuilder {
    strong_parents: StrongParents,
    weak_parents: WeakParents,
    shallow_like_parents: ShallowLikeParents,
    highest_supported_version: u8,
    protocol_parameters_hash: ProtocolParametersHash,
}

impl ValidationBlockBodyBuilder {
    /// Creates a new [`ValidationBlockBodyBuilder`].
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

    /// Adds strong parents to a [`ValidationBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_strong_parents(mut self, strong_parents: impl Into<StrongParents>) -> Self {
        self.strong_parents = strong_parents.into();
        self
    }

    /// Adds weak parents to a [`ValidationBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`ValidationBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.shallow_like_parents = shallow_like_parents.into();
        self
    }

    /// Adds a highest supported version to a [`ValidationBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_highest_supported_version(mut self, highest_supported_version: u8) -> Self {
        self.highest_supported_version = highest_supported_version;
        self
    }

    /// Adds a protocol parameter hash to a [`ValidationBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_protocol_parameters_hash(mut self, protocol_parameters_hash: ProtocolParametersHash) -> Self {
        self.protocol_parameters_hash = protocol_parameters_hash;
        self
    }

    /// Finishes the builder into a [`ValidationBlockBody`].
    pub fn finish(self) -> Result<ValidationBlockBody, BlockError> {
        verify_parents_sets(&self.strong_parents, &self.weak_parents, &self.shallow_like_parents)?;

        Ok(ValidationBlockBody {
            strong_parents: self.strong_parents,
            weak_parents: self.weak_parents,
            shallow_like_parents: self.shallow_like_parents,
            highest_supported_version: self.highest_supported_version,
            protocol_parameters_hash: self.protocol_parameters_hash,
        })
    }

    /// Finishes the builder into a [`BlockBody`].
    pub fn finish_block_body(self) -> Result<BlockBody, BlockError> {
        Ok(BlockBody::from(self.finish()?))
    }
}

impl From<ValidationBlockBody> for ValidationBlockBodyBuilder {
    fn from(value: ValidationBlockBody) -> Self {
        Self {
            strong_parents: value.strong_parents,
            weak_parents: value.weak_parents,
            shallow_like_parents: value.shallow_like_parents,
            highest_supported_version: value.highest_supported_version,
            protocol_parameters_hash: value.protocol_parameters_hash,
        }
    }
}

/// A Validation Block Body is a special type of block body used by validators to secure the network. It is recognized
/// by the Congestion Control of the IOTA 2.0 protocol and can be issued without burning Mana within the constraints of
/// the allowed validator throughput. It is allowed to reference more parent blocks than a normal Basic Block Body.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = BlockError)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(verify_with = verify_validation_block_body)]
pub struct ValidationBlockBody {
    /// Blocks that are strongly directly approved.
    strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    shallow_like_parents: ShallowLikeParents,
    /// The highest supported protocol version the issuer of this block supports.
    highest_supported_version: u8,
    /// The hash of the protocol parameters for the Highest Supported Version.
    #[packable(verify_with = verify_protocol_parameters_hash)]
    protocol_parameters_hash: ProtocolParametersHash,
}

impl ValidationBlockBody {
    pub const KIND: u8 = 1;

    /// Returns the strong parents of a [`ValidationBlockBody`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        &self.strong_parents
    }

    /// Returns the weak parents of a [`ValidationBlockBody`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        &self.weak_parents
    }

    /// Returns the shallow like parents of a [`ValidationBlockBody`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        &self.shallow_like_parents
    }

    /// Returns the highest supported protocol version of a [`ValidationBlockBody`].
    #[inline(always)]
    pub fn highest_supported_version(&self) -> u8 {
        self.highest_supported_version
    }

    /// Returns the protocol parameters hash of a [`ValidationBlockBody`].
    #[inline(always)]
    pub fn protocol_parameters_hash(&self) -> ProtocolParametersHash {
        self.protocol_parameters_hash
    }
}

fn verify_protocol_parameters_hash(
    hash: &ProtocolParametersHash,
    params: &ProtocolParameters,
) -> Result<(), ProtocolParametersError> {
    let params_hash = params.hash();

    if hash != &params_hash {
        return Err(ProtocolParametersError::Hash {
            expected: params_hash,
            actual: *hash,
        });
    }

    Ok(())
}

fn verify_validation_block_body(
    validation_block_body: &ValidationBlockBody,
    _: &ProtocolParameters,
) -> Result<(), BlockError> {
    verify_parents_sets(
        &validation_block_body.strong_parents,
        &validation_block_body.weak_parents,
        &validation_block_body.shallow_like_parents,
    )?;

    Ok(())
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::collections::BTreeSet;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{block::BlockId, TryFromDto};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ValidationBlockBodyDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub strong_parents: BTreeSet<BlockId>,
        #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
        pub weak_parents: BTreeSet<BlockId>,
        #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
        pub shallow_like_parents: BTreeSet<BlockId>,
        pub highest_supported_version: u8,
        pub protocol_parameters_hash: ProtocolParametersHash,
    }

    impl From<&ValidationBlockBody> for ValidationBlockBodyDto {
        fn from(value: &ValidationBlockBody) -> Self {
            Self {
                kind: ValidationBlockBody::KIND,
                strong_parents: value.strong_parents.to_set(),
                weak_parents: value.weak_parents.to_set(),
                shallow_like_parents: value.shallow_like_parents.to_set(),
                highest_supported_version: value.highest_supported_version,
                protocol_parameters_hash: value.protocol_parameters_hash,
            }
        }
    }

    impl TryFromDto<ValidationBlockBodyDto> for ValidationBlockBody {
        type Error = BlockError;

        fn try_from_dto_with_params_inner(
            dto: ValidationBlockBodyDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            if let Some(params) = params {
                verify_protocol_parameters_hash(&dto.protocol_parameters_hash, params)?;
            }

            ValidationBlockBodyBuilder::new(
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
