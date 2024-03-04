// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing nft output.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example burn_nft
//! ```

use iota_sdk::Wallet;

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
        let nfts_before = balance.nfts();
        println!("Balance before burning:\n{nfts_before:#?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let transaction = wallet.burn(*nft_id, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        wallet
            .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Tx accepted: {}/transactions/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );

        println!("Burned NFT '{}'", nft_id);

        let balance = wallet.sync(None).await?;
        let nfts_after = balance.nfts();
        println!("Balance after burning:\n{nfts_after:#?}",);
    } else {
        println!("No NFT available in the wallet");
    }

    Ok(())
}
