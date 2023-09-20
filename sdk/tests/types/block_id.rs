// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::{
    block::{
        protocol::ProtocolParameters, rand::bytes::rand_bytes_array, slot::SlotIndex, BlockHash, BlockId, BlockWrapper,
        BlockWrapperDto,
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

#[test]
fn basic_block_id_tagged_data_payload() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-tagged-data-payload
    let block_json = serde_json::json!({
        "protocolVersion":3,
        "networkId":"0",
        "issuingTime":"1694608262748556515",
        "slotCommitmentId":"0x49100d1d05557c7c204c221d5a3415570009752f7b10356d062c6d730d0568497e2c3c3d480d7d23",
        "latestFinalizedSlot":"2",
        "issuerId":"0x0000000000000000000000000000000000000000000000000000000000000000",
        "block": {
            "type":0,
            "strongParents":[
                "0x00627e507d71337c3c1647747e5a4b565f2a71480e62170e1b3f085e0e0a41321c017a4d2b3b4210",
                "0x03485a0516062d003a646a40676d5c1c1e0d574822443c630a08385836101a5c33181f2314573706",
                "0x4274246513610f1b044562676e125e1e11781e5a0a2b4c5d1d6e7c2b792c163e5f74393149311513",
                "0x7549226845340f730244733152263e553036684748704d5f74017c3b573c1b21345e59644c286b5e"
            ],
        "weakParents":[
            "0x12731a211b5323256c2c295564532a246125681d1163585f077149137578585e75407a5a4c51485a"
        ],
        "shallowLikeParents":[
            "0x1e1f5b10094a7e01134d65325f395c2d507703172e4b740814582b5a52536c1315706727541a582b"
        ],
        "payload":{
            "type":5,
            "tag":"0x68656c6c6f20776f726c64",
            "data":"0x01020304"
        },
        "burnedMana":"100"
        },
        "signature":{
            "type":0,
            "publicKey":"0x0000000000000000000000000000000000000000000000000000000000000000",
            "signature":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        }
    });

    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto).unwrap();
    let block_bytes = block.pack_to_vec();

    assert_eq!(
        block_bytes,
        [
            3, 0, 0, 0, 0, 0, 0, 0, 0, 227, 204, 145, 142, 60, 117, 132, 23, 73, 16, 13, 29, 5, 85, 124, 124, 32, 76,
            34, 29, 90, 52, 21, 87, 0, 9, 117, 47, 123, 16, 53, 109, 6, 44, 109, 115, 13, 5, 104, 73, 126, 44, 60, 61,
            72, 13, 125, 35, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 98, 126, 80, 125, 113, 51, 124, 60, 22, 71, 116, 126, 90, 75, 86,
            95, 42, 113, 72, 14, 98, 23, 14, 27, 63, 8, 94, 14, 10, 65, 50, 28, 1, 122, 77, 43, 59, 66, 16, 3, 72, 90,
            5, 22, 6, 45, 0, 58, 100, 106, 64, 103, 109, 92, 28, 30, 13, 87, 72, 34, 68, 60, 99, 10, 8, 56, 88, 54, 16,
            26, 92, 51, 24, 31, 35, 20, 87, 55, 6, 66, 116, 36, 101, 19, 97, 15, 27, 4, 69, 98, 103, 110, 18, 94, 30,
            17, 120, 30, 90, 10, 43, 76, 93, 29, 110, 124, 43, 121, 44, 22, 62, 95, 116, 57, 49, 73, 49, 21, 19, 117,
            73, 34, 104, 69, 52, 15, 115, 2, 68, 115, 49, 82, 38, 62, 85, 48, 54, 104, 71, 72, 112, 77, 95, 116, 1,
            124, 59, 87, 60, 27, 33, 52, 94, 89, 100, 76, 40, 107, 94, 1, 18, 115, 26, 33, 27, 83, 35, 37, 108, 44, 41,
            85, 100, 83, 42, 36, 97, 37, 104, 29, 17, 99, 88, 95, 7, 113, 73, 19, 117, 120, 88, 94, 117, 64, 122, 90,
            76, 81, 72, 90, 1, 30, 31, 91, 16, 9, 74, 126, 1, 19, 77, 101, 50, 95, 57, 92, 45, 80, 119, 3, 23, 46, 75,
            116, 8, 20, 88, 43, 90, 82, 83, 108, 19, 21, 112, 103, 39, 84, 26, 88, 43, 24, 0, 0, 0, 5, 0, 0, 0, 11,
            104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 4, 0, 0, 0, 1, 2, 3, 4, 100, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );

    let block_id = block.id(&ProtocolParameters::default()).to_string();

    assert_eq!(
        block_id,
        "0xd5b4743580e968c6418b279620fa690331b51cfff28464f8d767e43fd586f0190100000000000000"
    );
}

#[test]
fn basic_block_id_transaction_payload() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-transaction-payload
    let block_json = serde_json::json!({
        "protocolVersion":3,
        "networkId":"3650798313638353144",
        "issuingTime":"1694613463455944982",
        "slotCommitment":"0x5893048146ba125ea69c85a08bc122afec11baf6115ce03fc1d989df6aff2cdf1500000000000000",
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
