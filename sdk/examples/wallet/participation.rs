// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will:
//! * fetch participation/voting events from a node and register some or all of them with our wallet
//! * increase and decrease our voting power
//! * try to vote (the example aborts if you vote on an already ended voting)
//! * if a voting occurred, stops the voting and destroys the voting output
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example and there are funds on the first address
//! by running the `get_funds` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example wallet_participation
//! ```

use iota_sdk::{
    client::node_manager::node::Node,
    wallet::{account::types::participation::ParticipationEventRegistrationOptions, Result},
    Url, Wallet,
};

// The node that runs the participation plugin
const PARTICPATION_NODE_URL: &str = "https://api.testnet.shimmer.network";
// Some (voted on) participation event. You need to change it to perform an actual vote, otherwise the example will
// abort at the time where it tries to send a voting transaction.
const PARTICIPATION_EVENT_ID_1: &str = "0x5a5d145648cd5c100e64d4463f5cccf994a404dcc58e2c3bdfef3aa82266aa8d";
// An ignored participation event. You can empty the string if you don't care about it!
const IGNORED_PARTICIPATION_EVENT_ID: &str = "0x8f682b31fb9d9ff57d87dd6061d823a355eafe133f5d40f96aaca5c5a3d6fc5d";
// A deregistered participation event. You can empty the string if you don't care about it!
const DEREGISTERED_PARTICIPATION_EVENT: &str = "0x16f4b8fa61f40a666404ac446b6a74a0dea47342345311294676794d0dc8b67a";
// The amount of voting power we'll increase
const INCREASE_VOTING_POWER_AMOUNT: u64 = 1000001;
// The amount of voting power we'll then decrease
const DECREASE_VOTING_POWER_AMOUNT: u64 = 1;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // Provide the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let event_id = PARTICIPATION_EVENT_ID_1.parse()?;
    let node = Node {
        url: Url::parse(PARTICPATION_NODE_URL).map_err(iota_sdk::client::Error::Url)?,
        auth: None,
        disabled: false,
    };
    let _ = account
        .register_participation_events(&ParticipationEventRegistrationOptions {
            node,
            // We ignore this particular event
            events_to_ignore: (!IGNORED_PARTICIPATION_EVENT_ID.is_empty())
                .then_some(vec![IGNORED_PARTICIPATION_EVENT_ID.parse()?]),
            // We register all others. If you want to register only particular events provide their ids with a
            // `Some(vec![...])`
            events_to_register: None,
        })
        .await?;

    println!("Registered events:");
    let registered_participation_events = account.get_participation_events().await?;
    for (i, (id, event)) in registered_participation_events.iter().enumerate() {
        println!("EVENT #{i}");
        println!(
            "- id: {id}\n- name: {}\n- commence: {}\n- start: {}\n- end: {}\n- info: {}",
            event.data.name(),
            event.data.milestone_index_commence(),
            event.data.milestone_index_start(),
            event.data.milestone_index_end(),
            event.data.additional_info(),
        );
    }

    println!("Checking for participation event '{PARTICIPATION_EVENT_ID_1}'");
    if let Ok(Some(event)) = account.get_participation_event(event_id).await {
        println!("{event:#?}");

        println!("Getting event status for '{PARTICIPATION_EVENT_ID_1}'");
        let event_status = account.get_participation_event_status(&event_id).await?;
        println!("{event_status:#?}");
    } else {
        println!("Event not found");
    }

    ////////////////////////////////////////////////
    // deregister an event
    ////////////////////////////////////////////////
    if !DEREGISTERED_PARTICIPATION_EVENT.is_empty() {
        account
            .deregister_participation_event(&DEREGISTERED_PARTICIPATION_EVENT.parse()?)
            .await?;

        println!("Registered events (updated):");
        let registered_participation_events = account.get_participation_events().await?;
        for (i, (id, event)) in registered_participation_events.iter().enumerate() {
            println!("EVENT #{i}");
            println!(
                "- id: {id}\n- name: {}\n- commence: {}\n- start: {}\n- end: {}\n- info: {}",
                event.data.name(),
                event.data.milestone_index_commence(),
                event.data.milestone_index_start(),
                event.data.milestone_index_end(),
                event.data.additional_info(),
            );
        }
    }

    let balance = account.sync(None).await?;
    println!("Account synced");

    ////////////////////////////////////////////////
    // create voting output or increase voting power
    ////////////////////////////////////////////////

    println!("Current voting power: {}", balance.base_coin().voting_power());

    println!("Sending transaction to increase voting power...");
    let transaction = account.increase_voting_power(INCREASE_VOTING_POWER_AMOUNT).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    println!("Waiting for `increase voting power` transaction to be included...");
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let balance = account.sync(None).await?;
    println!("Account synced");
    println!("New voting power: {}", balance.base_coin().voting_power());

    let voting_output = account.get_voting_output().await?.unwrap();
    println!("Voting output:\n{:#?}", voting_output.output);

    ////////////////////////////////////////////////
    // decrease voting power
    ////////////////////////////////////////////////

    println!("Sending transaction to decrease voting power...");
    let transaction = account.decrease_voting_power(DECREASE_VOTING_POWER_AMOUNT).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    println!("Waiting for `decrease voting power` transaction to be included...");
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let balance = account.sync(None).await?;
    println!("Account synced");
    println!("New voting power: {}", balance.base_coin().voting_power());

    ////////////////////////////////////////////////
    // vote
    ////////////////////////////////////////////////

    println!("Sending transaction to vote...");
    let transaction = account.vote(Some(event_id), Some(vec![0])).await?;
    // NOTE!!!
    // from here on out, the example will only proceed if you've set up your own participation event and
    // changed the constants above with a valid (i.e. ongoing) event id for
    println!("Transaction sent: {}", transaction.transaction_id);

    println!("Waiting for `vote` transaction to be included...");
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    account.sync(None).await?;
    println!("Account synced");

    ////////////////////////////////////////////////
    // get voting overview
    ////////////////////////////////////////////////

    let overview = account.get_participation_overview(None).await?;
    println!("Particpation overview:\n{overview:?}");

    ////////////////////////////////////////////////
    // stop vote
    ////////////////////////////////////////////////

    let transaction = account.stop_participating(event_id).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    println!("Waiting for `stop participating` transaction to be included...");
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    account.sync(None).await?;
    println!("Account synced");

    ////////////////////////////////////////////////
    // destroy voting output
    ////////////////////////////////////////////////

    let voting_output = account.get_voting_output().await?.unwrap();
    println!("Voting output: {:?}", voting_output.output);

    // Decrease full amount, there should be no voting output afterwards
    let transaction = account.decrease_voting_power(voting_output.output.amount()).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    println!("Waiting for `decrease voting power` transaction to be included...");
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    account.sync(None).await?;
    println!("Account synced");

    assert!(account.get_voting_output().await.is_err());

    Ok(())
}
