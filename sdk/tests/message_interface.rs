// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "message_interface")]
use iota_sdk::client::{api::GetAddressesBuilderOptions, secret::SecretManagerDto};
#[cfg(feature = "message_interface")]
use iota_sdk::message_interface::{
    message_handler::Result, AccountMethod, ClientMessage, ManagerOptions, Response, WalletMessage,
};
#[cfg(feature = "message_interface")]
use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, ClientBuilder},
    wallet::account::types::AccountIdentifier,
};

#[tokio::test]
async fn new_message_interface_generate_addresses() -> Result<()> {
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
    let message = ClientMessage::GenerateAddresses {
        secret_manager: serde_json::from_str::<SecretManagerDto>(&secret_manager).unwrap(),
        options,
    };

    let response = client.send_message(message).await;
    match response {
        Response::GeneratedAddresses(addresses) => println!("{:?}", serde_json::to_string(&addresses).unwrap()),
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}

#[cfg(feature = "message_interface")]
#[tokio::test]
async fn new_message_interface_create_account() -> Result<()> {
    let storage_path = "test-storage/new_message_interface_create_account";
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
        #[cfg(feature = "storage")]
        storage_path: Some(storage_path.to_string()),
        client_options: Some(ClientBuilder::new().from_json(client_options).unwrap()),
        coin_type: Some(SHIMMER_COIN_TYPE),
        secret_manager: Some(serde_json::from_str(secret_manager).unwrap()),
    };

    let wallet = options.build_manager().await?;

    // create an account
    let response = wallet
        .send_message(WalletMessage::CreateAccount {
            alias: None,
            bech32_hrp: None,
        })
        .await;

    match response {
        Response::Account(account) => {
            assert_eq!(account.index, 0);
            let id = account.index;
            println!("Created account index: {id}")
        }
        _ => panic!("unexpected response {response:?}"),
    }

    let response = wallet
        .send_message(WalletMessage::CallAccountMethod {
            account_id: AccountIdentifier::Index(0),
            method: AccountMethod::UnspentOutputs { filter_options: None },
        })
        .await;

    match response {
        Response::OutputsData(_) => {}
        _ => panic!("unexpected response {response:?}"),
    }

    Ok(std::fs::remove_dir_all(storage_path).unwrap_or(()))
}
