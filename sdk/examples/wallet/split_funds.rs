// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example split_funds --release

use std::time::Instant;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    wallet::{ClientOptions, Result, Wallet},
};

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
        .finish()
        .await?;

    // Get account or create a new one
    let account_alias = "logger";
    let account = match wallet.get_account(account_alias.to_string()).await {
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

    let _address = account.generate_addresses(5, None).await?;
    let addresses = account.generate_addresses(300, None).await?;
    let mut bech32_addresses = Vec::new();
    for ad in addresses {
        bech32_addresses.push(ad.address().to_bech32());
    }

    let addresses = account.addresses().await?;
    println!("Addresses: {}", addresses.len());

    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("Syncing took: {:.2?}", now.elapsed());
    println!("Balance: {balance:?}");

    let addresses_with_unspent_outputs = account.addresses_with_unspent_outputs().await?;
    println!("Addresses with balance: {}", addresses_with_unspent_outputs.len());

    let token_supply = account.client().get_token_supply().await?;

    // send transaction
    for chunk in addresses.chunks(100).map(|x| x.to_vec()) {
        let outputs = chunk
            .into_iter()
            .map(|a| {
                BasicOutputBuilder::new_with_amount(1_000_000)
                    .unwrap()
                    .add_unlock_condition(AddressUnlockCondition::new(*a.address().as_ref()))
                    .finish_output(token_supply)
                    .unwrap()
            })
            .collect();
        match account.send(outputs, None).await {
            Ok(tx) => println!(
                "Block sent: {}/api/core/v2/blocks/{}",
                &std::env::var("NODE_URL").unwrap(),
                tx.block_id.expect("no block created yet")
            ),
            Err(e) => println!("{e}"),
        }
    }

    Ok(())
}
