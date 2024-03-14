// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will consolidate basic outputs from a wallet with only an AddressUnlockCondition by sending
//! them to the same address again.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example consolidate_outputs
//! ```

use iota_sdk::{wallet::ConsolidationParams, Wallet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "EXPLORER_URL", "STRONGHOLD_PASSWORD"] {
        if std::env::var(var).is_err() {
            panic!(".env variable '{}' is undefined, see .env.example", var);
        }
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Sync wallet to make sure it is updated with outputs from previous examples
    wallet.sync(None).await?;
    println!("Wallet synced");

    // List unspent outputs before consolidation.
    // The output we created with example `03_get_funds` and the basic output from `09_mint_native_tokens` have only one
    // unlock condition and it is an `AddressUnlockCondition`, and so they are valid for consolidation. They have the
    // same `AddressUnlockCondition`(the address of the wallet), so they will be consolidated into one
    // output.
    println!("Outputs BEFORE consolidation:");
    wallet
        .ledger()
        .await
        .unspent_outputs()
        .values()
        .enumerate()
        .for_each(|(i, output_with_ext_metadata)| {
            println!("OUTPUT #{i}");
            println!(
                "- amount: {:?}\n- native tokens: {:?}",
                output_with_ext_metadata.output.amount(),
                output_with_ext_metadata.output.native_token()
            )
        });

    println!("Sending consolidation transaction...");

    // Consolidate unspent outputs and print the consolidation transaction ID
    // Set `force` to true to force the consolidation even though the `output_threshold` isn't reached
    let transaction = wallet
        .consolidate_outputs(ConsolidationParams::new().with_force(true))
        .await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for the consolidation transaction to get accepted
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    // Sync wallet
    wallet.sync(None).await?;
    println!("Wallet synced");

    // Outputs after consolidation
    println!("Outputs AFTER consolidation:");
    wallet
        .ledger()
        .await
        .unspent_outputs()
        .values()
        .enumerate()
        .for_each(|(i, output_with_ext_metadata)| {
            println!("OUTPUT #{i}");
            println!(
                "- amount: {:?}\n- native tokens: {:?}",
                output_with_ext_metadata.output.amount(),
                output_with_ext_metadata.output.native_token()
            )
        });

    Ok(())
}
