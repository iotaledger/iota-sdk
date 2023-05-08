// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns the available API route groups of the node by calling `GET /api/routes`.
//!
//! `cargo run --example node_api_core_get_routes --release -- [NODE URL]`

use iota_sdk::client::{Client, Result};

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
        .with_node(&node_url)?
        .with_ignore_node_health()
        .finish()
        .await?;

    // Get routes.
    let routes = client.get_routes().await?;

    println!("{routes:#?}");

    Ok(())
}
