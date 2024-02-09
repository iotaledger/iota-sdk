// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a testnet transaction with a simulated ledger nano hardware wallet.
//!
//! To use the ledger nano simulator, run the following commands:
//! 1. `clone https://github.com/iotaledger/ledger-iota-app`
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

use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{
            ledger_nano::{LedgerOptions, LedgerSecretManager},
            MultiKeyOptions, SecretManageExt,
        },
        Client, Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
};

// const AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create a client instance
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager = LedgerSecretManager::new(true);

    let hrp = client.get_bech32_hrp().await?;

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(&LedgerOptions::new(
            MultiKeyOptions::new(IOTA_COIN_TYPE).with_address_range(0..2),
        ))
        .await?
        .into_iter()
        .map(|a| a.to_bech32(hrp))
        .collect::<Vec<_>>();

    println!("List of generated public addresses:\n{addresses:#?}\n");

    todo!("build a transaction that gets signed with the ledger nano");
    // let block = client
    //     .block()
    //     .with_secret_manager(&secret_manager)
    //     // Insert the output address and amount to spent. The amount cannot be zero.
    //     .with_output(
    //         // We generate an address from our seed so that we send the funds to ourselves
    //         addresses[1],
    //         1_000_000,
    //     )
    //     .await?
    //     .finish()
    //     .await?;

    // println!(
    //     "Block using ledger nano sent: {}/block/{}",
    //     std::env::var("EXPLORER_URL").unwrap(),
    //     block.id()
    // );

    // Ok(())
}
