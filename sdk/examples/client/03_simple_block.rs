// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a block without a payload.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example 03_simple_block
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client = Client::builder()
        .with_node(&env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let block = client.block().finish().await?;

    println!(
        "Empty block sent: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
