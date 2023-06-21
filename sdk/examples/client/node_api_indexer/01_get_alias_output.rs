// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets the alias output from the corresponding alias id by querying the
//! `api/indexer/v1/outputs/alias/{aliasId}` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_alias_output <ALIAS ID> [NODE URL]
//! ```

use std::env;

use iota_sdk::{
    client::{Client, Result},
    types::block::output::AliasId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the alias id from the command line, or panic.
    let alias_id = env::args()
        .nth(1)
        .expect("missing example argument: ALIAS ID")
        .parse::<AliasId>()?;

    // Get the output ID by providing the corresponding alias ID.
    let output_id = client.alias_output_id(alias_id).await?;

    println!("Alias output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;

    println!("{output_response:#?}",);

    Ok(())
}
