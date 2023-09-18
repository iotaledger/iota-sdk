// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::{
    block::payload::{
        transaction::{dto::TransactionPayloadDto, TransactionId, TransactionPayload},
        Payload,
    },
    TryFromDto,
};
use packable::PackableExt;

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", TransactionId::from_str(TRANSACTION_ID).unwrap()),
        "TransactionId(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649)"
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

    assert_eq!(transaction_id.packed_len(), 32);
    assert_eq!(transaction_id.pack_to_vec().len(), 32);
}

#[test]
fn pack_unpack_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let packed_transaction_id = transaction_id.pack_to_vec();

    assert_eq!(
        transaction_id,
        PackableExt::unpack_verified(packed_transaction_id.as_slice(), &()).unwrap()
    );
}

#[test]
fn transaction_id() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#transaction-id
    let transaction_payload_json = serde_json::json!({
      "type":6,
      "essence":{
        "type":2,
        "networkId":"3650798313638353144",
        "creationSlot":"28",
        "contextInputs":[],
        "inputs":[
          {
            "type":0,
            "transactionId":"0x24ff9b3038506fb1b406306a496001c3e24e2be07c838317922bf21d686a078f",
            "transactionOutputIndex":10
          }
        ],
        "inputsCommitment":"0xb70c6f86a1ea03a59a71d73dcd07e2082bbdf0ce971faa21748348bca22fb023",
        "outputs":[
          {
            "type":3,
            "amount":"10000",
            "mana":"0",
            "unlockConditions":[
              {
                "type":0,
                "address":{
                  "type":0,
                  "pubKeyHash":"0xd9f84458286dc41cd34789dec566cd096cf47de991aa36a97aebfaea14128f6d"
                }
              }
            ]
          }
        ],
        "allotments":[],
        "payload":{
          "type":5,
          "tag":"0x1d7b3e11697264111e130b0e",
          "data":"0x1d7b3e11697264111e130b0e"
        }
      },
      "unlocks":[
        {
          "type":0,
          "signature":{
            "type":0,
            "publicKey":"0x803361fe1effc899dca7f931d8ad07c01ba23aaa93f986adb04d4c17cf6368d8",
            "signature":"0xccddbac3aaac413e0193e16da3449f30c183d0e7eaa7f303dc12ae0dbc9fb890e449a52f9056e7d952ea796fd3e5645f60d9eb98ed91cb3261720fb528d2a105"
          }
        }
      ]
    });

    let transaction_payload_dto = serde_json::from_value::<TransactionPayloadDto>(transaction_payload_json).unwrap();
    let transaction_payload = TransactionPayload::try_from_dto(transaction_payload_dto).unwrap();
    let transaction_payload_bytes = Payload::from(transaction_payload.clone()).pack_to_vec();

    assert_eq!(
        transaction_payload_bytes,
        [
            6, 0, 0, 0, 2, 248, 88, 2, 55, 185, 61, 170, 50, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 36, 255, 155, 48,
            56, 80, 111, 177, 180, 6, 48, 106, 73, 96, 1, 195, 226, 78, 43, 224, 124, 131, 131, 23, 146, 43, 242, 29,
            104, 106, 7, 143, 10, 0, 183, 12, 111, 134, 161, 234, 3, 165, 154, 113, 215, 61, 205, 7, 226, 8, 43, 189,
            240, 206, 151, 31, 170, 33, 116, 131, 72, 188, 162, 47, 176, 35, 1, 0, 3, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 217, 248, 68, 88, 40, 109, 196, 28, 211, 71, 137, 222, 197, 102, 205, 9, 108,
            244, 125, 233, 145, 170, 54, 169, 122, 235, 250, 234, 20, 18, 143, 109, 0, 0, 0, 33, 0, 0, 0, 5, 0, 0, 0,
            12, 29, 123, 62, 17, 105, 114, 100, 17, 30, 19, 11, 14, 12, 0, 0, 0, 29, 123, 62, 17, 105, 114, 100, 17,
            30, 19, 11, 14, 1, 0, 0, 0, 128, 51, 97, 254, 30, 255, 200, 153, 220, 167, 249, 49, 216, 173, 7, 192, 27,
            162, 58, 170, 147, 249, 134, 173, 176, 77, 76, 23, 207, 99, 104, 216, 204, 221, 186, 195, 170, 172, 65, 62,
            1, 147, 225, 109, 163, 68, 159, 48, 193, 131, 208, 231, 234, 167, 243, 3, 220, 18, 174, 13, 188, 159, 184,
            144, 228, 73, 165, 47, 144, 86, 231, 217, 82, 234, 121, 111, 211, 229, 100, 95, 96, 217, 235, 152, 237,
            145, 203, 50, 97, 114, 15, 181, 40, 210, 161, 5
        ]
    );

    let transaction_id = transaction_payload.id().to_string();

    assert_eq!(
        transaction_id,
        "0xc4f095a7ee824c8fd53040c4143963153636d56bb2334167fd4f531472682533"
    );
}
