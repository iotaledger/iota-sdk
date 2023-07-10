// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::BlockId;
use serde::{Deserialize, Serialize};

const BLOCK_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerdeStringTest {
    #[serde(with = "iota_sdk::utils::serde::string")]
    pub block_id: BlockId,
}

#[test]
fn serde_string() {
    let block_id = BlockId::from_str(BLOCK_ID).unwrap();
    let test_1 = SerdeStringTest { block_id };
    let str = serde_json::to_string(&test_1).unwrap();
    let test_2 = serde_json::from_str(&str).unwrap();

    assert_eq!(test_1, test_2);
    assert!(str.contains(BLOCK_ID));
}
