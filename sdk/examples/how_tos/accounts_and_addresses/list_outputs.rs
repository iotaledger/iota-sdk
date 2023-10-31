// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will list all outputs of an account.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example list_outputs
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

    // Sync wallet
    wallet.sync(None).await?;

    // Print output ids
    println!("Output ids:");
    for output in wallet.outputs(None).await? {
        println!("{}", output.output_id);
    }

    // Print unspent output ids
    println!("Unspent output ids:");
    for output in wallet.unspent_outputs(None).await? {
        println!("{}", output.output_id);
    }

    Ok(())
}
