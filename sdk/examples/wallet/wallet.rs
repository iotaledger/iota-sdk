// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example wallet --release

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    wallet::{AddressWithAmount, ClientOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

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

    let _address = account.generate_addresses(5, None).await?;

    let addresses = account.addresses().await?;
    println!("Addresses: {}", addresses.len());

    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("Syncing took: {:.2?}", now.elapsed());
    println!("Balance: {balance:?}");

    let addresses_with_unspent_outputs = account.addresses_with_unspent_outputs().await?;
    println!("Addresses with balance: {}", addresses_with_unspent_outputs.len());

    // send transaction
    let outputs = vec![AddressWithAmount {
        address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
        amount: 1_000_000,
    }];
    let tx = account.send_amount(outputs, None).await?;
    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        tx.transaction_id,
        &std::env::var("NODE_URL").unwrap(),
        tx.block_id.expect("no block created yet")
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
