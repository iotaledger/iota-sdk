// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example burn_nft --release
// In this example we will burn an existing nft output
// Rename `.env.example` to `.env` first

use std::env;

use dotenv::dotenv;
use iota_sdk::wallet::{account_manager::AccountManager, Result};

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
        println!("Balance before burning:\n{balance:?}",);

        // Set the stronghold password
        manager
            .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let transaction = account.burn_nft(*nft_id, None).await?;

        account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        let balance = account.sync(None).await?;

        println!("Balance after burning:\n{balance:?}",);
    }

    Ok(())
}
