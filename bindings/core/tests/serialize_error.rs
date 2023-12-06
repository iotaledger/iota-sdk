// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE},
        secret::PublicKeyOptions,
        Error as ClientError,
    },
    wallet::Error as WalletError,
};
use iota_sdk_bindings_core::Error;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn custom_error_serialization() {
    // testing a unit-type-like error
    let error = Error::Client(ClientError::HealthyNodePoolEmpty);
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({
            "type": "wallet",
            "error:": "no healthy node available"
        })
    );

    // testing a tuple-like error
    let error = Error::Wallet(WalletError::InvalidMnemonic("nilly willy".to_string()));
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({
            "type": "wallet",
            "error:": "invalid mnemonic: nilly willy"
        })
    );

    // testing a struct-like error
    let error = Error::Wallet(WalletError::PublicKeyOptionsMismatch {
        old: serde_json::to_value(PublicKeyOptions::new(SHIMMER_COIN_TYPE)).unwrap(),
        new: serde_json::to_value(PublicKeyOptions::new(IOTA_COIN_TYPE)).unwrap(),
    });
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({
            "type": "wallet",
            "error:": "public key options mismatch, new: PublicKeyOptions { coin_type: 4218, account: 0, change: 0, address_index: 0 }, previous: PublicKeyOptions { coin_type: 4219, account: 0, change: 0, address_index: 0 }"
        })
    );
}
