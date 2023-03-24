// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET /api/core/v2/outputs/{outputId}`.
//! Find an output, as raw bytes, by its identifier.
//! Run: `cargo run --example node_api_core_get_output_raw --release -- [NODE URL] [OUTPUT ID]`.

use std::str::FromStr;

use iota_sdk::client::{block::output::OutputId, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    // Take the output ID from command line argument or use a default one.
    let output_id =
        OutputId::from_str(&std::env::args().nth(2).unwrap_or_else(|| {
            String::from("0xb66fd384cb5755668f1890ea2e41d699db9cf32f3bc422ad3c24ffeb9c7f01d00000")
        }))?;

    // Get the output as raw bytes.
    let output_bytes = client.get_output_raw(&output_id).await?;

    println!("Output bytes: {output_bytes:?}");

    Ok(())
}
