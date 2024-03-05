// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to destroy the first foundry there is in the wallet. This is only possible if its
//! circulating supply is 0 and no native tokens were burned.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example destroy_foundry
//! ```

use iota_sdk::{types::block::output::TokenId, Wallet};

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

    let foundry_count = balance.foundries().len();
    println!("Foundries before destroying: {foundry_count}");

    // We try to destroy the first foundry in the wallet
    if let Some(foundry_id) = balance.foundries().first() {
        let token_id = TokenId::from(*foundry_id);

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // Find the native tokens balance for this foundry if one exists.
        let native_tokens = balance.native_tokens().get(&token_id);
        if let Some(native_token) = native_tokens {
            let output = wallet.get_foundry_output(token_id).await?;
            // Check if all tokens are melted.
            if native_token.available() != output.as_foundry().token_scheme().as_simple().circulating_supply() {
                // We are not able to melt all tokens, because we don't own them or they are not unlocked.
                println!("We don't own all remaining tokens, aborting foundry destruction.");
                return Ok(());
            }

            println!("Melting remaining tokens..");
            // Melt all tokens so we can destroy the foundry.
            let transaction = wallet
                .melt_native_token(token_id, native_token.available(), None)
                .await?;
            println!("Transaction sent: {}", transaction.transaction_id);

            wallet
                .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
                .await?;

            println!(
                "Tx accepted: {}/transactions/{}",
                std::env::var("EXPLORER_URL").unwrap(),
                transaction.transaction_id
            );

            // Sync to make the foundry output available again, because it was used in the melting transaction.
            wallet.sync(None).await?;
        }
        println!("Destroying foundry..");

        let transaction = wallet.burn(*foundry_id, None).await?;

        println!("Transaction sent: {}", transaction.transaction_id);

        wallet
            .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Tx accepted: {}/transactions/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );

        // Resync to update the foundries list.
        let balance = wallet.sync(None).await?;

        let foundry_count = balance.foundries().len();
        println!("Foundries after destroying: {foundry_count}");
    } else {
        println!("No Foundry available in the wallet");
    }

    Ok(())
}
