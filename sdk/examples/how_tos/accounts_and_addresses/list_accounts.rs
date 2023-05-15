// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all accounts in the wallet.
//!
//! `cargo run --example list_accounts --release`

use iota_sdk::{wallet::{Wallet, Result}};

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the accounts and print the alias of each account
    for account in wallet.get_accounts().await? {
        println!("{}", account.alias().await);
    }

    Ok(())
}