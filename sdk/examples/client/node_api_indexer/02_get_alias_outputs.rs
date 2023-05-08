// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v1/outputs/alias`.
//!
//! `cargo run --example node_api_indexer_get_alias_outputs --release -- [NODE URL] [ADDRESS]`

use iota_sdk::{
    client::{node_api::indexer::query_parameters::QueryParameter, Client, Result},
    types::block::address::Bech32Address,
};

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

    // Take the address from command line argument or use a default one.
    let address = Bech32Address::try_from_str(
        std::env::args()
            .nth(2)
            .unwrap_or_else(|| String::from("rms1qrrdjmdkadtcnuw0ue5n9g4fmkelrj3dl26eyeshkha3w3uu0wheu5z5qqz")),
    )?;

    // Get output IDs of alias outputs that can be controlled by this address.
    let output_ids_response = client
        .alias_output_ids(vec![
            QueryParameter::Governor(address.clone()),
            QueryParameter::StateController(address),
        ])
        .await?;

    println!("Alias output IDs: {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(output_ids_response.items).await?;

    println!("{outputs_responses:#?}");

    Ok(())
}
