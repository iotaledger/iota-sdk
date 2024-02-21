// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod allotment;
mod parameters;
mod rewards;

use core::convert::Infallible;

pub(crate) use self::allotment::{verify_mana_allotments_sum, ManaAllotmentCount};
pub use self::{
    allotment::{ManaAllotment, ManaAllotments},
    parameters::ManaParameters,
    rewards::RewardsParameters,
};
use crate::types::block::slot::EpochIndex;

#[derive(Debug, PartialEq, Eq, strum::Display)]
#[allow(missing_docs)]
pub enum ManaError {
    #[strum(to_string = "invalid mana value: {0}")]
    InvalidManaValue(u64),
    #[strum(to_string = "invalid mana allotment count: {0}")]
    InvalidManaAllotmentCount(<ManaAllotmentCount as TryFrom<usize>>::Error),
    #[strum(to_string = "invalid mana allotment sum: {sum} greater than max of {max}")]
    InvalidManaAllotmentSum {
        max: u64,
        sum: u128,
    },
    #[strum(to_string = "mana allotments are not unique and/or sorted")]
    ManaAllotmentsNotUniqueSorted,
    InvalidEpochDiff {
        created: EpochIndex,
        target: EpochIndex,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for ManaError {}

impl From<Infallible> for ManaError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
