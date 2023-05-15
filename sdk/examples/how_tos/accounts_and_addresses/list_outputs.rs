// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all outputs of an account.
//!
//! `cargo run --example list_outputs --release`

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
    for output in account.outputs(None).await? {
        println!("{}", output.output_id);
    }

    Ok(())
}
