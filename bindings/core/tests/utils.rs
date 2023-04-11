// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{call_utility_method, method_handler::Result, Response, UtilityMethod};

#[tokio::test]
async fn utils() -> Result<()> {
    let response = call_utility_method(UtilityMethod::GenerateMnemonic).await;
    match response {
        Response::GeneratedMnemonic(mnemonic) => println!("{:?}", serde_json::to_string(&mnemonic).unwrap()),
        _ => panic!("Unexpected response type"),
    };

    let bech32_adddress = "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string();
    let method = UtilityMethod::Bech32ToHex {
        bech32: bech32_adddress.clone(),
    };

    let response = call_utility_method(method).await;
    match response {
        Response::Bech32ToHex(hex) => {
            match call_utility_method(UtilityMethod::HexToBech32 {
                hex,
                bech32_hrp: "rms".to_string(),
            })
            .await
            {
                Response::Bech32Address(address_bech32) => {
                    assert_eq!(address_bech32, bech32_adddress)
                }
                _ => panic!("Unexpected response type"),
            };
        }
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
