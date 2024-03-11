// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use core::convert::Infallible;

use crate::types::block::{
    capabilities::CapabilityError,
    context_input::ContextInputError,
    input::{InputError, UtxoInput},
    mana::ManaError,
    output::{
        feature::FeatureError, unlock_condition::UnlockConditionError, ChainId, NativeTokenError, OutputError,
        TokenSchemeError,
    },
    payload::{
        tagged_data::{TagLength, TaggedDataLength},
        InputCount, OutputCount,
    },
    semantic::TransactionFailureReason,
    unlock::UnlockError,
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum PayloadError {
    #[display(fmt = "invalid payload kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid payload length: expected {expected} but got {actual}")]
    Length { expected: usize, actual: usize },
    #[display(fmt = "invalid timestamp: {_0}")]
    Timestamp(String),
    #[display(fmt = "invalid network id: {_0}")]
    NetworkId(String),
    #[display(fmt = "network ID mismatch: expected {expected} but got {actual}")]
    NetworkIdMismatch { expected: u64, actual: u64 },
    #[display(fmt = "invalid tagged data length: {_0}")]
    TaggedDataLength(<TaggedDataLength as TryFrom<usize>>::Error),
    #[display(fmt = "invalid tag length: {_0}")]
    TagLength(<TagLength as TryFrom<usize>>::Error),
    #[display(fmt = "invalid input count: {_0}")]
    InputCount(<InputCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid output count: {_0}")]
    OutputCount(<OutputCount as TryFrom<usize>>::Error),
    #[display(fmt = "the signed transaction payload is too large. Its length is {length}, max length is {max_length}")]
    SignedTransactionPayloadLength { length: usize, max_length: usize },
    #[display(fmt = "invalid transaction amount sum: {_0}")]
    TransactionAmountSum(u128),
    #[display(fmt = "the transaction is too large. Its length is {length}, max length is {max_length}")]
    TransactionLength { length: usize, max_length: usize },
    #[display(fmt = "duplicate output chain: {_0}")]
    DuplicateOutputChain(ChainId),
    #[display(fmt = "duplicate UTXO {_0} in inputs")]
    DuplicateUtxo(UtxoInput),
    #[display(fmt = "missing creation slot")]
    MissingCreationSlot,
    #[display(fmt = "input count and unlock count mismatch: {input_count} != {unlock_count}")]
    InputUnlockCountMismatch { input_count: usize, unlock_count: usize },
    #[display(fmt = "missing commitment context input for staking feature")]
    MissingCommitmentInputForStakingFeature,
    #[display(fmt = "missing commitment context input for block issuer feature")]
    MissingCommitmentInputForBlockIssuerFeature,
    #[display(fmt = "missing commitment context input for delegation output")]
    MissingCommitmentInputForDelegationOutput,
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

crate::impl_from_error_via!(PayloadError via OutputError:
    NativeTokenError,
    ManaError,
    UnlockConditionError,
    FeatureError,
    TokenSchemeError,
);

impl From<Infallible> for PayloadError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}
