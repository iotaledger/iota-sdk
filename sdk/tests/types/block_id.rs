// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::{
    block::{slot::SlotIndex, Block, BlockDto, BlockId},
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
fn compute() {
    let block_dto_json = r#"{"protocolVersion":3,"networkId":"0","issuingTime":"0","slotCommitment":"0x8633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd70000000000000000","latestFinalizedSlot":"0","issuerId":"0x0000000000000000000000000000000000000000000000000000000000000000","block":{"type":1,"strongParents":["0x417c5700320912627b604d4c376a5a1663634b09703538570b1d52440b3e474639490b100a6f3608"],"weakParents":[],"shallowLikeParents":[],"highestSupportedVersion":3,"protocolParametersHash":"0x0000000000000000000000000000000000000000000000000000000000000000"},"signature":{"type":0,"publicKey":"0x714f5f07067012267c21426d382a52752f0b3208443e0e3c49183e0110494148","signature":"0x3e4a492924302b3b093f1e4266757a1d2041480a3861271d4c2e646d4e3d08360a3e765e1a385a784f6753276c233123475867370a184573195d530b41643a1d"}}"#;
    let block_dto = serde_json::from_str::<BlockDto>(block_dto_json).unwrap();
    let block = Block::try_from_dto(block_dto).unwrap();
    let block_hash = block.hash();
    let block_id = block_hash.with_slot_index(SlotIndex::new(0));

    assert_eq!(
        block_id.to_string(),
        "0x836b8e132a331263f3da03735d22f1cb99e42d86dab09447d3bec113005c82490000000000000000"
    );
}
