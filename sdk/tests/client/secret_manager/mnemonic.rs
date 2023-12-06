// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{
        constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{mnemonic::MnemonicSecretManager, PublicKeyOptions, SecretManageExt},
        Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn mnemonic_secret_manager() -> Result<()> {
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast",
    )?;

    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
        .await
        .unwrap()
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    Ok(())
}
