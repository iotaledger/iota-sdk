// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::CopyGetters;
use packable::Packable;

use crate::types::block::Error;

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
    pub(crate) profit_margin_exponent: u8,
    /// The length of the bootstrapping phase in epochs.
    pub(crate) bootstrapping_duration: u32,
    /// The ratio of the final rewards rate to the generation rate of Mana.
    pub(crate) reward_to_generation_ratio: u8,
    /// The rate of Mana rewards at the start of the bootstrapping phase.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) initial_target_rewards_rate: u64,
    /// The rate of Mana rewards after the bootstrapping phase.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) final_target_rewards_rate: u64,
    /// The exponent used for shifting operation during the pool rewards calculations.
    pub(crate) pool_coefficient_exponent: u8,
    // The number of epochs for which rewards are retained.
    pub(crate) retention_period: u16,
}
