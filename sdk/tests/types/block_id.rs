// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::{
    block::{
        output::RentStructure, protocol::ProtocolParameters, rand::bytes::rand_bytes_array, BlockHash, BlockId,
        BlockWrapper, BlockWrapperDto,
    },
    TryFromDto,
};
use packable::PackableExt;

const BLOCK_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6490000000000000000";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", BlockId::from_str(BLOCK_ID).unwrap()),
        format!(
            "BlockId {{ hash: BlockHash(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649), slot_index: SlotIndex(0) }}"
        )
    );
}

#[test]
fn from_str_valid() {
    BlockId::from_str(BLOCK_ID).unwrap();
}

#[test]
fn from_to_str() {
    assert_eq!(BLOCK_ID, BlockId::from_str(BLOCK_ID).unwrap().to_string());
}

// Validate that the length of a packed `BlockId` matches the declared `packed_len()`.
#[test]
fn packed_len() {
    let block_id = BlockId::from_str(BLOCK_ID).unwrap();

    assert_eq!(block_id.packed_len(), 40);
    assert_eq!(block_id.pack_to_vec().len(), 40);
}

// Validate that a `unpack` ∘ `pack` round-trip results in the original block id.
#[test]
fn pack_unpack_valid() {
    let block_id = BlockId::from_str(BLOCK_ID).unwrap();
    let packed_block_id = block_id.pack_to_vec();

    assert_eq!(packed_block_id.len(), block_id.packed_len());
    assert_eq!(
        block_id,
        PackableExt::unpack_verified(packed_block_id.as_slice(), &()).unwrap()
    );
}

#[test]
fn memory_layout() {
    let block_hash = BlockHash::new(rand_bytes_array());
    let slot_index = 12345.into();
    let block_id = block_hash.with_slot_index(slot_index);
    assert_eq!(slot_index, block_id.slot_index());
    let memory_layout =
        <[u8; BlockId::LENGTH]>::try_from([block_hash.as_ref(), &slot_index.to_le_bytes()].concat()).unwrap();
    assert_eq!(block_id.as_ref(), memory_layout);
}

fn protocol_parameters() -> ProtocolParameters {
    ProtocolParameters::new(3, "test", "rms", RentStructure::default(), 0, 1695275822, 10, 0).unwrap()
}

#[test]
fn basic_block_id_tagged_data_payload() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-tagged-data-payload

    let block_json = serde_json::json!({
          "protocolVersion":3,
    "networkId":"10549460113735494767",
    "issuingTime":"1695275834000000000",
    "slotCommitmentId":"0x498bf08a5ed287bc87340341ffab28706768cd3a7035ae5e33932d9a12bb30940000000000000000",
    "latestFinalizedSlot":"21",
    "issuerId":"0x3370746f30705b7d0b42597459714d45241e5a64761b09627c447b751c7e145c",
    "block":{
      "type":0,
      "strongParents":[
        "0x304442486c7a05361408585e4b5f7a67441c437528755a70041e0e557a6d4b2d7d4362083d492b57",
        "0x5f736978340a243d381b343b160b316a6b7d4b1e3c0355492e2e72113c2b126600157e69113c0b5c"
      ],
      "weakParents":[
        "0x0b5a48384f382f4a49471c4860683c6f0a0d446f012e1b117c4e405f5e24497c72691f43535c0b42"
      ],
      "shallowLikeParents":[
        "0x163007217803006078040b0f51507d3572355a457839095e572f125500401b7d220c772b56165a12"
      ],
      "payload":{
        "type":5,
        "tag":"0x68656c6c6f20776f726c64",
        "data":"0x01020304"
      },
      "burnedMana":"180500"
    },
    "signature":{
      "type":0,
      "publicKey":"0x024b6f086177156350111d5e56227242034e596b7e3d0901180873740723193c",
      "signature":"0x7c274e5e771d5d60202d334f06773d3672484b1e4e6f03231b4e69305329267a4834374b0f2e0d5c6c2f7779620f4f534c773b1679400c52303d1f23121a4049"
    }
      });

    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto).unwrap();
    let block_bytes = block.pack_to_vec();

    assert_eq!(
        block_bytes,
        [
            3, 111, 44, 91, 123, 20, 54, 103, 146, 0, 196, 223, 153, 99, 212, 134, 23, 73, 139, 240, 138, 94, 210, 135,
            188, 135, 52, 3, 65, 255, 171, 40, 112, 103, 104, 205, 58, 112, 53, 174, 94, 51, 147, 45, 154, 18, 187, 48,
            148, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 51, 112, 116, 111, 48, 112, 91, 125, 11, 66, 89, 116,
            89, 113, 77, 69, 36, 30, 90, 100, 118, 27, 9, 98, 124, 68, 123, 117, 28, 126, 20, 92, 0, 2, 48, 68, 66, 72,
            108, 122, 5, 54, 20, 8, 88, 94, 75, 95, 122, 103, 68, 28, 67, 117, 40, 117, 90, 112, 4, 30, 14, 85, 122,
            109, 75, 45, 125, 67, 98, 8, 61, 73, 43, 87, 95, 115, 105, 120, 52, 10, 36, 61, 56, 27, 52, 59, 22, 11, 49,
            106, 107, 125, 75, 30, 60, 3, 85, 73, 46, 46, 114, 17, 60, 43, 18, 102, 0, 21, 126, 105, 17, 60, 11, 92, 1,
            11, 90, 72, 56, 79, 56, 47, 74, 73, 71, 28, 72, 96, 104, 60, 111, 10, 13, 68, 111, 1, 46, 27, 17, 124, 78,
            64, 95, 94, 36, 73, 124, 114, 105, 31, 67, 83, 92, 11, 66, 1, 22, 48, 7, 33, 120, 3, 0, 96, 120, 4, 11, 15,
            81, 80, 125, 53, 114, 53, 90, 69, 120, 57, 9, 94, 87, 47, 18, 85, 0, 64, 27, 125, 34, 12, 119, 43, 86, 22,
            90, 18, 24, 0, 0, 0, 5, 0, 0, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 4, 0, 0, 0, 1,
            2, 3, 4, 20, 193, 2, 0, 0, 0, 0, 0, 0, 2, 75, 111, 8, 97, 119, 21, 99, 80, 17, 29, 94, 86, 34, 114, 66, 3,
            78, 89, 107, 126, 61, 9, 1, 24, 8, 115, 116, 7, 35, 25, 60, 124, 39, 78, 94, 119, 29, 93, 96, 32, 45, 51,
            79, 6, 119, 61, 54, 114, 72, 75, 30, 78, 111, 3, 35, 27, 78, 105, 48, 83, 41, 38, 122, 72, 52, 55, 75, 15,
            46, 13, 92, 108, 47, 119, 121, 98, 15, 79, 83, 76, 119, 59, 22, 121, 64, 12, 82, 48, 61, 31, 35, 18, 26,
            64, 73
        ]
    );

    let block_id = block.id(&protocol_parameters()).to_string();

    assert_eq!(
        block_id,
        "0xb2c397afa61262c10af75320a166d28be34debcc4449f272f90c8769681c0b710200000000000000"
    );
}

#[test]
fn basic_block_id_transaction_payload() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-transaction-payload

    let block_json = serde_json::json!({
      "protocolVersion":3,
      "networkId":"10549460113735494767","issuingTime":"1695275834000000000","slotCommitmentId":"0x498bf08a5ed287bc87340341ffab28706768cd3a7035ae5e33932d9a12bb30940000000000000000",
      "latestFinalizedSlot":"21",
      "issuerId":"0x3370746f30705b7d0b42597459714d45241e5a64761b09627c447b751c7e145c",
      "block":{
      "type":0,
      "strongParents":[
      "0x304442486c7a05361408585e4b5f7a67441c437528755a70041e0e557a6d4b2d7d4362083d492b57",
      "0x5f736978340a243d381b343b160b316a6b7d4b1e3c0355492e2e72113c2b126600157e69113c0b5c"
      ],
      "weakParents":[
      "0x0b5a48384f382f4a49471c4860683c6f0a0d446f012e1b117c4e405f5e24497c72691f43535c0b42"
      ],
      "shallowLikeParents":[
      "0x163007217803006078040b0f51507d3572355a457839095e572f125500401b7d220c772b56165a12"
      ],
      "payload":{
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
      },
      "burnedMana":"180500"
    },
    "signature":{
      "type":0,
      "publicKey":"0x024b6f086177156350111d5e56227242034e596b7e3d0901180873740723193c",
      "signature":"0x7c274e5e771d5d60202d334f06773d3672484b1e4e6f03231b4e69305329267a4834374b0f2e0d5c6c2f7779620f4f534c773b1679400c52303d1f23121a4049"
    }
    });

    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto).unwrap();
    let block_bytes = block.pack_to_vec();

    assert_eq!(
        block_bytes,
        [
            3, 111, 44, 91, 123, 20, 54, 103, 146, 0, 196, 223, 153, 99, 212, 134, 23, 73, 139, 240, 138, 94, 210, 135,
            188, 135, 52, 3, 65, 255, 171, 40, 112, 103, 104, 205, 58, 112, 53, 174, 94, 51, 147, 45, 154, 18, 187, 48,
            148, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 51, 112, 116, 111, 48, 112, 91, 125, 11, 66, 89, 116,
            89, 113, 77, 69, 36, 30, 90, 100, 118, 27, 9, 98, 124, 68, 123, 117, 28, 126, 20, 92, 0, 2, 48, 68, 66, 72,
            108, 122, 5, 54, 20, 8, 88, 94, 75, 95, 122, 103, 68, 28, 67, 117, 40, 117, 90, 112, 4, 30, 14, 85, 122,
            109, 75, 45, 125, 67, 98, 8, 61, 73, 43, 87, 95, 115, 105, 120, 52, 10, 36, 61, 56, 27, 52, 59, 22, 11, 49,
            106, 107, 125, 75, 30, 60, 3, 85, 73, 46, 46, 114, 17, 60, 43, 18, 102, 0, 21, 126, 105, 17, 60, 11, 92, 1,
            11, 90, 72, 56, 79, 56, 47, 74, 73, 71, 28, 72, 96, 104, 60, 111, 10, 13, 68, 111, 1, 46, 27, 17, 124, 78,
            64, 95, 94, 36, 73, 124, 114, 105, 31, 67, 83, 92, 11, 66, 1, 22, 48, 7, 33, 120, 3, 0, 96, 120, 4, 11, 15,
            81, 80, 125, 53, 114, 53, 90, 69, 120, 57, 9, 94, 87, 47, 18, 85, 0, 64, 27, 125, 34, 12, 119, 43, 86, 22,
            90, 18, 31, 1, 0, 0, 6, 0, 0, 0, 2, 248, 88, 2, 55, 185, 61, 170, 50, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 36, 255, 155, 48, 56, 80, 111, 177, 180, 6, 48, 106, 73, 96, 1, 195, 226, 78, 43, 224, 124, 131, 131,
            23, 146, 43, 242, 29, 104, 106, 7, 143, 10, 0, 183, 12, 111, 134, 161, 234, 3, 165, 154, 113, 215, 61, 205,
            7, 226, 8, 43, 189, 240, 206, 151, 31, 170, 33, 116, 131, 72, 188, 162, 47, 176, 35, 1, 0, 3, 16, 39, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 217, 248, 68, 88, 40, 109, 196, 28, 211, 71, 137, 222, 197,
            102, 205, 9, 108, 244, 125, 233, 145, 170, 54, 169, 122, 235, 250, 234, 20, 18, 143, 109, 0, 0, 0, 33, 0,
            0, 0, 5, 0, 0, 0, 12, 29, 123, 62, 17, 105, 114, 100, 17, 30, 19, 11, 14, 12, 0, 0, 0, 29, 123, 62, 17,
            105, 114, 100, 17, 30, 19, 11, 14, 1, 0, 0, 0, 128, 51, 97, 254, 30, 255, 200, 153, 220, 167, 249, 49, 216,
            173, 7, 192, 27, 162, 58, 170, 147, 249, 134, 173, 176, 77, 76, 23, 207, 99, 104, 216, 204, 221, 186, 195,
            170, 172, 65, 62, 1, 147, 225, 109, 163, 68, 159, 48, 193, 131, 208, 231, 234, 167, 243, 3, 220, 18, 174,
            13, 188, 159, 184, 144, 228, 73, 165, 47, 144, 86, 231, 217, 82, 234, 121, 111, 211, 229, 100, 95, 96, 217,
            235, 152, 237, 145, 203, 50, 97, 114, 15, 181, 40, 210, 161, 5, 20, 193, 2, 0, 0, 0, 0, 0, 0, 2, 75, 111,
            8, 97, 119, 21, 99, 80, 17, 29, 94, 86, 34, 114, 66, 3, 78, 89, 107, 126, 61, 9, 1, 24, 8, 115, 116, 7, 35,
            25, 60, 124, 39, 78, 94, 119, 29, 93, 96, 32, 45, 51, 79, 6, 119, 61, 54, 114, 72, 75, 30, 78, 111, 3, 35,
            27, 78, 105, 48, 83, 41, 38, 122, 72, 52, 55, 75, 15, 46, 13, 92, 108, 47, 119, 121, 98, 15, 79, 83, 76,
            119, 59, 22, 121, 64, 12, 82, 48, 61, 31, 35, 18, 26, 64, 73
        ]
    );

    let block_id = block.id(&protocol_parameters()).to_string();

    assert_eq!(
        block_id,
        "0x22215ad9e912989a4886d48a7147b23b753c251861cd0ed14649a11cd85028f60200000000000000"
    );
}

#[test]
fn validation_block_id() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#validation-block-id

    let block_json = serde_json::json!({
        "protocolVersion":3,
        "networkId":"10549460113735494767",
        "issuingTime":"1695275834000000000",
        "slotCommitmentId":"0x498bf08a5ed287bc87340341ffab28706768cd3a7035ae5e33932d9a12bb30940000000000000000",
        "latestFinalizedSlot":"0",
        "issuerId":"0x3370746f30705b7d0b42597459714d45241e5a64761b09627c447b751c7e145c",
        "block":{
            "type":1,
            "strongParents":[
                "0x304442486c7a05361408585e4b5f7a67441c437528755a70041e0e557a6d4b2d7d4362083d492b57",
                "0x5f736978340a243d381b343b160b316a6b7d4b1e3c0355492e2e72113c2b126600157e69113c0b5c"
            ],
            "weakParents":[
                "0x0b5a48384f382f4a49471c4860683c6f0a0d446f012e1b117c4e405f5e24497c72691f43535c0b42"
            ],
            "shallowLikeParents":[
                "0x163007217803006078040b0f51507d3572355a457839095e572f125500401b7d220c772b56165a12"
            ],
            "highestSupportedVersion":3,
            "protocolParametersHash":"0xf6021fae654975db2e82c17444dc8d43573cb4222f506fb46ba46a097cf8c873"
        },
        "signature":{
            "type":0,
            "publicKey":"0x024b6f086177156350111d5e56227242034e596b7e3d0901180873740723193c",
            "signature":"0x7c274e5e771d5d60202d334f06773d3672484b1e4e6f03231b4e69305329267a4834374b0f2e0d5c6c2f7779620f4f534c773b1679400c52303d1f23121a4049"
        }
    });

    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto).unwrap();
    let block_bytes = block.pack_to_vec();

    assert_eq!(
        block_bytes,
        [
            3, 111, 44, 91, 123, 20, 54, 103, 146, 0, 196, 223, 153, 99, 212, 134, 23, 73, 139, 240, 138, 94, 210, 135,
            188, 135, 52, 3, 65, 255, 171, 40, 112, 103, 104, 205, 58, 112, 53, 174, 94, 51, 147, 45, 154, 18, 187, 48,
            148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 112, 116, 111, 48, 112, 91, 125, 11, 66, 89, 116,
            89, 113, 77, 69, 36, 30, 90, 100, 118, 27, 9, 98, 124, 68, 123, 117, 28, 126, 20, 92, 1, 2, 48, 68, 66, 72,
            108, 122, 5, 54, 20, 8, 88, 94, 75, 95, 122, 103, 68, 28, 67, 117, 40, 117, 90, 112, 4, 30, 14, 85, 122,
            109, 75, 45, 125, 67, 98, 8, 61, 73, 43, 87, 95, 115, 105, 120, 52, 10, 36, 61, 56, 27, 52, 59, 22, 11, 49,
            106, 107, 125, 75, 30, 60, 3, 85, 73, 46, 46, 114, 17, 60, 43, 18, 102, 0, 21, 126, 105, 17, 60, 11, 92, 1,
            11, 90, 72, 56, 79, 56, 47, 74, 73, 71, 28, 72, 96, 104, 60, 111, 10, 13, 68, 111, 1, 46, 27, 17, 124, 78,
            64, 95, 94, 36, 73, 124, 114, 105, 31, 67, 83, 92, 11, 66, 1, 22, 48, 7, 33, 120, 3, 0, 96, 120, 4, 11, 15,
            81, 80, 125, 53, 114, 53, 90, 69, 120, 57, 9, 94, 87, 47, 18, 85, 0, 64, 27, 125, 34, 12, 119, 43, 86, 22,
            90, 18, 3, 246, 2, 31, 174, 101, 73, 117, 219, 46, 130, 193, 116, 68, 220, 141, 67, 87, 60, 180, 34, 47,
            80, 111, 180, 107, 164, 106, 9, 124, 248, 200, 115, 0, 2, 75, 111, 8, 97, 119, 21, 99, 80, 17, 29, 94, 86,
            34, 114, 66, 3, 78, 89, 107, 126, 61, 9, 1, 24, 8, 115, 116, 7, 35, 25, 60, 124, 39, 78, 94, 119, 29, 93,
            96, 32, 45, 51, 79, 6, 119, 61, 54, 114, 72, 75, 30, 78, 111, 3, 35, 27, 78, 105, 48, 83, 41, 38, 122, 72,
            52, 55, 75, 15, 46, 13, 92, 108, 47, 119, 121, 98, 15, 79, 83, 76, 119, 59, 22, 121, 64, 12, 82, 48, 61,
            31, 35, 18, 26, 64, 73
        ]
    );

    let block_id = block.id(&protocol_parameters()).to_string();

    assert_eq!(
        block_id,
        "0xe7577f23f82595fcf5501d3858666e5efe2e3063d715b03e43cdd93ea69d6af60200000000000000"
    );
}
