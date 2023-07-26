// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::SecretManagerDto, ClientBuilder},
    wallet::account::types::AccountIdentifier,
};
use iota_sdk_bindings_core::{AccountMethod, CallMethod, Response, Result, WalletMethod, WalletOptions};

#[tokio::test]
async fn create_account() -> Result<()> {
    let storage_path = "test-storage/create_account";
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
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_secret_manager(serde_json::from_str::<SecretManagerDto>(secret_manager).unwrap())
        .build()
        .await?;

    // create an account
    let response = wallet
        .call_method(WalletMethod::CreateAccount {
            alias: None,
            bech32_hrp: None,
            addresses: None,
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
        .call_method(WalletMethod::CallAccountMethod {
            account_id: AccountIdentifier::Index(0),
            method: AccountMethod::UnspentOutputs { filter_options: None },
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
async fn verify_accounts() -> Result<()> {
    let storage_path = "test-storage/verify_accounts";
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
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_secret_manager(serde_json::from_str::<SecretManagerDto>(secret_manager).unwrap())
        .build()
        .await?;

    let mut account_details = BTreeMap::new();
    let mut handle_response = |response| match response {
        Response::Account(account) => {
            account_details.insert(account.index, account);
        }
        _ => panic!("unexpected response {response:?}"),
    };

    // Create a few accounts
    for alias in ["Alice", "Bob", "Roger", "Denise", "Farquad", "Pikachu"] {
        handle_response(
            wallet
                .call_method(WalletMethod::CreateAccount {
                    alias: Some(alias.to_owned()),
                    bech32_hrp: None,
                    addresses: None,
                })
                .await,
        );
    }

    // Remove latest account
    match wallet.call_method(WalletMethod::RemoveLatestAccount).await {
        Response::Ok => {}
        response => panic!("unexpected response {response:?}"),
    }

    account_details.pop_last();

    // Get individual account details
    for account in account_details.values() {
        // By Index
        match wallet
            .call_method(WalletMethod::GetAccount {
                account_id: account.index.into(),
            })
            .await
        {
            Response::Account(details) => {
                assert_eq!(&account_details[&details.index], &details);
            }
            response => panic!("unexpected response {response:?}"),
        }

        // By Name
        match wallet
            .call_method(WalletMethod::GetAccount {
                account_id: account.alias.as_str().into(),
            })
            .await
        {
            Response::Account(details) => {
                assert_eq!(&account_details[&details.index], &details);
            }
            response => panic!("unexpected response {response:?}"),
        }
    }

    // Get account details
    match wallet.call_method(WalletMethod::GetAccounts).await {
        Response::Accounts(details) => {
            assert_eq!(account_details.len(), details.len());
            for detail in details {
                assert_eq!(&account_details[&detail.index], &detail);
            }
        }
        response => panic!("unexpected response {response:?}"),
    }

    // Get account indexes
    match wallet.call_method(WalletMethod::GetAccountIndexes).await {
        Response::AccountIndexes(indexes) => {
            assert_eq!(account_details.len(), indexes.len());
            for index in indexes {
                assert!(account_details.contains_key(&index));
            }
        }
        response => panic!("unexpected response {response:?}"),
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
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_secret_manager(serde_json::from_str::<SecretManagerDto>(secret_manager).unwrap())
        .build()
        .await?;

    // create an account
    let response = wallet
        .call_method(WalletMethod::CreateAccount {
            alias: None,
            bech32_hrp: None,
            addresses: None,
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

    // TODO reenable
    // // Send ClientMethod via the client from the wallet
    // let response = wallet.get_accounts().await?[0]
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
