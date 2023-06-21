// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets the first page of output ids when querying the
//! `api/indexer/v1/outputs/basic` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_random_basic_outputs [NODE_URL]
//! ```

use std::env;

use iota_sdk::client::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Get a single page with random output IDs by providing only `QueryParameter::Cursor(_)`.
    let output_ids_response = client.basic_output_ids([QueryParameter::Cursor(String::new())]).await?;

    println!("Basic output IDs from first page {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(&output_ids_response.items).await?;

    println!("{outputs_responses:#?}");

    Ok(())
}
