// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get outputs with quorum, which will compare the responses from the nodes.
//!
//! Make sure to have 3 nodes available for this example to run successfully, e.g.:
//! - http://localhost:8050
//! - http://your-vps:14265
//! - https://api.testnet.shimmer.network
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example quorum <NODE 1> <NODE 2> <NODE 3>
//! ```

use iota_sdk::client::{
    api::GetAddressesOptions, node_api::indexer::query_parameters::BasicOutputQueryParameters, secret::SecretManager,
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    #[allow(clippy::single_element_loop)]
    for var in ["MNEMONIC"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

    let node_1 = std::env::args().nth(1).expect("missing example argument: NODE 1");
    let node_2 = std::env::args().nth(2).expect("missing example argument: NODE 2");
    let node_3 = std::env::args().nth(3).expect("missing example argument: NODE 3");

    // Create a node client.
    let client = Client::builder()
        .with_node(&node_1)?
        .with_node(&node_2)?
        .with_node(&node_3)?
        .with_quorum(true)
        .with_min_quorum_size(3)
        .with_quorum_threshold(66)
        .finish()
        .await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Generate the first address
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_account_index(0)
                .with_range(0..1),
        )
        .await?;

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(
            addresses[0].clone(),
        ))
        .await?;
    println!("Address outputs: {output_ids_response:?}");

    Ok(())
}
