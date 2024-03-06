// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{
    context_input::{reward::RewardContextInputIndex, ContextInputCount},
    IdentifierError,
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum ContextInputError {
    #[display(fmt = "invalid context input kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid context input count: {_0}")]
    Count(<ContextInputCount as TryFrom<usize>>::Error),
    #[display(fmt = "context inputs are not unique and/or sorted")]
    NotUniqueSorted,
    #[display(fmt = "invalid reward input index: {_0}")]
    RewardIndex(<RewardContextInputIndex as TryFrom<u16>>::Error),
    #[from]
    Identifier(IdentifierError),
}

#[cfg(feature = "std")]
impl std::error::Error for ContextInputError {}

impl From<Infallible> for ContextInputError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
