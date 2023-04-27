// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will spam transactions from multiple threads simultaneously to our own address.
//!
//! `cargo run --example threads --release`

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
    let account_alias = "thread_account";
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

    // One address gets generated during account creation
    let address = account.addresses().await?[0].address().clone();
    println!("{}", address);

    let balance = account.sync(None).await?;
    println!("Balance: {balance:?}");

    if balance.base_coin().available() == 0 {
        panic!("Account has no available balance");
    }

    for _ in 0..1000 {
        let mut threads = Vec::new();
        for n in 0..10 {
            let account_ = account.clone();
            let address_ = *address.as_ref();

            threads.push(async move {
                tokio::spawn(async move {
                    // send transaction
                    let outputs = vec![
                        BasicOutputBuilder::new_with_amount(1_000_000)
                            .add_unlock_condition(AddressUnlockCondition::new(address_))
                            .finish_output(account_.client().get_token_supply().await?)?;
                        // amount of outputs in the transaction (one additional output might be added for the remaining amount)
                        1
                    ];
                    let transaction = account_.send(outputs, None).await?;
                    println!(
                        "Block from thread {} sent: {}/transaction/{}",
                        n,
                        &std::env::var("EXPLORER_URL").unwrap(),
                        transaction.transaction_id
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
                // Sync when getting an error, because that's probably when no outputs are available anymore
                println!("Syncing account...");
                account.sync(None).await?;
            }
        }
    }
    Ok(())
}
