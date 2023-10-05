// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::slot::EpochIndex;

/// Stakes coins to become eligible for committee selection, validate the network and receive Mana rewards.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
pub struct StakingFeature {
    /// The amount of coins that are locked and staked in the containing account.
    staked_amount: u64,
    /// The fixed cost of the validator, which it receives as part of its Mana rewards.
    fixed_cost: u64,
    /// The epoch index in which the staking started.
    start_epoch: EpochIndex,
    /// The epoch index in which the staking ends.
    end_epoch: EpochIndex,
}

impl StakingFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`StakingFeature`].
    pub const KIND: u8 = 5;

    /// Creates a new [`StakingFeature`].
    pub fn new(
        staked_amount: u64,
        fixed_cost: u64,
        start_epoch: impl Into<EpochIndex>,
        end_epoch: impl Into<EpochIndex>,
    ) -> Self {
        Self {
            staked_amount,
            fixed_cost,
            start_epoch: start_epoch.into(),
            end_epoch: end_epoch.into(),
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
    pub fn start_epoch(&self) -> EpochIndex {
        self.start_epoch
    }

    /// Returns the end epoch of the [`StakingFeature`].
    pub fn end_epoch(&self) -> EpochIndex {
        self.end_epoch
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::string;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct StakingFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "string")]
        staked_amount: u64,
        #[serde(with = "string")]
        fixed_cost: u64,
        start_epoch: EpochIndex,
        end_epoch: EpochIndex,
    }

    impl From<&StakingFeature> for StakingFeatureDto {
        fn from(value: &StakingFeature) -> Self {
            Self {
                kind: StakingFeature::KIND,
                staked_amount: value.staked_amount,
                fixed_cost: value.fixed_cost,
                start_epoch: value.start_epoch,
                end_epoch: value.end_epoch,
            }
        }
    }

    impl From<StakingFeatureDto> for StakingFeature {
        fn from(value: StakingFeatureDto) -> Self {
            Self::new(
                value.staked_amount,
                value.fixed_cost,
                value.start_epoch,
                value.end_epoch,
            )
        }
    }

    impl_serde_typed_dto!(StakingFeature, StakingFeatureDto, "staking feature");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for StakingFeature {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "stakedAmount": self.staked_amount.to_string(),
                "fixedCost": self.fixed_cost.to_string(),
                "startEpoch": self.start_epoch,
                "endEpoch": self.end_epoch,
            })
        }
    }

    impl FromJson for StakingFeature {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Ok(Self::new(
                value["stakedAmount"]
                    .to_str()?
                    .parse()
                    .map_err(|_| Error::InvalidField("stakedAmount"))?,
                value["fixedCost"]
                    .to_str()?
                    .parse()
                    .map_err(|_| Error::InvalidField("fixedCost"))?,
                value["startEpoch"].take_value::<EpochIndex>()?,
                value["endEpoch"].take_value::<EpochIndex>()?,
            ))
        }
    }
}
