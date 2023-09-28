// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    parent::Parents,
    rand::{block::rand_block_ids, number::rand_number_range},
};

/// Generates random strong parents.
pub fn rand_strong_parents<const MAX: u8>() -> Parents<1, MAX> {
    Parents::from_vec(rand_block_ids(rand_number_range(Parents::<1, MAX>::COUNT_RANGE).into())).unwrap()
}
