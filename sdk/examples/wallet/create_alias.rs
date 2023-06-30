// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an account output.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example and that funds are available by running
//! the `get_funds` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example create_account_output
//! ```

use std::env::var;

use iota_sdk::{wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;
    println!("Aliases BEFORE:\n{:#?}", balance.accounts());

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Sending the create-alias transaction...");

    // Create an account output
    let transaction = account.create_account_output(None, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );

    let balance = account.sync(None).await?;
    println!("Aliases AFTER:\n{:#?}", balance.accounts());

    Ok(())
}
