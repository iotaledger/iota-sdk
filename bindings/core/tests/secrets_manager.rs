// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_sdk::client::{api::GetAddressesOptions, constants::ETHER_COIN_TYPE, secret::SecretManager};
use iota_sdk_bindings_core::{call_secret_manager_method, Response, Result, SecretManagerMethod};
use tokio::sync::RwLock;

#[tokio::test]
async fn generate_ed25519_addresses() -> Result<()> {
    let secret_manager = Arc::new(RwLock::new(SecretManager::try_from_mnemonic(
        "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river".to_owned(),
    )?));

    let method = SecretManagerMethod::GenerateEd25519Addresses {
        options: GetAddressesOptions::default().with_range(0..1),
    };

    let response = call_secret_manager_method(&secret_manager, method).await;
    match response {
        Response::GeneratedEd25519Addresses(addresses) => assert_eq!(
            addresses[0],
            "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy"
        ),
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}

#[tokio::test]
async fn generate_evm_addresses() -> Result<()> {
    let secret_manager = Arc::new(RwLock::new(SecretManager::try_from_mnemonic(
        "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river".to_owned(),
    )?));

    let method = SecretManagerMethod::GenerateEvmAddresses {
        options: GetAddressesOptions::default()
            .with_range(0..1)
            .with_coin_type(ETHER_COIN_TYPE),
    };

    let response = call_secret_manager_method(&secret_manager, method).await;
    match response {
        Response::GeneratedEvmAddresses(addresses) => {
            assert_eq!(addresses[0], "0xcaefde2b487ded55688765964320ff390cd87828")
        }
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
