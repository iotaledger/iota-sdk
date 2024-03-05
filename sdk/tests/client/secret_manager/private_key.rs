// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{
    api::GetAddressesOptions,
    constants::SHIMMER_TESTNET_BECH32_HRP,
    secret::{private_key::PrivateKeySecretManager, SecretManager},
    ClientError,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn private_key_secret_manager_hex() -> Result<(), ClientError> {
    let dto = r#"{"privateKey": "0x9e845b327c44e28bdd206c7c9eff09c40680bc2512add57280baf5b064d7e6f6"}"#;
    let secret_manager: SecretManager = dto.parse()?;

    let address_0 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_account_index(0)
                .with_range(0..1),
        )
        .await
        .unwrap()[0]
        .clone();
    // Changing range generates the same address.
    let address_1 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_account_index(0)
                .with_range(1..2),
        )
        .await
        .unwrap()[0]
        .clone();
    // Changing account generates the same address.
    let address_2 = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_account_index(1)
                .with_range(0..1),
        )
        .await
        .unwrap()[0]
        .clone();

    assert_eq!(
        address_0,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );
    assert_eq!(
        address_1,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );
    assert_eq!(
        address_2,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}

#[tokio::test]
async fn private_key_secret_manager_bs58() -> Result<(), ClientError> {
    let secret_manager = SecretManager::from(PrivateKeySecretManager::try_from_b58(
        "BfnURR6WSXJA6RyBr3WqGU99UzrVbWk9GSQgJqKtTRxZ",
    )?);

    let address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_account_index(0)
                .with_range(0..1),
        )
        .await
        .unwrap()[0]
        .clone();

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}
