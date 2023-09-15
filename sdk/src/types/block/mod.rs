// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Core data types for blocks in the tangle.

#[macro_use]
mod r#macro;
mod block_id;
mod convert;
mod error;
mod issuer_id;

/// A module that provides types and syntactic validations of addresses.
pub mod address;
/// A module that provides types and syntactic validations of context inputs.
pub mod context_input;
/// A module that provides types and syntactic validations of blocks.
pub mod core;
/// A module that contains helper functions and types.
pub mod helper;
/// A module that provides types and syntactic validations of inputs.
pub mod input;
/// A module that provides types and syntactic validations of mana.
pub mod mana;
/// A module that provides types and syntactic validations of outputs.
pub mod output;
/// A module that provides types and syntactic validations of parents.
pub mod parent;
/// A module that provides types and syntactic validations of payloads.
pub mod payload;
/// A module that provides types and syntactic validations of protocol parameters.
pub mod protocol;
/// A module that provides utilities for random generation of types.
#[cfg(feature = "rand")]
pub mod rand;
/// A module that provides types and rules for semantic validation.
pub mod semantic;
/// A module that provides types and syntactic validations of signatures.
pub mod signature;
/// A module that provides types and syntactic validations of slots.
pub mod slot;
/// A module that provides types and syntactic validations of unlocks.
pub mod unlock;

pub(crate) use r#macro::create_bitflags;
#[cfg(feature = "serde")]
pub(crate) use r#macro::{impl_id, string_serde_impl};

#[cfg(feature = "serde")]
pub use self::core::dto::{BlockDto, BlockWrapperDto};
pub use self::{
    block_id::{BlockHash, BlockId},
    convert::ConvertTo,
    core::{Block, BlockWrapper},
    error::Error,
    issuer_id::IssuerId,
};

pub(crate) const PROTOCOL_VERSION: u8 = 3;
