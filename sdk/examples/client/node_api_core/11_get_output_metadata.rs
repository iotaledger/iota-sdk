// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns metadata about an output by its identifier by querying the `/api/core/v2/outputs/{outputId}` node endpoint.
//!
//! Make sure to provide a somewhat recent output id to make this example run successfully!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_output_metadata <OUTPUT ID> [NODE URL]
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
        .expect("missing example argument: OUTPUT ID")
        .parse::<OutputId>()?;

    // Get the output metadata.
    let output_metadata = client.get_output_metadata(&output_id).await?;

    println!("{output_metadata:#?}");

    Ok(())
}
