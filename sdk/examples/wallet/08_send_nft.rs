// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_nft --release
// In this example we will send an nft
// Rename `.env.example` to `.env` first

use std::env;

use dotenv::dotenv;
use iota_sdk::wallet::{account_manager::AccountManager, AddressAndNftId, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create the account manager
    let manager = AccountManager::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = manager.get_account("Alice").await?;
    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first nft
    if let Some(nft_id) = balance.nfts.first() {
        // Set the stronghold password
        manager
            .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let outputs = vec![AddressAndNftId {
            address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
            nft_id: *nft_id,
        }];

        let transaction = account.send_nft(outputs, None).await?;

        // Wait for transaction to get included
        account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
            transaction.transaction_id,
            &env::var("NODE_URL").unwrap(),
            transaction.block_id.expect("no block created yet")
        );
    }

    Ok(())
}
