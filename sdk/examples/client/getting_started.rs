// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This examples shows how to get the node info.
//!
//! ```sh
//! cargo run --release --example client_getting_started
//! ```

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("https://api.testnet.shimmer.network")? // Insert your node URL here
        .finish()
        .await?;

    let info = client.get_info().await?;
    println!("Node Info: {info:?}");

    Ok(())
}
