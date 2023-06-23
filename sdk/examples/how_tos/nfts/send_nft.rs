// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an nft.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example send_nft --release`

use std::env::var;

use iota_sdk::{
    wallet::{Result, SendNftParams},
    Wallet,
};

// The address to send the tokens to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first nft
    if let Some(nft_id) = balance.nfts().first() {
        // Set the stronghold password
        wallet
            .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let outputs = [SendNftParams::new(RECV_ADDRESS, *nft_id)?];

        println!("Sending NFT '{}' to '{}'...", nft_id, RECV_ADDRESS);

        let transaction = account.send_nft(outputs, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        // Wait for transaction to get included
        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;

        println!("Block included: {}/block/{}", var("EXPLORER_URL").unwrap(), block_id);
    } else {
        println!("No available NFTs");
    }

    Ok(())
}
