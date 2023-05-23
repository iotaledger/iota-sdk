// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send basic outputs with different feature blocks.
//!
//! `cargo run --example basic --release`

use iota_sdk::{
    client::{secret::SecretManager, utils::request_funds_from_faucet, Client, Result},
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

    let node_url = std::env::var("NODE_URL").unwrap();
    let explorer_url = std::env::var("EXPLORER_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(&faucet_url, &address.to_bech32(client.get_bech32_hrp().await?)).await?
    );

    let basic_output_builder =
        BasicOutputBuilder::new_with_amount(1_000_000).add_unlock_condition(AddressUnlockCondition::new(address));

    let outputs = vec![
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

    println!("Basic outputs block sent: {explorer_url}/block/{}", block.id());
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
}
