// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all accounts in the wallet.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example list_accounts
//! ```

use iota_sdk::{
    client::secret::SecretManager,
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .load_storage::<SecretManager>(std::env::var("WALLET_DB_PATH").unwrap())
        .await?
        .finish()
        .await?;

    // Get the accounts and print the alias of each account
    for account in wallet.get_accounts().await? {
        println!("{}", account.alias().await);
    }

    Ok(())
}
