// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};

use crypto::keys::bip39::Mnemonic;
#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::ledger_nano::LedgerSecretManager;
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
#[cfg(feature = "events")]
use iota_sdk::wallet::events::{WalletEvent, WalletEventType};
use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, GenerateAddressOptions, SecretManager},
        Error as ClientError,
    },
    types::block::address::ToBech32Ext,
    wallet::{ClientOptions, Error, Result, Wallet},
};

use crate::wallet::common::{setup, tear_down, DEFAULT_MNEMONIC, NODE_LOCAL};

#[tokio::test]
async fn wallet_address_generation_mnemonic() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_mnemonic";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(DEFAULT_MNEMONIC.to_owned())?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(IOTA_COIN_TYPE);

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let address = wallet.generate_ed25519_address(0, 0, None).await?;

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    tear_down(storage_path)
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn wallet_address_generation_stronghold() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_stronghold";
    setup(storage_path)?;

    iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

    let secret_manager = StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password".to_owned())
        .build("test-storage/wallet_address_generation_stronghold/test.stronghold")?;
    secret_manager
        .store_mnemonic(Mnemonic::from(DEFAULT_MNEMONIC.to_string()))
        .await?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;
    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(IOTA_COIN_TYPE);
    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let address = wallet.generate_ed25519_address(0, 0, None).await?;

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    tear_down(storage_path)
}

#[tokio::test]
#[cfg(feature = "ledger_nano")]
#[ignore = "requires ledger nano instance"]
async fn wallet_address_generation_ledger() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_ledger";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;
    let mut secret_manager = LedgerSecretManager::new(true);
    secret_manager.non_interactive = true;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::LedgerNano(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(IOTA_COIN_TYPE);

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let address = wallet.generate_ed25519_address(0, 0, None).await?;

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    let address_event = Arc::new(Mutex::new(None));
    let address_event_clone = address_event.clone();

    #[cfg(feature = "events")]
    wallet
        .listen([WalletEventType::LedgerAddressGeneration], move |event| {
            if let WalletEvent::LedgerAddressGeneration(address) = &event.event {
                *address_event_clone.lock().unwrap() = Some(address.address);
            } else {
                panic!("expected LedgerAddressGeneration")
            }
        })
        .await;

    let address = wallet
        .generate_ed25519_address(
            0,
            0,
            Some(GenerateAddressOptions {
                ledger_nano_prompt: true,
                ..Default::default()
            }),
        )
        .await?;

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    assert_eq!(
        address_event
            .lock()
            .unwrap()
            .unwrap()
            .inner()
            .to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    tear_down(storage_path)
}

#[tokio::test]
async fn wallet_address_generation_placeholder() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_placeholder";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Placeholder)
        .with_client_options(client_options)
        .with_coin_type(IOTA_COIN_TYPE);

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    if let Err(Error::Client(error)) = wallet.generate_ed25519_address(0, 0, None).await {
        assert!(matches!(*error, ClientError::PlaceholderSecretManager))
    } else {
        panic!("expected PlaceholderSecretManager")
    }

    tear_down(storage_path)
}
