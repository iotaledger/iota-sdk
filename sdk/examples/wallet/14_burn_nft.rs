// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example burn_nft --release
// In this example we will burn an existing nft output
// Rename `.env.example` to `.env` first

use std::str::FromStr;

use iota_sdk::{
    types::block::output::NftId,
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity but should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    let balance = account.balance().await?;
    println!("Balance before burning:\n{balance:?}",);

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Replace with an NftId that is available in the account
    let nft_id = NftId::from_str("0xe192461b30098a5da889ef6abc9e8130bf3b2d980450fa9201e5df404121b932")?;
    let transaction = account.burn_nft(nft_id, None).await?;

    account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let balance = account.sync(None).await?;

    println!("Balance after burning:\n{balance:?}",);

    Ok(())
}
