// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example ping --release

// In this example we will try to send transactions from multiple threads simultaneously to the first 1000 addresses of
// the second account (pong_account)

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::block::output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder,
    },
    wallet::{account_manager::AccountManager, ClientOptions, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let manager = AccountManager::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("pingdb")
        .finish()
        .await?;

    // Get account or create a new one
    let account_alias = "ping";
    let ping_account = match manager.get_account(account_alias.to_string()).await {
        Ok(account) => account,
        _ => {
            // first we'll create an example account and store it
            manager
                .create_account()
                .with_alias(account_alias.to_string())
                .finish()
                .await?
        }
    };
    let account_alias = "pong";
    let pong_account = match manager.get_account(account_alias.to_string()).await {
        Ok(account) => account,
        _ => {
            // first we'll create an example account and store it
            manager
                .create_account()
                .with_alias(account_alias.to_string())
                .finish()
                .await?
        }
    };

    let amount_addresses = 5;
    // generate addresses so we find all funds
    if ping_account.addresses().await?.len() < amount_addresses {
        ping_account
            .generate_addresses((amount_addresses - ping_account.addresses().await?.len()) as u32, None)
            .await?;
    }
    let balance = ping_account.sync(None).await?;
    println!("Balance: {balance:?}");
    // generate addresses from the second account to which we will send funds
    let pong_addresses = {
        let mut addresses = pong_account.addresses().await?;
        if addresses.len() < amount_addresses {
            addresses = pong_account
                .generate_addresses((amount_addresses - addresses.len()) as u32, None)
                .await?
        };
        println!(
            "{}",
            request_funds_from_faucet(
                &std::env::var("FAUCET_URL").unwrap(),
                &addresses[0].address().to_bech32()
            )
            .await?
        );
        addresses
    };

    for address_index in 0..1000 {
        let mut threads = Vec::new();
        for n in 1..4 {
            let ping_account_ = ping_account.clone();
            let pong_addresses_ = pong_addresses.clone();
            threads.push(async move {
                tokio::spawn(async move {
                    // send transaction
                    let outputs = vec![
                        // send one or two Mi for more different transactions
                        BasicOutputBuilder::new_with_amount(n * 1_000_000)?
                            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                                *pong_addresses_[address_index % amount_addresses].address().as_ref(),
                            )))
                            .finish_output(ping_account_.client().get_token_supply().await?)?,
                    ];
                    let tx = ping_account_.send(outputs, None).await?;
                    println!(
                        "Block from thread {} sent: {}/api/core/v2/blocks/{}",
                        n,
                        &std::env::var("NODE_URL").unwrap(),
                        tx.block_id.expect("no block created yet")
                    );
                    iota_sdk::wallet::Result::Ok(n)
                })
                .await
            });
        }

        let results = futures::future::try_join_all(threads).await?;
        for thread in results {
            if let Err(e) = thread {
                println!("{e}");
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    // wait until user press enter so background tasks keep running
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Ok(())
}
