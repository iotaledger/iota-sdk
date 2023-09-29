// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    core::Parents,
    rand::{block::rand_block_ids, number::rand_number_range},
};

/// Generates random parents.
pub fn rand_parents<const MIN: u8, const MAX: u8>() -> Parents<MIN, MAX> {
    Parents::from_set(rand_block_ids(
        rand_number_range(Parents::<MIN, MAX>::COUNT_RANGE).into(),
    ))
    .unwrap()
}
