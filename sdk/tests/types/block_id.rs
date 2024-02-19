// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::{
    block::{
        protocol::{ProtocolParameters, WorkScore},
        rand::bytes::rand_bytes_array,
        slot::SlotIndex,
        Block, BlockDto, BlockHash, BlockId,
    },
    TryFromDto,
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

const BLOCK_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", BlockId::from_str(BLOCK_ID).unwrap()),
        r#"BlockId { id: "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000", slot_index: SlotIndex(0) }"#
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

    assert_eq!(block_id.packed_len(), 36);
    assert_eq!(block_id.pack_to_vec().len(), 36);
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
    let slot_index = SlotIndex(12345);
    let block_id = block_hash.into_block_id(slot_index);
    assert_eq!(slot_index, block_id.slot_index());
    let memory_layout =
        <[u8; BlockId::LENGTH]>::try_from([block_hash.as_ref(), &slot_index.to_le_bytes()].concat()).unwrap();
    assert_eq!(block_id.as_ref(), memory_layout);
}

fn protocol_parameters() -> ProtocolParameters {
    let file = std::fs::read_to_string("./tests/types/fixtures/protocol_parameters.json").unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&file).unwrap();
    let params_json = &json["params"];
    let params = serde_json::from_value::<ProtocolParameters>(params_json.clone()).unwrap();
    assert_eq!(params.hash().to_string(), json["hash"]);
    serde_json::from_value::<ProtocolParameters>(params_json.clone()).unwrap()
}

// #[test]
// fn basic_block_tagged_data_payload_id() {
//     // Test vector from https://github.com/iotaledger/tips/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-tagged-data-payload
//     let protocol_parameters = protocol_parameters();
//     let file = std::fs::read_to_string("./tests/types/fixtures/basic_block_tagged_data_payload.json").unwrap();
//     let json = serde_json::from_str::<serde_json::Value>(&file).unwrap();
//     let block_json = &json["block"];
//     let block_dto = serde_json::from_value::<BlockDto>(block_json.clone()).unwrap();
//     let block = Block::try_from_dto(block_dto).unwrap();
//     let block_bytes = block.pack_to_vec();
//     let block_work_score = block.as_basic().work_score(protocol_parameters.work_score_parameters());

//     assert_eq!(prefix_hex::encode(&block_bytes), json["bytes"]);
//     assert_eq!(block, Block::unpack_unverified(block_bytes).unwrap());
//     assert_eq!(block.id(&protocol_parameters).to_string(), json["id"]);
//     assert_eq!(block_work_score, json["workScore"]);
// }

// #[test]
// fn basic_block_transaction_payload_id() {
//     // Test vector from https://github.com/iotaledger/tips/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-transaction-payload
//     let protocol_parameters = protocol_parameters();
//     let file = std::fs::read_to_string("./tests/types/fixtures/basic_block_transaction_payload.json").unwrap();
//     let json = serde_json::from_str::<serde_json::Value>(&file).unwrap();
//     let block_json = &json["block"];
//     let block_dto = serde_json::from_value::<BlockDto>(block_json.clone()).unwrap();
//     let block = Block::try_from_dto(block_dto).unwrap();
//     let block_bytes = block.pack_to_vec();
//     let block_work_score = block.as_basic().work_score(protocol_parameters.work_score_parameters());

//     assert_eq!(prefix_hex::encode(&block_bytes), json["bytes"]);
//     assert_eq!(block, Block::unpack_unverified(block_bytes).unwrap());
//     assert_eq!(block.id(&protocol_parameters).to_string(), json["id"]);
//     assert_eq!(block_work_score, json["workScore"]);
// }

// #[test]
// fn validation_block_id() {
//     // Test vector from https://github.com/iotaledger/tips/blob/tip46/tips/TIP-0046/tip-0046.md#validation-block-id
//     let file = std::fs::read_to_string("./tests/types/fixtures/validation_block.json").unwrap();
//     let json = serde_json::from_str::<serde_json::Value>(&file).unwrap();
//     let block_json = &json["block"];
//     let block_dto = serde_json::from_value::<BlockDto>(block_json.clone()).unwrap();
//     let block = Block::try_from_dto(block_dto).unwrap();
//     let block_bytes = block.pack_to_vec();

//     assert_eq!(prefix_hex::encode(&block_bytes), json["bytes"]);
//     assert_eq!(block, Block::unpack_unverified(block_bytes).unwrap());
//     assert_eq!(block.id(&protocol_parameters()).to_string(), json["id"]);
// }
