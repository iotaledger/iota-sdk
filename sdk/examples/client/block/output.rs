// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a transaction.
//!
//! `cargo run --example output --release`

use iota_sdk::{
    client::{secret::SecretManager, utils::request_funds_from_faucet, Client, Result},
    types::block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
};

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
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = client.get_addresses(&secret_manager).with_range(0..1).finish().await?[0];
    request_funds_from_faucet(&faucet_url, &address).await?;

    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!("{block:#?}");

    println!(
        "Block with custom output sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
