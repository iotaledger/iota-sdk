// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --example wallet --release`

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::address::Bech32Address,
    wallet::{ClientOptions, Result, SendAmountParams, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get account or create a new one
    let account_alias = "logger";
    let account = match wallet.get_account(account_alias).await {
        Ok(account) => account,
        _ => {
            // first we'll create an example account and store it
            wallet
                .create_account()
                .with_alias(account_alias.to_string())
                .finish()
                .await?
        }
    };

    // let accounts = wallet.get_accounts().await?;
    // println!("Accounts: {:?}", accounts);

    let _address = account.generate_ed25519_addresses(5, None).await?;

    let addresses = account.addresses().await?;
    println!("Addresses: {}", addresses.len());

    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("Syncing took: {:.2?}", now.elapsed());
    println!("Balance: {balance:?}");

    let addresses_with_unspent_outputs = account.addresses_with_unspent_outputs().await?;
    println!("Addresses with balance: {}", addresses_with_unspent_outputs.len());

    // send transaction
    let outputs = vec![SendAmountParams::new(
        Bech32Address::try_from_str("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")?,
        1_000_000,
    )];
    let transaction = account.send_amount(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("Syncing took: {:.2?}", now.elapsed());
    println!("Balance: {balance:?}");

    // // switch to mainnet
    // let client_options = ClientOptions::new()
    //     .with_node("https://chrysalis-nodes.iota.org/")?;
    // manager.set_client_options(client_options).await?;
    // let now = Instant::now();
    // manager.sync(None).await?;
    // println!("Syncing took: {:.2?}", now.elapsed());
    // println!("Balance: {:?}", account.balance().await?);

    // // switch back to testnet
    // let client_options = ClientOptions::new()
    //     .with_node(&std::env::var("NODE_URL").unwrap())?;
    // manager.set_client_options(client_options).await?;
    // let now = Instant::now();
    // manager.sync(None).await?;
    // println!("Syncing took: {:.2?}", now.elapsed());
    // println!("Balance: {:?}", account.balance().await?);

    Ok(())
}
