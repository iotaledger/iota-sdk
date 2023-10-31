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
        prefix_hex::encode(protocol_params_bytes),
        "0x00030a546573744a756e676c650374676c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000003f011102000a000000140000002097404c9015c15d2dd3f7df09002edb0b65000000000a0d0a0000000a0a0000000f001e000a000000140000001800000001000000000000000000000000000000000000000000000000350c0020a10700a0860100e803000064000000070507088204000002000000000000000801000000000000001f"
    );

    assert_eq!(
        protocol_params.hash().to_string(),
        "0x55dad02be983b85257ca6481ce51d77cd250a91c9ce7fe6b1db7295301297d7a"
    );
}
