// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_micro_transaction --release
// In this example we will send an amount below the minimum storage deposit
// Rename `.env.example` to `.env` first

use std::env;

use dotenv::dotenv;
use iota_sdk::wallet::{AddressWithAmount, Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let outputs = vec![AddressWithAmount::new(
        "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
        1,
    )];

    let transaction = account.send_amount(outputs, None).await?;

    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}
