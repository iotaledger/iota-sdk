// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    payload::signed_transaction::{TransactionHash, TransactionId},
    rand::bytes::rand_bytes_array,
    slot::SlotIndex,
};

/// Generates a random transaction id.
pub fn rand_transaction_id() -> TransactionId {
    TransactionId::new(rand_bytes_array())
}

/// Generates a random transaction id with a given slot index.
pub fn rand_transaction_id_with_slot_index(slot_index: impl Into<SlotIndex>) -> TransactionId {
    TransactionHash::new(rand_bytes_array()).into_transaction_id(slot_index.into())
}
