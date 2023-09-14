// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{issuer_id::IssuerId, rand::bytes::rand_bytes_array};

/// Generates a random issuer id.
pub fn rand_issuer_id() -> IssuerId {
    IssuerId::new(rand_bytes_array())
}
