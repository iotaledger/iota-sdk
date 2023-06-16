// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns all stored receipts for a given migration index by querying the `/api/core/v2/receipts/{migratedAt}` node
//! endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_receipts_migrated_at <MILESTONE_INDEX> [NODE URL]
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the transaction id from the command line, or panic.
    let milestone_index = env::args()
        .nth(1)
        .expect("missing example argument: MILESTONE INDEX")
        .parse()
        .expect("invalid milestone index");

    // Send the request.
    let receipts = client.get_receipts_migrated_at(milestone_index).await?;

    println!("Receipts:\n{receipts:#?}");

    Ok(())
}
