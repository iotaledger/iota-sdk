// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{payload::InputCount, IdentifierError};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum InputError {
    #[display(fmt = "invalid input kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid input count: {_0}")]
    Count(<InputCount as TryFrom<usize>>::Error),
    #[from]
    Identifier(IdentifierError),
}

#[cfg(feature = "std")]
impl std::error::Error for InputError {}

impl From<Infallible> for InputError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
