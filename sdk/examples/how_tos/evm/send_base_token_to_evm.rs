// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we send base tokens to L2 EVM.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_base_token_to_evm
//! ```

use iota_sdk::{
    types::block::{
        address::Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, SimpleTokenScheme, TokenScheme,
        },
    },
    wallet::{Result},
    Wallet,
};

// The base coin amount to send
const SEND_AMOUNT: u64 = 1_000_000;
// The address to send the coins to
const RECV_ADDRESS: &str = "0x...";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // Sync and get the balance
    let balance = account.sync(None).await?;
    let funds_available = balance.base_coin().available();
    println!("Balance:: {funds_available}");

    // Get first address from account
    let account_address = &account.addresses().await?[0];
    let bech32_address = account_address.address();
    println!("{bech32_address}");
    // let address = Address::try_from_bech32(bech32_address)?;
    // println!("{address}");

    Ok(())
}
