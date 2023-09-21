// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::slot::SlotCommitment;
use packable::PackableExt;

#[test]
fn slot_commitment_id() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip46/tips/TIP-0046/tip-0046.md#slot-commitment-id-1
    let slot_commitment_json = serde_json::json!({
      "protocolVersion":3,
      "index":"10",
      "previousCommitmentId":"0x4b024b3e47280d05272a7d136f0c464e4e136b734e6c427749413e286162077560652c007e37241a",
      "rootsId":"0x75614402763f5f045c040334631b791b4d755d626d504b134a505c001c516549",
      "cumulativeWeight":"100",
      "referenceManaCost":"6000"
    });

    let slot_commitment = serde_json::from_value::<SlotCommitment>(slot_commitment_json).unwrap();
    let slot_commitment_bytes = slot_commitment.pack_to_vec();

    assert_eq!(
        slot_commitment_bytes,
        [
            3, 10, 0, 0, 0, 0, 0, 0, 0, 75, 2, 75, 62, 71, 40, 13, 5, 39, 42, 125, 19, 111, 12, 70, 78, 78, 19, 107,
            115, 78, 108, 66, 119, 73, 65, 62, 40, 97, 98, 7, 117, 96, 101, 44, 0, 126, 55, 36, 26, 117, 97, 68, 2,
            118, 63, 95, 4, 92, 4, 3, 52, 99, 27, 121, 27, 77, 117, 93, 98, 109, 80, 75, 19, 74, 80, 92, 0, 28, 81,
            101, 73, 100, 0, 0, 0, 0, 0, 0, 0, 112, 23, 0, 0, 0, 0, 0, 0
        ]
    );

    let slot_commitment_id = slot_commitment.id().to_string();

    assert_eq!(
        slot_commitment_id,
        "0x3a73079f3dbf8c1744ae0b020b9767546e32f5bbbf4c6f0233da7b64f16581f80a00000000000000"
    );
}
