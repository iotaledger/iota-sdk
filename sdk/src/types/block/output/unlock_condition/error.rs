// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{address::AddressError, output::unlock_condition::UnlockConditionCount};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum UnlockConditionError {
    #[display(fmt = "invalid unlock condition kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid unlock condition count: {_0}")]
    Count(<UnlockConditionCount as TryFrom<usize>>::Error),
    #[display(fmt = "expiration unlock condition with slot index set to 0")]
    ExpirationZero,
    #[display(fmt = "timelock unlock condition with slot index set to 0")]
    TimelockZero,
    #[display(fmt = "unlock conditions are not unique and/or sorted")]
    NotUniqueSorted,
    #[display(fmt = "disallowed unlock condition at index {index} with kind {kind}")]
    Disallowed { index: usize, kind: u8 },
    #[display(fmt = "missing slot index")]
    MissingSlotIndex,
    #[from]
    Address(AddressError),
}

#[cfg(feature = "std")]
impl std::error::Error for UnlockConditionError {}

impl From<Infallible> for UnlockConditionError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
