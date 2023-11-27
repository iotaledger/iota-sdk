// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{slot::SlotCommitment, BlockDto};
use packable::PackableExt;
use serde::Deserialize;

fn assert_json_response<T>(path: &str)
where
    for<'a> T: Deserialize<'a>,
{
    let file = std::fs::read_to_string(&format!("./tests/types/api/fixtures/{path}")).unwrap();
    serde_json::from_str::<T>(&file).unwrap();
}

fn assert_binary_response<T: PackableExt>(path: &str, visitor: &T::UnpackVisitor) {
    let file = std::fs::read_to_string(&format!("./tests/types/api/fixtures/{path}")).unwrap();
    let bytes = hex::decode(file).unwrap();
    T::unpack_verified(bytes, visitor).unwrap();
}

#[test]
fn responses() {
    // assert_json_response::<BlockDto>("get-block-by-id-empty-response-example.json");
    assert_binary_response::<SlotCommitment>("get-commitment-response-binary-example", &());
}
