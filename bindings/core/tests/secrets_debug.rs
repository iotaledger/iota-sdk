// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::secret::SecretManagerDto;
use iota_sdk_bindings_core::{ClientMethod, Response, UtilsMethod, WalletOptions};

#[test]
fn method_interface_secrets_debug() {
    let client_method = ClientMethod::BuildAndPostBlock {
        secret_manager: None,
        options: None,
    };
    assert_eq!(
        format!("{:?}", client_method),
        "BuildAndPostBlock { secret_manager: None, options: None }"
    );

    #[cfg(feature = "ledger_nano")]
    {
        let client_method = ClientMethod::BuildAndPostBlock {
            secret_manager: Some(SecretManagerDto::LedgerNano(false)),
            options: None,
        };
        assert_eq!(
            format!("{:?}", client_method),
            "BuildAndPostBlock { secret_manager: Some(<omitted>), options: None }"
        );
    }

    let client_method = UtilsMethod::MnemonicToHexSeed {
        mnemonic: "mnemonic".to_string(),
    };
    assert_eq!(
        format!("{:?}", client_method),
        "MnemonicToHexSeed { mnemonic: <omitted> }"
    );

    let wallet_method = UtilsMethod::VerifyMnemonic {
        mnemonic: "mnemonic".to_string(),
    };
    assert_eq!(format!("{:?}", wallet_method), "VerifyMnemonic { mnemonic: <omitted> }");

    let response = Response::GeneratedMnemonic("mnemonic".to_string());
    assert_eq!(format!("{:?}", response), "GeneratedMnemonic(<omitted>)");

    let wallet_options = WalletOptions {
        storage_path: None,
        client_options: None,
        coin_type: None,
        secret_manager: Some(SecretManagerDto::Placeholder),
    };
    assert_eq!(
        format!("{:?}", wallet_options),
        "WalletOptions { storage_path: None, client_options: None, coin_type: None, secret_manager: Some(<omitted>) }"
    );
}
