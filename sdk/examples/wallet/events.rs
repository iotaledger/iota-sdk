// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --example events --features=events --release`

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

    wallet
        .listen(vec![], move |event| {
            println!("Received an event {event:?}");
        })
        .await;

    // Get account or create a new one
    let account_alias = "event_account";
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

    let balance = account.sync(None).await?;
    println!("Balance: {balance:?}");

    // send transaction
    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(Address::try_from_bech32(
                "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
            )?))
            .finish_output(account.client().get_token_supply().await?)?,
    ];

    let transaction = account.send(outputs, None).await?;

    println!("Transaction: {}", transaction.transaction_id);
    println!(
        "Block sent: {}/api/core/v2/blocks/{}",
        &std::env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );
    Ok(())
}
