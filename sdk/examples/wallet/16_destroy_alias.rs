// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to destroy the first alias there is in the account. This is only possible if possible
//! foundry outputs have circulating supply of 0.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example destroy_alias
//! ```

use iota_sdk::{wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let alias = "Alice";
    let account = wallet.get_account(alias).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first alias
    if let Some(alias_id) = balance.aliases().first() {
        let aliases_before = balance.aliases();
        println!("Aliases BEFORE destroying:\n{aliases_before:#?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Sending alias burn transaction...");

        let transaction = account.burn(*alias_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Transaction included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );

        println!("Burned Alias '{}'", alias_id);

        let balance = account.sync(None).await?;
        let aliases_after = balance.aliases();
        println!("Aliases AFTER destroying:\n{aliases_after:#?}",);
    } else {
        println!("No Alias available in account '{alias}'");
    }

    Ok(())
}
