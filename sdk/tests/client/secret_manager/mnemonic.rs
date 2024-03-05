// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{
    api::GetAddressesOptions, constants::SHIMMER_TESTNET_BECH32_HRP, secret::SecretManager, ClientError,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn mnemonic_secret_manager() -> Result<(), ClientError> {
    let dto = r#"{"mnemonic": "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"}"#;
    let secret_manager: SecretManager = dto.parse()?;

    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_account_index(0)
                .with_range(0..1),
        )
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}
