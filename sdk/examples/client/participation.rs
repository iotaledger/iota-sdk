// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to participate in voting events.
//!
//! Command to create an event, when your node is located on the same machine:
//! curl -X POST http://localhost:14265/api/participation/v1/admin/events -H 'Content-Type: application/json' -d '{"name":"Shimmer Proposal","milestoneIndexCommence":580,"milestoneIndexStart":600,"milestoneIndexEnd":700,"payload":{"type":0,"questions":[{"text":"Should we be on CMC rank #1 eoy?","answers":[{"value":1,"text":"Yes","additionalInfo":""},{"value":2,"text":"No","additionalInfo":""}],"additionalInfo":""}]},"additionalInfo":"Nothing needed here"}'
//! Command to delete an event:
//! curl -X DELETE http://localhost:14265/api/participation/v1/admin/events/<RETURNED_EVENT_ID>
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example participation
//! ```

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
        secret::SecretManager, Client, Result,
    },
    types::{
        api::plugins::participation::types::{Participation, ParticipationEventId, Participations, PARTICIPATION_TAG},
        block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    // Get the participation events.
    let events = client.events(None).await?;

    println!("{events:#?}");

    for event_id in &events.event_ids {
        let event_info = client.event(event_id).await?;
        println!("{event_info:#?}");
        let event_status = client.event_status(event_id, None).await?;
        println!("{event_status:#?}");
    }

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;
    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];

    let faucet_url = std::env::var("FAUCET_URL").unwrap();
    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&faucet_url, &address).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let address_participation = client.address_staking_status(address).await?;
    println!("{address_participation:#?}");

    let address_output_ids = client.address_participation_output_ids(address).await?;
    println!("{address_output_ids:#?}");

    for (output_id, _) in address_output_ids.outputs.into_iter() {
        let output_status = client.output_status(&output_id).await?;
        println!("{output_status:#?}");
    }

    // Get outputs for address and request if they're participating
    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(address),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    for output_id in output_ids_response.items {
        if let Ok(output_status) = client.output_status(&output_id).await {
            println!("{output_status:#?}");
        }
    }

    // Participate with one answer set to `1` for the first event
    participate(
        &client,
        events.event_ids.first().expect("No event available").to_owned(),
    )
    .await?;
    Ok(())
}

async fn participate(client: &Client, event_id: ParticipationEventId) -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let token_supply = client.get_token_supply().await?;
    let rent_structure = client.get_rent_structure().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(client).await?.with_range(0..1))
        .await?[0];

    let outputs = [BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output(token_supply)?];

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .with_tag(PARTICIPATION_TAG.as_bytes().to_vec())
        .with_data(
            Participations {
                participations: vec![Participation {
                    event_id,
                    answers: vec![1],
                }],
            }
            .to_bytes()?,
        )
        .finish()
        .await?;

    println!("{block:#?}");

    println!(
        "Block with participation data sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
