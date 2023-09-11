// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    mana::ManaAllotment,
    protocol::ProtocolParameters,
    rand::{number::rand_number_range, output::rand_account_id},
};

/// Generates a random mana allotment.
pub fn rand_mana_allotment(params: &ProtocolParameters) -> ManaAllotment {
    ManaAllotment::new(
        rand_account_id(),
        rand_number_range(0..params.mana_structure().max_mana()),
        params,
    )
    .unwrap()
}
