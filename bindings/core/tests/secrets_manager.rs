// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{api::GetAddressesOptions, secret::SecretManager};
use iota_sdk_bindings_core::{CallMethod, Response, Result, SecretManagerMethod};

#[tokio::test]
async fn generate_addresses() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(
        "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river",
    )?;

    let method = SecretManagerMethod::GenerateAddresses {
        options: GetAddressesOptions::default(),
    };

    let response = secret_manager.call_method(method).await;
    match response {
        Response::GeneratedAddresses(addresses) => assert_eq!(
            addresses[0],
            "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy"
        ),
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}

#[tokio::test]
async fn generate_evm_addresses() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(
        "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river",
    )?;

    let method = SecretManagerMethod::GenerateEvmAddresses {
        options: GetAddressesOptions::default(),
    };

    let response = secret_manager.call_method(method).await;
    match response {
        Response::GeneratedEvmAddresses(addresses) => {
            assert_eq!(addresses[0], "0xbfc69bd510a9a93449d31b99d23dbc55ecf1f499")
        }
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
