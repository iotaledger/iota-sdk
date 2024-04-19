// Copyright 2022-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{
        api::GetAddressesOptions,
        constants::{IOTA_COIN_TYPE, SHIMMER_COIN_TYPE},
        node_manager::node::{Node, NodeDto},
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    crypto::keys::bip44::Bip44,
    wallet::{ClientOptions, Wallet},
};
use pretty_assertions::assert_eq;
use url::Url;

use crate::wallet::common::{setup, tear_down, NODE_LOCAL};

// Backup and restore with Stronghold
#[ignore]
#[tokio::test]
async fn backup_and_restore() -> Result<(), Box<dyn std::error::Error>> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let storage_path = "test-storage/backup_and_restore";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    let stronghold_password = "some_hopefully_secure_password".to_owned();

    // Create directory if not existing, because stronghold panics otherwise
    std::fs::create_dir_all(storage_path).ok();
    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore/1.stronghold")?;

    stronghold.store_mnemonic(Mnemonic::from("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string())).await.unwrap();

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(client_options.clone())
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .with_storage_path("test-storage/backup_and_restore/1")
        .finish()
        .await?;

    wallet
        .backup_to_stronghold_snapshot(
            PathBuf::from("test-storage/backup_and_restore/backup.stronghold"),
            stronghold_password.clone(),
        )
        .await?;

    // restore from backup

    let stronghold = StrongholdSecretManager::builder()
        .password(stronghold_password.clone())
        .build("test-storage/backup_and_restore/2.stronghold")?;

    stronghold.store_mnemonic(Mnemonic::from("surprise own liquid gold embrace indoor cereal magnet wink purse similar unusual setup woman catch chuckle critic wet weasel ahead wasp cruise luggage pig".to_string())).await.unwrap();

    let restored_wallet = Wallet::builder()
        .with_storage_path("test-storage/backup_and_restore/2")
        .with_secret_manager(SecretManager::Stronghold(stronghold))
        .with_client_options(ClientOptions::new().with_ignore_node_health().with_node(NODE_LOCAL)?)
        // Build with a different coin type, to check if it gets replaced by the one from the backup
        .with_bip_path(Bip44::new(IOTA_COIN_TYPE))
        .finish()
        .await?;

    // Wrong password fails
    restored_wallet
        .restore_from_stronghold_snapshot(
            PathBuf::from("test-storage/backup_and_restore/backup.stronghold"),
            "wrong password".to_owned(),
            None,
            None,
        )
        .await
        .unwrap_err();

    // Correct password works, even after trying with a wrong one before
    restored_wallet
        .restore_from_stronghold_snapshot(
            PathBuf::from("test-storage/backup_and_restore/backup.stronghold"),
            stronghold_password,
            None,
            None,
        )
        .await?;

    // Validate restored data

    // Restored coin type is used
    assert_eq!(restored_wallet.bip_path().await.unwrap().coin_type, SHIMMER_COIN_TYPE);

    // compare restored client options
    let client_options = restored_wallet.client_options().await;
    let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
    assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

    assert_eq!(wallet.address().await.clone(), restored_wallet.address().await.clone());

    // secret manager is the same
    assert_eq!(
        wallet
            .secret_manager()
            .read()
            .await
            .generate_ed25519_addresses(GetAddressesOptions {
                coin_type: SHIMMER_COIN_TYPE,
                range: 0..1,
                ..Default::default()
            })
            .await?,
        restored_wallet
            .secret_manager()
            .read()
            .await
            .generate_ed25519_addresses(GetAddressesOptions {
                coin_type: SHIMMER_COIN_TYPE,
                range: 0..1,
                ..Default::default()
            })
            .await?,
    );
    tear_down(storage_path)
}

// // Backup and restore with Stronghold and MnemonicSecretManager
// #[tokio::test]
// async fn backup_and_restore_mnemonic_secret_manager() -> Result<(), WalletError> {
//     iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

//     let storage_path = "test-storage/backup_and_restore_mnemonic_secret_manager";
//     setup(storage_path)?;

//     let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

//     let secret_manager = MnemonicSecretManager::try_from_mnemonic(
//         "inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain
// glad warm early rain clutch slab august bleak".to_owned(),     )?;

//     let wallet = Wallet::builder()
//         .with_secret_manager(SecretManager::Mnemonic(secret_manager))
//         .with_client_options(client_options.clone())
//         .with_coin_type(SHIMMER_COIN_TYPE)
//         .with_storage_path("test-storage/backup_and_restore_mnemonic_secret_manager/1")
//         .finish()
//         .await?;

//     let stronghold_password = "some_hopefully_secure_password".to_owned();

//     // Create directory if not existing, because stronghold panics otherwise
//     std::fs::create_dir_all(storage_path).ok();
//     wallet
//         .backup(
//             PathBuf::from("test-storage/backup_and_restore_mnemonic_secret_manager/backup.stronghold"),
//             stronghold_password.clone(),
//         )
//         .await?;

//     // restore from backup

//     let secret_manager = MnemonicSecretManager::try_from_mnemonic(
//         "inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain
// glad warm early rain clutch slab august bleak".to_owned(),     )?;

//     let restore_wallet = Wallet::builder()
//         .with_storage_path("test-storage/backup_and_restore_mnemonic_secret_manager/2")
//         .with_secret_manager(SecretManager::Mnemonic(secret_manager))
//         // Build with a different coin type, to check if it gets replaced by the one from the backup
//         .with_coin_type(IOTA_COIN_TYPE)
//         .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
//         .finish()
//         .await?;

//     restore_wallet
//         .restore_backup(
//             PathBuf::from("test-storage/backup_and_restore_mnemonic_secret_manager/backup.stronghold"),
//             stronghold_password,
//             None,
//             None,
//         )
//         .await?;

//     // Validate restored data

//     // Restored coin type is used
//     let new_wallet = restore_wallet;
//     assert_eq!(new_wallet.data().await.coin_type(), &SHIMMER_COIN_TYPE);

//     // compare restored client options
//     let client_options = restore_wallet.client_options().await;
//     let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
//     assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

//     // Get wallet
//     let recovered_wallet = restore_wallet;
//     assert_eq!(wallet.address().clone(), recovered_wallet.address().clone());

//     // secret manager is the same
//     assert_eq!(
//         wallet.generate_ed25519_addresses(1, None).await?,
//         recovered_wallet.generate_ed25519_addresses(1, None).await?
//     );
//     tear_down(storage_path)
// }

// // Backup and restore with Stronghold
// #[tokio::test]
// async fn backup_and_restore_different_coin_type() -> Result<(), WalletError> {
//     iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

//     let storage_path = "test-storage/backup_and_restore_different_coin_type";
//     setup(storage_path)?;

//     let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

//     let stronghold_password = "some_hopefully_secure_password".to_owned();

//     // Create directory if not existing, because stronghold panics otherwise
//     std::fs::create_dir_all(storage_path).ok();
//     let stronghold = StrongholdSecretManager::builder()
//         .password(stronghold_password.clone())
//         .build("test-storage/backup_and_restore_different_coin_type/1.stronghold")?;

//     stronghold.store_mnemonic(Mnemonic::from("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string())).await.unwrap();

//     let wallet = Wallet::builder()
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(client_options.clone())
//         .with_coin_type(SHIMMER_COIN_TYPE)
//         .with_storage_path("test-storage/backup_and_restore_different_coin_type/1")
//         .finish()
//         .await?;

//     wallet
//         .backup(
//             PathBuf::from("test-storage/backup_and_restore_different_coin_type/backup.stronghold"),
//             stronghold_password.clone(),
//         )
//         .await?;

//     // restore from backup

//     let stronghold =
//         StrongholdSecretManager::builder().build("test-storage/backup_and_restore_different_coin_type/2.stronghold")?
// ;

//     let restore_wallet = Wallet::builder()
//         .with_storage_path("test-storage/backup_and_restore_different_coin_type/2")
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
//         // Build with a different coin type, to check if it gets replaced by the one from the backup
//         .with_coin_type(IOTA_COIN_TYPE)
//         .finish()
//         .await?;

//     // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
//     restore_wallet
//         .restore_backup(
//             PathBuf::from("test-storage/backup_and_restore_different_coin_type/backup.stronghold"),
//             stronghold_password,
//             Some(true),
//             None,
//         )
//         .await?;

//     // Validate restored data

//     // No wallet restored, because the coin type was different
//     assert!(restore_wallet.get_wallet_ledger().await?.is_empty());

//     // Restored coin type is not used and it's still the same one
//     let new_wallet = restore_wallet;
//     assert_eq!(new_wallet.data().await.coin_type(), &IOTA_COIN_TYPE);
//     // secret manager is the same
//     assert_eq!(
//         new_wallet.address().clone(),
//         "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
//     );

//     // compare restored client options
//     let client_options = restore_wallet.client_options().await;
//     let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_OTHER).unwrap()));
//     assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

//     tear_down(storage_path)
// }

// // Backup and restore with Stronghold
// #[tokio::test]
// async fn backup_and_restore_same_coin_type() -> Result<(), WalletError> {
//     iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

//     let storage_path = "test-storage/backup_and_restore_same_coin_type";
//     setup(storage_path)?;

//     let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

//     let stronghold_password = "some_hopefully_secure_password".to_owned();

//     // Create directory if not existing, because stronghold panics otherwise
//     std::fs::create_dir_all(storage_path).ok();
//     let stronghold = StrongholdSecretManager::builder()
//         .password(stronghold_password.clone())
//         .build("test-storage/backup_and_restore_same_coin_type/1.stronghold")?;

//     stronghold.store_mnemonic(Mnemonic::from("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string())).await.unwrap();

//     let wallet = Wallet::builder()
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(client_options.clone())
//         .with_coin_type(SHIMMER_COIN_TYPE)
//         .with_storage_path("test-storage/backup_and_restore_same_coin_type/1")
//         .finish()
//         .await?;

//     let wallet_before_backup = wallet;

//     wallet
//         .backup(
//             PathBuf::from("test-storage/backup_and_restore_same_coin_type/backup.stronghold"),
//             stronghold_password.clone(),
//         )
//         .await?;

//     // restore from backup

//     let stronghold =
//         StrongholdSecretManager::builder().build("test-storage/backup_and_restore_same_coin_type/2.stronghold")?;

//     let restore_wallet = Wallet::builder()
//         .with_storage_path("test-storage/backup_and_restore_same_coin_type/2")
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
//         // Build with same coin type
//         .with_coin_type(SHIMMER_COIN_TYPE)
//         .finish()
//         .await?;

//     // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
//     restore_wallet
//         .restore_backup(
//             PathBuf::from("test-storage/backup_and_restore_same_coin_type/backup.stronghold"),
//             stronghold_password,
//             Some(true),
//             None,
//         )
//         .await?;

//     // Validate restored data

//     // The wallet is restored, because the coin type is the same
//     let restored_wallet = restore_wallet.get_wallet_ledger().await?;
//     assert!(restored_wallet.is_some());

//     // addresses are still there
//     assert_eq!(
//         restored_wallet.address().clone(),
//         wallet_before_backup.address().clone()
//     );

//     // compare client options, they are not restored
//     let client_options = restore_wallet.client_options().await;
//     let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_OTHER).unwrap()));
//     assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

//     tear_down(storage_path)
// }

// // Backup and restore with Stronghold
// #[tokio::test]
// async fn backup_and_restore_different_coin_type_dont_ignore() -> Result<(), WalletError> {
//     iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

//     let storage_path = "test-storage/backup_and_restore_different_coin_type_dont_ignore";
//     setup(storage_path)?;

//     let client_options = ClientOptions::new().with_node(NODE_OTHER)?;

//     let stronghold_password = "some_hopefully_secure_password".to_owned();

//     // Create directory if not existing, because stronghold panics otherwise
//     std::fs::create_dir_all(storage_path).ok();
//     let stronghold = StrongholdSecretManager::builder()
//         .password(stronghold_password.clone())
//         .build("test-storage/backup_and_restore_different_coin_type_dont_ignore/1.stronghold")?;

//     stronghold.store_mnemonic(Mnemonic::from("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string())).await.unwrap();

//     let wallet = Wallet::builder()
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(client_options.clone())
//         .with_coin_type(SHIMMER_COIN_TYPE)
//         .with_storage_path("test-storage/backup_and_restore_different_coin_type_dont_ignore/1")
//         .finish()
//         .await?;

//     wallet
//         .backup(
//             PathBuf::from("test-storage/backup_and_restore_different_coin_type_dont_ignore/backup.stronghold"),
//             stronghold_password.clone(),
//         )
//         .await?;

//     // restore from backup

//     let stronghold = StrongholdSecretManager::builder()
//         .build("test-storage/backup_and_restore_different_coin_type_dont_ignore/2.stronghold")?;

//     let restore_wallet = Wallet::builder()
//         .with_storage_path("test-storage/backup_and_restore_different_coin_type_dont_ignore/2")
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(ClientOptions::new().with_node(NODE_LOCAL)?)
//         // Build with a different coin type, to check if it gets replaced by the one from the backup
//         .with_coin_type(IOTA_COIN_TYPE)
//         .finish()
//         .await?;

//     // restore with ignore_if_coin_type_mismatch: Some(true) to not overwrite the coin type
//     restore_wallet
//         .restore_backup(
//             PathBuf::from("test-storage/backup_and_restore_different_coin_type_dont_ignore/backup.stronghold"),
//             stronghold_password,
//             Some(false),
//             None,
//         )
//         .await?;

//     // Validate restored data

//     // No wallet restored, because the coin type was different
//     let restored_wallet = restore_wallet.get_wallet_ledger().await?;
//     assert_eq!(
//         wallet.address().clone(),
//         restored_wallet.address().clone(),
//     );

//     // TODO: Restored coin type is used
//     let new_wallet = restore_wallet;
//     assert_eq!(new_wallet.data().await.coin_type(), &SHIMMER_COIN_TYPE);
//     // secret manager is restored
//     assert_eq!(
//         new_wallet.address().clone(),
//         "smr1qzvjvjyqxgfx4f0m3xhn2rj24e03dwsmjz082735y3wx88v2gudu2afedhu"
//     );

//     // compare client options, they are not restored
//     let client_options = restore_wallet.client_options().await;
//     let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
//     assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

//     tear_down(storage_path)
// }

// #[tokio::test]
// async fn backup_and_restore_bech32_hrp_mismatch() -> Result<(), WalletError> {
//     iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

//     let storage_path = "test-storage/backup_and_restore_bech32_hrp_mismatch";
//     setup(storage_path)?;

//     let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

//     let stronghold_password = "some_hopefully_secure_password".to_owned();

//     // Create directory if not existing, because stronghold panics otherwise
//     std::fs::create_dir_all(storage_path).ok();
//     let stronghold = StrongholdSecretManager::builder()
//         .password(stronghold_password.clone())
//         .build("test-storage/backup_and_restore_bech32_hrp_mismatch/1.stronghold")?;

//     stronghold.store_mnemonic(Mnemonic::from("inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak".to_string())).await.unwrap();

//     let wallet = Wallet::builder()
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(client_options.clone())
//         .with_coin_type(SHIMMER_COIN_TYPE)
//         .with_storage_path("test-storage/backup_and_restore_bech32_hrp_mismatch/1")
//         .finish()
//         .await?;

//     wallet
//         .backup(
//             PathBuf::from("test-storage/backup_and_restore_bech32_hrp_mismatch/backup.stronghold"),
//             stronghold_password.clone(),
//         )
//         .await?;

//     // restore from backup

//     let stronghold =
//         StrongholdSecretManager::builder().build("test-storage/backup_and_restore_bech32_hrp_mismatch/2.stronghold")?
// ;

//     let restore_wallet = Wallet::builder()
//         .with_storage_path("test-storage/backup_and_restore_bech32_hrp_mismatch/2")
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(ClientOptions::new().with_node(NODE_OTHER)?)
//         // Build with a different coin type, to check if it gets replaced by the one from the backup
//         .with_coin_type(IOTA_COIN_TYPE)
//         .finish()
//         .await?;

//     restore_wallet
//         .restore_backup(
//             PathBuf::from("test-storage/backup_and_restore_bech32_hrp_mismatch/backup.stronghold"),
//             stronghold_password,
//             None,
//             Some(iota_sdk::types::block::address::Hrp::from_str_unchecked("otherhrp")),
//         )
//         .await?;

//     // Validate restored data

//     // compare restored client options
//     let client_options = restore_wallet.client_options().await;
//     let node_dto = NodeDto::Node(Node::from(Url::parse(NODE_LOCAL).unwrap()));
//     assert!(client_options.node_manager_builder.nodes.contains(&node_dto));

//     // No restored wallet because the bech32 hrp was different
//     let restored_wallet = restore_wallet.get_wallet_ledger().await?;
//     assert!(restored_wallet.is_empty());

//     // Restored coin type is used
//     let new_wallet = restore_wallet;
//     assert_eq!(new_wallet.details().await.coin_type(), &SHIMMER_COIN_TYPE);

//     // secret manager is the same
//     assert_eq!(
//         wallet.generate_ed25519_addresses(1, None).await?,
//         new_wallet.generate_ed25519_addresses(1, None).await?
//     );
//     tear_down(storage_path)
// }

// // Restore a Stronghold snapshot without secret manager data
// #[tokio::test]
// async fn restore_no_secret_manager_data() -> Result<(), WalletError> {
//     iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

//     let storage_path = "test-storage/restore_no_secret_manager_data";
//     setup(storage_path)?;

//     let stronghold = StrongholdSecretManager::builder().build(storage_path.to_string() + "/wallet.stronghold")?;

//     let restore_wallet = Wallet::builder()
//         .with_storage_path(storage_path)
//         .with_secret_manager(SecretManager::Stronghold(stronghold))
//         .with_client_options(ClientOptions::new().with_node(NODE_LOCAL)?)
//         .with_coin_type(IOTA_COIN_TYPE)
//         .finish()
//         .await?;

//     let stronghold_password = "some_hopefully_secure_password".to_owned();

//     restore_wallet
//         .restore_backup(
//             PathBuf::from("./tests/wallet/fixtures/no_secret_manager_data.stronghold"),
//             stronghold_password.clone(),
//             None,
//             None,
//         )
//         .await?;

//     restore_wallet.set_stronghold_password(stronghold_password).await?;

//     // Backup is restored also without any secret manager data inside and the seed is available
//     // Backup was created with mnemonic: "inhale gorilla deny three celery song category owner lottery rent author
//     // wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak"
//     assert_eq!(
//         restore_wallet.generate_ed25519_address(0, 0, None).await?.to_string(),
//         "0xc2ece328eb3d9bbc51d471dd17fd3665aa8c6bae63c78d64c13977efbb8b011e"
//     );
//     tear_down(storage_path)
// }
