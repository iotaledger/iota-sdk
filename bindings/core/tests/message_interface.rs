// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{api::GetAddressesBuilderOptions, constants::SHIMMER_COIN_TYPE, secret::SecretManagerDto, ClientBuilder},
    wallet::account::types::AccountIdentifier,
};
use iota_sdk_bindings_core::{
    call_client_method, call_wallet_method, message_handler::Result, AccountMethod, ClientMethod, ManagerOptions,
    Response, WalletMethod,
};

#[tokio::test]
async fn generate_addresses() -> Result<()> {
    let client_config = r#"{
            "nodes":[],
            "localPow":true,
            "fallbackToLocalPow": true
    }"#
    .to_string();

    let client = ClientBuilder::new().from_json(&client_config)?.finish()?;

    let secret_manager = format!(
        "{{\"mnemonic\":\"{}\"}}",
        "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river"
    );
    let options = GetAddressesBuilderOptions {
        coin_type: None,
        account_index: None,
        range: Some(std::ops::Range { start: 0, end: 10 }),
        internal: None,
        bech32_hrp: Some("atoi".to_string()),
        options: None,
    };
    let message = ClientMethod::GenerateAddresses {
        secret_manager: serde_json::from_str::<SecretManagerDto>(&secret_manager).unwrap(),
        options,
    };

    let response = call_client_method(&client, message).await;
    match response {
        Response::GeneratedAddresses(addresses) => println!("{:?}", serde_json::to_string(&addresses).unwrap()),
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}

#[tokio::test]
async fn create_account() -> Result<()> {
    let storage_path = "test-storage/create_account";
    std::fs::remove_dir_all(storage_path).unwrap_or(());

    let secret_manager = r#"{"Mnemonic":"about solution utility exist rail budget vacuum major survey clerk pave ankle wealth gym gossip still medal expect strong rely amazing inspire lazy lunar"}"#;
    let client_options = r#"{
            "nodes":[
               {
                  "url":"http://localhost:14265",
                  "auth":null,
                  "disabled":false
               }
            ]
         }"#;

    let options = ManagerOptions {
        storage_path: Some(storage_path.to_string()),
        client_options: Some(ClientBuilder::new().from_json(client_options).unwrap()),
        coin_type: Some(SHIMMER_COIN_TYPE),
        secret_manager: Some(serde_json::from_str(secret_manager).unwrap()),
    };

    let wallet = options.build_manager().await?;

    // create an account
    let response = call_wallet_method(
        &wallet,
        WalletMethod::CreateAccount {
            alias: None,
            bech32_hrp: None,
        },
    )
    .await;

    match response {
        Response::Account(account) => {
            assert_eq!(account.index, 0);
            let id = account.index;
            println!("Created account index: {id}")
        }
        _ => panic!("unexpected response {response:?}"),
    }

    let response = call_wallet_method(
        &wallet,
        WalletMethod::CallAccountMethod {
            account_id: AccountIdentifier::Index(0),
            method: AccountMethod::UnspentOutputs { filter_options: None },
        },
    )
    .await;

    match response {
        Response::OutputsData(_) => {}
        _ => panic!("unexpected response {response:?}"),
    }

    Ok(std::fs::remove_dir_all(storage_path).unwrap_or(()))
}

#[tokio::test]
async fn client_from_wallet() -> Result<()> {
    let storage_path = "test-storage/client_from_wallet";
    std::fs::remove_dir_all(storage_path).unwrap_or(());

    let secret_manager = r#"{"Mnemonic":"about solution utility exist rail budget vacuum major survey clerk pave ankle wealth gym gossip still medal expect strong rely amazing inspire lazy lunar"}"#;
    let client_options = r#"{
            "nodes":[
               {
                  "url":"http://localhost:14265",
                  "auth":null,
                  "disabled":false
               }
            ]
         }"#;

    let options = ManagerOptions {
        storage_path: Some(storage_path.to_string()),
        client_options: Some(ClientBuilder::new().from_json(client_options).unwrap()),
        coin_type: Some(SHIMMER_COIN_TYPE),
        secret_manager: Some(serde_json::from_str(secret_manager).unwrap()),
    };

    let wallet = options.build_manager().await?;

    // create an account
    let response = call_wallet_method(
        &wallet,
        WalletMethod::CreateAccount {
            alias: None,
            bech32_hrp: None,
        },
    )
    .await;

    match response {
        Response::Account(account) => {
            assert_eq!(account.index, 0);
            let id = account.index;
            println!("Created account index: {id}")
        }
        _ => panic!("unexpected response {response:?}"),
    }

    // Send ClientMethod via the client from the wallet
    let response = call_client_method(wallet.get_accounts().await?[0].client(), ClientMethod::GenerateMnemonic).await;

    match response {
        Response::GeneratedMnemonic(_) => {}
        _ => panic!("unexpected response {response:?}"),
    }

    Ok(std::fs::remove_dir_all(storage_path).unwrap_or(()))
}

#[test]
fn message_interface_secrets_debug() {
    let client_message = ClientMethod::MnemonicToHexSeed {
        mnemonic: "mnemonic".to_string(),
    };
    assert_eq!(
        format!("{:?}", client_message),
        "MnemonicToHexSeed { mnemonic: <omitted> }"
    );

    let client_message = ClientMethod::BuildAndPostBlock {
        secret_manager: None,
        options: None,
    };
    assert_eq!(
        format!("{:?}", client_message),
        "BuildAndPostBlock { secret_manager: None, options: None }"
    );

    #[cfg(feature = "ledger_nano")]
    {
        let client_message = ClientMethod::BuildAndPostBlock {
            secret_manager: Some(SecretManagerDto::LedgerNano(false)),
            options: None,
        };
        assert_eq!(
            format!("{:?}", client_message),
            "BuildAndPostBlock { secret_manager: Some(<omitted>), options: None }"
        );
    }

    let wallet_message = WalletMethod::VerifyMnemonic {
        mnemonic: "mnemonic".to_string(),
    };
    assert_eq!(
        format!("{:?}", wallet_message),
        "VerifyMnemonic { mnemonic: <omitted> }"
    );

    let response = Response::GeneratedMnemonic("mnemonic".to_string());
    assert_eq!(format!("{:?}", response), "GeneratedMnemonic(<omitted>)");
}
