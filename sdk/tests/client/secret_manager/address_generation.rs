// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "stronghold")]
use crypto::keys::bip39::Mnemonic;
use crypto::keys::bip44::Bip44;
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::{ledger_nano::LedgerSecretManager, GenerateAddressOptions};
#[cfg(feature = "events")]
use iota_sdk::wallet::events::{WalletEvent, WalletEventType};
use iota_sdk::{
    client::{
        api::GetAddressesOptions,
        constants::IOTA_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        ClientError,
    },
    types::block::address::{Hrp, ToBech32Ext},
    wallet::{ClientOptions, Wallet},
};
use pretty_assertions::assert_eq;

use crate::client::common::{setup, tear_down, DEFAULT_MNEMONIC, NODE_LOCAL};

#[tokio::test]
async fn address_generation_mnemonic() -> Result<(), Box<dyn std::error::Error>> {
    let secret_manager =
        SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(DEFAULT_MNEMONIC.to_owned())?);

    let address = secret_manager
        .generate_ed25519_address(IOTA_COIN_TYPE, 0, 0, "smr", None)
        .await?;

    assert_eq!(
        address,
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    Ok(())
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn address_generation_stronghold() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/address_generation_stronghold";
    setup(storage_path)?;

    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let secret_manager = StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password".to_owned())
        .build(format!("{}/test.stronghold", storage_path))?;

    secret_manager
        .store_mnemonic(Mnemonic::from(DEFAULT_MNEMONIC.to_string()))
        .await?;

    let secret_manager = SecretManager::Stronghold(secret_manager);

    let address = secret_manager
        .generate_ed25519_address(IOTA_COIN_TYPE, 0, 0, "smr", None)
        .await?;

    assert_eq!(
        address,
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    tear_down(storage_path)
}

#[tokio::test]
#[cfg(feature = "ledger_nano")]
#[ignore = "requires ledger nano instance"]
async fn address_generation_ledger() -> Result<(), Box<dyn std::error::Error>> {
    let mut secret_manager = LedgerSecretManager::new(true);
    secret_manager.non_interactive = true;

    let secret_manager = SecretManager::LedgerNano(secret_manager);

    let address = secret_manager
        .generate_ed25519_address(IOTA_COIN_TYPE, 0, 0, "smr", None)
        .await?;

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    Ok(())
}

#[tokio::test]
async fn address_generation_placeholder() {
    let secret_manager = SecretManager::Placeholder;

    assert!(matches!(
        secret_manager
            .generate_ed25519_address(IOTA_COIN_TYPE, 0, 0, "smr", None)
            .await,
        Err(ClientError::PlaceholderSecretManager)
    ));
}
