// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The payload module defines the core data types for representing block payloads.

pub mod milestone;
pub mod tagged_data;
pub mod transaction;
pub mod treasury_transaction;

use alloc::boxed::Box;
use core::ops::Deref;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable, PackableExt,
};

pub(crate) use self::{
    milestone::{MilestoneMetadataLength, MilestoneOptionCount, ReceiptFundsCount, SignatureCount},
    tagged_data::{TagLength, TaggedDataLength},
    transaction::{InputCount, OutputCount},
};
pub use self::{
    milestone::{MilestoneOptions, MilestonePayload},
    tagged_data::TaggedDataPayload,
    transaction::TransactionPayload,
    treasury_transaction::TreasuryTransactionPayload,
};
use crate::types::block::{protocol::ProtocolParameters, Error};

/// A generic payload that can represent different types defining block payloads.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
pub enum Payload {
    /// A transaction payload.
    Transaction(TransactionPayload),
    /// A milestone payload.
    Milestone(MilestonePayload),
    /// A treasury transaction payload.
    TreasuryTransaction(TreasuryTransactionPayload),
    /// A tagged data payload.
    TaggedData(TaggedDataPayload),
}

impl core::fmt::Debug for Payload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transaction(payload) => payload.fmt(f),
            Self::Milestone(payload) => payload.fmt(f),
            Self::TreasuryTransaction(payload) => payload.fmt(f),
            Self::TaggedData(payload) => payload.fmt(f),
        }
    }
}

impl From<TransactionPayload> for Payload {
    fn from(payload: TransactionPayload) -> Self {
        Self::Transaction(payload)
    }
}

impl From<MilestonePayload> for Payload {
    fn from(payload: MilestonePayload) -> Self {
        Self::Milestone(payload)
    }
}

impl From<TreasuryTransactionPayload> for Payload {
    fn from(payload: TreasuryTransactionPayload) -> Self {
        Self::TreasuryTransaction(payload)
    }
}

impl From<TaggedDataPayload> for Payload {
    fn from(payload: TaggedDataPayload) -> Self {
        Self::TaggedData(payload)
    }
}

impl Payload {
    /// Returns the payload kind of a `Payload`.
    pub fn kind(&self) -> u32 {
        match self {
            Self::Transaction(_) => TransactionPayload::KIND,
            Self::Milestone(_) => MilestonePayload::KIND,
            Self::TreasuryTransaction(_) => TreasuryTransactionPayload::KIND,
            Self::TaggedData(_) => TaggedDataPayload::KIND,
        }
    }
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
            Self::Milestone(milestone) => {
                MilestonePayload::KIND.pack(packer)?;
                milestone.pack(packer)
            }
            Self::TreasuryTransaction(treasury_transaction) => {
                TreasuryTransactionPayload::KIND.pack(packer)?;
                treasury_transaction.pack(packer)
            }
            Self::TaggedData(tagged_data) => {
                TaggedDataPayload::KIND.pack(packer)?;
                tagged_data.pack(packer)
            }
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u32::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            TransactionPayload::KIND => {
                Self::from(TransactionPayload::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            }
            MilestonePayload::KIND => Self::from(MilestonePayload::unpack::<_, VERIFY>(unpacker, visitor).coerce()?),
            TreasuryTransactionPayload::KIND => {
                Self::from(TreasuryTransactionPayload::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            }
            TaggedDataPayload::KIND => Self::from(TaggedDataPayload::unpack::<_, VERIFY>(unpacker, &()).coerce()?),
            k => return Err(Error::InvalidPayloadKind(k)).map_err(UnpackError::Packable),
        })
    }
}

/// Representation of an optional [`Payload`].
/// Essentially an `Option<Payload>` with a different [`Packable`] implementation, to conform to specs.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OptionalPayload(Option<Box<Payload>>);

impl OptionalPayload {
    fn pack_ref<P: Packer>(payload: &Payload, packer: &mut P) -> Result<(), P::Error> {
        (payload.packed_len() as u32).pack(packer)?;
        payload.pack(packer)
    }
}

impl<T: Into<Payload>> From<Option<T>> for OptionalPayload {
    fn from(payload: Option<T>) -> Self {
        Self(payload.map(|p| Box::new(p.into())))
    }
}

impl<T: Into<Payload>> From<T> for OptionalPayload {
    fn from(payload: T) -> Self {
        Self(Some(Box::new(payload.into())))
    }
}

impl Deref for OptionalPayload {
    type Target = Option<Box<Payload>>;

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
                Ok(Self(Some(Box::new(payload))))
            }
        } else {
            Ok(Self(None))
        }
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    pub use super::{
        milestone::dto::MilestonePayloadDto, tagged_data::dto::TaggedDataPayloadDto,
        transaction::dto::TransactionPayloadDto, treasury_transaction::dto::TreasuryTransactionPayloadDto,
    };
    use crate::types::block::Error;

    /// Describes all the different payload types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum PayloadDto {
        Transaction(TransactionPayloadDto),
        Milestone(MilestonePayloadDto),
        TreasuryTransaction(TreasuryTransactionPayloadDto),
        TaggedData(TaggedDataPayloadDto),
    }

    impl From<TransactionPayloadDto> for PayloadDto {
        fn from(payload: TransactionPayloadDto) -> Self {
            Self::Transaction(payload)
        }
    }

    impl From<MilestonePayloadDto> for PayloadDto {
        fn from(payload: MilestonePayloadDto) -> Self {
            Self::Milestone(payload)
        }
    }

    impl From<TreasuryTransactionPayloadDto> for PayloadDto {
        fn from(payload: TreasuryTransactionPayloadDto) -> Self {
            Self::TreasuryTransaction(payload)
        }
    }

    impl From<TaggedDataPayloadDto> for PayloadDto {
        fn from(payload: TaggedDataPayloadDto) -> Self {
            Self::TaggedData(payload)
        }
    }

    impl From<&Payload> for PayloadDto {
        fn from(value: &Payload) -> Self {
            match value {
                Payload::Transaction(p) => Self::Transaction(TransactionPayloadDto::from(p)),
                Payload::Milestone(p) => Self::Milestone(MilestonePayloadDto::from(p)),
                Payload::TreasuryTransaction(p) => Self::TreasuryTransaction(TreasuryTransactionPayloadDto::from(p)),
                Payload::TaggedData(p) => Self::TaggedData(TaggedDataPayloadDto::from(p)),
            }
        }
    }

    impl Payload {
        pub fn try_from_dto(value: &PayloadDto, protocol_parameters: &ProtocolParameters) -> Result<Self, Error> {
            Ok(match value {
                PayloadDto::Transaction(p) => Self::from(TransactionPayload::try_from_dto(p, protocol_parameters)?),
                PayloadDto::Milestone(p) => Self::from(MilestonePayload::try_from_dto(p, protocol_parameters)?),
                PayloadDto::TreasuryTransaction(p) => Self::from(TreasuryTransactionPayload::try_from_dto(
                    p,
                    protocol_parameters.token_supply(),
                )?),
                PayloadDto::TaggedData(p) => Self::from(TaggedDataPayload::try_from(p)?),
            })
        }

        pub fn try_from_dto_unverified(value: &PayloadDto) -> Result<Self, Error> {
            Ok(match value {
                PayloadDto::Transaction(p) => Self::from(TransactionPayload::try_from_dto_unverified(p)?),
                PayloadDto::Milestone(p) => Self::from(MilestonePayload::try_from_dto_unverified(p)?),
                PayloadDto::TreasuryTransaction(p) => {
                    Self::from(TreasuryTransactionPayload::try_from_dto_unverified(p)?)
                }
                PayloadDto::TaggedData(p) => Self::from(TaggedDataPayload::try_from(p)?),
            })
        }
    }
}
