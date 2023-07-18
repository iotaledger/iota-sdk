// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a basic output with an expiration unlock condition.
//!
//! When the receiver (address in AddressUnlockCondition) doesn't consume the output before it gets expired, the sender
//! (address in ExpirationUnlockCondition) will get the full control back.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example expiration
//! ```

use iota_sdk::{
    client::{api::GetAddressesOptions, request_funds_from_faucet, secret::SecretManager, Client, Result},
    types::block::output::{
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        BasicOutputBuilder,
    },
};

const AMOUNT: u64 = 255_100;

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

    let addresses = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..2))
        .await?;
    let sender_address = addresses[0];
    let receiver_address = addresses[1];

    let token_supply = client.get_token_supply().await?;

    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &sender_address).await?,
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let tomorrow = (std::time::SystemTime::now() + std::time::Duration::from_secs(24 * 3600))
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock went backwards")
        .as_secs()
        .try_into()
        .unwrap();

    let outputs = [
        // with storage deposit return
        BasicOutputBuilder::new_with_amount(AMOUNT)
            .add_unlock_condition(AddressUnlockCondition::new(receiver_address))
            // If the receiver does not consume this output, we Unlock after a day to avoid
            // locking our funds forever.
            .add_unlock_condition(ExpirationUnlockCondition::new(sender_address, tomorrow)?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Block with ExpirationUnlockCondition transaction sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
}
