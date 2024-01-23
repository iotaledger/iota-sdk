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
    // The number of epochs for which rewards are retained.
    retention_period: u16,
}

impl Default for RewardsParameters {
    fn default() -> Self {
        Self {
            profit_margin_exponent: 8,
            bootstrapping_duration: EpochIndex(1079),
            mana_share_coefficient: 2,
            decay_balancing_constant_exponent: 8,
            decay_balancing_constant: 1,
            pool_coefficient_exponent: 11,
            retention_period: 384,
        }
    }
}
