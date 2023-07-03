// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Stakes IOTA coins to become eligible for committee selection, validate the network and receive Mana rewards.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakingFeature {
    /// The amount of IOTA coins that are locked and staked in the containing account.
    staked_amount: u64,
    /// The fixed cost of the validator, which it receives as part of its Mana rewards.
    fixed_cost: u64,
    /// The epoch index in which the staking started.
    start_epoch: u64,
    /// The epoch index in which the staking ends.
    end_epoch: u64,
}

impl StakingFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`StakingFeature`].
    pub const KIND: u8 = 5;

    /// Creates a new [`StakingFeature`].
    pub fn new(staked_amount: u64, fixed_cost: u64, start_epoch: u64, end_epoch: u64) -> Self {
        Self {
            staked_amount,
            fixed_cost,
            start_epoch,
            end_epoch,
        }
    }

    /// Returns the staked amount of the [`StakingFeature`].
    pub fn staked_amount(&self) -> u64 {
        self.staked_amount
    }

    /// Returns the fixed cost of the [`StakingFeature`].
    pub fn fixed_cost(&self) -> u64 {
        self.fixed_cost
    }

    /// Returns the start epoch of the [`StakingFeature`].
    pub fn start_epoch(&self) -> u64 {
        self.start_epoch
    }

    /// Returns the end epoch of the [`StakingFeature`].
    pub fn end_epoch(&self) -> u64 {
        self.end_epoch
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct StakingFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub staked_amount: u64,
        pub fixed_cost: u64,
        pub start_epoch: u64,
        pub end_epoch: u64,
    }
}
