// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block, with custom parents, which can be used for promoting.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_custom_parents
//! ```

use iota_sdk::{
    client::{constants::IOTA_COIN_TYPE, secret::SecretManager, Client, Result},
    types::block::parent::StrongParents,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let protocol_parameters = client.get_protocol_parameters().await?;

    // Use tips as custom parents.
    let tips = client.get_tips().await?;
    println!("Custom tips:\n{tips:#?}");

    // Create and send the block with custom parents.
    let block = client
        .finish_basic_block_builder(
            todo!("issuer id"),
            todo!("issuing time"),
            Some(StrongParents::from_vec(tips)?),
            None,
            IOTA_COIN_TYPE,
            &secret_manager,
        )
        .await?;

    println!("{block:#?}");

    println!(
        "Block with custom parents sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id(&protocol_parameters)
    );

    Ok(())
}
