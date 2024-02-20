// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::payload::signed_transaction::TransactionId;
use packable::PackableExt;
use pretty_assertions::assert_eq;

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", TransactionId::from_str(TRANSACTION_ID).unwrap()),
        r#"TransactionId { id: "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000", slot_index: SlotIndex(0) }"#
    );
}

#[test]
fn from_str_valid() {
    TransactionId::from_str(TRANSACTION_ID).unwrap();
}

#[test]
fn from_to_str() {
    assert_eq!(
        TRANSACTION_ID,
        TransactionId::from_str(TRANSACTION_ID).unwrap().to_string()
    );
}

#[test]
fn packed_len() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();

    assert_eq!(transaction_id.packed_len(), TransactionId::LENGTH);
    assert_eq!(transaction_id.pack_to_vec().len(), TransactionId::LENGTH);
}

#[test]
fn pack_unpack_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let packed_transaction_id = transaction_id.pack_to_vec();

    assert_eq!(
        transaction_id,
        PackableExt::unpack_bytes_verified(packed_transaction_id.as_slice(), &()).unwrap()
    );
}

// TODO: re-enable when source is updated
// #[test]
// fn transaction_id() {
//     // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#transaction-id

//     let transaction_payload_json = serde_json::json!({
//         "type": 1,
//         "transaction": {
//           "networkId": "14147312347886322761",
//           "creationSlot": 1048576,
//           "contextInputs": [
//             {
//               "type": 0,
//               "commitmentId": "0x760702593b59500420722f3a1634005f11360b133a030f46282c0f690a55084855000000"
//             },
//             {
//               "type": 1,
//               "accountId": "0x3407603d0f725b7e7214205f254305743d5362512f36153236435e796b6a1c2e"
//             },
//             {
//               "type": 2,
//               "index": 0
//             }
//           ],
//           "inputs": [
//             {
//               "type": 0,
//               "transactionId": "0x3ca1f23b83708ee7c59d6d7fe71453106bb0a0abc1c9cc4b340c755238ae6daa00000000",
//               "transactionOutputIndex": 0
//             },
//             {
//               "type": 0,
//               "transactionId": "0xecb673f194640b2067b8da136b5f5437c2c723e7f3fdaa53984d7588ed21071a00000000",
//               "transactionOutputIndex": 0
//             }
//           ],
//           "allotments": [
//             {
//               "accountId": "0x0e0f253479566103415e29060f79772445531564733e214b54084358413f7c70",
//               "mana": "6648"
//             },
//             {
//               "accountId": "0x445e204c1f747503106b5663664c43591e63235804057c445d073a5f10597e2d",
//               "mana": "9988"
//             }
//           ],
//           "capabilities": "0x01",
//           "outputs": [
//             {
//               "type": 0,
//               "amount": "100000",
//               "mana": "0",
//               "unlockConditions": [
//                 {
//                   "type": 0,
//                   "address": {
//                     "type": 0,
//                     "pubKeyHash": "0x7f34f61bd0ecd2654a1ec3c9bf3fbc0de91abcbd7397e09faaaffc89d17a8f6e"
//                   }
//                 }
//               ],
//               "features": [
//                 // TODO re-enable with NT PR
//                 // {
//                 //   "type": 4,
//                 //   "id": "0x082a1c2429352945216e3f547a03226b2f014d3d2e185a2459473a362c4d124d511a6c2d6000",
//                 //   "amount": "0xd54f92ae8c34fbb4"
//                 // }
//               ]
//             },
//             {
//               "type": 1,
//               "amount": "100000",
//               "mana": "5000",
//               "accountId": "0x0000000000000000000000000000000000000000000000000000000000000000",
//               "stateIndex": 0,
//               "foundryCounter": 0,
//               "unlockConditions": [
//                 {
//                   "type": 4,
//                   "address": {
//                     "type": 0,
//                     "pubKeyHash": "0x7f34f61bd0ecd2654a1ec3c9bf3fbc0de91abcbd7397e09faaaffc89d17a8f6e"
//                   }
//                 },
//                 {
//                   "type": 5,
//                   "address": {
//                     "type": 0,
//                     "pubKeyHash": "0x7f34f61bd0ecd2654a1ec3c9bf3fbc0de91abcbd7397e09faaaffc89d17a8f6e"
//                   }
//                 }
//               ],
//               "features": [
//                 {
//                   "type": 2,
//                   "data": "0x1e69562e763b1125080c1a7161390e42"
//                 }
//               ]
//             }
//           ]
//         },
//         "unlocks": [
//           {
//             "type": 0,
//             "signature": {
//               "type": 0,
//               "publicKey": "0xa6bbccb2380a3a941a7bfdd5f2afcb8a6f5236bbe12ae8b931b593efd76864b6",
//               "signature":
// "0x98a18fd0083c7d9b6b05e218c7f764bb915148762ce342d795f7acac4083b40dbfc01f5fd23f6d1e652eee0e5951b87dd6307adf1389f8f16c08ade12be01c0a"
//             }
//           },
//           {
//             "type": 1,
//             "reference": 0
//           }
//         ]
//     });

//     let transaction_payload_dto =
//         serde_json::from_value::<SignedTransactionPayloadDto>(transaction_payload_json).unwrap();
//     let transaction_payload = SignedTransactionPayload::try_from_dto(transaction_payload_dto).unwrap();
//     let transaction_payload_bytes = Payload::from(transaction_payload.clone()).pack_to_vec();

//     // assert_eq!(
//     //     prefix_hex::encode(transaction_payload_bytes),
//     //
// "0x01490443ee9f5955c400001000030001760702593b59500420722f3a1634005f11360b133a030f46282c0f690a55084855000000023407603d0f725b7e7214205f254305743d5362512f36153236435e796b6a1c2e0300000200003ca1f23b83708ee7c59d6d7fe71453106bb0a0abc1c9cc4b340c755238ae6daa00000000000000ecb673f194640b2067b8da136b5f5437c2c723e7f3fdaa53984d7588ed21071a00000000000002000e0f253479566103415e29060f79772445531564733e214b54084358413f7c70f819000000000000445e204c1f747503106b5663664c43591e63235804057c445d073a5f10597e2d0427000000000000010100000000020000a08601000000000000000000000000000100007f34f61bd0ecd2654a1ec3c9bf3fbc0de91abcbd7397e09faaaffc89d17a8f6e0104082a1c2429352945216e3f547a03226b2f014d3d2e185a2459473a362c4d124d511a6c2d6000b4fb348cae924fd500000000000000000000000000000000000000000000000001a08601000000000088130000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000204007f34f61bd0ecd2654a1ec3c9bf3fbc0de91abcbd7397e09faaaffc89d17a8f6e05007f34f61bd0ecd2654a1ec3c9bf3fbc0de91abcbd7397e09faaaffc89d17a8f6e010210001e69562e763b1125080c1a7161390e420002000000a6bbccb2380a3a941a7bfdd5f2afcb8a6f5236bbe12ae8b931b593efd76864b698a18fd0083c7d9b6b05e218c7f764bb915148762ce342d795f7acac4083b40dbfc01f5fd23f6d1e652eee0e5951b87dd6307adf1389f8f16c08ade12be01c0a010000"
//     // );

//     // let transaction_id = transaction_payload.id().to_string();

//     // assert_eq!(
//     //     transaction_id,
//     //     "0xc4f095a7ee824c8fd53040c4143963153636d56bb2334167fd4f531472682533"
//     // );
// }
