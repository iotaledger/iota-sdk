// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Returns milestone data as JSON by its identifier by calling
//! `GET /api/core/v2/milestones/{milestoneId}`.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_milestone_by_id [MILESTONE ID] [NODE URL]`
//! ```

use std::env;

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::milestone::MilestoneId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(2)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the milestone id from the command line, or use a default.
    let info = client.get_info().await?;
    let milestone_id = env::args()
        .nth(1)
        .unwrap_or_else(|| info.node_info.status.latest_milestone.milestone_id.unwrap())
        .parse::<MilestoneId>()?;

    // Send the request.
    let milestone = client.get_milestone_by_id(&milestone_id).await?;

    println!("{milestone:#?}");

    Ok(())
}
