// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{client::Error as ClientError, wallet::Error as WalletError};
use iota_sdk_bindings_core::Error;

#[test]
fn custom_error_serialization() {
    let error = Error::Client(ClientError::HealthyNodePoolEmpty);
    assert_eq!(
        serde_json::to_string(&error).unwrap(),
        "{\"type\":\"client\",\"error\":\"no healthy node available\"}"
    );
    let error = Error::Wallet(WalletError::AccountNotFound("Alice".to_string()));
    assert_eq!(
        serde_json::to_string(&error).unwrap(),
        "{\"type\":\"wallet\",\"error\":\"account Alice not found\"}"
    );
}
