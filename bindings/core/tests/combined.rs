// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use crypto::keys::bip44::Bip44;
use iota_sdk::client::{constants::SHIMMER_COIN_TYPE, secret::SecretManagerDto, ClientBuilder};
use iota_sdk_bindings_core::{
    CallMethod, ClientMethod, Response, Result, WalletCommandMethod, WalletMethod, WalletOptions,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn create_wallet() -> Result<()> {
    let storage_path = "test-storage/create_wallet";
    std::fs::remove_dir_all(storage_path).ok();

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

    let wallet = WalletOptions::default()
        .with_storage_path(storage_path.to_string())
        .with_client_options(ClientBuilder::new().from_json(client_options).unwrap())
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_secret_manager(serde_json::from_str::<SecretManagerDto>(secret_manager).unwrap())
        .build()
        .await?;

    let response = wallet
        .call_method(WalletMethod::CallMethod {
            method: WalletCommandMethod::UnspentOutputs { filter_options: None },
        })
        .await;

    match response {
        Response::OutputsData(_) => {}
        _ => panic!("unexpected response {response:?}"),
    }

    std::fs::remove_dir_all(storage_path).ok();
    Ok(())
}

#[tokio::test]
async fn client_from_wallet() -> Result<()> {
    let storage_path = "test-storage/client_from_wallet";
    std::fs::remove_dir_all(storage_path).ok();

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

    let wallet = WalletOptions::default()
        .with_storage_path(storage_path.to_string())
        .with_client_options(ClientBuilder::new().from_json(client_options).unwrap())
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_secret_manager(serde_json::from_str::<SecretManagerDto>(secret_manager).unwrap())
        .build()
        .await?;

    // TODO reenable
    // // Send ClientMethod via the client from the wallet
    // let response = wallet
    //     .client()
    //     .call_method(ClientMethod::GetHealth)
    //     .await;

    // match response {
    //     Response::Bool(_) => {}
    //     _ => panic!("unexpected response {response:?}"),
    // }

    std::fs::remove_dir_all(storage_path).ok();
    Ok(())
}
