// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    protocol::ProtocolParameters, rand::bytes::rand_bytes_array, slot::SlotIndex, BlockHash, BlockId, BlockWrapper,
    BlockWrapperDto,
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
fn compute() {
    let protocol_parameters = ProtocolParameters::default();
    let protocol_parameters_hash = protocol_parameters.hash();
    let slot_index = SlotIndex::new(11_u64);
    let issuing_time = slot_index.to_timestamp(
        protocol_parameters.genesis_unix_timestamp(),
        protocol_parameters.slot_duration_in_seconds(),
    );
    let network_id = protocol_parameters.network_id();

    let block_dto_json = serde_json::json!({
        "protocolVersion": 3,
        "networkId": network_id.to_string(),
        "issuingTime": issuing_time.to_string(),
        "slotCommitmentId": "0x8633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd70000000000000000",
        "latestFinalizedSlot": "0",
        "issuerId": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "block": {
            "type":1,
            "strongParents": [ "0x417c5700320912627b604d4c376a5a1663634b09703538570b1d52440b3e474639490b100a6f3608" ],
            "weakParents": [],
            "shallowLikeParents": [],
            "highestSupportedVersion": 3,
            "protocolParametersHash": protocol_parameters_hash
        },
        "signature": {
            "type": 0,
            "publicKey": "0x714f5f07067012267c21426d382a52752f0b3208443e0e3c49183e0110494148",
            "signature": "0x3e4a492924302b3b093f1e4266757a1d2041480a3861271d4c2e646d4e3d08360a3e765e1a385a784f6753276c233123475867370a184573195d530b41643a1d"
        }
    });
    let block_dto = serde_json::from_value::<BlockWrapperDto>(block_dto_json).unwrap();
    let block = BlockWrapper::try_from_dto(block_dto, protocol_parameters).unwrap();
    let block_id = block.id();

    // TODO: Independently verify this value
    assert_eq!(
        block_id.to_string(),
        "0x38c43e391e222e6a4afd1f48948b17a6d03c5cd63fb8dce69fbb493eaf7c5d150b00000000000000"
    );
    assert_eq!(
        block_id.hash().to_string(),
        "0x38c43e391e222e6a4afd1f48948b17a6d03c5cd63fb8dce69fbb493eaf7c5d15"
    );
    assert_eq!(block_id.slot_index(), slot_index);
}

// TODO can't really be done at the moment, would be easier if/when we remove protocol parameters from the wrapper.
// #[test]
// fn basic_block_tagged_data_payload_id() {
//     // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-tagged-data-payload
//     let block_json = r#"
//     {
//         "protocolVersion":3,
//         "networkId":"0",
//         "issuingTime":"1694608262748556515",
//         "slotCommitmentId":"0x49100d1d05557c7c204c221d5a3415570009752f7b10356d062c6d730d0568497e2c3c3d480d7d23",
//         "latestFinalizedSlot":"2",
//         "issuerId":"0x0000000000000000000000000000000000000000000000000000000000000000",
//         "block": {
//           "type":0,
//           "strongParents":[
//             "0x00627e507d71337c3c1647747e5a4b565f2a71480e62170e1b3f085e0e0a41321c017a4d2b3b4210",
//             "0x03485a0516062d003a646a40676d5c1c1e0d574822443c630a08385836101a5c33181f2314573706",
//             "0x4274246513610f1b044562676e125e1e11781e5a0a2b4c5d1d6e7c2b792c163e5f74393149311513",
//             "0x7549226845340f730244733152263e553036684748704d5f74017c3b573c1b21345e59644c286b5e"
//           ],
//           "weakParents":[
//             "0x12731a211b5323256c2c295564532a246125681d1163585f077149137578585e75407a5a4c51485a"
//           ],
//           "shallowLikeParents":[
//             "0x1e1f5b10094a7e01134d65325f395c2d507703172e4b740814582b5a52536c1315706727541a582b"
//           ],
//           "payload":{
//             "type":5,
//             "tag":"0x68656c6c6f20776f726c64",
//             "data":"0x01020304"
//           },
//           "burnedMana":"100"
//         },
//         "signature":{
//           "type":0,
//           "publicKey":"0x0000000000000000000000000000000000000000000000000000000000000000",
//           "signature":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
//         }
//       }
//       "#;

//     let block_dto = serde_json::from_str::<BlockWrapperDto>(block_json).unwrap();
//     let block = BlockWrapper::try_from_dto(block_dto, ProtocolParameters::default()).unwrap();
//     let block_bytes = block.pack_to_vec();

//     assert_eq!(
//         block_bytes,
//         [
//             3, 0, 0, 0, 0, 0, 0, 0, 0, 227, 204, 145, 142, 60, 117, 132, 23, 73, 16, 13, 29, 5, 85, 124, 124, 32, 76,
//             34, 29, 90, 52, 21, 87, 0, 9, 117, 47, 123, 16, 53, 109, 6, 44, 109, 115, 13, 5, 104, 73, 126, 44, 60,
// 61,             72, 13, 125, 35, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 0, 0,             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 98, 126, 80, 125, 113, 51, 124, 60, 22, 71, 116, 126, 90,
// 75, 86,             95, 42, 113, 72, 14, 98, 23, 14, 27, 63, 8, 94, 14, 10, 65, 50, 28, 1, 122, 77, 43, 59, 66, 16,
// 3, 72, 90,             5, 22, 6, 45, 0, 58, 100, 106, 64, 103, 109, 92, 28, 30, 13, 87, 72, 34, 68, 60, 99, 10, 8,
// 56, 88, 54, 16,             26, 92, 51, 24, 31, 35, 20, 87, 55, 6, 66, 116, 36, 101, 19, 97, 15, 27, 4, 69, 98, 103,
// 110, 18, 94, 30,             17, 120, 30, 90, 10, 43, 76, 93, 29, 110, 124, 43, 121, 44, 22, 62, 95, 116, 57, 49, 73,
// 49, 21, 19, 117,             73, 34, 104, 69, 52, 15, 115, 2, 68, 115, 49, 82, 38, 62, 85, 48, 54, 104, 71, 72, 112,
// 77, 95, 116, 1,             124, 59, 87, 60, 27, 33, 52, 94, 89, 100, 76, 40, 107, 94, 1, 18, 115, 26, 33, 27, 83,
// 35, 37, 108, 44, 41,             85, 100, 83, 42, 36, 97, 37, 104, 29, 17, 99, 88, 95, 7, 113, 73, 19, 117, 120, 88,
// 94, 117, 64, 122, 90,             76, 81, 72, 90, 1, 30, 31, 91, 16, 9, 74, 126, 1, 19, 77, 101, 50, 95, 57, 92, 45,
// 80, 119, 3, 23, 46, 75,             116, 8, 20, 88, 43, 90, 82, 83, 108, 19, 21, 112, 103, 39, 84, 26, 88, 43, 24, 0,
// 0, 0, 5, 0, 0, 0, 11,             104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 4, 0, 0, 0, 1, 2, 3, 4, 100,
// 0, 0, 0, 0, 0, 0, 0, 0,             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 0, 0, 0, 0, 0, 0, 0, 0, 0,             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]
//     );

//     // let slot_commitment_id = slot_commitment.id().to_string();

//     // assert_eq!(
//     //     slot_commitment_id,
//     //     "0x3a73079f3dbf8c1744ae0b020b9767546e32f5bbbf4c6f0233da7b64f16581f80a00000000000000"
//     // );
// }
