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
    /// Used for shift operation during calculation of profit margin.
    profit_margin_exponent: u8,
    /// The length of the bootstrapping phase in epochs.
    bootstrapping_duration: EpochIndex,
    /// The coefficient used for calculation of initial rewards.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    mana_share_coefficient: u64,
    /// The exponent used for calculation of the initial reward.
    decay_balancing_constant_exponent: u8,
    /// An integer approximation which is calculated using the `decay_balancing_constant_exponent`.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    decay_balancing_constant: u64,
    /// The exponent used for shifting operation during the pool rewards calculations.
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
