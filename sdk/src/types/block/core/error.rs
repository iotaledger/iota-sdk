// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{
    context_input::ContextInputError,
    input::InputError,
    mana::ManaError,
    output::{
        feature::FeatureError, unlock_condition::UnlockConditionError, NativeTokenError, OutputError, TokenSchemeError,
    },
    payload::PayloadError,
    protocol::ProtocolParametersError,
    signature::SignatureError,
    unlock::UnlockError,
    IdentifierError,
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum BlockError {
    #[display(fmt = "invalid block body kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid block length {_0}")]
    Length(usize),
    #[display(fmt = "remaining bytes after block")]
    RemainingBytesAfterBlock,
    #[display(fmt = "invalid parent count")]
    InvalidParentCount,
    #[display(fmt = "weak parents are not disjoint to strong or shallow like parents")]
    NonDisjointParents,
    #[display(fmt = "parents are not unique and/or sorted")]
    ParentsNotUniqueSorted,
    #[display(fmt = "network ID mismatch: expected {expected} but got {actual}")]
    NetworkIdMismatch { expected: u64, actual: u64 },
    #[display(fmt = "protocol version mismatch: expected {expected} but got {actual}")]
    ProtocolVersionMismatch { expected: u8, actual: u8 },
    #[display(fmt = "unsupported address kind: {_0}")]
    UnsupportedAddressKind(u8),
    #[display(fmt = "the block slot is before its contained transaction creation slot")]
    BlockSlotBeforeTransactionCreationSlot,
    #[display(fmt = "the transaction commitment slot is not in the allowed block slot interval")]
    TransactionCommitmentSlotNotInBlockSlotInterval,
    #[display(fmt = "the transaction commitment slot is after the block commitment slot")]
    TransactionCommitmentSlotAfterBlockCommitmentSlot,
    #[from]
    Payload(PayloadError),
    #[from]
    Signature(SignatureError),
    #[from]
    Identifier(IdentifierError),
    #[from]
    ProtocolParameters(ProtocolParametersError),
}

#[cfg(feature = "std")]
impl std::error::Error for BlockError {}

crate::impl_from_error_via!(BlockError via PayloadError:
    UnlockError,
    ContextInputError,
    NativeTokenError,
    ManaError,
    UnlockConditionError,
    FeatureError,
    TokenSchemeError,
    InputError,
    OutputError
);

impl From<Infallible> for BlockError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
