// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will fetch all inputs from a given transaction id.
//!
//! Make sure to provide a somewhat recent transaction id to make this example run successfully!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example inputs_from_transaction_id <TRANSACTION_ID>
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::transaction::TransactionId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    let mut args = std::env::args().skip(1);

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;
    let transaction_id = args
        .next()
        .expect("missing example argument: TRANSACTION ID")
        .parse::<TransactionId>()?;

    let inputs = client.inputs_from_transaction_id(&transaction_id).await?;

    println!("Transaction inputs:\n{:#?}", inputs);

    Ok(())
}
