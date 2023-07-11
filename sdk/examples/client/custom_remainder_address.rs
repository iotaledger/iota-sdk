// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a certain amount of tokens to a given receiver address and any remaining
//! tokens to a custom remainder address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example custom_remainder_address [AMOUNT]
//! ```

use iota_sdk::client::{
    api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
    secret::SecretManager, Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let amount = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(9_000_000);

    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a client instance.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..3),
        )
        .await?;

    let sender_address = &addresses[0];
    let receiver_address = &addresses[1];
    let remainder_address = &addresses[2];

    println!("sender address: {sender_address}");
    println!("receiver address: {receiver_address}");
    println!("remainder address: {remainder_address}");

    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), sender_address).await?,
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let output_ids_response = client
        .basic_output_ids([QueryParameter::Address(*sender_address)])
        .await?;
    println!("{output_ids_response:#?}");

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_output(receiver_address, amount)
        .await?
        .with_custom_remainder_address(remainder_address)?
        .finish()
        .await?;

    println!(
        "Block with custom remainder sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
