// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The payload module defines the core data types for representing block payloads.

pub mod candidacy_announcement;
pub mod signed_transaction;
pub mod tagged_data;

use alloc::{boxed::Box, string::String};
use core::{convert::Infallible, ops::Deref};

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
    capabilities::CapabilityError,
    context_input::ContextInputError,
    input::{InputError, UtxoInput},
    mana::ManaError,
    output::{
        feature::FeatureError, unlock_condition::UnlockConditionError, ChainId, NativeTokenError, OutputError,
        TokenSchemeError,
    },
    payload::tagged_data::{TagLength, TaggedDataLength},
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    semantic::TransactionFailureReason,
    unlock::UnlockError,
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum PayloadError {
    #[display(fmt = "invalid payload kind: {_0}")]
    InvalidPayloadKind(u8),
    #[display(fmt = "invalid payload length: expected {expected} but got {actual}")]
    InvalidPayloadLength { expected: usize, actual: usize },
    #[display(fmt = "invalid timestamp: {_0}")]
    InvalidTimestamp(String),
    #[display(fmt = "invalid network id: {_0}")]
    InvalidNetworkId(String),
    #[display(fmt = "network ID mismatch: expected {expected} but got {actual}")]
    NetworkIdMismatch { expected: u64, actual: u64 },
    #[display(fmt = "invalid tagged data length: {_0}")]
    InvalidTaggedDataLength(<TaggedDataLength as TryFrom<usize>>::Error),
    #[display(fmt = "invalid tag length: {_0}")]
    InvalidTagLength(<TagLength as TryFrom<usize>>::Error),
    #[display(fmt = "invalid input count: {_0}")]
    InvalidInputCount(<InputCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid output count: {_0}")]
    InvalidOutputCount(<OutputCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid transaction amount sum: {_0}")]
    InvalidTransactionAmountSum(u128),
    #[display(fmt = "duplicate output chain: {_0}")]
    DuplicateOutputChain(ChainId),
    #[display(fmt = "duplicate UTXO {_0} in inputs")]
    DuplicateUtxo(UtxoInput),
    #[display(fmt = "missing creation slot")]
    MissingCreationSlot,
    #[display(fmt = "input count and unlock count mismatch: {input_count} != {unlock_count}")]
    InputUnlockCountMismatch { input_count: usize, unlock_count: usize },
    #[from]
    TransactionSemantic(TransactionFailureReason),
    #[from]
    Input(InputError),
    #[from]
    Output(OutputError),
    #[from]
    Unlock(UnlockError),
    #[from]
    ContextInput(ContextInputError),
    #[from]
    Capabilities(CapabilityError),
}

#[cfg(feature = "std")]
impl std::error::Error for PayloadError {}

macro_rules! impl_from_error_via {
    ($via:ident: $($err:ident),+$(,)?) => {
        $(
        impl From<$err> for PayloadError {
            fn from(error: $err) -> Self {
                Self::from($via::from(error))
            }
        }
        )+
    };
}
impl_from_error_via!(OutputError: NativeTokenError, ManaError, UnlockConditionError, FeatureError, TokenSchemeError);

impl From<Infallible> for PayloadError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

/// A generic payload that can represent different types defining block payloads.
#[derive(Clone, Eq, PartialEq, From, Packable)]
#[packable(unpack_error = PayloadError)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = PayloadError::InvalidPayloadKind)]
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
    type UnpackError = PayloadError;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match &self.0 {
            None => 0u32.pack(packer),
            Some(payload) => Self::pack_ref(payload, packer),
        }
    }

    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let len = u32::unpack_inner(unpacker, visitor).coerce()? as usize;

        if len > 0 {
            unpacker.ensure_bytes(len)?;

            let start_opt = unpacker.read_bytes();

            let payload = Payload::unpack(unpacker, visitor)?;

            let actual_len = if let (Some(start), Some(end)) = (start_opt, unpacker.read_bytes()) {
                end - start
            } else {
                payload.packed_len()
            };

            if len != actual_len {
                Err(UnpackError::Packable(PayloadError::InvalidPayloadLength {
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
    use crate::types::TryFromDto;

    /// Describes all the different payload types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PayloadDto {
        TaggedData(Box<TaggedDataPayload>),
        SignedTransaction(Box<SignedTransactionPayloadDto>),
        CandidacyAnnouncement,
    }

    impl<'de> Deserialize<'de> for PayloadDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = serde_json::Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(serde_json::Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom(core::concat!("invalid PayloadDto type")))?
                    as u8
                {
                    TaggedDataPayload::KIND => Self::from(TaggedDataPayload::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(alloc::format!(
                            core::concat!("cannot deserialize TaggedDataPayload: {}"),
                            e
                        ))
                    })?),
                    SignedTransactionPayload::KIND => {
                        Self::from(SignedTransactionPayloadDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(alloc::format!(
                                core::concat!("cannot deserialize SignedTransactionPayload: {}"),
                                e
                            ))
                        })?)
                    }
                    CandidacyAnnouncementPayload::KIND => Self::CandidacyAnnouncement,
                    _ => return Err(serde::de::Error::custom(core::concat!("invalid PayloadDto type"))),
                },
            )
        }
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
        type Error = PayloadError;

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
