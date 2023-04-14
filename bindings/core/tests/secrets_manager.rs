// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{api::GetAddressesBuilderOptions, generate_mnemonic, secret::SecretManager};
use iota_sdk_bindings_core::{call_secret_manager_method, Response, Result, SecretManagerMethod};

#[tokio::test]
async fn generate_addresses() -> Result<()> {
    let mut secret_manager = SecretManager::try_from_mnemonic(&generate_mnemonic()?)?;

    let method = SecretManagerMethod::GenerateAddresses {
        options: GetAddressesBuilderOptions::default(),
    };

    let response = call_secret_manager_method(&mut secret_manager, method).await;
    match response {
        Response::GeneratedAddresses(addresses) => println!("{addresses:?}"),
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
