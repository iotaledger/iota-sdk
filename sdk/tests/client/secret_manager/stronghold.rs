// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::SecretManager,
    ClientError,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn stronghold_secret_manager() -> Result<(), ClientError> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let dto = r#"{"stronghold": {"password": "some_hopefully_secure_password", "snapshotPath": "snapshot_test_dir/test.stronghold"}}"#;
    let mnemonic = crypto::keys::bip39::Mnemonic::from(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_owned(),
    );

    let mut secret_manager: SecretManager = dto.parse()?;

    // The mnemonic only needs to be stored the first time
    if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
        secret_manager.store_mnemonic(mnemonic.clone()).await.unwrap();
    } else {
        panic!("expect a Stronghold secret manager, but it's not the case!");
    }

    let address = secret_manager
        .generate_ed25519_address(SHIMMER_COIN_TYPE, 0, 0, SHIMMER_TESTNET_BECH32_HRP, None)
        .await
        .unwrap();

    assert_eq!(
        address,
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a"
    );

    // Calling store_mnemonic() twice should fail, because we would otherwise overwrite the stored entry
    if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
        assert!(secret_manager.store_mnemonic(mnemonic).await.is_err());
    } else {
        panic!("expect a Stronghold secret manager, but it's not the case!");
    }

    // Remove garbage after test, but don't care about the result
    std::fs::remove_dir_all("snapshot_test_dir").ok();
    Ok(())
}

#[tokio::test]
async fn stronghold_mnemonic_missing() -> Result<(), ClientError> {
    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    // Cleanup of a possibly failed run
    std::fs::remove_dir_all("stronghold_mnemonic_missing").ok();

    let stronghold_secret_manager = iota_sdk::client::secret::stronghold::StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password".to_owned())
        .build("stronghold_mnemonic_missing/test.stronghold")?;

    // Generating addresses will fail because no mnemonic has been stored
    let error = SecretManager::Stronghold(stronghold_secret_manager)
        .generate_ed25519_address(SHIMMER_COIN_TYPE, 0, 0, SHIMMER_TESTNET_BECH32_HRP, None)
        .await
        .unwrap_err();

    match error {
        iota_sdk::client::ClientError::Stronghold(iota_sdk::client::stronghold::Error::MnemonicMissing) => {}
        _ => panic!("expected StrongholdMnemonicMissing error"),
    }

    // Remove garbage after test, but don't care about the result
    std::fs::remove_dir_all("stronghold_mnemonic_missing").ok();
    Ok(())
}
