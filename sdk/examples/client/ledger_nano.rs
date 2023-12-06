// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create testnet addresses with a simulated ledger nano hardware wallet.
//!
//! To use the ledger nano simulator, run the following commands:
//! 1. `clone https://github.com/iotaledger/ledger-iota-app`
//! 2. `cd ledger-shimmer-app`
//! 3. `git submodule init && git submodule update --recursive`
//! 4. `./build.sh -m nanos|nanox|nanosplus -s`
//!
//! Then, open the ledger nano web interface at `http://localhost:5000`.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example ledger_nano
//! ```

use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP},
        secret::{
            ledger_nano::{LedgerOptions, LedgerSecretManager},
            MultiKeyOptions, SecretManageExt,
        },
        Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let secret_manager = LedgerSecretManager::new(true);

    println!("{:?}", secret_manager.get_ledger_nano_status().await);

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(&LedgerOptions::new(
            MultiKeyOptions::new(IOTA_COIN_TYPE).with_address_range(0..2),
        ))
        .await?
        .into_iter()
        .map(|a| a.to_bech32(IOTA_TESTNET_BECH32_HRP))
        .collect::<Vec<_>>();

    println!("List of generated public addresses:\n{addresses:?}\n");

    Ok(())
}
