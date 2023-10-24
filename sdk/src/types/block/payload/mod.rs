// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The payload module defines the core data types for representing block payloads.

pub mod candidacy_announcement;
pub mod tagged_data;
pub mod transaction;

use alloc::boxed::Box;
use core::ops::Deref;

use derive_more::From;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

pub use self::{
    candidacy_announcement::CandidacyAnnouncementPayload, tagged_data::TaggedDataPayload,
    transaction::TransactionPayload,
};
pub(crate) use self::{
    tagged_data::{TagLength, TaggedDataLength},
    transaction::{ContextInputCount, InputCount, OutputCount},
};
use crate::types::block::{protocol::ProtocolParameters, Error};

/// A generic payload that can represent different types defining block payloads.
#[derive(Clone, Eq, PartialEq, From)]
pub enum Payload {
    /// A transaction payload.
    Transaction(Box<TransactionPayload>),
    /// A tagged data payload.
    TaggedData(Box<TaggedDataPayload>),
    /// A candidacy announcement payload.
    CandidacyAnnouncement(CandidacyAnnouncementPayload),
}

impl core::fmt::Debug for Payload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transaction(payload) => payload.fmt(f),
            Self::TaggedData(payload) => payload.fmt(f),
            Self::CandidacyAnnouncement(payload) => payload.fmt(f),
        }
    }
}

impl From<TransactionPayload> for Payload {
    fn from(payload: TransactionPayload) -> Self {
        Self::Transaction(Box::new(payload))
    }
}

impl From<TaggedDataPayload> for Payload {
    fn from(payload: TaggedDataPayload) -> Self {
        Self::TaggedData(Box::new(payload))
    }
}

impl Payload {
    /// Returns the payload kind of a `Payload`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Transaction(_) => TransactionPayload::KIND,
            Self::TaggedData(_) => TaggedDataPayload::KIND,
            Self::CandidacyAnnouncement(_) => CandidacyAnnouncementPayload::KIND,
        }
    }

    crate::def_is_as_opt!(Payload: Transaction, TaggedData);
}

impl Packable for Payload {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Transaction(transaction) => {
                TransactionPayload::KIND.pack(packer)?;
                transaction.pack(packer)
            }
            Self::TaggedData(tagged_data) => {
                TaggedDataPayload::KIND.pack(packer)?;
                tagged_data.pack(packer)
            }
            Self::CandidacyAnnouncement(_) => CandidacyAnnouncementPayload::KIND.pack(packer),
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            TransactionPayload::KIND => {
                Self::from(TransactionPayload::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            }
            TaggedDataPayload::KIND => Self::from(TaggedDataPayload::unpack::<_, VERIFY>(unpacker, &()).coerce()?),
            CandidacyAnnouncementPayload::KIND => Self::from(CandidacyAnnouncementPayload),
            k => return Err(UnpackError::Packable(Error::InvalidPayloadKind(k))),
        })
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

    pub use super::transaction::dto::TransactionPayloadDto;
    use super::*;
    use crate::types::{block::Error, TryFromDto, ValidationParams};

    /// Describes all the different payload types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum PayloadDto {
        Transaction(Box<TransactionPayloadDto>),
        TaggedData(Box<TaggedDataPayload>),
        CandidacyAnnouncement,
    }

    impl From<TransactionPayloadDto> for PayloadDto {
        fn from(payload: TransactionPayloadDto) -> Self {
            Self::Transaction(Box::new(payload))
        }
    }

    impl From<TaggedDataPayload> for PayloadDto {
        fn from(payload: TaggedDataPayload) -> Self {
            Self::TaggedData(Box::new(payload))
        }
    }

    impl From<&Payload> for PayloadDto {
        fn from(value: &Payload) -> Self {
            match value {
                Payload::Transaction(p) => Self::Transaction(Box::new(TransactionPayloadDto::from(p.as_ref()))),
                Payload::TaggedData(p) => Self::TaggedData(p.clone()),
                Payload::CandidacyAnnouncement(_) => Self::CandidacyAnnouncement,
            }
        }
    }

    impl TryFromDto for Payload {
        type Dto = PayloadDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            Ok(match dto {
                PayloadDto::Transaction(p) => {
                    Self::from(TransactionPayload::try_from_dto_with_params_inner(*p, params)?)
                }
                PayloadDto::TaggedData(p) => Self::from(*p),
                PayloadDto::CandidacyAnnouncement => Self::from(CandidacyAnnouncementPayload),
            })
        }
    }
}
