// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns milestone data as JSON by its index by querying the
//! `/api/core/v2/milestones/by-index/{index}` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_milestone_by_index [MILESTONE INDEX] [NODE URL]
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the milestone index from the command line, or use a default.
    let milestone_index = if let Some(s) = env::args().nth(1) {
        s.parse().expect("invalid milestone index")
    } else {
        client.get_info().await?.node_info.status.latest_milestone.index
    };

    // Send the request.
    let milestone = client.get_milestone_by_index(milestone_index).await?;

    println!("{milestone:#?}");

    Ok(())
}
