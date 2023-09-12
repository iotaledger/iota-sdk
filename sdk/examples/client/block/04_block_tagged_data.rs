// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block with a tagged data payload.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_tagged_data [TAG] [DATA]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::{Payload, TaggedDataPayload},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Create and send the block with tag and data.
    let wrapper = client
        .finish_basic_block_builder(
            todo!("issuer id"),
            todo!("block signature"),
            todo!("issuing time"),
            None,
            Some(Payload::TaggedData(Box::new(
                TaggedDataPayload::new(
                    std::env::args()
                        .nth(1)
                        .unwrap_or_else(|| "Hello".to_string())
                        .as_bytes(),
                    std::env::args()
                        .nth(2)
                        .unwrap_or_else(|| "Tangle".to_string())
                        .as_bytes(),
                )
                .unwrap(),
            ))),
        )
        .await?;

    println!("{wrapper:#?}\n");

    if let Some(Payload::TaggedData(payload)) = wrapper.as_basic().payload() {
        println!(
            "Tag: {}",
            String::from_utf8(payload.tag().to_vec()).expect("found invalid UTF-8")
        );
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("found invalid UTF-8")
        );
    }

    println!(
        "Block with tag and data sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        wrapper.id()
    );

    Ok(())
}
