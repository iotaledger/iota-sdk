// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, ClientError},
    wallet::WalletError,
};
use iota_sdk_bindings_core::Error;
use pretty_assertions::assert_eq;

#[test]
fn custom_error_serialization() {
    // testing a unit-type-like error
    let error = Error::Client(ClientError::HealthyNodePoolEmpty);
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        serde_json::json!({
            "type": "client",
            "error": {
                "type": "healthyNodePoolEmpty",
                "error": "no healthy node available"
            }
        })
    );

    // testing a tuple-like error
    let error = Error::Wallet(WalletError::InvalidMnemonic("nilly willy".to_string()));
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        serde_json::json!({
            "type": "wallet",
            "error": {
                "type": "invalidMnemonic",
                "error": "invalid mnemonic: nilly willy"
            }
        })
    );

    // testing a struct-like error
    let error = Error::Wallet(WalletError::BipPathMismatch {
        old_bip_path: None,
        new_bip_path: Some(Bip44::new(SHIMMER_COIN_TYPE)),
    });
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        serde_json::json!({
            "type": "wallet",
            "error": {
                "type": "bipPathMismatch",
                "error": "BIP44 mismatch: Some(Bip44 { coin_type: 4219, account: 0, change: 0, address_index: 0 }), existing bip path is: None"
            }
        })
    );
}
