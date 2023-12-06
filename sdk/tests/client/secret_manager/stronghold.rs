// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{
        constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{stronghold::StrongholdSecretManager, PublicKeyOptions, SecretManageExt, SecretManagerConfig},
        Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
};
use pretty_assertions::assert_eq;
use serde_json::json;

#[tokio::test]
async fn stronghold_secret_manager() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let secret_manager = StrongholdSecretManager::from_config(&serde_json::from_value(json!({
        "password": "some_hopefully_secure_password",
        "snapshotPath": "snapshot_test_dir/test.stronghold"
    }))?)?;
    let mnemonic = Mnemonic::from(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast",
    );

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic.clone()).await?;

    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
        .await?
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    // Calling store_mnemonic() twice should fail, because we would otherwise overwrite the stored entry
    assert!(secret_manager.store_mnemonic(mnemonic).await.is_err());

    // Remove garbage after test, but don't care about the result
    std::fs::remove_dir_all("snapshot_test_dir").ok();
    Ok(())
}

#[tokio::test]
async fn stronghold_mnemonic_missing() -> Result<()> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    // Cleanup of a possibly failed run
    std::fs::remove_dir_all("stronghold_mnemonic_missing").ok();

    let stronghold_secret_manager = iota_sdk::client::secret::stronghold::StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password".to_owned())
        .build("stronghold_mnemonic_missing/test.stronghold")?;

    // Generating addresses will fail because no mnemonic has been stored
    let error = stronghold_secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
        .await
        .unwrap_err();

    match error {
        iota_sdk::client::Error::Stronghold(iota_sdk::client::stronghold::Error::MnemonicMissing) => {}
        _ => panic!("expected StrongholdMnemonicMissing error"),
    }

    // Remove garbage after test, but don't care about the result
    std::fs::remove_dir_all("stronghold_mnemonic_missing").ok();
    Ok(())
}
