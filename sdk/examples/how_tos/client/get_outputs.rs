// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v1/outputs/basic`.
//!
//! `cargo run --release --example get_outputs -- [NODE_URL] [ADDRESS]`.

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
            .as_deref()
            .unwrap_or("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy"),
    )?;

    // Get output IDs of basic outputs that can be controlled by this address without further unlock constraints.
    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(address),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    println!("First output of query:");
    println!("ID: {:#?}", output_ids_response.first().expect("No outputs found"));

    // Get the outputs by their IDs.
    let outputs_response = client.get_outputs(&output_ids_response.items).await?;

    println!("{:#?}", outputs_response.first().unwrap());

    Ok(())
}
