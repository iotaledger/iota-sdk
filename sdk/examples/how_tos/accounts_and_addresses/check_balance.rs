// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sync the account and get the balance.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example check_balance
//! ```

use iota_sdk::{types::block::address::ToBech32Ext, wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_alias("Alice")
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Sync and get the balance
    let balance = wallet.sync(None).await?;
    println!("{balance:#?}");

    let explorer_url = std::env::var("EXPLORER_URL").ok();
    let addr_base_url = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();
    println!("{addr_base_url}{}", wallet.address_as_bech32().await);

    Ok(())
}
