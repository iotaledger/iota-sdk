// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
#[cfg(feature = "storage")]
use iota_sdk::{
    client::constants::SHIMMER_COIN_TYPE,
    client::node_manager::node::{Node, NodeDto},
    wallet::WalletError,
};
use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    crypto::keys::bip44::Bip44,
    types::block::{address::Bech32Address, protocol::iota_mainnet_protocol_parameters},
    wallet::{ClientOptions, Wallet},
};
use pretty_assertions::assert_eq;
#[cfg(feature = "storage")]
use url::Url;

#[cfg(feature = "storage")]
use crate::wallet::common::NODE_OTHER;
use crate::wallet::common::{make_wallet, setup, tear_down, DEFAULT_MNEMONIC, NODE_LOCAL};

#[cfg(feature = "storage")]
#[tokio::test]
async fn update_client_options() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/update_client_options";
    setup(storage_path)?;

    let mnemonic = Mnemonic::from(DEFAULT_MNEMONIC.to_owned());
    let wallet = make_wallet(storage_path, mnemonic.clone(), Some(NODE_OTHER)).await?;

    let node_dto_old = NodeDto::Node(Node::from(Url::parse(NODE_OTHER).unwrap()));
    let node_dto_new = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));

    let client_options = wallet.client_options().await;
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto_old));
    assert!(!client_options.node_manager_builder.nodes.contains(&node_dto_new));

    wallet
        .set_client_options(ClientOptions::new().with_node(NODE_LOCAL)?)
        .await?;

    let client_options = wallet.client_options().await;
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto_new));
    assert!(!client_options.node_manager_builder.nodes.contains(&node_dto_old));

    // The client options are also updated in the database and available the next time
    drop(wallet);
    let wallet = make_wallet(storage_path, mnemonic, None).await?;
    let client_options = wallet.client_options().await;
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto_new));
    assert!(!client_options.node_manager_builder.nodes.contains(&node_dto_old));

    tear_down(storage_path)
}

// #[cfg(feature = "storage")]
// #[tokio::test]
// async fn different_seed() -> Result<(), Box<dyn std::error::Error>> {
//     let storage_path = "test-storage/different_seed";
//     setup(storage_path)?;

//     let wallet = make_wallet(storage_path, None, None).await?;

//     drop(wallet);

//     // Recreate Wallet with a different mnemonic
//     // Generating a new wallet needs to return an error, because the seed from the secret_manager is different
//     assert!(make_wallet(storage_path, None, None).await.is_err());

//     tear_down(storage_path)
// }

#[cfg(feature = "storage")]
#[tokio::test]
async fn changed_bip_path() -> Result<(), Box<dyn std::error::Error>> {
    use iota_sdk::crypto::keys::bip44::Bip44;

    let storage_path = "test-storage/changed_coin_type";
    setup(storage_path)?;

    let mnemonic = Mnemonic::from(DEFAULT_MNEMONIC.to_owned());
    let wallet = make_wallet(storage_path, mnemonic.clone(), None).await?;

    drop(wallet);

    let result = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
            mnemonic.clone(),
        )?))
        .with_bip_path(Bip44::new(IOTA_COIN_TYPE))
        .with_storage_path(storage_path)
        .finish()
        .await;

    // Building the wallet with another coin type needs to return an error, because a different coin type was used in
    // the existing account
    assert!(matches!(result, Err(WalletError::BipPathMismatch {
        new_bip_path: Some(new_bip_path),
        old_bip_path: Some(old_bip_path),
    }) if new_bip_path == Bip44::new(IOTA_COIN_TYPE) && old_bip_path == Bip44::new(SHIMMER_COIN_TYPE)));

    // Building the wallet with the same coin type still works
    assert!(
        Wallet::builder()
            .with_secret_manager(SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
                mnemonic,
            )?))
            .with_storage_path(storage_path)
            .finish()
            .await
            .is_ok()
    );

    tear_down(storage_path)
}

#[tokio::test]
async fn shimmer_coin_type() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/shimmer_coin_type";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, Some(Mnemonic::from(DEFAULT_MNEMONIC.to_owned())), None).await?;

    // Creating a new account with providing a coin type will use the Shimmer coin type with shimmer testnet bech32 hrp
    assert_eq!(
        Bech32Address::try_new("smr", wallet.address().await)?.to_string(),
        // Address generated with bip32 path: [44, 4219, 0, 0, 0]
        "smr1qq724zgvdujt3jdcd3xzsuqq7wl9pwq3dvsa5zvx49rj9tme8cat65xq7jz"
    );

    tear_down(storage_path)
}

#[tokio::test]
async fn iota_coin_type() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/iota_coin_type";
    setup(storage_path)?;

    let client_options = ClientOptions::new()
        .with_node(NODE_LOCAL)?
        .with_protocol_parameters(iota_mainnet_protocol_parameters().clone());
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(DEFAULT_MNEMONIC.to_owned())?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(IOTA_COIN_TYPE));

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    // Creating a new account with providing a coin type will use the iota coin type with shimmer testnet bech32 hrp
    assert_eq!(
        Bech32Address::try_new("smr", wallet.address().await)?.to_string(),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0]
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    tear_down(storage_path)
}

#[cfg(feature = "storage")]
#[tokio::test]
async fn update_node_auth() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/update_node_auth";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, Some(NODE_OTHER)).await?;

    let node_auth = iota_sdk::client::node_manager::node::NodeAuth {
        jwt: Some("jwt".to_string()),
        basic_auth_name_pwd: None,
    };
    wallet
        .update_node_auth(Url::parse(NODE_OTHER).unwrap(), Some(node_auth.clone()))
        .await?;

    let client_options = wallet.client_options().await;

    let node = client_options.node_manager_builder.nodes.into_iter().next().unwrap();
    if let NodeDto::Node(node) = node {
        assert_eq!(node.auth.expect("missing provided auth"), node_auth);
    } else {
        panic!("Wrong node dto");
    };

    tear_down(storage_path)
}
