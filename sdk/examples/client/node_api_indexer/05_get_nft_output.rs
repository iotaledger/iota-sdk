// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets the nft output from the corresponding nft id by querying the
//! `api/indexer/v1/outputs/nft/{nftId}` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_nft_output <NFT ID> [NODE URL]
//! ```

use std::env;

use iota_sdk::{
    client::{Client, Result},
    types::block::output::NftId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the alias id from the command line, or panic.
    let nft_id = env::args()
        .nth(1)
        .expect("missing example argument: NFT ID")
        .parse::<NftId>()?;

    // Get the output ID by the NFT ID.
    let output_id = client.nft_output_id(nft_id).await?;
    println!("NFT output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;
    println!("{output_response:#?}",);

    Ok(())
}
