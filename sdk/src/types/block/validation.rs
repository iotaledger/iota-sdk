// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use super::{
    core::verify_parents,
    parent::{ShallowLikeParents, StrongParents, WeakParents},
    protocol::{ProtocolParameters, ProtocolParametersHash},
    Error,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationBlock {
    /// Blocks that are strongly directly approved.
    pub(crate) strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    pub(crate) weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    pub(crate) shallow_like_parents: ShallowLikeParents,
    /// The highest supported protocol version the issuer of this block supports.
    pub(crate) highest_supported_version: u8,
    /// The hash of the protocol parameters for the Highest Supported Version.
    pub(crate) protocol_parameters_hash: ProtocolParametersHash,
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

    /// Returns the shallow like parents of a [`ValidationBlock`].
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

        // TODO: Is this actually right/needed?
        if VERIFY {
            validate_protocol_params_hash(&protocol_parameters_hash, visitor).map_err(UnpackError::Packable)?;
        }

        let block = Self {
            strong_parents,
            weak_parents,
            shallow_like_parents,
            highest_supported_version,
            protocol_parameters_hash,
        };

        Ok(block)
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

pub(crate) mod dto {
    use alloc::collections::BTreeSet;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{
        block::{BlockId, Error},
        TryFromDto, ValidationParams,
    };

    /// The block object that nodes gossip around in the network.
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
                strong_parents: value.strong_parents().to_set(),
                weak_parents: value.weak_parents().to_set(),
                shallow_like_parents: value.shallow_like_parents().to_set(),
                highest_supported_version: value.highest_supported_version(),
                protocol_parameters_hash: value.protocol_parameters_hash(),
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
            Ok(Self {
                strong_parents: StrongParents::from_set(dto.strong_parents)?,
                weak_parents: WeakParents::from_set(dto.weak_parents)?,
                shallow_like_parents: ShallowLikeParents::from_set(dto.shallow_like_parents)?,
                highest_supported_version: dto.highest_supported_version,
                protocol_parameters_hash: dto.protocol_parameters_hash,
            })
        }
    }
}
