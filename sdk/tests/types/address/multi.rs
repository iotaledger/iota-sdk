// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    address::{AccountAddress, Address, Ed25519Address, MultiAddress, ToBech32Ext, WeightedAddress},
    output::AccountId,
    rand::bytes::rand_bytes_array,
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn ordered_by_packed_bytes() {
    let mut bytes_1 = rand_bytes_array::<32>();
    bytes_1[0] = 0;
    let mut bytes_2 = bytes_1;
    bytes_2[0] = 1;

    let weighted_1 = WeightedAddress::new(AccountAddress::from(AccountId::from(bytes_1)), 1).unwrap();
    let weighted_2 = WeightedAddress::new(Ed25519Address::from(bytes_2), 1).unwrap();

    let multi_1 = MultiAddress::new([weighted_1, weighted_2], 2).unwrap();
    let bytes = multi_1.pack_to_vec();
    let multi_2 = MultiAddress::unpack_bytes_verified(bytes, &()).unwrap();

    assert!(multi_2.addresses()[0].address().is_ed25519());
    assert!(multi_2.addresses()[1].address().is_account());
}

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
    let multi_address_unpacked = Address::unpack_bytes_verified(multi_address_bytes, &()).unwrap();

    assert_eq!(multi_address, multi_address_unpacked);
    assert_eq!(
        multi_address.as_multi().to_string(),
        "0x00fc8b85f0bfed38130b4c6fe789a51167e4178624b6a01ba400eeb348c7462d",
    );
    assert_eq!(
        multi_address.to_bech32_unchecked("iota"),
        "iota19qq0ezu97zl76wqnpdxxleuf55gk0eqhscjtdgqm5sqwav6gcarz6vvesnk"
    );
}
