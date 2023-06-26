// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing nft output.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --example burn_nft`

use std::env::var;

use iota_sdk::{wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let alias = "Alice";
    let account = wallet.get_account(alias).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first nft
    if let Some(nft_id) = balance.nfts().first() {
        let nfts_before = balance.nfts();
        println!("Balance before burning:\n{nfts_before:#?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let transaction = account.burn(*nft_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!("Block included: {}/block/{}", var("EXPLORER_URL").unwrap(), block_id);

        println!("Burned NFT '{}'", nft_id);

        let balance = account.sync(None).await?;
        let nfts_after = balance.nfts();
        println!("Balance after burning:\n{nfts_after:#?}",);
    } else {
        println!("No NFT available in account '{alias}'");
    }

    Ok(())
}
