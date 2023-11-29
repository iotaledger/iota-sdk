// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a transaction.
//!
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example advanced_transaction`

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{AddressUnlockCondition, TimelockUnlockCondition},
            BasicOutputBuilder,
        },
        slot::SlotIndex,
    },
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())?;

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    // Get the wallet we generated with `create_wallet`.
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish(&secret_manager)
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(&secret_manager, None).await?;

    if balance.base_coin().available() >= 1_000_000 {
        // Set the stronghold password
        secret_manager
            .set_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // TODO better time-based UX ?
        // Create an output with amount 1_000_000 and a timelock of 1000 slots.
        let slot_index = SlotIndex::from(1000);
        let basic_output = BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(Bech32Address::try_from_str(
                "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
            )?))
            .add_unlock_condition(TimelockUnlockCondition::new(slot_index)?)
            .finish_output()?;

        let transaction = wallet.send_outputs(&secret_manager, vec![basic_output], None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        // Wait for transaction to get included
        let block_id = wallet
            .reissue_transaction_until_included(&secret_manager, &transaction.transaction_id, None, None)
            .await?;
        println!(
            "Block sent: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );
    }

    Ok(())
}
