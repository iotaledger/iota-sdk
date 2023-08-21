// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{mana::ManaAllotment, rand::output::rand_account_id};

/// Generates a random mana allotment.
pub fn rand_mana_allotment(mana: u64) -> ManaAllotment {
    ManaAllotment::new(rand_account_id(), mana).unwrap()
}
