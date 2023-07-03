// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets the account output from the corresponding account id by querying the
//! `api/indexer/v2/outputs/account/{accountId}` node endpoint.
//!
//! Make sure that the node has the indexer plugin enabled.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_indexer_get_account_output <ACCOUNT_ID> [NODE URL]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::output::AccountId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the account id from the command line, or panic.
    let account_id = std::env::args()
        .nth(1)
        .expect("missing example argument: ACCOUNT_ID")
        .parse::<AliasId>()?;

    // Get the output ID by providing the corresponding account ID.
    let output_id = client.account_output_id(account_id).await?;

    println!("Account output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;

    println!("{output_response:#?}",);

    Ok(())
}
