// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::CopyGetters;
use packable::Packable;

use crate::types::block::{slot::EpochIndex, Error};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct RewardsParameters {
    /// The number of validation blocks that should be issued by a selected validator per slot during its epoch duties.
    validator_blocks_per_slot: u8,
    /// Used for shift operation for calculation of profit margin.
    profit_margin_exponent: u8,
    /// The length in epochs of the bootstrapping phase, (approx 3 years).
    bootstrapping_duration: EpochIndex,
    /// The coefficient used for calculation of initial rewards, relative to the term theta/(1-theta) from the
    /// Whitepaper, with theta = 2/3.
    mana_share_coefficient: u64,
    /// The exponent used for calculation of the initial reward.
    decay_balancing_constant_exponent: u8,
    /// Needs to be an integer approximation calculated based on chosen DecayBalancingConstantExponent.
    decay_balancing_constant: u64,
    /// The exponent used for shifting operation in the pool rewards calculations.
    pool_coefficient_exponent: u8,
}

impl Default for RewardsParameters {
    fn default() -> Self {
        // TODO: use actual values
        Self {
            validator_blocks_per_slot: Default::default(),
            profit_margin_exponent: Default::default(),
            bootstrapping_duration: Default::default(),
            mana_share_coefficient: Default::default(),
            decay_balancing_constant_exponent: Default::default(),
            decay_balancing_constant: Default::default(),
            pool_coefficient_exponent: Default::default(),
        }
    }
}
