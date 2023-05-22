// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: Example description
//!
//! `cargo run --example participation --features=participation --release`

use std::str::FromStr;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        node_manager::node::Node,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    types::api::plugins::participation::types::ParticipationEventId,
    wallet::{account::types::participation::ParticipationEventRegistrationOptions, ClientOptions, Result, Wallet},
    Url,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Generates a wallet.log file with logs for debugging.
    let logger_output_config = fern_logger::LoggerOutputConfigBuilder::new()
        .name("wallet.log")
        .target_exclusions(&["h2", "hyper", "rustls"])
        .level_filter(log::LevelFilter::Debug);
    let config = fern_logger::LoggerConfig::build()
        .with_output(logger_output_config)
        .finish();
    fern_logger::logger_init(config).unwrap();

    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let client_options = ClientOptions::new()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .with_ignore_node_health();

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get account or create a new one.
    let account_alias = "participation";
    let account = match wallet.get_account(account_alias).await {
        Ok(account) => account,
        _ => {
            // First we'll create an example account and store it.
            wallet
                .create_account()
                .with_alias(account_alias.to_string())
                .finish()
                .await?
        }
    };

    let event_id =
        ParticipationEventId::from_str("0x80f57f6368933b61af9b3d8e1b152cf5d23bf4537f6362778b0a7302a7000d48")?;
    let node = Node {
        url: Url::parse("http://localhost:14265").map_err(iota_sdk::client::Error::Url)?,
        auth: None,
        disabled: false,
    };
    account
        .register_participation_events(&ParticipationEventRegistrationOptions {
            node,
            events_to_ignore: Some(vec![event_id]),
            events_to_register: None,
        })
        .await?;

    let registered_participation_events = account.get_participation_events().await?;

    println!("registered events: {registered_participation_events:?}");

    let event = account.get_participation_event(event_id).await;
    println!("event: {event:?}");

    let event_status = account.get_participation_event_status(&event_id).await?;
    println!("event status: {event_status:?}");

    // account.deregister_participation_event(event_id).await?;
    // let registered_participation_events = account.get_participation_events().await?;
    // println!("registered events: {registered_participation_events:?}");

    let address = account.addresses().await?;
    let faucet_response =
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address[0].address().to_string()).await?;
    println!("{faucet_response}");

    account.sync(None).await?;

    ////////////////////////////////
    //// create voting output or increase voting power
    //// ////////////////////////////

    let transaction = account.increase_voting_power(1000001).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Increase voting power block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    account.sync(None).await?;

    let voting_output = account.get_voting_output().await?.unwrap();
    println!("Voting output: {:?}", voting_output.output);

    ////////////////////////////////
    //// decrease voting power
    //// ////////////////////////////

    // let transaction = account.decrease_voting_power(1).await?;
    // println!("Transaction sent: {}", transaction.transaction_id);

    // let block_id = account
    //     .retry_transaction_until_included(&transaction.transaction_id, None, None)
    //     .await?;
    // println!(
    //     "Decrease voting power {}/block/{}",
    //     transaction.transaction_id,
    //     std::env::var("EXPLORER_URL").unwrap(),
    //     block_id
    // );

    // account.sync(None).await?;

    ////////////////////////////////
    //// vote
    //// ////////////////////////////

    let transaction = account.vote(Some(event_id), Some(vec![0])).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Vote block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    account.sync(None).await?;

    ////////////////////////////////
    //// get voting overview
    //// ////////////////////////////

    let overview = account.get_participation_overview(None).await?;
    println!("overview: {overview:?}");

    ////////////////////////////////
    //// stop vote
    //// ////////////////////////////

    let transaction = account.stop_participating(event_id).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Stop participating block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    account.sync(None).await?;

    ////////////////////////////////
    //// destroy voting output
    //// ////////////////////////////

    // let voting_output = account.get_voting_output().await?;
    // println!("Voting output: {:?}", voting_output.output);

    // // Decrease full amount, there should be no voting output afterwards
    // let transaction = account.decrease_voting_power(voting_output.output.amount()).await?;
    // println!("Transaction sent: {}", transaction.transaction_id);

    // let block_id = account
    //     .retry_transaction_until_included(&transaction.transaction_id, None, None)
    //     .await?;
    // println!(
    //     "Transaction included: {}/block/{}",
    //     std::env::var("EXPLORER_URL").unwrap(),
    //     block_id
    // );
    // account.sync(None).await?;

    // assert!(account.get_voting_output().await.is_err());

    Ok(())
}
