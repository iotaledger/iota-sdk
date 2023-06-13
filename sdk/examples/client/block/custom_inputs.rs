// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a certain amount of tokens to some address using custom inputs.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example custom_inputs [AMOUNT] [ADDRESS]
//! ```

use std::{env, time::Duration};

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
        secret::SecretManager, Client, Result,
    },
    types::block::input::UtxoInput,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();
    let faucet_url = env::var("FAUCET_URL").unwrap();
    let seed = env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap();

    let mut args = env::args().skip(1);
    let amount = args.next().map(|s| s.parse::<u64>().unwrap()).unwrap_or(1_000_000u64);

    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()
        .await?;

    let secret_manager = SecretManager::try_from_hex_seed(seed)?;

    // Get the first address of the seed
    let first_address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];
    println!("1st address: {first_address:#?}");

    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&faucet_url, &first_address).await?
    );
    tokio::time::sleep(Duration::from_secs(15)).await;

    let output_ids_response = client
        .basic_output_ids([QueryParameter::Address(first_address)])
        .await?;
    println!("{output_ids_response:?}");

    // If no custom address is provided, we will use the first address from the seed.
    let recv_address = args.next().map(|s| s.parse().unwrap()).unwrap_or(first_address);

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(UtxoInput::from(output_ids_response.items[0]))?
        //.with_input_range(20..25)
        .with_output(recv_address, amount)
        .await?
        .finish()
        .await?;

    println!("{block:#?}");

    println!(
        "Block with custom inputs sent: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    Ok(())
}
