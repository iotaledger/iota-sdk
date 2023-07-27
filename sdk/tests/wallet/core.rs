// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
#[cfg(feature = "storage")]
use iota_sdk::{
    client::constants::SHIMMER_COIN_TYPE,
    client::node_manager::node::{Node, NodeDto},
    wallet::Error,
    Url,
};
use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::address::Bech32Address,
    wallet::{ClientOptions, Result, Wallet},
};

#[cfg(feature = "storage")]
use crate::wallet::common::NODE_OTHER;
use crate::wallet::common::{make_wallet, setup, tear_down, DEFAULT_MNEMONIC, NODE_LOCAL};

#[cfg(feature = "storage")]
#[tokio::test]
async fn update_client_options() -> Result<()> {
    let storage_path = "test-storage/update_client_options";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, Some(NODE_OTHER)).await?;

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
    let wallet = make_wallet(storage_path, None, None).await?;
    let client_options = wallet.client_options().await;
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto_new));
    assert!(!client_options.node_manager_builder.nodes.contains(&node_dto_old));

    tear_down(storage_path)
}

#[cfg(feature = "storage")]
#[tokio::test]
async fn different_seed() -> Result<()> {
    let storage_path = "test-storage/different_seed";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let _account = wallet.create_account().with_alias("Alice").finish().await?;

    drop(_account);
    drop(wallet);

    // Recreate Wallet with a different mnemonic
    let wallet = make_wallet(storage_path, None, None).await?;

    // Generating a new account needs to return an error, because the seed from the secret_manager is different
    assert!(wallet.create_account().with_alias("Bob").finish().await.is_err());

    tear_down(storage_path)
}

#[cfg(feature = "storage")]
#[tokio::test]
async fn changed_coin_type() -> Result<()> {
    let storage_path = "test-storage/changed_coin_type";
    setup(storage_path)?;

    let mnemonic = Mnemonic::from(DEFAULT_MNEMONIC.to_owned());

    let wallet = make_wallet(storage_path, Some(mnemonic.clone()), None).await?;
    let _account = wallet.create_account().with_alias("Alice").finish().await?;

    drop(_account);
    drop(wallet);

    let err = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
            mnemonic.clone(),
        )?))
        .with_coin_type(IOTA_COIN_TYPE)
        .with_storage_path(storage_path)
        .finish()
        .await;

    // Building the wallet with another coin type needs to return an error, because a different coin type was used in
    // the existing account
    assert!(matches!(
        err,
        Err(Error::InvalidCoinType {
            new_coin_type: IOTA_COIN_TYPE,
            existing_coin_type: SHIMMER_COIN_TYPE
        })
    ));

    // Building the wallet with the same coin type still works
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
            mnemonic,
        )?))
        .with_storage_path(storage_path)
        .finish()
        .await?;
    // Also still possible to create a new account
    assert!(wallet.create_account().with_alias("Bob").finish().await.is_ok());

    tear_down(storage_path)
}

#[tokio::test]
async fn shimmer_coin_type() -> Result<()> {
    let storage_path = "test-storage/shimmer_coin_type";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, Some(Mnemonic::from(DEFAULT_MNEMONIC.to_owned())), None).await?;
    let account = wallet.create_account().finish().await?;

    // Creating a new account with providing a coin type will use the Shimmer coin type with shimmer testnet bech32 hrp
    assert_eq!(
        Bech32Address::try_new("smr", account.addresses().await?[0].address())?.to_string(),
        // Address generated with bip32 path: [44, 4219, 0, 0, 0]
        "smr1qq724zgvdujt3jdcd3xzsuqq7wl9pwq3dvsa5zvx49rj9tme8cat65xq7jz"
    );

    tear_down(storage_path)
}

#[tokio::test]
async fn iota_coin_type() -> Result<()> {
    let storage_path = "test-storage/iota_coin_type";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(DEFAULT_MNEMONIC.to_owned())?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(IOTA_COIN_TYPE);

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let account = wallet.create_account().finish().await?;

    // Creating a new account with providing a coin type will use the iota coin type with shimmer testnet bech32 hrp
    assert_eq!(
        Bech32Address::try_new("smr", account.addresses().await?[0].address())?.to_string(),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0]
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    tear_down(storage_path)
}

#[cfg(feature = "storage")]
#[tokio::test]
async fn update_node_auth() -> Result<()> {
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
