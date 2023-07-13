// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs, io, path::Path};

#[cfg(feature = "stronghold")]
use iota_sdk::client::stronghold::StrongholdAdapter;
use iota_sdk::{wallet::Result, Wallet};

use crate::wallet::common::{setup, tear_down};

// Db created with wallet.rs commit 8dd389ddeed0d95bb493c38f376b41a6a9127148
#[cfg(feature = "stronghold")]
#[tokio::test]
async fn check_existing_db() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let storage_path = "check_existing_db_test";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_db_test", storage_path).unwrap();

    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    // Migrate old snapshots.
    let _ = StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "check_existing_db_test/strongholdfile",
        "STRONGHOLD_PASSWORD".to_owned().into(),
        "wallet.rs",
        100,
        None,
        None,
    );

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
        account.generate_ed25519_addresses(1, None).await?[0]
            .address()
            .to_string(),
        "rms1qzjclfjq0azmq2yzkkk7ugfhdf55nzvs57r8twk2h36wuqv950dxv00tzfx"
    );

    let transactions = account.transactions().await;
    assert_eq!(transactions.len(), 2);

    let pending_transactions = account.pending_transactions().await;
    assert_eq!(pending_transactions.len(), 1);

    let incoming_transactions = account.incoming_transactions().await;
    assert_eq!(incoming_transactions.len(), 1);

    let unspent_outputs = account.unspent_outputs(None).await?;
    assert_eq!(unspent_outputs.len(), 9);

    // balance depends on the network
    if &account.client().get_network_name().await? == "private_tangle1" {
        let balance = account.balance().await?;
        assert_eq!(balance.base_coin().total(), 100000000000);
        assert_eq!(balance.base_coin().available(), 99996954100);
    }

    tear_down(storage_path)
}

// Db created with wallet.rs commit 2dd9974c1bc05c2b0b7d6f0ee100deb2da60d071
#[cfg(feature = "ledger_nano")]
#[tokio::test]
async fn check_existing_db_1() -> Result<()> {
    let storage_path = "check_existing_1_db_test";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_1_db_test", storage_path).unwrap();

    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    assert!(matches!(
        *wallet.get_secret_manager().read().await,
        iota_sdk::client::secret::SecretManager::LedgerNano(_)
    ));

    assert_eq!(wallet.get_accounts().await?.len(), 1);

    let client_options = wallet.client_options().await;
    assert_eq!(client_options.node_manager_builder.nodes.len(), 1);

    let account = wallet.get_account("Alice").await?;

    let addresses = account.addresses().await?;
    // One public address
    assert_eq!(addresses.len(), 1);
    // Wallet was created with mnemonic: "glory promote mansion idle axis finger extra february uncover one trip
    // resource lawn turtle enact monster seven myth punch hobby comfort wild raise skin"
    assert_eq!(
        addresses[0].address().to_string(),
        "tst1qqqxdxwaq8ewc7xaph7x9mjpd9fysr0qsj4fpc6kw20wxswmjt7t5r4eupa"
    );
    assert!(!addresses[0].internal());

    let transactions = account.transactions().await;
    assert_eq!(transactions.len(), 5);

    let pending_transactions = account.pending_transactions().await;
    assert_eq!(pending_transactions.len(), 0);

    let incoming_transactions = account.incoming_transactions().await;
    assert_eq!(incoming_transactions.len(), 0);

    let unspent_outputs = account.unspent_outputs(None).await?;
    assert_eq!(unspent_outputs.len(), 6);

    tear_down(storage_path)
}

// Db created with wallet.rs commit b5132eb545cd0a2043640677bca335efee4029b8
#[cfg(feature = "stronghold")]
#[tokio::test]
async fn check_existing_db_2() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let storage_path = "check_existing_2_db_test";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_2_db_test", storage_path).unwrap();

    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    // Migrate old snapshots.
    let _ = StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "check_existing_2_db_test/walletstronghold",
        "STRONGHOLD_PASSWORD".to_owned().into(),
        "wallet.rs",
        100,
        None,
        None,
    );

    // Test if setting stronghold password still works
    wallet.set_stronghold_password("STRONGHOLD_PASSWORD".to_owned()).await?;

    assert_eq!(wallet.get_accounts().await?.len(), 2);

    let client_options = wallet.client_options().await;
    assert_eq!(client_options.node_manager_builder.nodes.len(), 1);

    let account = wallet.get_account("Alice").await?;

    let addresses = account.addresses().await?;
    // One public address
    assert_eq!(addresses.len(), 1);
    // Wallet was created with mnemonic: "memory waste latin swing spy must tail leaf eyebrow meat any river resist sort
    // paper bacon aware edit tragic shop mirror ramp foster blue"
    assert_eq!(
        addresses[0].address().to_string(),
        "tst1qqpaynvgh3d3q30zjzjz27g95pzsevw2m5pk75rq32vm7x8ap7hpy2pdy9y"
    );
    assert!(!addresses[0].internal());

    let transactions = account.transactions().await;
    assert_eq!(transactions.len(), 5);

    use std::str::FromStr;
    let tx = account
        .get_transaction(&iota_sdk::types::block::payload::transaction::TransactionId::from_str(
            "0x09bb7e0a77f944a4625428d2cdc7a637f5bb5d9a877c9c0b116c909ab4a6795d",
        )?)
        .await
        .expect("missing tx");

    let iota_sdk::types::block::payload::transaction::TransactionEssence::Regular(essence) = tx.payload.essence();
    if let iota_sdk::types::block::payload::Payload::TaggedData(tagged_data_payload) = essence.payload().unwrap() {
        assert_eq!(tagged_data_payload.tag(), "Stardust".as_bytes());
        assert_eq!(tagged_data_payload.data(), "Stardust".as_bytes());
    } else {
        panic!("expected tagged data payload")
    }
    assert_eq!(
        tx.inclusion_state,
        iota_sdk::wallet::account::types::InclusionState::Pending
    );

    let pending_transactions = account.pending_transactions().await;
    assert_eq!(pending_transactions.len(), 1);

    let incoming_transactions = account.incoming_transactions().await;
    assert_eq!(incoming_transactions.len(), 0);

    let unspent_outputs = account.unspent_outputs(None).await?;
    assert_eq!(unspent_outputs.len(), 4);

    tear_down(storage_path)
}

// Db created with iota-sdk commit 5d47eaf362fa769ca3c55c5e947fc7fcd9d6457f (npm @iota/wallet@2.0.3-rc.35)
#[cfg(feature = "stronghold")]
#[tokio::test]
async fn check_existing_db_3() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let storage_path = "check_existing_3_db_test";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_3_db_test", storage_path).unwrap();

    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    // Migrate old snapshots.
    let _ = StrongholdAdapter::migrate_snapshot_v2_to_v3(
        "check_existing_3_db_test/walletstronghold",
        "A12345678*".to_owned().into(),
        "wallet.rs",
        100,
        None,
        None,
    );

    // Test if setting stronghold password still works
    wallet.set_stronghold_password("A12345678*".to_owned()).await?;

    assert_eq!(wallet.get_accounts().await?.len(), 2);

    let client_options = wallet.client_options().await;
    assert_eq!(client_options.node_manager_builder.nodes.len(), 1);

    let account = wallet.get_account("Alice").await?;

    let addresses = account.addresses().await?;
    // One public address
    assert_eq!(addresses.len(), 1);
    // Wallet was created with mnemonic: "bulk spoon broken license diary nominee tribe visit used giant rail insect
    // lesson home toast autumn cancel alley park give same wet wash vanish"
    assert_eq!(
        addresses[0].address().to_string(),
        "rms1qrhdre8ra8n42h30zkf9jjlezefrull082dgcrtrpfyng3qtsgnywquf58d"
    );
    assert!(!addresses[0].internal());

    let transactions = account.transactions().await;
    assert_eq!(transactions.len(), 4);

    let pending_transactions = account.pending_transactions().await;
    assert_eq!(pending_transactions.len(), 1);

    let pending_transactions = account.incoming_transactions().await;
    assert_eq!(pending_transactions.len(), 24);

    tear_down(storage_path)
}

// Db created with iota-sdk commit cbf89d995d62cdcd2a2c29e363af6e8e4debcc54 (pypi iota-sdk==1.0.0rc0)
#[cfg(feature = "stronghold")]
#[tokio::test]
async fn check_existing_db_4() -> Result<()> {
    let storage_path = "check_existing_4_db_test";
    setup(storage_path)?;
    // Copy db so the original doesn't get modified
    copy_folder("./tests/wallet/fixtures/check_existing_4_db_test", storage_path).unwrap();

    let wallet = Wallet::builder().with_storage_path(storage_path).finish().await?;

    // Commented because it wasn't created with encrypt_work_factor 0
    // wallet.set_stronghold_password("STRONGHOLD_PASSWORD".to_owned()).await?;

    assert_eq!(wallet.get_accounts().await?.len(), 1);

    let client_options = wallet.client_options().await;
    assert_eq!(client_options.node_manager_builder.nodes.len(), 1);

    let account = wallet.get_account("Alice").await?;

    let addresses = account.addresses().await?;
    // One public address
    assert_eq!(addresses.len(), 1);
    // Wallet was created with mnemonic: "width scatter jaguar sponsor erosion enable cave since ancient first garden
    // royal luggage exchange ritual exotic play wall clinic ride autumn divert spin exchange"
    assert_eq!(
        addresses[0].address().to_string(),
        "rms1qz7tvqdr2usmecs2f669vccl5nw6yhh3xnl6dkas3zxv56esx7zw2ekjesf"
    );
    assert!(!addresses[0].internal());

    let transactions = account.transactions().await;
    assert_eq!(transactions.len(), 1);

    let pending_transactions = account.pending_transactions().await;
    assert_eq!(pending_transactions.len(), 1);

    let pending_transactions = account.incoming_transactions().await;
    assert_eq!(pending_transactions.len(), 13);

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
