// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will fetch all inputs from a given transaction id.
//! 
//! `cargo run --example inputs_from_transaction_id --release`

use iota_sdk::client::{block::payload::transaction::TransactionId, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    let client = Client::builder().with_node(&node_url)?.finish()?;

    let transaction_id =
        "0xaf7579fb57746219561072c2cc0e4d0fbb8d493d075bd21bf25ae81a450c11ef".parse::<TransactionId>()?;

    let inputs = client.inputs_from_transaction_id(&transaction_id).await?;

    println!("Transaction inputs {:?}", inputs);

    Ok(())
}
