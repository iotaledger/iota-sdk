// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send 100 basic outputs to our first address.
//!
//! `cargo run --example split_funds --release`

use iota_sdk::client::{request_funds_from_faucet, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
    // balance.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(&faucet_url, &address.to_bech32(client.get_bech32_hrp().await?)).await?
    );

    // wait so the faucet can send the funds
    // tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let mut block_builder = client.block().with_secret_manager(&secret_manager);
    // Insert the output address and amount to spent. The amount cannot be zero.
    for _ in 0..100 {
        block_builder = block_builder
            .with_output(
                // We generate an address from our seed so that we send the funds to ourselves
                &client.get_addresses(&secret_manager).with_range(0..1).finish().await?[0],
                1_000_000,
            )
            .await?;
    }
    let block = block_builder.finish().await?;

    println!(
        "Block with split funds sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    Ok(())
}
