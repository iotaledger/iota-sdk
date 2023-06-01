// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "stronghold", feature = "storage"))]
use std::path::PathBuf;

#[cfg(all(feature = "stronghold", feature = "storage"))]
use iota_sdk::wallet::{ClientOptions, Result, Wallet};
#[cfg(all(feature = "stronghold", feature = "storage"))]
use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE},
        node_manager::node::{Node, NodeDto},
        secret::{mnemonic::MnemonicSecretManager, stronghold::StrongholdSecretManager, SecretManager},
    },
    Url,
};

use crate::wallet::common::{setup, tear_down, NODE_LOCAL, NODE_OTHER};

#[tokio::test]
#[cfg(all(feature = "stronghold", feature = "storage"))]
// Backup and restore with Stronghold
async fn backup_and_restore() -> Result<()> {
    let storage_path = "test-storage/backup_and_restore";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    let stronghold_password = "some_hopefully_secure_password".to_owned();

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore/1.stronghold")?;

    stronghold.store_mnemonic("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string()).await.unwrap();

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("test-storage/backup_and_restore/1")
        .finish()
        .await?;

    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    wallet
        .backup(
            PathBuf::from("test-storage/backup_and_restore/backup.stronghold"),
            stronghold_password.clone(),
        )
        .await?;

    // restore from backup

    let stronghold = StrongholdSecretManager::builder().build("test-storage/backup_and_restore/2.stronghold")?;

    let restore_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore/2")
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_coin_type(IOTA_COIN_TYPE)
        .finish()
        .await?;

    // Wrong password fails
    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore/backup.stronghold"),
            "wrong password".to_string(),
            None,
            None,
        )
        .await
        .unwrap_err();

    // Correct password works, even after trying with a wrong one before
    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore/backup.stronghold"),
            stronghold_password,
            None,
            None,
        )
        .await?;

    // Validate restored data

    // Restored coin type is used
    let new_account = restore_wallet.create_account().finish().await?;
    assert_eq!(new_account.details().await.coin_type(), &SHIMMER_COIN_TYPE);

    // compare restored client options
    let client_options = restore_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    // Get account
    let recovered_account = restore_wallet.get_account("Alice").await?;
    assert_eq!(account.addresses().await?, recovered_account.addresses().await?);

    // secret manager is the same
    assert_eq!(
        account.generate_ed25519_addresses(1, None).await?,
        recovered_account.generate_ed25519_addresses(1, None).await?
    );
    tear_down(storage_path)
}

#[tokio::test]
#[cfg(all(feature = "stronghold", feature = "storage"))]
// Backup and restore with Stronghold and MnemonicSecretManager
async fn backup_and_restore_mnemonic_secret_manager() -> Result<()> {
    let storage_path = "test-storage/backup_and_restore_mnemonic_secret_manager";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(
        "inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak",
    )?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("test-storage/backup_and_restore_mnemonic_secret_manager/1")
        .finish()
        .await?;

    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    let stronghold_password = "some_hopefully_secure_password";

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    wallet
        .backup(
            PathBuf::from("test-storage/backup_and_restore_mnemonic_secret_manager/backup.stronghold"),
            stronghold_password.to_string(),
        )
        .await?;

    // restore from backup

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(
        "inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak",
    )?;

    let restore_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore_mnemonic_secret_manager/2")
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_coin_type(IOTA_COIN_TYPE)
        .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
        .finish()
        .await?;

    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore_mnemonic_secret_manager/backup.stronghold"),
            stronghold_password.to_string(),
            None,
            None,
        )
        .await?;

    // Validate restored data

    // Restored coin type is used
    let new_account = restore_wallet.create_account().finish().await?;
    assert_eq!(new_account.details().await.coin_type(), &SHIMMER_COIN_TYPE);

    // compare restored client options
    let client_options = restore_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    // Get account
    let recovered_account = restore_wallet.get_account("Alice").await?;
    assert_eq!(account.addresses().await?, recovered_account.addresses().await?);

    // secret manager is the same
    assert_eq!(
        account.generate_ed25519_addresses(1, None).await?,
        recovered_account.generate_ed25519_addresses(1, None).await?
    );
    tear_down(storage_path)
}

#[tokio::test]
#[cfg(all(feature = "stronghold", feature = "storage"))]
// Backup and restore with Stronghold
async fn backup_and_restore_different_coin_type() -> Result<()> {
    let storage_path = "test-storage/backup_and_restore_different_coin_type";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    let stronghold_password = "some_hopefully_secure_password".to_owned();

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore_different_coin_type/1.stronghold")?;

    stronghold.store_mnemonic("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string()).await.unwrap();

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("test-storage/backup_and_restore_different_coin_type/1")
        .finish()
        .await?;

    // Create one account
    wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    wallet
        .backup(
            PathBuf::from("test-storage/backup_and_restore_different_coin_type/backup.stronghold"),
            stronghold_password.clone(),
        )
        .await?;

    // restore from backup

    let stronghold =
        StrongholdSecretManager::builder().build("test-storage/backup_and_restore_different_coin_type/2.stronghold")?;

    let restore_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore_different_coin_type/2")
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_coin_type(IOTA_COIN_TYPE)
        .finish()
        .await?;

    // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore_different_coin_type/backup.stronghold"),
            stronghold_password,
            Some(true),
            None,
        )
        .await?;

    // Validate restored data

    // No accounts restored, because the coin type was different
    assert!(restore_wallet.get_accounts().await?.is_empty());

    // Restored coin type is not used and it's still the same one
    let new_account = restore_wallet.create_account().finish().await?;
    assert_eq!(new_account.details().await.coin_type(), &IOTA_COIN_TYPE);
    // secret manager is the same
    assert_eq!(
        new_account.addresses().await?[0].address(),
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    // compare restored client options
    let client_options = restore_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_OTHER).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    tear_down(storage_path)
}

#[tokio::test]
#[cfg(all(feature = "stronghold", feature = "storage"))]
// Backup and restore with Stronghold
async fn backup_and_restore_same_coin_type() -> Result<()> {
    let storage_path = "test-storage/backup_and_restore_same_coin_type";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    let stronghold_password = "some_hopefully_secure_password".to_owned();

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore_same_coin_type/1.stronghold")?;

    stronghold.store_mnemonic("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string()).await.unwrap();

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("test-storage/backup_and_restore_same_coin_type/1")
        .finish()
        .await?;

    // Create one account
    let account_before_backup = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    wallet
        .backup(
            PathBuf::from("test-storage/backup_and_restore_same_coin_type/backup.stronghold"),
            stronghold_password.clone(),
        )
        .await?;

    // restore from backup

    let stronghold =
        StrongholdSecretManager::builder().build("test-storage/backup_and_restore_same_coin_type/2.stronghold")?;

    let restore_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore_same_coin_type/2")
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
        // Build with same coin type
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore_same_coin_type/backup.stronghold"),
            stronghold_password,
            Some(true),
            None,
        )
        .await?;

    // Validate restored data

    // The account is restored, because the coin type is the same
    let restored_accounts = restore_wallet.get_accounts().await?;
    assert_eq!(restored_accounts.len(), 1);

    // addresses are still there
    assert_eq!(
        restored_accounts[0].addresses().await?,
        account_before_backup.addresses().await?
    );

    // compare client options, they are not restored
    let client_options = restore_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_OTHER).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    tear_down(storage_path)
}

#[tokio::test]
#[cfg(all(feature = "stronghold", feature = "storage"))]
// Backup and restore with Stronghold
async fn backup_and_restore_different_coin_type_dont_ignore() -> Result<()> {
    let storage_path = "test-storage/backup_and_restore_different_coin_type_dont_ignore";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_OTHER)?;

    let stronghold_password = "some_hopefully_secure_password".to_owned();

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore_different_coin_type_dont_ignore/1.stronghold")?;

    stronghold.store_mnemonic("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string()).await.unwrap();

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("test-storage/backup_and_restore_different_coin_type_dont_ignore/1")
        .finish()
        .await?;

    // Create one account
    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    wallet
        .backup(
            PathBuf::from("test-storage/backup_and_restore_different_coin_type_dont_ignore/backup.stronghold"),
            stronghold_password.clone(),
        )
        .await?;

    // restore from backup

    let stronghold = StrongholdSecretManager::builder()
        .build("test-storage/backup_and_restore_different_coin_type_dont_ignore/2.stronghold")?;

    let restore_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore_different_coin_type_dont_ignore/2")
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(ClientOptions::new().with_node(NODE_LOCAL)?)
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_coin_type(IOTA_COIN_TYPE)
        .finish()
        .await?;

    // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore_different_coin_type_dont_ignore/backup.stronghold"),
            stronghold_password,
            Some(false),
            None,
        )
        .await?;

    // Validate restored data

    // No accounts restored, because the coin type was different
    let restored_account = restore_wallet.get_account("Alice").await?;
    assert_eq!(
        account.addresses().await?[0].address(),
        restored_account.addresses().await?[0].address(),
    );

    // Restored coin type is used
    let new_account = restore_wallet.create_account().finish().await?;
    assert_eq!(new_account.details().await.coin_type(), &SHIMMER_COIN_TYPE);
    // secret manager is restored
    assert_eq!(
        new_account.addresses().await?[0].address(),
        "smr1qzvjvjyqxgfx4f0m3xhn2rj24e03dwsmjz082735y3wx88v2gudu2afedhu"
    );

    // compare client options, they are not restored
    let client_options = restore_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    tear_down(storage_path)
}

#[tokio::test]
#[cfg(all(feature = "stronghold", feature = "storage"))]
async fn backup_and_restore_bech32_hrp_mismatch() -> Result<()> {
    let storage_path = "test-storage/backup_and_restore_bech32_hrp_mismatch";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    let stronghold_password = "some_hopefully_secure_password".to_owned();

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore_bech32_hrp_mismatch/1.stronghold")?;

    stronghold.store_mnemonic("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string()).await.unwrap();

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(client_options.clone())
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("test-storage/backup_and_restore_bech32_hrp_mismatch/1")
        .finish()
        .await?;

    let account = wallet.create_account().with_alias("Alice".to_string()).finish().await?;

    wallet
        .backup(
            PathBuf::from("test-storage/backup_and_restore_bech32_hrp_mismatch/backup.stronghold"),
            stronghold_password.clone(),
        )
        .await?;

    // restore from backup

    let stronghold =
        StrongholdSecretManager::builder().build("test-storage/backup_and_restore_bech32_hrp_mismatch/2.stronghold")?;

    let restore_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore_bech32_hrp_mismatch/2")
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_coin_type(IOTA_COIN_TYPE)
        .finish()
        .await?;

    restore_wallet
        .restore_backup(
            PathBuf::from("test-storage/backup_and_restore_bech32_hrp_mismatch/backup.stronghold"),
            stronghold_password,
            None,
            Some(iota_sdk::types::block::address::Hrp::from_str_unchecked("otherhrp")),
        )
        .await?;

    // Validate restored data

    // compare restored client options
    let client_options = restore_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    // No restored accounts because the bech32 hrp was different
    let restored_accounts = restore_wallet.get_accounts().await?;
    assert!(restored_accounts.is_empty());

    // Restored coin type is used
    let new_account = restore_wallet.create_account().finish().await?;
    assert_eq!(new_account.details().await.coin_type(), &SHIMMER_COIN_TYPE);

    // secret manager is the same
    assert_eq!(
        account.generate_ed25519_addresses(1, None).await?,
        new_account.generate_ed25519_addresses(1, None).await?
    );
    tear_down(storage_path)
}
