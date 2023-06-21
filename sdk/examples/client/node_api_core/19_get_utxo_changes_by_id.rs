// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Gets all UTXO changes of a given milestone by milestone identifier by querying the
//! `/api/core/v2/milestones/{milestoneId}/utxo-changes` node endpoint.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example node_api_core_get_utxo_changes_by_id [MILESTONE ID] [NODE URL]
//! ```

use std::env;

use iota_sdk::{
    client::{Client, Result},
    types::block::payload::milestone::MilestoneId,
};

#[tokio::main]
async fn main() -> Result<()> {
    // If not provided we use the default node from the `.env` file.
    dotenvy::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = env::args().nth(2).unwrap_or_else(|| env::var("NODE_URL").unwrap());

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    // Take the milestone id from the command line, or use a default.
    let milestone_id = if let Some(s) = env::args().nth(1) {
        s
    } else {
        client
            .get_info()
            .await?
            .node_info
            .status
            .latest_milestone
            .milestone_id
            .unwrap()
    }
    .parse::<MilestoneId>()
    .expect("invalid milestone id");

    // Send the request.
    let utxo_changes = client.get_utxo_changes_by_id(&milestone_id).await?;

    println!("{utxo_changes:#?}");

    Ok(())
}
