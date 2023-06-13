// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we'll demonstrate how to listen to wallet events by sending some amount of base coins to an
//! address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example events
//! ```

use std::env::var;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    },
    wallet::{ClientOptions, Result, Wallet},
};

// The amount of base coins we'll send
const SEND_AMOUNT: u64 = 1_000_000;
// The address we'll be sending coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    wallet
        .listen([], move |event| {
            println!("RECEIVED AN EVENT:\n{:?}", event.event);
        })
        .await;

    // Get or create an account
    let alias = "Alice";
    let account = if let Ok(account) = wallet.get_account(alias).await {
        account
    } else {
        println!("Creating account '{alias}'");
        wallet.create_account().with_alias(alias.to_string()).finish().await?
    };

    let balance = account.sync(None).await?;
    println!("Balance BEFORE:\n{:#?}", balance.base_coin());

    // send transaction
    let outputs = [BasicOutputBuilder::new_with_amount(SEND_AMOUNT)
        .add_unlock_condition(AddressUnlockCondition::new(Address::try_from_bech32(RECV_ADDRESS)?))
        .finish_output(account.client().get_token_supply().await?)?];

    let transaction = account.send(outputs, None).await?;
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
    println!("Balance AFTER:\n{:#?}", balance.base_coin());

    Ok(())
}
