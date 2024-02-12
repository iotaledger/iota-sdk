// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an account output.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example and that funds are available by running
//! the `get_funds` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example create_account_output
//! ```

use iota_sdk::{wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(None).await?;
    println!("Accounts BEFORE:\n{:#?}", balance.accounts());

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Sending the create-account transaction...");

    // Create an account output
    let transaction = wallet.create_account_output(None, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let balance = wallet.sync(None).await?;
    println!("Accounts AFTER:\n{:#?}", balance.accounts());

    Ok(())
}
