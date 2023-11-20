// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;

use crate::types::block::{
    output::{NativeToken, TokenId},
    rand::{bytes::rand_bytes_array, number::rand_number},
};

/// Generates a random [`NativeToken`].
pub fn rand_native_token() -> NativeToken {
    NativeToken::new(TokenId::from(rand_bytes_array()), U256::from(rand_number::<u64>() + 1)).unwrap()
}
