// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use primitive_types::U256;

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[allow(missing_docs)]
pub enum TokenSchemeError {
    #[display(fmt = "invalid token scheme kind {_0}")]
    Kind(u8),
    #[display(fmt = "invalid foundry output supply: minted {minted}, melted {melted} max {max}")]
    Supply { minted: U256, melted: U256, max: U256 },
}

#[cfg(feature = "std")]
impl std::error::Error for TokenSchemeError {}

impl From<Infallible> for TokenSchemeError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
