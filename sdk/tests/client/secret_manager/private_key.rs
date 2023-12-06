// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        constants::SHIMMER_TESTNET_BECH32_HRP,
        secret::{private_key::PrivateKeySecretManager, SecretManageExt},
        Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn private_key_secret_manager_hex() -> Result<()> {
    let secret_manager = PrivateKeySecretManager::try_from_hex(
        "0x9e845b327c44e28bdd206c7c9eff09c40680bc2512add57280baf5b064d7e6f6".to_owned(),
    )?;

    // Private key manager only implements generation from it's key with no options,
    // therefore only one address can be created
    let address = secret_manager
        .generate::<Ed25519Address>(&())
        .await
        .unwrap()
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}

#[tokio::test]
async fn private_key_secret_manager_bs58() -> Result<()> {
    let secret_manager = PrivateKeySecretManager::try_from_b58("BfnURR6WSXJA6RyBr3WqGU99UzrVbWk9GSQgJqKtTRxZ")?;

    let address = secret_manager
        .generate::<Ed25519Address>(&())
        .await
        .unwrap()
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}
