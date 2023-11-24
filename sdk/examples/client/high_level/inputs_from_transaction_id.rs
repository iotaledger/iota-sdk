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

    for var in ["NODE_URL"] {
        if std::env::var(var).is_err() {
            panic!(".env variable '{}' is undefined, see .env.example", var);
        }
    }

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;
    let transaction_id = std::env::args()
        .nth(1)
        .expect("missing example argument: TRANSACTION ID")
        .parse::<TransactionId>()?;

    let inputs = client.inputs_from_transaction_id(&transaction_id).await?;

    println!("Transaction inputs:\n{:#?}", inputs);

    Ok(())
}
