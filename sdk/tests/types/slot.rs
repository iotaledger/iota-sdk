// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::slot::{SlotCommitment, SlotCommitmentId};
use packable::PackableExt;
use pretty_assertions::assert_eq;
use serde::Deserialize;

#[derive(Deserialize)]
struct SlotCommitmentFixture {
    commitment: SlotCommitment,
    bytes: String,
    id: SlotCommitmentId,
}

#[test]
fn slot_commitment_id_index() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#slot-commitment-id-1
    let fixture = serde_json::from_value::<SlotCommitmentFixture>(
        serde_json::from_reader(std::fs::File::open("./tests/types/fixtures/slot_commitment.json").unwrap()).unwrap(),
    )
    .unwrap();
    let commitment_bytes = fixture.commitment.pack_to_vec();
    let commitment_id = fixture.commitment.id();

    assert_eq!(prefix_hex::encode(&commitment_bytes), fixture.bytes);
    assert_eq!(
        fixture.commitment,
        SlotCommitment::unpack_verified(commitment_bytes, &()).unwrap()
    );
    assert_eq!(commitment_id, fixture.id);
    assert_eq!(commitment_id.slot_index(), fixture.commitment.slot());
}
