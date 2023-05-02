// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v1/outputs/foundry`.
//!
//! `cargo run --example node_api_indexer_get_foundry_outputs --release -- [NODE URL] [ADDRESS]`

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
        .finish()?;

    // Take the address from command line argument or use a default one.
    let alias_address = Bech32Address::try_from_str(
        std::env::args()
            .nth(2)
            .unwrap_or_else(|| String::from("rms1prd5mdmy84mgzwwklzkrl8ym02p2y3dkr8af7lqclnv0pan7274uyjrmwx5")),
    )?;

    // Get output IDs of foundry outputs that can be controlled by this address.
    let output_ids_response = client
        .foundry_output_ids(vec![QueryParameter::AliasAddress(alias_address)])
        .await?;

    println!("Foundry output IDs: {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(output_ids_response.items).await?;

    println!("{outputs_responses:#?}");

    Ok(())
}
