// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a testnet transaction with a simulated ledger nano hardware wallet.
//!
//! To use the ledger nano simulator, run the following commands:
//! 1. `clone https://github.com/iotaledger/ledger-shimmer-app`
//! 2. `cd ledger-shimmer-app`
//! 3. `git submodule init && git submodule update --recursive`
//! 4. `./build.sh -m nanos|nanox|nanosplus -s`
//!
//! Then, open the ledger nano web interface at `http://localhost:5000`. You'll have to approve the
//! transaction the same way as you would with a regular ledger nano device.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example ledger_nano_transaction
//! ```

use iota_sdk::client::{
    api::GetAddressesOptions,
    secret::{ledger_nano::LedgerSecretManager, SecretManager},
    Client, Result,
};

const AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a client instance
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
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

    println!("List of generated public addresses:\n{addresses:#?}\n");

    let recv_address = addresses[1];

    println!("Sending {AMOUNT} to {recv_address}");
    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_output(recv_address, AMOUNT)
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
