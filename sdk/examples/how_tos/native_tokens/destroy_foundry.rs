// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to destroy the first foundry there is in the account. This is only possible if its
//! circulating supply is 0 and no native tokens were burned.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example destroy_foundry
//! ```

use iota_sdk::{
    types::block::output::TokenId,
    wallet::{account::types::NativeTokensBalance, Result},
    Wallet, U256,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let alias = "Alice";
    let account = wallet.get_account(alias).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // We try to destroy the first foundry in the account
    if let Some(foundry_id) = balance.foundries().first() {
        let foundry_count = balance.foundries().len();
        println!("Foundries before destroying: {foundry_count}");

        let token_id = TokenId::from(*foundry_id);

        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // Check if all tokens are melted.
        let native_tokens: Vec<&NativeTokensBalance> = balance
            .native_tokens()
            .iter()
            .filter(|native_token| *native_token.token_id() == token_id)
            .collect();
        if native_tokens.len() > 0 {
            let output = account.get_foundry_output(token_id).await?;
            let total: U256 = native_tokens
                .clone()
                .iter()
                .map(|b| b.available())
                .fold(U256::zero(), |a, b| a + b);
            if total != output.as_foundry().token_scheme().as_simple().circulating_supply() {
                // We are not able to melt all tokens, because we dont own them or they are not unlocked.
                println!("We dont own all remaining tokens, aborting foundry destruction.");
                return Ok(());
            }

            println!("Melting remaining tokens..");
            for native_token_balance in native_tokens {
                // Melt all tokens so we can destroy the foundry.
                let tx = account
                    .melt_native_token(*native_token_balance.token_id(), native_token_balance.available(), None)
                    .await?;
                println!("Transaction sent: {}", transaction.transaction_id);

                account
                    .retry_transaction_until_included(&tx.transaction_id, None, None)
                    .await?;
                println!(
                    "Block included: {}/block/{}",
                    std::env::var("EXPLORER_URL").unwrap(),
                    block_id
                );
            }

            // Update account so input selection works instead of throwing `UnfulfillableRequirement`.
            account.sync(None).await?;
        }
        println!("Destroying foundry..");

        let transaction = account.prepare_burn(*foundry_id, None).await?;

        let transaction = account.sign_and_submit_transaction(transaction, None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        println!(
            "Block included: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );

        // Resync to update the foundries list.
        let balance = account.sync(None).await?;
        let foundry_count = balance.foundries().len();
        println!("Foundries after destroying: {foundry_count}");
    } else {
        println!("No Foundry available in account '{alias}'");
    }

    Ok(())
}
