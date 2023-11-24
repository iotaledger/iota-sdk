// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all accounts in the wallet.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example list_accounts
//! ```

use iota_sdk::{wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH"] {
        if std::env::var(var).is_err() {
            panic!(".env variable '{}' is undefined, see .env.example", var);
        }
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Get the accounts and print the alias of each account
    for account in wallet.get_accounts().await? {
        println!("{}", account.alias().await);
    }

    Ok(())
}
