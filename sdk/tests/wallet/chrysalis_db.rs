// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs, io, path::Path};

// #[cfg(feature = "stronghold")]
// use iota_sdk::client::stronghold::StrongholdAdapter;
use iota_sdk::wallet::{migration::migrate_db_from_chrysalis_to_stardust, Result};

use crate::wallet::common::{setup, tear_down};

// #[cfg(feature = "stronghold")]
#[tokio::test]
async fn migrate_chrysalis_db() -> Result<()> {
    // iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let storage_path = "migrate_chrysalis_db/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db/db", storage_path).unwrap();

    migrate_db_from_chrysalis_to_stardust("migrate_chrysalis_db", None).await?;

    // let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    // // Test if setting stronghold password still works
    // wallet.set_stronghold_password("STRONGHOLD_PASSWORD".to_owned()).await?;

    // assert_eq!(wallet.get_accounts().await?.len(), 1);

    // let client_options = wallet.client_options().await;
    // assert_eq!(client_options.node_manager_builder.nodes.len(), 1);

    // let account = wallet.get_account("Alice").await?;

    // let addresses = account.addresses().await?;
    // // One public and one internal address
    // assert_eq!(addresses.len(), 2);
    // // Wallet was created with mnemonic: "rapid help true please need desk oppose seminar busy large tree speed
    // pepper // adult hair duty mad chief boil pass coin biology survey fish"
    // assert_eq!(
    //     addresses[0].address().to_string(),
    //     "rms1qzsw70tha0y4n78s0x0p99ayvz7nl7mzcye7yk8l3s8m6zrfg7slud2ve9f"
    // );
    // assert!(!addresses[0].internal());
    // assert_eq!(
    //     addresses[1].address().to_string(),
    //     "rms1qzjwe5plkaywncpv32x5dqqav8fe9zgyzl78cmjlnvzlcghnx489wuevhzf"
    // );
    // assert!(addresses[1].internal());

    // assert_eq!(
    //     account.generate_ed25519_addresses(1, None).await?[0]
    //         .address()
    //         .to_string(),
    //     "rms1qzjclfjq0azmq2yzkkk7ugfhdf55nzvs57r8twk2h36wuqv950dxv00tzfx"
    // );

    tear_down(storage_path)
}

#[tokio::test]
async fn migrate_chrysalis_db_encrypted() -> Result<()> {
    let storage_path = "migrate_chrysalis_db_encrypted/db";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/chrysalis-db-encrypted/db", storage_path).unwrap();

    migrate_db_from_chrysalis_to_stardust("migrate_chrysalis_db_encrypted", Some("password")).await?;

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
