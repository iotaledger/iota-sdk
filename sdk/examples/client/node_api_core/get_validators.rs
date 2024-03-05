// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example returns the validators known by the node by querying the corresponding endpoint.
//! You can provide a custom PAGE_SIZE and additionally a CURSOR from a previous request.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example node_api_core_get_validators [PAGE_SIZE] [CURSOR] [NODE_URL]
//! ```

use iota_sdk::client::{Client, ClientError};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    let page_size = std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap());
    let cursor = std::env::args().nth(2);

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(3)
        .unwrap_or_else(|| std::env::var("NODE_URL").expect("NODE_URL not set"));

    // Create a node client.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_ignore_node_health()
        .finish()
        .await?;

    // Get validators.
    let validators = client.get_validators(page_size, cursor).await?;

    println!("{validators:#?}");

    Ok(())
}
