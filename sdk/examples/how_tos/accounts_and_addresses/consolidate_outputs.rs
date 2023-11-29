// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will consolidate basic outputs from a wallet with only an AddressUnlockCondition by sending
//! them to the same address again.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example consolidate_outputs
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    types::block::address::ToBech32Ext,
    wallet::{ConsolidationParams, Result},
    Wallet,
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

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish(&secret_manager)
        .await?;

    // Sync wallet to make sure it is updated with outputs from previous examples
    wallet.sync(&secret_manager, None).await?;
    println!("Wallet synced");

    // List unspent outputs before consolidation.
    // The output we created with example `03_get_funds` and the basic output from `09_mint_native_tokens` have only one
    // unlock condition and it is an `AddressUnlockCondition`, and so they are valid for consolidation. They have the
    // same `AddressUnlockCondition`(the address of the wallet), so they will be consolidated into one
    // output.
    let outputs = wallet.unspent_outputs(None).await;
    println!("Outputs BEFORE consolidation:");
    outputs.iter().enumerate().for_each(|(i, output_data)| {
        println!("OUTPUT #{i}");
        println!(
            "- address: {:?}\n- amount: {:?}\n- native tokens: {:?}",
            output_data.address.clone().to_bech32_unchecked("rms"),
            output_data.output.amount(),
            output_data.output.native_token()
        )
    });

    println!("Sending consolidation transaction...");

    // Consolidate unspent outputs and print the consolidation transaction ID
    // Set `force` to true to force the consolidation even though the `output_threshold` isn't reached
    let transaction = wallet
        .consolidate_outputs(&secret_manager, ConsolidationParams::new().with_force(true))
        .await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for the consolidation transaction to get confirmed
    let block_id = wallet
        .reissue_transaction_until_included(&secret_manager, &transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    // Sync wallet
    wallet.sync(&secret_manager, None).await?;
    println!("Wallet synced");

    // Outputs after consolidation
    let outputs = wallet.unspent_outputs(None).await;
    println!("Outputs AFTER consolidation:");
    outputs.iter().enumerate().for_each(|(i, output_data)| {
        println!("OUTPUT #{i}");
        println!(
            "- address: {:?}\n- amount: {:?}\n- native tokens: {:?}",
            output_data.address.clone().to_bech32_unchecked("rms"),
            output_data.output.amount(),
            output_data.output.native_token()
        )
    });

    Ok(())
}
