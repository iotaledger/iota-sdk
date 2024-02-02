// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling for input selection.

use std::fmt::Debug;

use primitive_types::U256;

use super::Requirement;
use crate::types::block::output::{ChainId, OutputId, TokenId};

/// Errors related to input selection.
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Block error.
    #[error("{0}")]
    Block(#[from] crate::types::block::Error),
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
    /// No available inputs were provided to input selection.
    #[error("no available inputs provided")]
    NoAvailableInputsProvided,
    /// Required input is forbidden.
    #[error("required input {0} is forbidden")]
    RequiredInputIsForbidden(OutputId),
    /// Required input is not available.
    #[error("required input {0} is not available")]
    RequiredInputIsNotAvailable(OutputId),
    /// Unfulfillable requirement.
    #[error("unfulfillable requirement {0:?}")]
    UnfulfillableRequirement(Requirement),
    /// Unsupported address type.
    #[error("unsupported address type {0}")]
    // TODO replace with string when 2.0 has Address::kind_str
    UnsupportedAddressType(u8),
}
