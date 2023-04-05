// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example transaction --release
// In this example we will send a transaction
// Rename `.env.example` to `.env` first

use iota_sdk::wallet::{AddressWithAmount, Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity but should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Send a transaction with 1 Mi
    let outputs = vec![AddressWithAmount {
        address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
        amount: 1_000_000,
    }];
    let transaction = account.send_amount(outputs, None).await?;

    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &std::env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}
