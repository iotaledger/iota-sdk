// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing nft output.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example burn_nft
//! ```

use std::env::var;

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let alias = var("ACCOUNT_ALIAS_1").unwrap();
    let account = wallet.get_account(&alias).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first nft
    if let Some(nft_id) = balance.nfts().first() {
        println!("Balance BEFORE burning:\n{balance:?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(&var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Sending NFT burn transaction...");

        let transaction = account.burn(*nft_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Transaction included: {}/block/{}",
            var("EXPLORER_URL").unwrap(),
            block_id
        );

        let balance = account.sync(None).await?;

        println!("Balance AFTER burning:\n{balance:?}",);
    } else {
        println!("No NFT available in account '{alias}'");
    }

    Ok(())
}
