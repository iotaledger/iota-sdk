// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an NFT.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example send_nft
//! ```

use iota_sdk::{wallet::SendNftParams, Wallet};

// The address to send the tokens to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Create the wallet
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(None).await?;

    // Get the first nft
    if let Some(nft_id) = balance.nfts().first() {
        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let outputs = [SendNftParams::new(RECV_ADDRESS, *nft_id)?];

        println!("Sending NFT '{}' to '{}'...", nft_id, RECV_ADDRESS);

        let transaction = wallet.send_nft(outputs, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        wallet
            .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Tx accepted: {}/transactions/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );
    } else {
        println!("No available NFTs");
    }

    Ok(())
}
