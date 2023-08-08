// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs, io, path::Path};

use iota_sdk::{
    client::{constants::IOTA_COIN_TYPE, secret::SecretManager, Password},
    wallet::{migration::migrate_db_from_chrysalis_to_stardust, ClientOptions, Result},
    Wallet,
};

use crate::wallet::common::{setup, tear_down};

#[tokio::test]
async fn migrate_chrysalis_db() -> Result<()> {
    let storage_path = "migrate_chrysalis_db/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db/db", storage_path).unwrap();

    migrate_db_from_chrysalis_to_stardust("migrate_chrysalis_db".into(), None).await?;

    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;
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
        alice_acc_details.public_addresses()[0].address(),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0].address(),
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address(),
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address(),
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    // // Tests if setting stronghold password still works, commented because age encryption is very slow in CI
    // wallet.set_stronghold_password("password".to_owned()).await?;
    // // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup" assert_eq!(
    //     wallet.generate_ed25519_address(0, 0, None).await?.to_bech32(Hrp::from_str_unchecked("rms")),
    //     "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    // );

    tear_down("migrate_chrysalis_db")
}

#[tokio::test]
async fn migrate_chrysalis_db_encrypted() -> Result<()> {
    let storage_path = "migrate_chrysalis_db_encrypted/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db-encrypted/db", storage_path).unwrap();

    migrate_db_from_chrysalis_to_stardust(
        "migrate_chrysalis_db_encrypted".into(),
        Some("password".to_string().into()),
    )
    .await?;

    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;
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
        alice_acc_details.public_addresses()[0].address(),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0].address(),
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address(),
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address(),
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    // // Tests if setting stronghold password still works, commented because age encryption is very slow in CI
    // wallet.set_stronghold_password("password".to_owned()).await?;
    // // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup" assert_eq!(
    //     wallet.generate_ed25519_address(0, 0, None).await?.to_bech32(Hrp::from_str_unchecked("rms")),
    //     "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    // );

    tear_down("migrate_chrysalis_db_encrypted")
}

#[tokio::test]
async fn migrate_chrysalis_stronghold() -> Result<()> {
    let storage_path = "migrate_chrysalis_stronghold";
    setup(storage_path)?;

    // TODO: original doesn't get modified, so not needed?
    // Copy stronghold file so the original doesn't get modified
    fs::create_dir_all("migrate_chrysalis_stronghold")?;
    std::fs::copy(
        "./tests/wallet/fixtures/chrysalis-backup.stronghold",
        "migrate_chrysalis_stronghold/chrysalis-backup.stronghold",
    )?;

    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;
    let wallet = Wallet::builder()
        .with_storage_path(storage_path)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_client_options(client_options)
        .with_secret_manager(SecretManager::Placeholder)
        .finish()
        .await?;

    // TODO: create extra stronghold with encryption work factor 0
    wallet
        .restore_backup(
            "migrate_chrysalis_stronghold/chrysalis-backup.stronghold".into(),
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
        alice_acc_details.public_addresses()[0].address(),
        "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    );
    assert_eq!(alice_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        alice_acc_details.internal_addresses()[0].address(),
        "rms1qz4tac74vympq4hqqz8g9egrkhscn9743svd9xxh2w99qf5cd8vcxrmspmw"
    );

    let bob_acc_details = accounts[1].details().await;
    assert_eq!(bob_acc_details.public_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.public_addresses()[0].address(),
        "rms1qql3h5vxh2sxa93yadh7f4rkr7f9g9e65wlytazeu688mpcvhvmd2xvfq8y"
    );
    assert_eq!(bob_acc_details.internal_addresses().len(), 1);
    assert_eq!(
        bob_acc_details.internal_addresses()[0].address(),
        "rms1qq4c9kl7vz0yssjw02w7jda56lec4ss3anfq03gwzdxzl92hcfjz7daxdfg"
    );

    // // Tests if setting stronghold password still works, commented because age encryption is very slow in CI
    // wallet.set_stronghold_password("password".to_owned()).await?;
    // // Wallet was created with mnemonic: "extra dinosaur float same hockey cheese motor divert cry misery response
    // hawk gift hero pool clerk hill mask man code dragon jacket dog soup" assert_eq!(
    //     wallet.generate_ed25519_address(0, 0, None).await?.to_bech32(Hrp::from_str_unchecked("rms")),
    //     "rms1qqqu7qry22f6v7d2d9aesny9vjtf56unpevkfzfudddlcq5ja9clv44sef6"
    // );

    tear_down("migrate_chrysalis_db_encrypted")
}

fn copy_folder(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
    }
    Ok(())
}
