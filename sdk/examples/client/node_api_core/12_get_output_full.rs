// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Find an output with its metadata, by its identifier by querying the `/api/core/v3/outputs/{outputId}/full` node
//! endpoint.
//!
//! Make sure to provide a somewhat recent output id to make this example run successfully!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_output_full <OUTPUT ID> [NODE URL]
//! ```

use iota_sdk::{client::Client, types::block::output::OutputId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").expect("NODE_URL not set"));

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the output id from the command line, or panic.
    let output_id = std::env::args()
        .nth(1)
        .expect("missing example argument: OUTPUT ID")
        .parse::<OutputId>()?;

    // Get the output with its metadata.
    let output_with_metadata = client.get_output_with_metadata(&output_id).await?;

    println!("{output_with_metadata:?}");

    Ok(())
}
