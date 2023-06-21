// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Find an output, as JSON, by its identifier by querying the `/api/core/v3/outputs/{outputId}` node endpoint.
//!
//! Make sure to provide a somewhat recent output id to make this example run successfully!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_output <OUTPUT ID> [NODE URL]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::output::OutputId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the output id from the command line, or panic.
    let output_id = std::env::args()
        .nth(1)
        .expect("missing example argument: OUTPUT ID")
        .parse::<OutputId>()?;

    // Get the output.
    let output = client.get_output(&output_id).await?;

    println!("{output:#?}");

    Ok(())
}
