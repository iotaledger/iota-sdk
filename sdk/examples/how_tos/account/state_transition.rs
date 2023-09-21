// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will update the state metadata of an account output.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example state_transition
//! ```

use iota_sdk::{types::block::output::AccountOutputBuilder, wallet::Result, Wallet};

// The metadata for the next state
const NEW_STATE_METADATA: &str = "updated state metadata 1";

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_alias("Alice")
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = wallet.sync(None).await?;

    // Get the first account
    let account_id = balance
        .accounts()
        .first()
        .expect("No account output available in the account.");

    let account_output_data = wallet
        .unspent_account_output(account_id)
        .await?
        .expect("account not found in unspent outputs");
    println!(
        "Account '{account_id}' found in unspent output: '{}'",
        account_output_data.output_id
    );

    let token_supply = wallet.client().get_token_supply().await?;
    let rent_structure = wallet.client().get_rent_structure().await?;

    let account_output = account_output_data.output.as_account();
    let updated_account_output = AccountOutputBuilder::from(account_output)
        // Minimum required storage deposit will change if the new metadata has a different size, so we will update
        // the amount
        .with_minimum_storage_deposit(rent_structure)
        .with_state_metadata(NEW_STATE_METADATA.as_bytes().to_vec())
        .with_state_index(account_output.state_index() + 1)
        .finish_output(token_supply)?;

    println!("Sending transaction...",);
    let transaction = wallet.send_outputs(vec![updated_account_output], None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    Ok(())
}
