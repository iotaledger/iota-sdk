// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example returns the health of the node by querying its `/health` endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example get_health [NODE_URL]
//! ```

use iota_sdk::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| std::env::var("NODE_URL").expect("NODE_URL not set"));

    // Create a node client.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_ignore_node_health()
        .finish()
        .await?;

    // Get node health.
    let is_healthy = client.get_health(&node_url).await?;
    let is_network_healthy = client.get_network_health().await?;

    println!("Healthy: node {is_healthy}, network {is_network_healthy}");

    Ok(())
}
