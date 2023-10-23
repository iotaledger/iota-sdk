// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs, io, path::Path};

use iota_sdk::{
    client::{constants::IOTA_COIN_TYPE, secret::SecretManager, Password},
    types::block::address::{Hrp, ToBech32Ext},
    wallet::{
        migration::migrate_db_chrysalis_to_stardust,
        storage::{StorageKind, StorageOptions},
        ClientOptions, Result,
    },
    Wallet,
};
use pretty_assertions::assert_eq;
use zeroize::Zeroizing;

use crate::wallet::common::{setup, tear_down};

const CHRYSALIS_ACCOUNT_INDEXATION_KEY: &str = "iota-wallet-account-indexation";

#[tokio::test]
async fn migrate_chrysalis_db() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
    let storage_path = "migrate_chrysalis_db/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db/db", storage_path).unwrap();
    std::fs::copy(
        "./tests/wallet/fixtures/chrysalis-db/wallet.stronghold",
        "migrate_chrysalis_db/wallet.stronghold",
    )
    .unwrap();

    migrate_db_chrysalis_to_stardust("migrate_chrysalis_db", None, None).await?;

    let client_options = ClientOptions::new();
    let wallet = Wallet::builder()
        .with_storage_path("migrate_chrysalis_db")
        .with_client_options(client_options)
        .finish()
        .await?;

    let accounts = wallet.get_accounts().await?;
    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].alias().await, "Alice");
    assert_eq!(accounts[1].alias().await, "Bob");

    let alice_acc_details = accounts[0].details().await;
    assert_eq!(alice_acc_details.public_addresses().len(), 2);
    assert_eq!(
        alice_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0]
            .address()
            .try_to_bech32("rms")?,
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    let chrysalis_data = wallet.get_chrysalis_data().await?.unwrap();
    let accounts_indexation = chrysalis_data.get(CHRYSALIS_ACCOUNT_INDEXATION_KEY).unwrap();
    assert_eq!(
        accounts_indexation,
        "[{\"key\":\"wallet-account://b5e020ec9a67eb7ce07be742116bd27ae722e1159098c89dd7e50d972a7b13fc\"},{\"key\":\"wallet-account://e59975e320b8433916b4946bb1e21107e8d3f36d1e587782cbd35acf59c90d1a\"}]"
    );

    // Tests if setting stronghold password still works
    wallet.set_stronghold_password("password".to_owned()).await?;
    // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup"
    assert_eq!(
        wallet
            .generate_ed25519_address(0, 0, None)
            .await?
            .to_bech32(Hrp::from_str_unchecked("rms")),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );

    tear_down("migrate_chrysalis_db")
}

#[tokio::test]
async fn migrate_chrysalis_db_encrypted() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
    let storage_path = "migrate_chrysalis_db_encrypted/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db-encrypted/db", storage_path).unwrap();
    std::fs::copy(
        "./tests/wallet/fixtures/chrysalis-db-encrypted/wallet.stronghold",
        "migrate_chrysalis_db_encrypted/wallet.stronghold",
    )
    .unwrap();

    // error on wrong password
    assert!(matches!(
        migrate_db_chrysalis_to_stardust(
            "migrate_chrysalis_db_encrypted",
            Some("wrong-password".to_string().into()),
            None)
        .await,
        Err(iota_sdk::wallet::Error::Migration(err)) if err.contains("XCHACHA20-POLY1305")
    ));

    migrate_db_chrysalis_to_stardust(
        "migrate_chrysalis_db_encrypted",
        Some("password".to_string().into()),
        None,
    )
    .await?;

    let client_options = ClientOptions::new();
    let wallet = Wallet::builder()
        .with_storage_path("migrate_chrysalis_db_encrypted")
        .with_client_options(client_options)
        .finish()
        .await?;

    let accounts = wallet.get_accounts().await?;
    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].alias().await, "Alice");
    assert_eq!(accounts[1].alias().await, "Bob");

    let alice_acc_details = accounts[0].details().await;
    assert_eq!(alice_acc_details.public_addresses().len(), 2);
    assert_eq!(
        alice_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0]
            .address()
            .try_to_bech32("rms")?,
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    let chrysalis_data = wallet.get_chrysalis_data().await?.unwrap();
    let accounts_indexation = chrysalis_data.get(CHRYSALIS_ACCOUNT_INDEXATION_KEY).unwrap();
    assert_eq!(
        accounts_indexation,
        "[{\"key\":\"wallet-account://b5e020ec9a67eb7ce07be742116bd27ae722e1159098c89dd7e50d972a7b13fc\"},{\"key\":\"wallet-account://e59975e320b8433916b4946bb1e21107e8d3f36d1e587782cbd35acf59c90d1a\"}]"
    );

    // Tests if setting stronghold password still works
    wallet.set_stronghold_password("password".to_owned()).await?;
    // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup"
    assert_eq!(
        wallet
            .generate_ed25519_address(0, 0, None)
            .await?
            .to_bech32(Hrp::from_str_unchecked("rms")),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );

    tear_down("migrate_chrysalis_db_encrypted")
}

#[tokio::test]
async fn migrate_chrysalis_db_encrypted_encrypt_new() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
    let storage_path = "migrate_chrysalis_db_encrypted_encrypt_new/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db-encrypted/db", storage_path).unwrap();
    std::fs::copy(
        "./tests/wallet/fixtures/chrysalis-db-encrypted/wallet.stronghold",
        "migrate_chrysalis_db_encrypted_encrypt_new/wallet.stronghold",
    )
    .unwrap();

    migrate_db_chrysalis_to_stardust(
        "migrate_chrysalis_db_encrypted_encrypt_new",
        Some("password".to_string().into()),
        Some(Zeroizing::new([0u8; 32])),
    )
    .await?;

    let client_options = ClientOptions::new();
    let wallet = Wallet::builder()
        .with_storage_options(
            StorageOptions::new(
                "migrate_chrysalis_db_encrypted_encrypt_new".into(),
                StorageKind::Rocksdb,
            )
            .with_encryption_key([0u8; 32]),
        )
        .with_client_options(client_options)
        .finish()
        .await?;

    let accounts = wallet.get_accounts().await?;
    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].alias().await, "Alice");
    assert_eq!(accounts[1].alias().await, "Bob");

    let alice_acc_details = accounts[0].details().await;
    assert_eq!(alice_acc_details.public_addresses().len(), 2);
    assert_eq!(
        alice_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0]
            .address()
            .try_to_bech32("rms")?,
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    let chrysalis_data = wallet.get_chrysalis_data().await?.unwrap();
    let accounts_indexation = chrysalis_data.get(CHRYSALIS_ACCOUNT_INDEXATION_KEY).unwrap();
    assert_eq!(
        accounts_indexation,
        "[{\"key\":\"wallet-account://b5e020ec9a67eb7ce07be742116bd27ae722e1159098c89dd7e50d972a7b13fc\"},{\"key\":\"wallet-account://e59975e320b8433916b4946bb1e21107e8d3f36d1e587782cbd35acf59c90d1a\"}]"
    );

    // Tests if setting stronghold password still works
    wallet.set_stronghold_password("password".to_owned()).await?;
    // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup"
    assert_eq!(
        wallet
            .generate_ed25519_address(0, 0, None)
            .await?
            .to_bech32(Hrp::from_str_unchecked("rms")),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );

    tear_down("migrate_chrysalis_db_encrypted_encrypt_new")
}

#[tokio::test]
async fn migrate_chrysalis_stronghold() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
    let storage_path = "migrate_chrysalis_stronghold";
    setup(storage_path)?;

    let client_options = ClientOptions::new();
    let wallet = Wallet::builder()
        .with_storage_path(storage_path)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_client_options(client_options)
        .with_secret_manager(SecretManager::Placeholder)
        .finish()
        .await?;

    wallet
        .restore_backup(
            "./tests/wallet/fixtures/chrysalis-backup-work-factor-0.stronghold".into(),
            Password::from("password".to_string()),
            None,
            None,
        )
        .await?;

    let accounts = wallet.get_accounts().await?;
    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].alias().await, "Alice");
    assert_eq!(accounts[1].alias().await, "Bob");

    let alice_acc_details = accounts[0].details().await;
    assert_eq!(alice_acc_details.public_addresses().len(), 2);
    assert_eq!(
        alice_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0]
            .address()
            .try_to_bech32("rms")?,
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    let chrysalis_data = wallet.get_chrysalis_data().await?.unwrap();
    // CHRYSALIS_ACCOUNT_INDEXATION_KEY
    let accounts_indexation = chrysalis_data
        .get("0xddc058ad3b93b5a575b0051aafbc8ff17ad0415d7aa1c54d")
        .unwrap();
    assert_eq!(
        accounts_indexation,
        "[{\"key\":\"wallet-account://b5e020ec9a67eb7ce07be742116bd27ae722e1159098c89dd7e50d972a7b13fc\"},{\"key\":\"wallet-account://e59975e320b8433916b4946bb1e21107e8d3f36d1e587782cbd35acf59c90d1a\"}]"
    );

    // Tests if setting stronghold password still works, commented because age encryption is very slow in CI
    wallet.set_stronghold_password("password".to_owned()).await?;
    // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup"
    assert_eq!(
        wallet
            .generate_ed25519_address(0, 0, None)
            .await?
            .to_bech32(iota_sdk::types::block::address::Hrp::from_str_unchecked("rms")),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );

    tear_down(storage_path)
}

#[tokio::test]
async fn migrate_empty_chrysalis_db() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
    let storage_path = "migrate_empty_chrysalis_db";
    setup(storage_path)?;

    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_4_db_test", storage_path).unwrap();

    assert!(matches!(
        migrate_db_chrysalis_to_stardust("migrate_empty_chrysalis_db", None, None).await,
        Err(iota_sdk::wallet::error::Error::Migration(msg)) if msg == "no chrysalis data to migrate"
    ));

    // add empty /db folder
    fs::create_dir("migrate_empty_chrysalis_db/db")?;
    assert!(matches!(
        migrate_db_chrysalis_to_stardust("migrate_empty_chrysalis_db", None, None).await,
        Err(iota_sdk::wallet::error::Error::Migration(msg)) if msg == "no chrysalis data to migrate"
    ));

    // stardust wallet data is still there
    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;
    assert_eq!(wallet.get_accounts().await?.len(), 1);

    tear_down(storage_path)
}

fn copy_folder(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
    }
    Ok(())
}

#[cfg(feature = "ledger_nano")]
#[tokio::test]
async fn migrate_chrysalis_db_ledger() -> Result<()> {
    let storage_path = "migrate_chrysalis_db_ledger/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db-ledger/db", storage_path).unwrap();

    migrate_db_chrysalis_to_stardust("migrate_chrysalis_db_ledger", None, None).await?;

    let client_options = ClientOptions::new();
    let wallet = Wallet::builder()
        .with_storage_path("migrate_chrysalis_db_ledger")
        .with_client_options(client_options)
        .finish()
        .await?;

    let accounts = wallet.get_accounts().await?;
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].alias().await, "ledger");

    let alice_acc_details = accounts[0].details().await;
    assert_eq!(alice_acc_details.public_addresses().len(), 4);
    // mnemonic: glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact
    // monster seven myth punch hobby comfort wild raise skin
    assert_eq!(
        alice_acc_details.public_addresses()[0].address().try_to_bech32("rms")?,
        "rms1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruskth6z"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0]
            .address()
            .try_to_bech32("rms")?,
        "rms1qzev23h8qtdfjzzx4jqrdfaw2nnnwu2m4hhu2tkdmp2wrt6y8qwq22963tv"
    );

    let chrysalis_data = wallet.get_chrysalis_data().await?.unwrap();
    let accounts_indexation = chrysalis_data.get(CHRYSALIS_ACCOUNT_INDEXATION_KEY).unwrap();
    assert_eq!(
        accounts_indexation,
        "[{\"key\":\"wallet-account://2b9bd865368556d58f9d5a9fd44c30205f1fc80b09cde1dcb9b3a37748210854\"}]"
    );

    tear_down("migrate_chrysalis_db_ledger")
}
