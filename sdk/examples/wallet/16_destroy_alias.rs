// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will destroy an existing alias output. This is only possible if possible foundry outputs have
//! circulating supply of 0. Rename `.env.example` to `.env` first.
//!
//! `cargo run --example destroy_alias --release`

use std::str::FromStr;

use iota_sdk::{
    types::block::output::AliasId,
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    let balance = account.balance().await?;
    println!("Balance before destroying:\n{balance:?}",);

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Replace with an AliasId that is available in the account
    let alias_id = AliasId::from_str("0x57f1bafae0ef43190597a0dfe72ef1477b769560203c1854c6fb427c486e6530")?;
    let transaction = account.destroy_alias(alias_id, None).await?;

    account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let balance = account.sync(None).await?;

    println!("Balance after destroying:\n{balance:?}",);

    Ok(())
}
