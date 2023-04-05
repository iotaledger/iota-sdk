// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::Result;
#[cfg(feature = "stronghold")]
use {
    iota_sdk::client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    iota_sdk::wallet::{ClientOptions, Wallet},
    std::path::PathBuf,
};

use crate::wallet::common::{make_wallet, setup, tear_down};

#[tokio::test]
async fn account_ordering() -> Result<()> {
    let storage_path = "test-storage/account_ordering";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    for _ in 0..100 {
        let _account = wallet.create_account().finish().await?;
    }
    std::fs::remove_dir_all("test-storage/account_ordering").unwrap_or(());
    #[cfg(debug_assertions)]
    wallet.verify_integrity().await?;
    tear_down(storage_path)
}

#[cfg(feature = "storage")]
#[tokio::test]
async fn remove_latest_account() -> Result<()> {
    let storage_path = "test-storage/remove_latest_account";
    setup(storage_path)?;

    let recreated_account_index = {
        let wallet = make_wallet(storage_path, None, None).await?;

        // Create two accounts.
        let first_account = wallet.create_account().finish().await?;
        let _second_account = wallet.create_account().finish().await?;
        assert!(wallet.get_accounts().await.unwrap().len() == 2);

        // Remove `second_account`.
        wallet
            .remove_latest_account()
            .await
            .expect("cannot remove latest account");

        // Check if the `second_account` was removed successfully.
        let accounts = wallet.get_accounts().await.unwrap();
        assert!(accounts.len() == 1);
        assert_eq!(
            *accounts.get(0).unwrap().read().await.index(),
            *first_account.read().await.index()
        );

        // Remove `first_account`.
        wallet
            .remove_latest_account()
            .await
            .expect("cannot remove latest account");

        // Check if the `first_account` was removed successfully. All accounts should be removed.
        let accounts = wallet.get_accounts().await.unwrap();
        assert!(accounts.is_empty());

        // Try remove another time (even if there is nothing to remove).
        wallet
            .remove_latest_account()
            .await
            .expect("cannot remove latest account");

        let accounts = wallet.get_accounts().await.unwrap();
        assert!(accounts.is_empty());

        // Recreate a new account and return their index.

        let recreated_account = wallet.create_account().finish().await?;
        assert_eq!(wallet.get_accounts().await.unwrap().len(), 1);
        let recreated_account_index = *recreated_account.read().await.index();

        recreated_account_index
    };

    // Restore dropped `Wallet` from above.
    let wallet = make_wallet(storage_path, None, None).await?;

    let accounts = wallet.get_accounts().await.unwrap();

    // Check if accounts with `recreated_account_index` exist.
    assert_eq!(accounts.len(), 1);
    assert_eq!(*accounts.get(0).unwrap().read().await.index(), recreated_account_index);

    #[cfg(debug_assertions)]
    wallet.verify_integrity().await?;

    tear_down(storage_path)
}

#[tokio::test]
async fn account_alias_already_exists() -> Result<()> {
    let storage_path = "test-storage/account_alias_already_exists";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let _account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;
    assert!(
        &wallet
            .create_account()
            .with_alias("Alice".to_string())
            .finish()
            .await
            .is_err()
    );
    assert!(
        &wallet
            .create_account()
            .with_alias("alice".to_string())
            .finish()
            .await
            .is_err()
    );
    assert!(
        &wallet
            .create_account()
            .with_alias("ALICE".to_string())
            .finish()
            .await
            .is_err()
    );
    // Other alias works
    assert!(
        &wallet
            .create_account()
            .with_alias("Bob".to_string())
            .finish()
            .await
            .is_ok()
    );

    tear_down(storage_path)
}

#[tokio::test]
async fn account_rename_alias() -> Result<()> {
    let storage_path = "test-storage/account_rename_alias";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    assert_eq!(account.alias().await, "Alice".to_string());

    // rename account
    account.set_alias("Bob").await?;

    assert_eq!(account.alias().await, "Bob".to_string());

    tear_down(storage_path)
}

#[tokio::test]
async fn account_first_address_exists() -> Result<()> {
    let storage_path = "test-storage/account_first_address_exists";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    // When the account is generated, the first public address also gets generated and added to it
    assert_eq!(account.addresses().await?.len(), 1);
    // First address is a public address
    assert_eq!(account.addresses().await?.first().unwrap().internal(), &false);

    tear_down(storage_path)
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn account_creation_stronghold() -> Result<()> {
    let storage_path = "test-storage/account_creation_stronghold";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node("http://localhost:14265")?;
    let mnemonic = "inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak";

    // Create directory before, because stronghold would panic otherwise
    std::fs::create_dir_all(storage_path).unwrap_or(());
    let mut stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password")
        .build(PathBuf::from(
            "test-storage/account_creation_stronghold/test.stronghold",
        ))?;
    stronghold_secret_manager.store_mnemonic(mnemonic.to_string()).await?;
    let secret_manager = SecretManager::Stronghold(stronghold_secret_manager);

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(secret_manager)
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE);
    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let _account = wallet.create_account().finish().await?;

    tear_down(storage_path)
}
