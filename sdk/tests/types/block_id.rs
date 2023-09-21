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
    ProtocolParameters::new(3, "test", "rms", RentStructure::default(), 0, 1695275822000000, 10, 0).unwrap()
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
        "networkId":"3650798313638353144",
        "issuingTime":"1694613463455944982",
        "slotCommitmentId":"0x5893048146ba125ea69c85a08bc122afec11baf6115ce03fc1d989df6aff2cdf1500000000000000",
        "latestFinalizedSlot":"21",
        "issuerId":"0x907c02e9302e0f0571f10f885594e56d8c54ff0708ab7a39bc1b74d396b93b12",
        "block":{
            "type":0,
            "strongParents":[
                "0xac5668c6ba7b3471a37e4a64c6611090b67babd368e1f5b670e30fb4eadba59d1c00000000000000"
            ],
            "weakParents":[],
            "shallowLikeParents":[],
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
            "publicKey":"0x293dc170d9a59474e6d81cfba7f7d924c09b25d7166bcfba606e53114d0a758b",
            "signature":"0xf4dca05ba024867984cfd657aac7c4f782a8bf5830dcbea482f351c578ffb4db061dd30a31816e9b7ba6e7096d63ff1209f9a2a0449ac7277cae5d3ebc87310f"
        }
    });

    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto).unwrap();
    let block_bytes = block.pack_to_vec();

    assert_eq!(
        block_bytes,
        [
            3, 248, 88, 2, 55, 185, 61, 170, 50, 22, 209, 226, 112, 247, 121, 132, 23, 88, 147, 4, 129, 70, 186, 18,
            94, 166, 156, 133, 160, 139, 193, 34, 175, 236, 17, 186, 246, 17, 92, 224, 63, 193, 217, 137, 223, 106,
            255, 44, 223, 21, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 144, 124, 2, 233, 48, 46, 15, 5, 113, 241,
            15, 136, 85, 148, 229, 109, 140, 84, 255, 7, 8, 171, 122, 57, 188, 27, 116, 211, 150, 185, 59, 18, 0, 1,
            172, 86, 104, 198, 186, 123, 52, 113, 163, 126, 74, 100, 198, 97, 16, 144, 182, 123, 171, 211, 104, 225,
            245, 182, 112, 227, 15, 180, 234, 219, 165, 157, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 1, 0, 0, 6, 0, 0, 0, 2,
            248, 88, 2, 55, 185, 61, 170, 50, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 36, 255, 155, 48, 56, 80, 111,
            177, 180, 6, 48, 106, 73, 96, 1, 195, 226, 78, 43, 224, 124, 131, 131, 23, 146, 43, 242, 29, 104, 106, 7,
            143, 10, 0, 183, 12, 111, 134, 161, 234, 3, 165, 154, 113, 215, 61, 205, 7, 226, 8, 43, 189, 240, 206, 151,
            31, 170, 33, 116, 131, 72, 188, 162, 47, 176, 35, 1, 0, 3, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 217, 248, 68, 88, 40, 109, 196, 28, 211, 71, 137, 222, 197, 102, 205, 9, 108, 244, 125, 233,
            145, 170, 54, 169, 122, 235, 250, 234, 20, 18, 143, 109, 0, 0, 0, 33, 0, 0, 0, 5, 0, 0, 0, 12, 29, 123, 62,
            17, 105, 114, 100, 17, 30, 19, 11, 14, 12, 0, 0, 0, 29, 123, 62, 17, 105, 114, 100, 17, 30, 19, 11, 14, 1,
            0, 0, 0, 128, 51, 97, 254, 30, 255, 200, 153, 220, 167, 249, 49, 216, 173, 7, 192, 27, 162, 58, 170, 147,
            249, 134, 173, 176, 77, 76, 23, 207, 99, 104, 216, 204, 221, 186, 195, 170, 172, 65, 62, 1, 147, 225, 109,
            163, 68, 159, 48, 193, 131, 208, 231, 234, 167, 243, 3, 220, 18, 174, 13, 188, 159, 184, 144, 228, 73, 165,
            47, 144, 86, 231, 217, 82, 234, 121, 111, 211, 229, 100, 95, 96, 217, 235, 152, 237, 145, 203, 50, 97, 114,
            15, 181, 40, 210, 161, 5, 20, 193, 2, 0, 0, 0, 0, 0, 0, 41, 61, 193, 112, 217, 165, 148, 116, 230, 216, 28,
            251, 167, 247, 217, 36, 192, 155, 37, 215, 22, 107, 207, 186, 96, 110, 83, 17, 77, 10, 117, 139, 244, 220,
            160, 91, 160, 36, 134, 121, 132, 207, 214, 87, 170, 199, 196, 247, 130, 168, 191, 88, 48, 220, 190, 164,
            130, 243, 81, 197, 120, 255, 180, 219, 6, 29, 211, 10, 49, 129, 110, 155, 123, 166, 231, 9, 109, 99, 255,
            18, 9, 249, 162, 160, 68, 154, 199, 39, 124, 174, 93, 62, 188, 135, 49, 15
        ]
    );

    let block_id = block.id(&ProtocolParameters::default()).to_string();

    assert_eq!(
        block_id,
        "0xddebfd0434bfe8140c938d4124ab76e9fee7e18ca7a7be1ba7331d42868724d91c00000000000000"
    );
}

#[test]
fn validation_block_id() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#validation-block-id

    let block_json = serde_json::json!({
        "protocolVersion":3,
        "networkId":"0",
        "issuingTime":"1694671670149673852",
        "slotCommitmentId":"0xe0a1befd84ec67c1f70e97db1b2011fb391af164c89b92ecf277add20e3bdba10700000000000000",
        "latestFinalizedSlot":"7",
        "issuerId":"0x375358f92cc94750669598b0aaa55a6ff73310b90710e1fad524c0f911be0fea",
        "block":{
            "type":1,
            "strongParents":[
                "0x4c0f68416d3eba7a7a8a570983ac63bbacb94f97908cde435292932af37d8a060e00000000000000"
            ],
            "weakParents":[],
            "shallowLikeParents":[],
            "highestSupportedVersion":3,
            "protocolParametersHash":"0xf8b1cbfe835723facdcd83b2ec042346a4404df11082366e906894aa9ac7e813"
        },
        "signature":{
            "type":0,
            "publicKey":"0x05c1de274451db8de8182d64c6ee0dca3ae0c9077e0b4330c976976171d79064",
            "signature":"0x4b7b3bd8f184334740ac04443b774ef299fe56b24e88f3d60d1df6a33b4581b0c791c98085eb9c2d48e7646b17261c43d4e7be1ecf4e015d5488395017d82906"
        }
    });

    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto).unwrap();
    let block_bytes = block.pack_to_vec();

    assert_eq!(
        block_bytes,
        [
            3, 0, 0, 0, 0, 0, 0, 0, 0, 124, 115, 48, 190, 231, 174, 132, 23, 224, 161, 190, 253, 132, 236, 103, 193,
            247, 14, 151, 219, 27, 32, 17, 251, 57, 26, 241, 100, 200, 155, 146, 236, 242, 119, 173, 210, 14, 59, 219,
            161, 7, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 55, 83, 88, 249, 44, 201, 71, 80, 102, 149, 152, 176,
            170, 165, 90, 111, 247, 51, 16, 185, 7, 16, 225, 250, 213, 36, 192, 249, 17, 190, 15, 234, 1, 1, 76, 15,
            104, 65, 109, 62, 186, 122, 122, 138, 87, 9, 131, 172, 99, 187, 172, 185, 79, 151, 144, 140, 222, 67, 82,
            146, 147, 42, 243, 125, 138, 6, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 248, 177, 203, 254, 131, 87, 35, 250,
            205, 205, 131, 178, 236, 4, 35, 70, 164, 64, 77, 241, 16, 130, 54, 110, 144, 104, 148, 170, 154, 199, 232,
            19, 0, 5, 193, 222, 39, 68, 81, 219, 141, 232, 24, 45, 100, 198, 238, 13, 202, 58, 224, 201, 7, 126, 11,
            67, 48, 201, 118, 151, 97, 113, 215, 144, 100, 75, 123, 59, 216, 241, 132, 51, 71, 64, 172, 4, 68, 59, 119,
            78, 242, 153, 254, 86, 178, 78, 136, 243, 214, 13, 29, 246, 163, 59, 69, 129, 176, 199, 145, 201, 128, 133,
            235, 156, 45, 72, 231, 100, 107, 23, 38, 28, 67, 212, 231, 190, 30, 207, 78, 1, 93, 84, 136, 57, 80, 23,
            216, 41, 6
        ]
    );

    let block_id = block.id(&ProtocolParameters::default()).to_string();

    assert_eq!(
        block_id,
        "0x37ea373f2c0c7b78ab87298e754931f775df985171ddad7e9e9fb64d8b5219a70e00000000000000"
    );
}
