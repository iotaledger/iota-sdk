// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example, we'll demonstrate how to listen to wallet events by sending some amount of base coins to an
//! address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example events --release
//! ```

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

// The account aliases used in this example
const ACCOUNT_ALIAS: &str = "event_account";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";
// The amount of base coins we'll send
const SEND_AMOUNT: u64 = 1_000_000;
// The address we'll be sending coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path(WALLET_DB_PATH)
        .finish()
        .await?;

    wallet
        .listen(vec![], move |event| {
            println!("RECEIVED AN EVENT:\n{:?}", event.event);
        })
        .await;

    // Get or create an account
    let account = if let Ok(account) = wallet.get_account(ACCOUNT_ALIAS).await {
        account
    } else {
        println!("Creating account '{ACCOUNT_ALIAS}'");
        wallet
            .create_account()
            .with_alias(ACCOUNT_ALIAS.to_string())
            .finish()
            .await?
    };

    let balance = account.sync(None).await?;
    println!("Balance BEFORE:\n{:#?}", balance.base_coin());

    // send transaction
    let outputs = vec![
        BasicOutputBuilder::new_with_amount(SEND_AMOUNT)
            .add_unlock_condition(AddressUnlockCondition::new(Address::try_from_bech32(RECV_ADDRESS)?))
            .finish_output(account.client().get_token_supply().await?)?,
    ];

    let transaction = account.send(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let balance = account.sync(None).await?;
    println!("Balance AFTER:\n{:#?}", balance.base_coin());

    Ok(())
}
