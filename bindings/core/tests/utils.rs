// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::address::{Bech32Address, Hrp};
use iota_sdk_bindings_core::{call_utils_method, Response, UtilsMethod};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn utils() -> Result<(), Box<dyn std::error::Error>> {
    let response = call_utils_method(UtilsMethod::GenerateMnemonic);
    match response {
        Response::GeneratedMnemonic(mnemonic) => println!("{:?}", serde_json::to_string(&mnemonic)?),
        _ => panic!("Unexpected response type"),
    };

    let bech32_address =
        Bech32Address::try_from_str("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?;
    let method = UtilsMethod::ParseBech32Address {
        address: bech32_address.clone(),
    };

    let response = call_utils_method(method);
    match response {
        Response::ParsedBech32Address(address) => {
            match call_utils_method(UtilsMethod::AddressToBech32 {
                address,
                bech32_hrp: Hrp::from_str_unchecked("rms"),
            }) {
                Response::Bech32Address(address_bech32) => {
                    assert_eq!(address_bech32, bech32_address)
                }
                _ => panic!("Unexpected response type"),
            };
        }
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
