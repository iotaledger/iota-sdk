// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{
    core::{parent::verify_parents_sets, BlockBody, Parents},
    payload::{OptionalPayload, Payload},
    protocol::ProtocolParameters,
    Error,
};

pub type StrongParents = Parents<1, 8>;
pub type WeakParents = Parents<0, 8>;
pub type ShallowLikeParents = Parents<0, 8>;

/// A builder for a [`BasicBlockBody`].
pub struct BasicBlockBodyBuilder {
    strong_parents: StrongParents,
    weak_parents: WeakParents,
    shallow_like_parents: ShallowLikeParents,
    payload: OptionalPayload,
    max_burned_mana: u64,
}

impl BasicBlockBodyBuilder {
    /// Creates a new [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn new(strong_parents: StrongParents, max_burned_mana: u64) -> Self {
        Self {
            strong_parents,
            weak_parents: WeakParents::default(),
            shallow_like_parents: ShallowLikeParents::default(),
            payload: OptionalPayload::default(),
            max_burned_mana,
        }
    }

    /// Adds strong parents to a [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_strong_parents(mut self, strong_parents: impl Into<StrongParents>) -> Self {
        self.strong_parents = strong_parents.into();
        self
    }

    /// Adds weak parents to a [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_weak_parents(mut self, weak_parents: impl Into<WeakParents>) -> Self {
        self.weak_parents = weak_parents.into();
        self
    }

    /// Adds shallow like parents to a [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_shallow_like_parents(mut self, shallow_like_parents: impl Into<ShallowLikeParents>) -> Self {
        self.shallow_like_parents = shallow_like_parents.into();
        self
    }

    /// Adds a payload to a [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Adds max burned mana to a [`BasicBlockBodyBuilder`].
    #[inline(always)]
    pub fn with_max_burned_mana(mut self, max_burned_mana: u64) -> Self {
        self.max_burned_mana = max_burned_mana;
        self
    }

    /// Finishes the builder into a [`BasicBlockBody`].
    pub fn finish(self) -> Result<BasicBlockBody, Error> {
        verify_parents_sets(&self.strong_parents, &self.weak_parents, &self.shallow_like_parents)?;

        Ok(BasicBlockBody {
            strong_parents: self.strong_parents,
            weak_parents: self.weak_parents,
            shallow_like_parents: self.shallow_like_parents,
            payload: self.payload,
            max_burned_mana: self.max_burned_mana,
        })
    }

    /// Finishes the builder into a [`BlockBody`].
    pub fn finish_block_body(self) -> Result<BlockBody, Error> {
        Ok(BlockBody::from(self.finish()?))
    }
}

impl From<BasicBlockBody> for BasicBlockBodyBuilder {
    fn from(value: BasicBlockBody) -> Self {
        Self {
            strong_parents: value.strong_parents,
            weak_parents: value.weak_parents,
            shallow_like_parents: value.shallow_like_parents,
            payload: value.payload,
            max_burned_mana: value.max_burned_mana,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicBlockBody {
    /// Blocks that are strongly directly approved.
    strong_parents: StrongParents,
    /// Blocks that are weakly directly approved.
    weak_parents: WeakParents,
    /// Blocks that are directly referenced to adjust opinion.
    shallow_like_parents: ShallowLikeParents,
    /// The optional [`Payload`] of the block.
    payload: OptionalPayload,
    /// The amount of Mana the Account identified by [`IssuerId`](super::IssuerId) is at most willing to burn for this
    /// block.
    max_burned_mana: u64,
}

impl BasicBlockBody {
    pub const KIND: u8 = 0;

    /// Returns the strong parents of a [`BasicBlockBody`].
    #[inline(always)]
    pub fn strong_parents(&self) -> &StrongParents {
        &self.strong_parents
    }

    /// Returns the weak parents of a [`BasicBlockBody`].
    #[inline(always)]
    pub fn weak_parents(&self) -> &WeakParents {
        &self.weak_parents
    }

    /// Returns the shallow like parents of a [`BasicBlockBody`].
    #[inline(always)]
    pub fn shallow_like_parents(&self) -> &ShallowLikeParents {
        &self.shallow_like_parents
    }

    /// Returns the optional payload of a [`BasicBlockBody`].
    #[inline(always)]
    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }

    /// Returns the max burned mana of a [`BasicBlockBody`].
    #[inline(always)]
    pub fn max_burned_mana(&self) -> u64 {
        self.max_burned_mana
    }
}

impl Packable for BasicBlockBody {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.strong_parents.pack(packer)?;
        self.weak_parents.pack(packer)?;
        self.shallow_like_parents.pack(packer)?;
        self.payload.pack(packer)?;
        self.max_burned_mana.pack(packer)?;

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
            verify_parents_sets(&strong_parents, &weak_parents, &shallow_like_parents)
                .map_err(UnpackError::Packable)?;
        }

        let payload = OptionalPayload::unpack::<_, VERIFY>(unpacker, visitor)?;

        let max_burned_mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        Ok(Self {
            strong_parents,
            weak_parents,
            shallow_like_parents,
            payload,
            max_burned_mana,
        })
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

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BasicBlockBodyDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub strong_parents: BTreeSet<BlockId>,
        pub weak_parents: BTreeSet<BlockId>,
        pub shallow_like_parents: BTreeSet<BlockId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
        #[serde(with = "crate::utils::serde::string")]
        pub max_burned_mana: u64,
    }

    impl From<&BasicBlockBody> for BasicBlockBodyDto {
        fn from(value: &BasicBlockBody) -> Self {
            Self {
                kind: BasicBlockBody::KIND,
                strong_parents: value.strong_parents.to_set(),
                weak_parents: value.weak_parents.to_set(),
                shallow_like_parents: value.shallow_like_parents.to_set(),
                payload: value.payload.as_ref().map(Into::into),
                max_burned_mana: value.max_burned_mana,
            }
        }
    }

    impl TryFromDto for BasicBlockBody {
        type Dto = BasicBlockBodyDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: &ValidationParams<'_>) -> Result<Self, Self::Error> {
            BasicBlockBodyBuilder::new(StrongParents::from_set(dto.strong_parents)?, dto.max_burned_mana)
                .with_weak_parents(WeakParents::from_set(dto.weak_parents)?)
                .with_shallow_like_parents(ShallowLikeParents::from_set(dto.shallow_like_parents)?)
                .with_payload(
                    dto.payload
                        .map(|payload| Payload::try_from_dto_with_params_inner(payload, params))
                        .transpose()?,
                )
                .finish()
        }
    }
}
