// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns the included block, as raw bytes, of a transaction by querying the
//! `/api/core/v2/transactions/{transactionId}/included-block` node endpoint.
//!
//! Make sure to provide a somewhat recent transaction id to make this example run successfully!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_included_block_raw <TRANSACTION_ID> [NODE URL]
//! ```

use std::env;

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("NODE_URL").expect("NODE_URL not set"));

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the transaction id from the command line, or panic.
    let transaction_id = env::args()
        .nth(1)
        .expect("missing example argument: TRANSACTION ID")
        .parse()?;

    // Send the request.
    let block_bytes = client.get_included_block_raw(&transaction_id).await?;

    println!("Block bytes: {block_bytes:?}");

    Ok(())
}
