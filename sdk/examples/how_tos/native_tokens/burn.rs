// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will burn an existing native token, this will not increase the melted supply in the foundry,
//! therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
//! output that minted it.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! You may provide a TOKEN_ID that is available in the wallet. You can check this by running the
//! `get_balance` example. You can create a new native token by running the `create_native_token` example.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example burn_native_token [TOKEN_ID]
//! ```

use iota_sdk::{
    types::block::output::{NativeToken, TokenId},
    Wallet, U256,
};

// The minimum available native token amount to search for in the wallet
const MIN_AVAILABLE_AMOUNT: u64 = 11;
// The amount of the native token to burn
const BURN_AMOUNT: u64 = 1;

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

    // Take the given token id, or use a default.
    let token_id = std::env::args()
        .nth(1)
        .map(|s| s.parse::<TokenId>().expect("invalid token id"))
        .unwrap_or_else(|| TokenId::from(*balance.foundries().first().unwrap()));

    if let Some(native_token_balance) = balance
        .native_tokens()
        .get(&token_id)
        .filter(|native_token| native_token.available() >= U256::from(MIN_AVAILABLE_AMOUNT))
    {
        println!("Balance before burning: {native_token_balance:#?}");

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // Burn a native token
        let transaction = wallet.burn(NativeToken::new(token_id, BURN_AMOUNT)?, None).await?;
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

        print!("Balance after burning: ");
        if let Some(native_token_balance) = balance.native_tokens().get(&token_id) {
            println!("{native_token_balance:#?}");
        } else {
            println!("No remaining tokens");
        }
    } else {
        println!(
            "Native token '{token_id}' doesn't exist or there's not at least '{MIN_AVAILABLE_AMOUNT}' tokens of it in the wallet"
        );
    }

    Ok(())
}
