// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{call_utility_method, method_handler::Result, Response, UtilityMethod};

#[tokio::test]
async fn utils() -> Result<()> {
    let method = UtilityMethod::GenerateMnemonic;

    let response = call_utility_method(method).await;
    match response {
        Response::GeneratedMnemonic(mnemonic) => println!("{:?}", serde_json::to_string(&mnemonic).unwrap()),
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
