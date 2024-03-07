// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::{
    block::{
        helper::network_name_to_id,
        payload::Payload,
        protocol::iota_mainnet_protocol_parameters,
        rand::{
            block::{rand_basic_block_body_builder_with_strong_parents, rand_block, rand_block_with_block_body},
            parents::rand_strong_parents,
            payload::rand_tagged_data_payload,
        },
        Block, BlockDto, BlockError,
    },
    TryFromDto,
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

// TODO reenable tests
// #[test]
// fn invalid_length() {
//     let res = BlockBuilder::new(Parents::from_vec(rand_block_ids(2)).unwrap())
//         .with_payload(TaggedDataPayload::new(vec![42], vec![0u8; Block::LENGTH_MAX - Block::LENGTH_MIN -
// 9]).unwrap())         .finish();

//     assert!(matches!(res, Err(Error::InvalidBlockLength(len)) if len == Block::LENGTH_MAX + 33));
// }

// #[test]
// fn unpack_valid_no_remaining_bytes() {
//     assert!(
//         Block::unpack_strict(
//             vec![
//                 2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168,
// 202,                 82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62,
// 38, 20,                 121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0,
// 0, 0, 21,                 205, 91, 7, 0, 0, 0, 0,
//             ]
//             .as_slice(),
//             &protocol_parameters()
//         )
//         .is_ok()
//     )
// }

// #[test]
// fn invalid_length() {
//     let res = BlockBuilder::new(Parents::from_vec(rand_block_ids(2)).unwrap())
//         .with_nonce(42)
//         .with_payload(TaggedDataPayload::new(vec![42], vec![0u8; Block::LENGTH_MAX - Block::LENGTH_MIN -
// 9]).unwrap())         .finish();

//     assert!(matches!(res, Err(Error::InvalidBlockLength(len)) if len == Block::LENGTH_MAX + 33));
// }

// #[test]
// fn unpack_valid_no_remaining_bytes() {
//     assert!(
//         Block::unpack_strict(
//             vec![
//                 2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168,
// 202,                 82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62,
// 38, 20,                 121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0,
// 0, 0, 21,                 205, 91, 7, 0, 0, 0, 0,
//             ]
//             .as_slice(),
//             &protocol_parameters()
//         )
//         .is_ok()
//     )
// }

// #[test]
// fn unpack_invalid_remaining_bytes() {
//     assert!(matches!(
//         Block::unpack_strict(
//             vec![
//                 2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168,
// 202,                 82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62,
// 38, 20,                 121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0,
// 0, 0, 21,                 205, 91, 7, 0, 0, 0, 0, 42
//             ]
//             .as_slice(),
//             &protocol_parameters()
//         ),
//         Err(UnpackError::Packable(Error::RemainingBytesAfterBlock))
//     ))
// }

// Validate that a `unpack` ∘ `pack` round-trip results in the original block.
#[test]
fn pack_unpack_valid() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let block = rand_block(protocol_parameters.clone());
    let packed_block = block.pack_to_vec();

    assert_eq!(packed_block.len(), block.packed_len());
    assert_eq!(
        block,
        PackableExt::unpack_bytes_verified(packed_block.as_slice(), protocol_parameters).unwrap()
    );
}

#[test]
fn getters() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let parents = rand_strong_parents();
    let payload = Payload::from(rand_tagged_data_payload());

    let block_body = rand_basic_block_body_builder_with_strong_parents(parents.clone())
        .with_payload(payload.clone())
        .finish_block_body()
        .unwrap();
    let block = rand_block_with_block_body(protocol_parameters.clone(), block_body);

    assert_eq!(block.protocol_version(), protocol_parameters.version());
    assert_eq!(*block.as_basic().strong_parents(), parents);
    assert_eq!(*block.as_basic().payload().as_ref().unwrap(), &payload);
}

#[test]
fn dto_mismatch_version() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let protocol_parameters_hash = protocol_parameters.hash();
    let slot_index = 11_u64;
    let issuing_time = protocol_parameters.genesis_unix_timestamp()
        + (slot_index - 1) * protocol_parameters.slot_duration_in_seconds() as u64;
    let network_id = protocol_parameters.network_id();
    let protocol_version = 4;
    let block_dto_json = serde_json::json!({
        "header": {
            "protocolVersion": protocol_version,
            "networkId": network_id.to_string(),
            "issuingTime": issuing_time.to_string(),
            "slotCommitmentId": "0x8633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd700000000",
            "latestFinalizedSlot": 0,
            "issuerId": "0x0000000000000000000000000000000000000000000000000000000000000000",
        },
        "body": {
            "type":1,
            "strongParents": [ "0x417c5700320912627b604d4c376a5a1663634b09703538570b1d52440b3e474639490b10" ],
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
    let block_dto = serde_json::from_value::<BlockDto>(block_dto_json).unwrap();
    let block_res = Block::try_from_dto_with_params(block_dto, protocol_parameters);

    assert_eq!(
        block_res,
        Err(BlockError::ProtocolVersionMismatch {
            expected: protocol_parameters.version(),
            actual: protocol_version
        })
    );
}

#[test]
fn dto_mismatch_network_id() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let protocol_parameters_hash = protocol_parameters.hash();
    let slot_index = 11_u64;
    let issuing_time = protocol_parameters.genesis_unix_timestamp()
        + (slot_index - 1) * protocol_parameters.slot_duration_in_seconds() as u64;
    let network_id = network_name_to_id("invalid-network");
    let block_dto_json = serde_json::json!({
        "header": {
            "protocolVersion": 3,
            "networkId": network_id.to_string(),
            "issuingTime": issuing_time.to_string(),
            "slotCommitmentId": "0x8633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd700000000",
            "latestFinalizedSlot": 0,
            "issuerId": "0x0000000000000000000000000000000000000000000000000000000000000000",
        },
        "body": {
            "type":1,
            "strongParents": [ "0x417c5700320912627b604d4c376a5a1663634b09703538570b1d52440b3e474639490b10" ],
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
    let block_dto = serde_json::from_value::<BlockDto>(block_dto_json).unwrap();
    let block_res = Block::try_from_dto_with_params(block_dto, protocol_parameters);

    assert_eq!(
        block_res,
        Err(BlockError::NetworkIdMismatch {
            expected: protocol_parameters.network_id(),
            actual: network_id
        })
    );
}
