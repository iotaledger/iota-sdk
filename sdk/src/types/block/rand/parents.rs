// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    parent::StrongParents,
    rand::{block::rand_block_ids, number::rand_number_range},
};

/// Generates random strong parents.
pub fn rand_strong_parents() -> StrongParents {
    StrongParents::from_set(rand_block_ids(rand_number_range(StrongParents::COUNT_RANGE).into())).unwrap()
}
