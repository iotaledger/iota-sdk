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

use iota_sdk::client::{
    api::GetAddressesOptions,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{ledger_nano::LedgerSecretManager, SecretManager},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let ledger_nano = LedgerSecretManager::new(true);

    println!("{:?}", ledger_nano.get_ledger_nano_status().await);

    let secret_manager = SecretManager::LedgerNano(ledger_nano);

    // Generate addresses with custom account index and range
    let addresses = secret_manager
        .generate_ed25519_addresses_as_bech32(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_account_index(0)
                .with_range(0..2),
        )
        .await?;

    println!("List of generated public addresses:\n{addresses:?}\n");

    Ok(())
}
