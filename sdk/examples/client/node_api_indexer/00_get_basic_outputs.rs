// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v1/outputs/basic`.
//!
//! `cargo run --example node_api_indexer_get_basic_outputs --release -- <NODE_URL> <ADDRESS>`.

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
        .finish()?;

    // Take the address from command line argument or use a default one.
    let address = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("rms1qrrdjmdkadtcnuw0ue5n9g4fmkelrj3dl26eyeshkha3w3uu0wheu5z5qqz"));

    // Get output IDs of basic outputs that can be controlled by this address without further unlock constraints.
    let output_ids_response = client
        .basic_output_ids(vec![
            QueryParameter::Address(address),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    println!("Address output IDs {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(output_ids_response.items).await?;

    println!("Basic outputs: {outputs_responses:#?}");

    Ok(())
}
