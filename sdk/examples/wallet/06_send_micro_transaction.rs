// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an amount below the minimum storage deposit.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example send_micro_transaction --release
//! ```

use iota_sdk::wallet::{account::TransactionOptions, AddressWithAmount, Result, Wallet};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The base coin micro amount to send
const SEND_MICRO_AMOUNT: u64 = 1;
// The address to send the coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    // May want to ensure the account is synced before sending a transaction.
    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    println!("Sending '{}' coins to '{}'...", SEND_MICRO_AMOUNT, RECV_ADDRESS);

    // Send a micro transaction
    let outputs = vec![AddressWithAmount::new(RECV_ADDRESS.to_string(), SEND_MICRO_AMOUNT)];

    let transaction = account
        .send_amount(
            outputs,
            TransactionOptions {
                allow_micro_amount: true,
                ..Default::default()
            },
        )
        .await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
