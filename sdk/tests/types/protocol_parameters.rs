// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::protocol::ProtocolParameters;
use packable::PackableExt;
use pretty_assertions::assert_eq;

// // Test from https://github.com/iotaledger/tips/blob/tip49/tips/TIP-0049/tip-0049.md#protocol-parameter-hash
// #[test]
// fn serde_packable_hash() {
//     let file = std::fs::read_to_string("./tests/types/fixtures/protocol_parameters.json").unwrap();
//     let json = serde_json::from_str::<serde_json::Value>(&file).unwrap();
//     let params_json = &json["params"];
//     let params = serde_json::from_value::<ProtocolParameters>(params_json.clone()).unwrap();
//     let params_bytes = params.pack_to_vec();

//     assert_eq!(prefix_hex::encode(&params_bytes), json["bytes"]);
//     assert_eq!(params, ProtocolParameters::unpack_verified(params_bytes, &()).unwrap());
//     assert_eq!(params.hash().to_string(), json["hash"]);
// }
