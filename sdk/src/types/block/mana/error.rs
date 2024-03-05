// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{mana::allotment::ManaAllotmentCount, slot::EpochIndex};

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[allow(missing_docs)]
pub enum ManaError {
    #[display(fmt = "invalid mana value: {_0}")]
    Value(u64),
    #[display(fmt = "invalid mana allotment count: {_0}")]
    AllotmentCount(<ManaAllotmentCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid mana allotment sum: {sum} greater than max of {max}")]
    AllotmentSum { max: u64, sum: u128 },
    #[display(fmt = "mana allotments are not unique and/or sorted")]
    AllotmentsNotUniqueSorted,
    #[display(fmt = "invalid epoch diff: created {created}, target {target}")]
    EpochDiff { created: EpochIndex, target: EpochIndex },
}

#[cfg(feature = "std")]
impl std::error::Error for ManaError {}

impl From<Infallible> for ManaError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
