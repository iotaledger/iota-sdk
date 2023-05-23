// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all outputs of an account.
//!
//! `cargo run --release --all-features --example list_outputs`

use std::println;

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account
    let account = wallet.get_account("Alice").await?;

    // Sync account
    account.sync(None).await?;

    // Print output ids
    println!("Output ids:");
    for output in account.outputs(None).await? {
        println!("{}", output.output_id);
    }

    // Print unspent output ids
    println!("Unspent output ids:");
    for output in account.unspent_outputs(None).await? {
        println!("{}", output.output_id);
    }

    Ok(())
}
