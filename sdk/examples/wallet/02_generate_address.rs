// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will generate an address for an already existing wallet.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example generate_address`
//! ```

use std::env::var;

use iota_sdk::wallet::{Result, Wallet};

// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 2;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account(&var("ACCOUNT_ALIAS_1").unwrap()).await?;

    // Provide the stronghold password
    wallet
        .set_stronghold_password(&var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let explorer_url = var("EXPLORER_URL").ok();
    let prepended = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();

    println!("Current addresses:");
    for address in account.addresses().await? {
        println!(" - {prepended}{}", address.address());
    }

    // Generate some addresses
    let addresses = account.generate_addresses(NUM_ADDRESSES_TO_GENERATE, None).await?;
    println!("Generated {} new addresses:", addresses.len());
    let account_addresses = account.addresses().await?;
    for address in addresses.iter() {
        if account_addresses.contains(address) {
            println!(" - {prepended}{}", address.address());
        } else {
            unreachable!("this should never happen");
        }
    }
    Ok(())
}
