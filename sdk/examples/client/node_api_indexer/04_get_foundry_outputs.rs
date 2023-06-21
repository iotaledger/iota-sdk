// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets all foundry output ids associated with an alias address by querying the
//! `api/indexer/v2/outputs/foundry` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_foundry_outputs <ADDRESS> [NODE URL]
//! ```

use iota_sdk::{
    client::{node_api::indexer::query_parameters::QueryParameter, Client, Result},
    types::block::address::Bech32Address,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder()
        // The node needs to have the indexer plugin enabled.
        .with_node(&node_url)?
        .finish()
        .await?;

    // Take the address from the command line, or panic.
    let address = std::env::args()
        .nth(1)
        .expect("missing example argument: ADDRESS")
        .parse::<Bech32Address>()?;

    // Get output IDs of foundry outputs that can be controlled by this address.
    let output_ids_response = client
        .foundry_output_ids([QueryParameter::AliasAddress(address)])
        .await?;

    println!("Foundry output IDs: {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(&output_ids_response.items).await?;

    println!("{outputs_responses:#?}");

    Ok(())
}
