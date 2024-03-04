// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use derive_more::{Display, From};

#[derive(Debug, PartialEq, Eq, Display, From)]
#[allow(missing_docs)]
pub struct IdentifierError(pub prefix_hex::Error);

#[cfg(feature = "std")]
impl std::error::Error for IdentifierError {}

impl From<Infallible> for IdentifierError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
