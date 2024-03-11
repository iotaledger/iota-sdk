// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{
    api::GetAddressesOptions,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::SecretManager,
    ClientError,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn mnemonic_secret_manager() -> Result<(), ClientError> {
    let dto = r#"{"mnemonic": "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"}"#;
    let secret_manager: SecretManager = dto.parse()?;

    let address = secret_manager
        .generate_ed25519_address(SHIMMER_COIN_TYPE, 0, 0, SHIMMER_TESTNET_BECH32_HRP, None)
        .await
        .unwrap();

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}
