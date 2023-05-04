// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will generate an address for an already exisiting wallet.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example generate_address`
//! ```

use iota_sdk::wallet::{Result, Wallet};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE: u32 = 1;
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    // Provide the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Generate some addresses
    let addresses = account.generate_addresses(NUM_ADDRESSES_TO_GENERATE, None).await?;

    println!("NEW ADDRESSES:");
    let explorer_url = std::env::var("EXPLORER_URL").ok();
    let prepended = explorer_url.map(|url| format!("{url}/addr/")).unwrap_or_default();
    for address in account.addresses().await? {
        if addresses.contains(&address) {
            println!(" - {prepended}{}", address.address());
        }
    }
    Ok(())
}
