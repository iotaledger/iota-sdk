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
    /// Used for shift operation during calculation of profit margin.
    profit_margin_exponent: u8,
    /// The length of the bootstrapping phase in epochs.
    bootstrapping_duration: EpochIndex,
    /// The ratio of the final rewards rate to the generation rate of Mana.
    reward_to_generation_ratio: u8,
    /// The rate of Mana rewards at the start of the bootstrapping phase.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    initial_target_rewards_rate: u64,
    /// The rate of Mana rewards after the bootstrapping phase.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    final_target_rewards_rate: u64,
    /// The exponent used for shifting operation during the pool rewards calculations.
    pool_coefficient_exponent: u8,
    // The number of epochs for which rewards are retained.
    retention_period: u16,
}

impl Default for RewardsParameters {
    fn default() -> Self {
        Self {
            profit_margin_exponent: 8,
            bootstrapping_duration: EpochIndex(1079),
            reward_to_generation_ratio: 2,
            initial_target_rewards_rate: 616067521149261,
            final_target_rewards_rate: 226702563632670,
            pool_coefficient_exponent: 11,
            retention_period: 384,
        }
    }
}
