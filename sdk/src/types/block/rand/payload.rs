// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    payload::{tagged_data::TaggedDataPayload, Payload},
    rand::{bytes::rand_bytes, number::rand_number_range},
};

/// Generates a random tagged data payload.
pub fn rand_tagged_data_payload() -> TaggedDataPayload {
    TaggedDataPayload::new(
        rand_bytes(rand_number_range(TaggedDataPayload::TAG_LENGTH_RANGE).into()),
        rand_bytes(rand_number_range(0..10000)),
    )
    .unwrap()
}

/// Generates a random payload for a block.
pub fn rand_payload_for_block() -> Payload {
    // TODO complete
    rand_tagged_data_payload().into()
}
