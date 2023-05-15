// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a basic output with an expiration unlock condition.
//!
//! When the receiver (address in AddressUnlockCondition) doesn't consume the output before it gets expired, the sender
//! (address in ExpirationUnlockCondition) will get the full control back.
//!
//! `cargo run --example expiration --release`

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use iota_sdk::{
    client::{api::GetAddressesOptions, request_funds_from_faucet, secret::SecretManager, Client, Result},
    types::block::output::{
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
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

    let addresses = secret_manager
        .get_raw_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..2))
        .await?;
    let sender_address = addresses[0];
    let receiver_address = addresses[1];

    let token_supply = client.get_token_supply().await?;

    request_funds_from_faucet(&faucet_url, &sender_address.to_bech32(client.get_bech32_hrp().await?)).await?;
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let tomorrow = (SystemTime::now() + Duration::from_secs(24 * 3600))
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_secs()
        .try_into()
        .unwrap();

    let outputs = vec![
        // with storage deposit return
        BasicOutputBuilder::new_with_amount(255_100)
            .add_unlock_condition(AddressUnlockCondition::new(receiver_address))
            // If the receiver does not consume this output, we Unlock after a day to avoid
            // locking our funds forever.
            .add_unlock_condition(ExpirationUnlockCondition::new(sender_address, tomorrow)?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Block with ExpirationUnlockCondition transaction sent: {explorer_url}/block/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
}
