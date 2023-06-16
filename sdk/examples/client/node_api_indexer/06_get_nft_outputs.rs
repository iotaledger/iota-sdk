// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets all nft output ids accociated with an alias address by querying the
//! `api/indexer/v1/outputs/nft` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_nft_outputs <ADDRESS> [NODE URL]
//! ```

use std::env;

use iota_sdk::{
    client::{node_api::indexer::query_parameters::QueryParameter, Client, Result},
    types::block::address::Bech32Address,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client with that node.
    let client = Client::builder()
        // The node needs to have the indexer plugin enabled.
        .with_node(&node_url)?
        .finish()
        .await?;

    // Take the address from the command line, or panic.
    let address = env::args()
        .nth(1)
        .expect("missing example argument: ADDRESS")
        .parse::<Bech32Address>()?;

    // Get output IDs of NFT outputs that can be controlled by this address without further unlock constraints.
    let output_ids_response = client
        .nft_output_ids([
            QueryParameter::Address(address),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    println!("NFT output IDs {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(&output_ids_response.items).await?;

    println!("{outputs_responses:#?}");

    Ok(())
}
