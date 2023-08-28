// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::public_key::PublicKey;

#[test]
#[cfg(feature = "serde")]
fn public_key_de_serialization() {
    let serialized_str =
        r#"{"type":0,"publicKey":"0xfc5cef12850558d533a6c485051b4397fd9b4349f90ab62c58e3ea99f1f47d16"}"#;
    let ed25519_public_key: PublicKey = serde_json::from_str(serialized_str).unwrap();
    assert_eq!(serde_json::to_string(&ed25519_public_key).unwrap(), serialized_str);
}
