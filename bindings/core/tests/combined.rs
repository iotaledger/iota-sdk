// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE},
        secret::{mnemonic::MnemonicSecretManager, SecretManagerDto},
        ClientBuilder,
    },
    types::{
        block::{
            payload::{dto::PayloadDto, Payload, TaggedDataPayload},
            Block, BlockBodyDto,
        },
        TryFromDto,
    },
};
use iota_sdk_bindings_core::{
    call_client_method, call_secret_manager_method, CallMethod, ClientMethod, Response, Result, SecretManagerMethod,
    WalletMethod, WalletOptions,
};
use pretty_assertions::assert_eq;

#[cfg(feature = "storage")]
#[tokio::test]
async fn create_wallet() -> Result<()> {
    let storage_path = "test-storage/create_wallet";
    std::fs::remove_dir_all(storage_path).ok();

    let secret_manager = r#"{"Mnemonic":"about solution utility exist rail budget vacuum major survey clerk pave ankle wealth gym gossip still medal expect strong rely amazing inspire lazy lunar"}"#;
    let client_options = r#"{
            "nodes":[
               {
                  "url":"http://localhost:8050",
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
        .call_method(WalletMethod::UnspentOutputs { filter_options: None })
        .await;

    match response {
        Response::OutputsData(_) => {}
        _ => panic!("unexpected response {response:?}"),
    }

    std::fs::remove_dir_all(storage_path).ok();
    Ok(())
}

#[cfg(feature = "storage")]
#[tokio::test]
async fn client_from_wallet() -> Result<()> {
    let storage_path = "test-storage/client_from_wallet";
    std::fs::remove_dir_all(storage_path).ok();

    let secret_manager = r#"{"Mnemonic":"about solution utility exist rail budget vacuum major survey clerk pave ankle wealth gym gossip still medal expect strong rely amazing inspire lazy lunar"}"#;
    let client_options = r#"{
            "nodes":[
               {
                  "url":"http://localhost:8050",
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

// TODO reenable
// #[cfg(feature = "storage")]
// #[tokio::test]
// async fn build_and_sign_block() -> Result<()> {
//     let storage_path = "test-storage/build_and_sign_block";
//     std::fs::remove_dir_all(storage_path).ok();

//     let secret_manager = MnemonicSecretManager::try_from_mnemonic(
//         "about solution utility exist rail budget vacuum major survey clerk pave ankle wealth gym gossip still medal
// expect strong rely amazing inspire lazy lunar",     ).unwrap();
//     let client = ClientBuilder::default()
//         .with_nodes(&["http://localhost:8050"])
//         .unwrap()
//         .finish()
//         .await
//         .unwrap();

//     let payload = PayloadDto::from(&Payload::from(
//         TaggedDataPayload::new("Hello".as_bytes(), "Tangle".as_bytes()).unwrap(),
//     ));

//     // Get an unsigned block
//     let response = call_client_method(
//         &client,
//         ClientMethod::BuildBasicBlock {
//             issuer_id: AccountId::null(),
//             payload: Some(payload.clone()),
//         },
//     )
//     .await;

//     let unsigned_block = match response {
//         Response::UnsignedBlock(unsigned_block) => {
//             match &unsigned_block.block {
//                 BlockBodyDto::Basic(b) => assert_eq!(b.payload.as_ref(), Some(&payload)),
//                 BlockBodyDto::Validation(v) => panic!("unexpected block body {v:?}"),
//             }
//             unsigned_block
//         }
//         _ => panic!("unexpected response {response:?}"),
//     };

//     // Sign the block using the secret manager
//     let response = call_secret_manager_method(
//         &secret_manager,
//         SecretManagerMethod::SignBlock {
//             unsigned_block,
//             chain: Bip44::new(IOTA_COIN_TYPE),
//         },
//     )
//     .await;

//     let block = match response {
//         Response::Block(block) => {
//             match &block.body {
//                 BlockBodyDto::Basic(b) => assert_eq!(b.payload.as_ref(), Some(&payload)),
//                 BlockBodyDto::Validation(v) => panic!("unexpected block {v:?}"),
//             }
//             block_body
//         }
//         _ => panic!("unexpected response {response:?}"),
//     };

//     // Get the block ID
//     let response = call_client_method(
//         &client,
//         ClientMethod::BlockId {
//             block: block.clone(),
//         },
//     )
//     .await;

//     match response {
//         Response::BlockId(block_id) => {
//             assert_eq!(
//                 block_id,
//                 Block::try_from_dto(block)
//                     .unwrap()
//                     .id(&client.get_protocol_parameters().await.unwrap())
//             );
//         }
//         _ => panic!("unexpected response {response:?}"),
//     };

//     Ok(())
// }
