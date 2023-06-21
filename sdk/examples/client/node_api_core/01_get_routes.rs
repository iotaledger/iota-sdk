// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns the available API route groups of the node by querying its `/api/routes` endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_routes [NODE URL]
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(1).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_ignore_node_health()
        .finish()
        .await?;

    // Get routes.
    let routes = client.get_routes().await?;

    println!("Routes:\n{routes:#?}");

    Ok(())
}
