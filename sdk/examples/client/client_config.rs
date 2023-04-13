// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a client from a JSON config.
//! 
//! `cargo run --example client_config --release`

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .from_json(
            r#"{
                "nodes":[
                   {
                      "url":"http://localhost:14265/",
                      "auth":null,
                      "disabled":false
                   },
                   {
                      "url":"https://api.testnet.shimmer.network",
                      "auth":null,
                      "disabled":false
                   }
                ],
                "localPow":true,
                "apiTimeout":{
                   "secs":20,
                   "nanos":0
                }
             }"#,
        )?
        .finish()?;

    let info = client.get_info().await?;
    println!("Node Info: {info:?}");

    Ok(())
}
