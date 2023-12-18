// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::address::{Address, ToBech32Ext};
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn hash() {}

#[test]
fn json_packable_bech32() {
    // Test from https://github.com/iotaledger/tips/blob/tip52/tips/TIP-0052/tip-0052.md#bech32

    let multi_address_json = serde_json::json!({
        "type": 40,
        "addresses": [
          {
            "address": {
              "type": 0,
              "pubKeyHash": "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            },
            "weight": 1
          },
          {
            "address": {
              "type": 0,
              "pubKeyHash": "0x53fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            },
            "weight": 1
          },
          {
            "address": {
              "type": 0,
              "pubKeyHash": "0x54fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            },
            "weight": 1
          },
          {
            "address": {
              "type": 8,
              "accountId": "0x55fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            },
            "weight": 2
          },
          {
            "address": {
              "type": 16,
              "nftId": "0x56fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            },
            "weight": 3
          }
        ],
        "threshold": 2
    });
    let multi_address = serde_json::from_value::<Address>(multi_address_json).unwrap();
    let multi_address_bytes = multi_address.pack_to_vec();
    let multi_address_unpacked = Address::unpack_verified(multi_address_bytes, &()).unwrap();

    let multi = multi_address.as_multi();
    // TODO: confirm externally
    assert_eq!(
        "0xbe1c9aea43c6da13e34afa2dedd1ddeed7d72d779a2af96700b6cafbbc73d082",
        format!("{}", multi)
    );

    assert_eq!(multi_address, multi_address_unpacked);
    assert_eq!(
        multi_address.to_bech32_unchecked("iota"),
        "iota19qq0ezu97zl76wqnpdxxleuf55gk0eqhscjtdgqm5sqwav6gcarz6vvesnk"
    );
}
