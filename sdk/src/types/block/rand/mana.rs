// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    mana::ManaAllotment,
    rand::{number::rand_number, output::rand_account_id},
};

/// Generates a random mana allotment.
pub fn rand_mana_allotment() -> ManaAllotment {
    ManaAllotment::new(rand_account_id(), rand_number()).unwrap()
}
