// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Find an output, as raw bytes, by its identifier by calling `GET /api/core/v2/outputs/{outputId}`.
//!
//! Make sure to provide a somewhat recent output id to make this example run successfully!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_output_raw <OUTPUT ID> [NODE URL]
//! ```

use std::env;

use iota_sdk::{
    client::{Client, Result},
    types::block::output::OutputId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the output id from the command line, or panic.
    let output_id = env::args()
        .nth(1)
        .expect("missing example argument: output id")
        .parse::<OutputId>()?;

    // Get the output as raw bytes.
    let output_bytes = client.get_output_raw(&output_id).await?;

    println!("Output bytes:\n{output_bytes:?}");

    Ok(())
}
