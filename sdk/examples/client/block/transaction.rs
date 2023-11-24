// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a certain amount of tokens to some address by using the block builder only.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example transaction [AMOUNT] [ADDRESS]
//! ```

use iota_sdk::client::{api::GetAddressesOptions, request_funds_from_faucet, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "EXPLORER_URL", "FAUCET_URL", "MNEMONIC"] {
        if std::env::var(var).is_err() {
            panic!(".env variable '{}' is undefined, see .env.example", var);
        }
    }

    let node_url = std::env::var("NODE_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    let amount = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(1_000_000u64);

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Get the first address of the seed
    let first_address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];
    println!("1st address: {first_address:#?}");

    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&faucet_url, &first_address).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    // If no custom address is provided, we will use the first address from the seed.
    let recv_address = std::env::args()
        .nth(2)
        .map(|s| s.parse().unwrap())
        .unwrap_or(first_address);

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(recv_address, amount)
        .await?
        .finish()
        .await?;

    println!("{block:#?}");

    println!(
        "Transaction sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
