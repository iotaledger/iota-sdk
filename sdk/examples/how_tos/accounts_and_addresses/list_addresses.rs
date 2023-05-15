// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all addresses of an account.
//!
//! `cargo run --example list_addresses --release --features="rocksdb stronghold"`

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account
    let account = wallet.get_account("Alice").await?;

    for address in account.addresses().await? {
        println!("{}", address.address());
    }

    Ok(())
}
