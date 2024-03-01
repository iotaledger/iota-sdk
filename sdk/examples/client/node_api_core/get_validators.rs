// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example returns the validators of the node by querying its `/validators` endpoint.
//! The result is paginated with a page size of 1. You can provide a LIST_INDEX to request
//! a particular validator.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example node_api_core_get_validators [LIST_INDEX] [NODE_URL]
//! ```

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    let list_index = std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap());

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").expect("NODE_URL not set"));

    // Create a node client.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_ignore_node_health()
        .finish()
        .await?;

    let slot_index = client.get_node_info().await?.info.status.latest_finalized_slot;
    let cursor = list_index.map(|i| format!("{slot_index},{i}"));

    // Get validators.
    let res = client.get_validators(1, cursor).await?;

    println!("{res:?}");

    Ok(())
}
