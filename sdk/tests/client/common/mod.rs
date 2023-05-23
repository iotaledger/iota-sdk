// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::redundant_pub_crate, dead_code)]

mod constants;

use iota_sdk::client::{
    api::GetAddressesOptions, constants::SHIMMER_COIN_TYPE, node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet, secret::SecretManager, Client, Result,
};

pub use self::constants::{FAUCET_URL, NODE_LOCAL};

/// Sets up a Client with node health ignored.
pub async fn setup_client_with_node_health_ignored() -> Client {
    Client::builder().with_node(NODE_LOCAL).unwrap().finish().await.unwrap()
}

/// Create a client with `DEFAULT_DEVNET_NODE_URL` and a random mnemonic, request funds from the faucet to the first
/// address and wait until they arrived.
pub async fn create_client_and_secret_manager_with_funds(mnemonic: Option<&str>) -> Result<(Client, SecretManager)> {
    let client = Client::builder().with_node(NODE_LOCAL)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(mnemonic.unwrap_or(&Client::generate_mnemonic().unwrap()))?;

    let address = secret_manager
        .get_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1),
        )
        .await?[0];

    request_funds_from_faucet(FAUCET_URL, &address).await?;

    // Continue only after funds are received
    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let output_ids_response = client
            .basic_output_ids(vec![
                QueryParameter::Address(address),
                QueryParameter::HasExpiration(false),
                QueryParameter::HasTimelock(false),
                QueryParameter::HasStorageDepositReturn(false),
            ])
            .await?;

        if !output_ids_response.is_empty() {
            return Ok((client, secret_manager));
        }
    }
    panic!("Faucet no longer wants to hand over coins");
}
