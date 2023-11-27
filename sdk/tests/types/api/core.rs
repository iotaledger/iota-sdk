// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::BlockDto;
use serde::Deserialize;

fn assert_response<T>(path: &str)
where
    for<'a> T: Deserialize<'a>,
{
    serde_json::from_str::<T>(&std::fs::read_to_string(path).unwrap()).unwrap();
}

#[test]
fn responses() {
    assert_response::<BlockDto>("./tests/types/api/fixtures/get-block-by-id-empty-response-example.json");
}
