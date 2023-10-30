// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::protocol::ProtocolParameters;
use packable::PackableExt;

// Test from https://github.com/iotaledger/tips/blob/tip49/tips/TIP-0049/tip-0049.md#protocol-parameter-hash
#[test]
fn params_serde_hash() {
    let protocol_params_string = std::fs::read_to_string("./tests/types/fixtures/protocol_parameters.json").unwrap();
    let protocol_params_json = serde_json::from_str(&protocol_params_string).unwrap();
    let protocol_params = serde_json::from_value::<ProtocolParameters>(protocol_params_json).unwrap();
    let protocol_params_bytes = protocol_params.pack_to_vec();

    assert_eq!(
        protocol_params_bytes,
        [
            0, 3, 10, 84, 101, 115, 116, 74, 117, 110, 103, 108, 101, 3, 116, 103, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            193, 93, 45, 211, 247, 223, 9, 0, 46, 219, 11, 101, 0, 0, 0, 0, 10, 13, 63, 1, 17, 2, 0, 10, 0, 0, 0, 20,
            0, 0, 0, 32, 151, 64, 76, 144, 21, 10, 0, 0, 0, 10, 10, 0, 0, 0, 15, 0, 30, 0, 10, 0, 0, 0, 20, 0, 0, 0,
            24, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 12, 0, 32, 161,
            7, 0, 160, 134, 1, 0, 232, 3, 0, 0, 100, 0, 0, 0, 7, 5, 7, 8, 130, 4, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 8, 1,
            0, 0, 0, 0, 0, 0, 0, 31
        ]
    );

    assert_eq!(
        protocol_params.hash().to_string(),
        "0x0c00425134785bf2dbe42e4ec7e288009ebdc38ced797beaa45d5213092021cb"
    );
}
