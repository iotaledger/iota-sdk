// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    slot::{RootsId, SlotCommitment, SlotCommitmentId, SlotIndex},
    PROTOCOL_VERSION,
};

#[test]
fn slot_commitment_id() {
    let commitment = SlotCommitment::new(
        PROTOCOL_VERSION,
        SlotIndex::new(10),
        SlotCommitmentId::from_str(
            "0x20e07a0ea344707d69a08b90be7ad14eec8326cf2b8b86c8ec23720fab8dcf8ec43a30e4a8cc3f1f",
        )
        .unwrap(),
        RootsId::from_str("0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f").unwrap(),
        5,
        10,
    );
    // TODO: Independently verify this value
    assert_eq!(
        &commitment.id().to_string(),
        "0x2f3ad38aa65d20ede9dcd6a045dccdd3332cf38192c4875308bb77116e8650880a00000000000000"
    )
}
