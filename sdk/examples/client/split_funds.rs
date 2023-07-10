// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send 100 basic outputs to our first address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example client_split_funds
//! ```

use iota_sdk::client::{api::GetAddressesOptions, request_funds_from_faucet, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic
    // `MNEMONIC` must contain non-zero balance.
    dotenvy::dotenv().ok();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];
    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address,).await?
    );
    // wait so the faucet can send the funds
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let mut block_builder = client.block().with_secret_manager(&secret_manager);
    // Insert the output address and amount to spent. The amount cannot be zero.
    for _ in 0..100 {
        block_builder = block_builder.with_output(address, 1_000_000).await?;
    }
    let block = block_builder.finish().await?;

    println!(
        "Block with split funds sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
