// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::slot::SlotCommitment;
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn slot_commitment_id_index() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#slot-commitment-id-1
    let file = std::fs::read_to_string("./tests/types/fixtures/slot_commitment.json").unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&file).unwrap();
    let slot_commitment_json = &json["commitment"];
    let slot_commitment = serde_json::from_value::<SlotCommitment>(slot_commitment_json.clone()).unwrap();
    let slot_commitment_bytes = slot_commitment.pack_to_vec();
    let slot_commitment_id = slot_commitment.id();

    assert_eq!(prefix_hex::encode(&slot_commitment_bytes), json["bytes"]);
    assert_eq!(
        slot_commitment,
        SlotCommitment::unpack_unverified(slot_commitment_bytes).unwrap()
    );
    assert_eq!(slot_commitment_id.to_string(), json["id"]);
    assert_eq!(slot_commitment_id.slot_index(), slot_commitment.slot());
}
