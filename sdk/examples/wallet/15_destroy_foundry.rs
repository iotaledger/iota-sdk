// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will try to destroy the first foundry there is in the account. This is only possible if its
//! circulating supply is 0 and no native tokens were burned.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example destroy_foundry
//! ```

use std::env::var;

use iota_sdk::wallet::{Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account(&var("ACCOUNT_ALIAS_1").unwrap()).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // We try to destroy the first foundry in the account
    if let Some(foundry_id) = balance.foundries().first() {
        let foundries_before = balance.foundries();
        println!("Foundries BEFORE destroying:\n{foundries_before:#?}",);

        // Set the stronghold password
        wallet
            .set_stronghold_password(&var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        println!("Sending foundry burn transaction...");

        let transaction = account.burn(*foundry_id, None).await?;
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
        let foundries_after = balance.foundries();
        println!("Foundries AFTER destroying:\n{foundries_after:#?}",);
    }

    Ok(())
}
