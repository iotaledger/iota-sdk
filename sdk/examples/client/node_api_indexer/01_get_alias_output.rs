// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! TODO: <insert example description> by calling
//! `GET api/indexer/v2/outputs/alias/{accountId}`.
//!
//! `cargo run --example node_api_indexer_get_account_output --release -- [NODE URL] [ACCOUNT ID]`

use std::str::FromStr;

use iota_sdk::{
    client::{Client, Result},
    types::block::output::AccountId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses secrets in environment variables for simplicity which should not be done in production.
        dotenvy::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder()
        // The node needs to have the indexer plugin enabled.
        .with_node(&node_url)?
        .finish()
        .await?;

    // Take the account ID from command line argument or use a default one.
    let account_id = AccountId::from_str(
        &std::env::args()
            .nth(2)
            .unwrap_or_else(|| String::from("0xdb4db7643d768139d6f8ac3f9c9b7a82a245b619fa9f7c18fcd8f0f67e57abc2")),
    )?;

    // Get the output ID by the account ID.
    let output_id = client.account_output_id(account_id).await?;

    println!("Account output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;

    println!("{output_response:#?}",);

    Ok(())
}
