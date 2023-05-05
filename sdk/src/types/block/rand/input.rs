// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    input::{Input, UtxoInput},
    rand::{number::rand_number, output::rand_output_id},
};

/// Generates a random Utxo input.
pub fn rand_utxo_input() -> UtxoInput {
    rand_output_id().into()
}

/// Generates a random input.
pub fn rand_input() -> Input {
    #[allow(clippy::modulo_one)]
    match rand_number::<u64>() % 1 {
        0 => rand_utxo_input().into(),
        _ => unreachable!(),
    }
}
