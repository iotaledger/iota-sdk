// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The payload module defines the core data types for representing block payloads.

pub mod candidacy_announcement;
pub mod signed_transaction;
pub mod tagged_data;

use alloc::boxed::Box;
use core::ops::Deref;

use derive_more::From;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

pub(crate) use self::signed_transaction::{InputCount, OutputCount};
pub use self::{
    candidacy_announcement::CandidacyAnnouncementPayload, signed_transaction::SignedTransactionPayload,
    tagged_data::TaggedDataPayload,
};
use crate::types::block::{
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    Error,
};

/// A generic payload that can represent different types defining block payloads.
#[derive(Clone, Eq, PartialEq, From, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = Error::InvalidPayloadKind)]
pub enum Payload {
    /// A tagged data payload.
    #[packable(tag = TaggedDataPayload::KIND)]
    TaggedData(Box<TaggedDataPayload>),
    /// A signed transaction payload.
    #[packable(tag = SignedTransactionPayload::KIND)]
    SignedTransaction(Box<SignedTransactionPayload>),
    /// A candidacy announcement payload.
    #[packable(tag = CandidacyAnnouncementPayload::KIND)]
    CandidacyAnnouncement(CandidacyAnnouncementPayload),
}

impl core::fmt::Debug for Payload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TaggedData(payload) => payload.fmt(f),
            Self::SignedTransaction(payload) => payload.fmt(f),
            Self::CandidacyAnnouncement(payload) => payload.fmt(f),
        }
    }
}

impl From<TaggedDataPayload> for Payload {
    fn from(payload: TaggedDataPayload) -> Self {
        Self::TaggedData(Box::new(payload))
    }
}

impl From<SignedTransactionPayload> for Payload {
    fn from(payload: SignedTransactionPayload) -> Self {
        Self::SignedTransaction(Box::new(payload))
    }
}

impl Payload {
    /// Returns the payload kind of a `Payload`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::TaggedData(_) => TaggedDataPayload::KIND,
            Self::SignedTransaction(_) => SignedTransactionPayload::KIND,
            Self::CandidacyAnnouncement(_) => CandidacyAnnouncementPayload::KIND,
        }
    }

    crate::def_is_as_opt!(Payload: TaggedData, SignedTransaction, CandidacyAnnouncement);
}

impl WorkScore for Payload {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::TaggedData(tagged_data) => tagged_data.work_score(params),
            Self::SignedTransaction(signed_transaction) => signed_transaction.work_score(params),
            Self::CandidacyAnnouncement(candidacy_announcement) => candidacy_announcement.work_score(params),
        }
    }
}

/// Representation of an optional [`Payload`].
/// Essentially an `Option<Payload>` with a different [`Packable`] implementation, to conform to specs.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptionalPayload(Option<Payload>);

impl OptionalPayload {
    fn pack_ref<P: Packer>(payload: &Payload, packer: &mut P) -> Result<(), P::Error> {
        (payload.packed_len() as u32).pack(packer)?;
        payload.pack(packer)
    }
}

impl<T: Into<Payload>> From<Option<T>> for OptionalPayload {
    fn from(payload: Option<T>) -> Self {
        Self(payload.map(|p| p.into()))
    }
}

impl<T: Into<Payload>> From<T> for OptionalPayload {
    fn from(payload: T) -> Self {
        Self(Some(payload.into()))
    }
}

impl Deref for OptionalPayload {
    type Target = Option<Payload>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Packable for OptionalPayload {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match &self.0 {
            None => 0u32.pack(packer),
            Some(payload) => Self::pack_ref(payload, packer),
        }
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let len = u32::unpack::<_, VERIFY>(unpacker, &()).coerce()? as usize;

        if len > 0 {
            unpacker.ensure_bytes(len)?;

            let start_opt = unpacker.read_bytes();

            let payload = Payload::unpack::<_, VERIFY>(unpacker, visitor)?;

            let actual_len = if let (Some(start), Some(end)) = (start_opt, unpacker.read_bytes()) {
                end - start
            } else {
                payload.packed_len()
            };

            if len != actual_len {
                Err(UnpackError::Packable(Error::InvalidPayloadLength {
                    expected: len,
                    actual: actual_len,
                }))
            } else {
                Ok(Self(Some(payload)))
            }
        } else {
            Ok(Self(None))
        }
    }
}

#[cfg(feature = "serde")]
pub mod dto {
    use serde::{Deserialize, Serialize};

    pub use super::signed_transaction::dto::SignedTransactionPayloadDto;
    use super::*;
    use crate::types::{block::Error, TryFromDto};

    /// Describes all the different payload types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum PayloadDto {
        TaggedData(Box<TaggedDataPayload>),
        SignedTransaction(Box<SignedTransactionPayloadDto>),
        CandidacyAnnouncement,
    }

    impl From<TaggedDataPayload> for PayloadDto {
        fn from(payload: TaggedDataPayload) -> Self {
            Self::TaggedData(Box::new(payload))
        }
    }

    impl From<SignedTransactionPayloadDto> for PayloadDto {
        fn from(payload: SignedTransactionPayloadDto) -> Self {
            Self::SignedTransaction(Box::new(payload))
        }
    }

    impl From<&Payload> for PayloadDto {
        fn from(value: &Payload) -> Self {
            match value {
                Payload::TaggedData(p) => Self::TaggedData(p.clone()),
                Payload::SignedTransaction(p) => {
                    Self::SignedTransaction(Box::new(SignedTransactionPayloadDto::from(p.as_ref())))
                }
                Payload::CandidacyAnnouncement(_) => Self::CandidacyAnnouncement,
            }
        }
    }

    impl TryFromDto<PayloadDto> for Payload {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: PayloadDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            Ok(match dto {
                PayloadDto::TaggedData(p) => Self::from(*p),
                PayloadDto::SignedTransaction(p) => {
                    Self::from(SignedTransactionPayload::try_from_dto_with_params_inner(*p, params)?)
                }
                PayloadDto::CandidacyAnnouncement => Self::from(CandidacyAnnouncementPayload),
            })
        }
    }
}
