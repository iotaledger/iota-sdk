// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send basic outputs with different feature blocks.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example basic
//! ```

use iota_sdk::{
    client::{api::GetAddressesOptions, secret::SecretManager, utils::request_funds_from_faucet, Client, Result},
    types::block::output::{
        feature::MetadataFeature,
        unlock_condition::{
            AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
            TimelockUnlockCondition,
        },
        BasicOutputBuilder,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in the ".env" file. Since the output amount cannot be zero, the seed must contain
    // non-zero balance.
    dotenvy::dotenv().ok();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];

    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address).await?,
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let basic_output_builder =
        BasicOutputBuilder::new_with_amount(1_000_000).add_unlock_condition(AddressUnlockCondition::new(address));

    let outputs = [
        // most simple output
        basic_output_builder.clone().finish_output(token_supply)?,
        // with metadata feature block
        basic_output_builder
            .clone()
            .add_feature(MetadataFeature::new([13, 37])?)
            .finish_output(token_supply)?,
        // with storage deposit return
        basic_output_builder
            .clone()
            .with_amount(234_100)
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                address,
                234_000,
                token_supply,
            )?)
            .finish_output(token_supply)?,
        // with expiration
        basic_output_builder
            .clone()
            .add_unlock_condition(ExpirationUnlockCondition::new(address, 1)?)
            .finish_output(token_supply)?,
        // with timelock
        basic_output_builder
            .clone()
            .add_unlock_condition(TimelockUnlockCondition::new(1)?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Basic outputs block sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
}
