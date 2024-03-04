// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This examples shows how to get the node info.
//!
//! ```sh
//! cargo run --release --example client_getting_started
//! ```

use iota_sdk::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .with_node("https://api.testnet.shimmer.network")? // Insert your node URL here
        .finish()
        .await?;

    let node_info = client.get_node_info().await?;
    println!("Node Info: {node_info:?}");

    Ok(())
}
