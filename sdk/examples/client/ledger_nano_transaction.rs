// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a transaction with a ledger nano hardware wallet.
//!
//! `cargo run --example ledger_nano_transaction --features=ledger_nano --release`

use iota_sdk::client::{
    api::GetAddressesOptions,
    secret::{ledger_nano::LedgerSecretManager, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()
        .await?;

    let secret_manager = SecretManager::LedgerNano(LedgerSecretManager::new(true));

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..2),
        )
        .await?;

    println!("List of generated public addresses:\n{addresses:?}\n");

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            addresses[1],
            1_000_000,
        )
        .await?
        .finish()
        .await?;

    println!(
        "Block using ledger nano sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
