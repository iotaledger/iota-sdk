// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs, io, path::Path};

use iota_sdk::wallet::{Result, Wallet};

use crate::wallet::common::{setup, tear_down};

#[cfg(all(feature = "stronghold", feature = "rocksdb"))]
#[tokio::test]
async fn check_existing_db() -> Result<()> {
    let storage_path = "check_existing_db_test";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_db_test", storage_path).unwrap();

    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    // Test if setting stronghold password still works
    wallet.set_stronghold_password("STRONGHOLD_PASSWORD".to_owned()).await?;

    assert_eq!(wallet.get_accounts().await?.len(), 1);

    let client_options = wallet.client_options().await;
    assert_eq!(client_options.node_manager_builder.nodes.len(), 1);

    let account = wallet.get_account("Alice").await?;

    let addresses = account.addresses().await?;
    // One public and one internal address
    assert_eq!(addresses.len(), 2);
    // Wallet was created with mnemonic: "rapid help true please need desk oppose seminar busy large tree speed pepper
    // adult hair duty mad chief boil pass coin biology survey fish"
    assert_eq!(
        addresses[0].address().to_string(),
        "rms1qzsw70tha0y4n78s0x0p99ayvz7nl7mzcye7yk8l3s8m6zrfg7slud2ve9f"
    );
    assert!(!addresses[0].internal());
    assert_eq!(
        addresses[1].address().to_string(),
        "rms1qzjwe5plkaywncpv32x5dqqav8fe9zgyzl78cmjlnvzlcghnx489wuevhzf"
    );
    assert!(addresses[1].internal());

    assert_eq!(
        account.generate_addresses(1, None).await?[0].address().to_string(),
        "rms1qzjclfjq0azmq2yzkkk7ugfhdf55nzvs57r8twk2h36wuqv950dxv00tzfx"
    );

    let transactions = account.transactions().await?;
    assert_eq!(transactions.len(), 2);

    let pending_transactions = account.pending_transactions().await?;
    assert_eq!(pending_transactions.len(), 1);

    let incoming_transactions = account.incoming_transactions().await?;
    assert_eq!(incoming_transactions.len(), 1);

    let unspent_outputs = account.unspent_outputs(None).await?;
    assert_eq!(unspent_outputs.len(), 9);

    // balance depends on the network
    if &account.client().get_network_name().await? == "private_tangle1" {
        let account_balance = account.balance().await?;
        assert_eq!(account_balance.base_coin().total(), 100000000000);
        assert_eq!(account_balance.base_coin().available(), 99996954100);
    }

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
