// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will consolidate all funds in a range of addresses.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example address_consolidation [ADDRESS INDEX START] [ADDRESS COUNT]
//! ```

use iota_sdk::client::{api::GetAddressesOptions, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let address_range_start = std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()).unwrap_or(0);
    let address_range_len = std::env::args().nth(2).map(|s| s.parse::<u32>().unwrap()).unwrap_or(10);

    let address_range = address_range_start..address_range_start + address_range_len;
    println!("Address consolidation range: {:?}", address_range);

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Here all funds will be send to the address with the lowest index in the range
    let address = client
        .consolidate_funds(
            &secret_manager,
            GetAddressesOptions {
                range: address_range,
                ..Default::default()
            },
        )
        .await?;

    println!(
        "Funds consolidated to: {}/addr/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        address
    );

    Ok(())
}
