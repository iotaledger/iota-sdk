// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets all UTXO changes of a given milestone by milestone index by querying the
//! `/api/core/v2/milestones/by-index/{index}/utxo-changes` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_utxo_changes_by_index [MILESTONE INDEX] [NODE URL]
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the milestone index from the command line, or use a default.
    let info = client.get_info().await?;
    let milestone_index = env::args()
        .nth(1)
        .map(|s| s.parse::<u32>().expect("invalid milestone index"))
        .unwrap_or_else(|| info.node_info.status.latest_milestone.index);

    // Send the request.
    let utxo_changes = client.get_utxo_changes_by_index(milestone_index).await?;

    println!("{utxo_changes:#?}");

    Ok(())
}
