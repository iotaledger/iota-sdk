// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will generate addresses for an already existing wallet.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example create_address`
//! ```

use iota_sdk::{wallet::Result, Wallet};

// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 5;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_alias("Alice")
        .finish()
        .await?;

    // Provide the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let explorer_url = std::env::var("EXPLORER_URL").ok();
    let address_url = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();

    println!("Current addresses:");
    for address in wallet.addresses().await? {
        println!(" - {address_url}{}", address.address());
    }

    // Generate some addresses
    let new_addresses = wallet
        .generate_ed25519_addresses(NUM_ADDRESSES_TO_GENERATE, None)
        .await?;
    println!("Generated {} new addresses:", new_addresses.len());
    let account_addresses = wallet.addresses().await?;
    for new_address in new_addresses.iter() {
        assert!(account_addresses.contains(new_address));
        println!(" - {address_url}{}", new_address.address());
    }
    Ok(())
}
