// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v2/outputs/basic`.
//!
//! `cargo run --example node_api_indexer_get_random_basic_outputs --release -- [NODE URL]`

use iota_sdk::client::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses secrets in environment variables for simplicity which should not be done in production.
        dotenvy::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder()
        // The node needs to have the indexer plugin enabled.
        .with_node(&node_url)?
        .finish()
        .await?;

    // Get a single page with random output IDs by providing only `QueryParameter::Cursor(_)`.
    let output_ids_response = client.basic_output_ids([QueryParameter::Cursor(String::new())]).await?;

    println!("Basic output IDs from first page {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(&output_ids_response.items).await?;

    println!("{outputs_responses:#?}");

    Ok(())
}
