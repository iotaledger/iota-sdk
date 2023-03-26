// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example destroy_foundry --release
// In this example we will destroy an existing foundry output. This is only possible if its circulating supply is 0.
// Rename `.env.example` to `.env` first

use std::{env, str::FromStr};

use dotenv::dotenv;
use iota_sdk::{
    types::block::output::FoundryId,
    wallet::{account_manager::AccountManager, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create the account manager
    let manager = AccountManager::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = manager.get_account("Alice").await?;

    let balance = account.balance().await?;
    println!("Balance before destroying:\n{balance:?}",);

    // Set the stronghold password
    manager
        .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Replace with an FoundryId that is available in the account
    let foundry_id =
        FoundryId::from_str("0x0857f1bafae0ef43190597a0dfe72ef1477b769560203c1854c6fb427c486e65300100000000")?;
    let transaction = account.destroy_foundry(foundry_id, None).await?;

    account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let balance = account.sync(None).await?;

    println!("Balance after destroying:\n{balance:?}",);

    Ok(())
}
