// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to find the index and type (public or internal) of an address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example search_address [BECH32_ADDRESS] [START_INDEX] [RANGE_SIZE]
//! ```

use iota_sdk::client::{
    api::{search_address, GetAddressesOptions},
    constants::SHIMMER_COIN_TYPE,
    secret::SecretManager,
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let address = if let Some(addr) = std::env::args()
        .nth(1)
        .map(|addr| addr.parse().expect("invalid address"))
    {
        addr
    } else {
        secret_manager
            .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
            .await?[0]
    };
    println!("Search address: {address:#?}");

    let address_range_start = std::env::args().nth(2).map(|s| s.parse::<u32>().unwrap()).unwrap_or(0);
    let address_range_len = std::env::args().nth(3).map(|s| s.parse::<u32>().unwrap()).unwrap_or(10);
    let address_range = address_range_start..address_range_start + address_range_len;

    let (address_index, is_internal) = search_address(
        &secret_manager,
        client.get_bech32_hrp().await?,
        SHIMMER_COIN_TYPE,
        0,
        address_range,
        &address.into(),
    )
    .await?;
    println!("Address index: {address_index}\nIs internal address: {is_internal}");

    Ok(())
}
