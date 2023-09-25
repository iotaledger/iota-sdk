// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will update the state controller of an account output.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example governance_transition
//! ```

use iota_sdk::{
    types::block::output::{
        unlock_condition::StateControllerAddressUnlockCondition, AccountOutputBuilder, UnlockCondition,
    },
    wallet::Result,
    Wallet,
};

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

    // May want to ensure the wallet is synced before sending a transaction.
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

    // Generate a new address, which will be the new state controller
    let new_state_controller = wallet.generate_ed25519_address(None).await?;

    let token_supply = wallet.client().get_token_supply().await?;

    let account_output = account_output_data.output.as_account();
    let updated_account_output = AccountOutputBuilder::from(account_output)
        .replace_unlock_condition(UnlockCondition::StateControllerAddress(
            StateControllerAddressUnlockCondition::new(new_state_controller),
        ))
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
