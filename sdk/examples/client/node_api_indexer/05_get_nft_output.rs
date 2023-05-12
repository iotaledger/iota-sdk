// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v1/outputs/nft/{nftId}`.
//!
//! `cargo run --example node_api_indexer_get_nft_output --release -- [NODE URL] [NFT ID]`

use std::str::FromStr;

use iota_sdk::{
    client::{Client, Result},
    types::block::output::NftId,
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

    // Take the NFT ID from command line argument or use a default one.
    let nft_id = NftId::from_str(
        &std::env::args()
            .nth(2)
            .unwrap_or_else(|| String::from("0xa3691952eaed71f85553f6e94fab82d5ed57301b2308be7c13d1f7df2be98995")),
    )?;

    // Get the output ID by the NFT ID.
    let output_id = client.nft_output_id(nft_id).await?;

    println!("NFT output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;

    println!("{output_response:#?}",);

    Ok(())
}
