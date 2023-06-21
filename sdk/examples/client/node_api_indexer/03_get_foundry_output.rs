// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets the foundry output from the corresponding foundry id by querying the
//! `api/indexer/v2/outputs/foundry/{foundryId}` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_foundry_output <FOUNDRY ID> [NODE URL]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::output::FoundryId,
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
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the foundry id from the command line, or panic.
    let foundry_id = std::env::args()
        .nth(1)
        .expect("missing example argument: FOUNDRY ID")
        .parse::<FoundryId>()?;

    // Get the output ID by providing the corresponding foundry ID.
    let output_id = client.foundry_output_id(foundry_id).await?;

    println!("Foundry output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;

    println!("{output_response:#?}",);

    Ok(())
}
