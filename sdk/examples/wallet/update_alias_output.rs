// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will update the state metadata of an account output.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example update_account_output
//! ```

use std::{env::var, str::FromStr, time::Instant};

use iota_sdk::{
    types::block::output::{AccountId, AccountOutputBuilder, Output},
    wallet::{Account, Result},
    Wallet,
};

// Replace with an account id held in an unspent output of the account
const ACCOUNT_ID: &str = "0xc94fc4d280d63c7de09c8cc49ecefba6192e104d200ab7472db9e943e0feef7c";
// The metadata for the next state
const NEW_STATE_METADATA: &str = "updated state metadata 1";

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    sync_and_print_balance(&account).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Get the account output by its account id
    let account_id = AccountId::from_str(ACCOUNT_ID)?;
    if let Some(account_output_data) = account.unspent_account_output(&account_id).await? {
        println!(
            "Alias '{ACCOUNT_ID}' found in unspent output: '{}'",
            account_output_data.output_id
        );

        let token_supply = account.client().get_token_supply().await?;
        let rent_structure = account.client().get_rent_structure().await?;

        let account_output = account_output_data.output.as_account();
        let updated_account_output = AccountOutputBuilder::from(account_output)
            // Update the account id, as it might still be null
            .with_account_id(account_output.account_id_non_null(&account_output_data.output_id))
            // Minimum required storage deposit will change if the new metadata has a different size, so we will update
            // the amount
            .with_minimum_storage_deposit(rent_structure)
            .with_state_metadata(NEW_STATE_METADATA.as_bytes().to_vec())
            .with_state_index(account_output.state_index() + 1)
            .finish_output(token_supply)?;

        println!("Sending transaction...",);
        send_and_wait_for_inclusion(&account, vec![updated_account_output]).await?;
    } else {
        panic!("account doesn't exist or is not unspent");
    }

    println!("Example finished successfully");
    Ok(())
}

async fn sync_and_print_balance(account: &Account) -> Result<()> {
    let alias = account.alias().await;
    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("{alias}'s account synced in: {:.2?}", now.elapsed());
    println!("{alias}'s base coin balance:\n{:#?}", balance.base_coin());
    println!("{alias}'s accounts:\n{:#?}", balance.accounts());
    Ok(())
}

async fn send_and_wait_for_inclusion(account: &Account, outputs: Vec<Output>) -> Result<()> {
    let transaction = account.send(outputs, None).await?;
    println!(
        "Transaction sent: {}/transaction/{}",
        var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
