// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an alias output.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example create_alias --release`

use std::env;

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Create an alias output
    let transaction = account.create_alias_output(None, None).await?;
    println!(
        "Block sent: {}/block/{}",
        &env::var("EXPLORER_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}
