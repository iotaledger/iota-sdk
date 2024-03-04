// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod allotment;
mod error;
mod parameters;
mod rewards;

pub(crate) use self::allotment::verify_mana_allotments_sum;
pub use self::{
    allotment::{ManaAllotment, ManaAllotments},
    error::ManaError,
    parameters::ManaParameters,
    rewards::RewardsParameters,
};
