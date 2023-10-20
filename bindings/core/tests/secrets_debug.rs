// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::secret::SecretManagerDto;
use iota_sdk_bindings_core::{ClientMethod, Response, UtilsMethod, WalletOptions};
use pretty_assertions::assert_eq;

#[test]
fn method_interface_secrets_debug() {
    let utils_method = UtilsMethod::MnemonicToHexSeed {
        mnemonic: "mnemonic".to_string(),
    };
    assert_eq!(
        format!("{:?}", utils_method),
        "MnemonicToHexSeed { mnemonic: <omitted> }"
    );

    let wallet_method = UtilsMethod::VerifyMnemonic {
        mnemonic: "mnemonic".to_string(),
    };
    assert_eq!(format!("{:?}", wallet_method), "VerifyMnemonic { mnemonic: <omitted> }");

    let response = Response::GeneratedMnemonic("mnemonic".to_string());
    assert_eq!(format!("{:?}", response), "GeneratedMnemonic(<omitted>)");

    let wallet_options = WalletOptions::default().with_secret_manager(SecretManagerDto::Placeholder);
    assert_eq!(
        format!("{:?}", wallet_options),
        "WalletOptions { storage_path: None, client_options: None, coin_type: None, secret_manager: Some(<omitted>) }"
    );
}
