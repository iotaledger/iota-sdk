// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint an existing native token with its foundry.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! You may provide a TOKEN_ID that is available in the wallet. The foundry
//! output which minted it needs to be available as well. You can check this by
//! running the `get_balance` example. You can create a new native token by running
//! the `create_native_token` example.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_native_token [TOKEN_ID]
//! ```

use iota_sdk::{types::block::output::TokenId, Wallet};

// The amount of native tokens to mint
const MINT_AMOUNT: u64 = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(None).await?;

    // Find first foundry and corresponding token id
    let token_id = std::env::args()
        .nth(1)
        .map(|s| s.parse::<TokenId>().expect("invalid token id"))
        .unwrap_or_else(|| TokenId::from(*balance.foundries().first().unwrap()));

    if let Some(native_token_balance) = balance.native_tokens().get(&token_id) {
        let available_balance = native_token_balance.available();
        println!("Balance before minting: {available_balance}");
    } else {
        println!("Couldn't find native token '{token_id}' in the wallet");
        return Ok(());
    }

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Mint some more native tokens
    let transaction = wallet.mint_native_token(token_id, MINT_AMOUNT, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    let balance = wallet.sync(None).await?;
    let available_balance = balance.native_tokens().get(&token_id).unwrap().available();
    println!("Balance after minting: {available_balance:?}",);

    Ok(())
}
