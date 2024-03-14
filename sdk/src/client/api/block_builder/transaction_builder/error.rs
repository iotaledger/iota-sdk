// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling for transaction builder.

use std::fmt::Debug;

use primitive_types::U256;

use super::Requirement;
use crate::types::block::{
    context_input::ContextInputError,
    mana::ManaError,
    output::{ChainId, NativeTokenError, OutputError, OutputId, TokenId},
    payload::PayloadError,
    semantic::TransactionFailureReason,
    signature::SignatureError,
    unlock::UnlockError,
    BlockError,
};

/// Errors related to transaction builder.
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum TransactionBuilderError {
    #[error("additional inputs required for {0:?}, but additional input selection is disabled")]
    AdditionalInputsRequired(Requirement),
    /// Can't burn and transition an output at the same time.
    #[error("can't burn and transition an output at the same time, chain ID: {0}")]
    BurnAndTransition(ChainId),
    #[error("mana rewards provided without an associated burn or custom input, output ID: {0}")]
    ExtraManaRewards(OutputId),
    /// Insufficient amount provided.
    #[error("insufficient amount: found {found}, required {required}")]
    InsufficientAmount {
        /// The amount found.
        found: u64,
        /// The required amount.
        required: u64,
    },
    /// Insufficient mana provided.
    #[error(
        "insufficient mana: found {found}, required {required}, slots remaining until enough mana {slots_remaining}"
    )]
    InsufficientMana {
        /// The amount found.
        found: u64,
        /// The required amount.
        required: u64,
        /// The number of slots remaining before this transaction will have generated enough mana.
        slots_remaining: u32,
    },
    /// Insufficient native token amount provided.
    #[error("insufficient native token amount: found {found}, required {required}")]
    InsufficientNativeTokenAmount {
        /// The token ID.
        token_id: TokenId,
        /// The amount found.
        found: U256,
        /// The required amount.
        required: U256,
    },
    /// Invalid amount of inputs.
    #[error("invalid amount of inputs: {0}")]
    InvalidInputCount(usize),
    /// Invalid amount of outputs.
    #[error("invalid amount of outputs: {0}")]
    InvalidOutputCount(usize),
    /// No input with matching ed25519 address provided.
    #[error("no input with matching ed25519 address provided")]
    MissingInputWithEd25519Address,
    /// No available inputs were provided to transaction builder.
    #[error("no available inputs provided")]
    NoAvailableInputsProvided,
    /// Required input is not available.
    #[error("required input {0} is not available")]
    RequiredInputIsNotAvailable(OutputId),
    /// Unfulfillable requirement.
    #[error("unfulfillable requirement {0:?}")]
    UnfulfillableRequirement(Requirement),
    /// Unsupported address type.
    #[error("unsupported address type {0}")]
    UnsupportedAddressType(String),
    /// Block error.
    #[error("{0}")]
    Block(#[from] BlockError),
    /// Output errors.
    #[error("{0}")]
    Output(#[from] OutputError),
    /// Payload errors.
    #[error("{0}")]
    Payload(#[from] PayloadError),
    /// Signature errors.
    #[error("{0}")]
    Signature(#[from] SignatureError),
    /// Mana errors.
    #[error("{0}")]
    Mana(#[from] ManaError),
    /// Native token errors.
    #[error("{0}")]
    NativeToken(#[from] NativeTokenError),
    /// Context input errors.
    #[error("{0}")]
    ContextInput(#[from] ContextInputError),
    /// Unlock errors.
    #[error("{0}")]
    Unlock(#[from] UnlockError),
    /// Semantic errors.
    #[error("{0}")]
    Semantic(#[from] TransactionFailureReason),
}
