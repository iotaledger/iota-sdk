// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "stronghold")]
use crypto::keys::bip39::Mnemonic;
use crypto::keys::bip44::Bip44;
#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::ledger_nano::{LedgerOptions, LedgerSecretManager};
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
#[cfg(feature = "events")]
use iota_sdk::wallet::events::{WalletEvent, WalletEventType};
use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{mnemonic::MnemonicSecretManager, PublicKeyOptions, SecretManageExt},
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
    wallet::{ClientOptions, Result, Wallet, WalletBuilder},
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{setup, tear_down, DEFAULT_MNEMONIC, NODE_LOCAL};

#[tokio::test]
async fn wallet_address_generation_mnemonic() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_mnemonic";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(DEFAULT_MNEMONIC.to_owned())?;

    #[allow(unused_mut)]
    let mut wallet_builder = WalletBuilder::new()
        .with_secret_manager(secret_manager)
        .with_client_options(client_options)
        .with_public_key_options(PublicKeyOptions::new(IOTA_COIN_TYPE))
        .with_signing_options(Bip44::new(IOTA_COIN_TYPE));

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let address = (*wallet.secret_manager().read().await)
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await?;

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
    let mut wallet_builder = WalletBuilder::new()
        .with_secret_manager(secret_manager)
        .with_client_options(client_options)
        .with_public_key_options(PublicKeyOptions::new(IOTA_COIN_TYPE))
        .with_signing_options(Bip44::new(IOTA_COIN_TYPE));
    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let address = (*wallet.secret_manager().read().await)
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await?;

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        "smr1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3ulygj9p"
    );

    tear_down(storage_path)
}

#[tokio::test]
#[cfg(all(feature = "ledger_nano", feature = "events"))]
#[ignore = "requires ledger nano instance"]
async fn wallet_address_generation_ledger() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_ledger";
    setup(storage_path)?;

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;
    let mut secret_manager = LedgerSecretManager::new(true);
    secret_manager.non_interactive = true;

    #[allow(unused_mut)]
    let mut wallet_builder = WalletBuilder::new()
        .with_secret_manager(secret_manager)
        .with_client_options(client_options)
        .with_public_key_options(LedgerOptions::new(PublicKeyOptions::new(IOTA_COIN_TYPE)))
        .with_signing_options(Bip44::new(IOTA_COIN_TYPE));

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    let address = (*wallet.secret_manager().read().await)
        .generate::<Ed25519Address>(&LedgerOptions::new(PublicKeyOptions::new(IOTA_COIN_TYPE)))
        .await?
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

    wallet
        .listen([WalletEventType::LedgerAddressGeneration], move |event| {
            if let WalletEvent::LedgerAddressGeneration(address) = event {
                sender
                    .try_send(address.address.clone())
                    .expect("too many LedgerAddressGeneration events");
            } else {
                panic!("expected LedgerAddressGeneration event")
            }
        })
        .await;

    let address = (*wallet.secret_manager().read().await)
        .generate::<Ed25519Address>(
            &LedgerOptions::new(PublicKeyOptions::new(IOTA_COIN_TYPE)).with_ledger_nano_prompt(true),
        )
        .await?
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    assert_eq!(
        address.to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    assert_eq!(
        receiver
            .recv()
            .await
            .expect("never received event")
            .into_inner()
            .to_bech32_unchecked("smr"),
        // Address generated with bip32 path: [44, 4218, 0, 0, 0].
        // This address was generated with a MnemonicSecretManager and the ledger simulator mnemonic.
        // "glory promote mansion idle axis finger extra february uncover one trip resource lawn turtle enact monster
        // seven myth punch hobby comfort wild raise skin".
        "smr1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mruy3qdpm"
    );

    tear_down(storage_path)
}

// #[tokio::test]
// async fn wallet_address_generation_placeholder() -> Result<()> {
//     let storage_path = "test-storage/wallet_address_generation_placeholder";
//     setup(storage_path)?;

//     let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

//     #[allow(unused_mut)]
//     let mut wallet_builder = Wallet::builder()
//         .with_secret_manager(SecretManager::Placeholder)
//         .with_client_options(client_options)
//         .with_bip_path(Bip44::new(IOTA_COIN_TYPE));

//     #[cfg(feature = "storage")]
//     {
//         wallet_builder = wallet_builder.with_storage_path(storage_path);
//     }
//     let wallet = wallet_builder.finish().await?;

//     if let Err(Error::Client(error)) = wallet.generate_ed25519_address(0, 0, None).await {
//         assert!(matches!(*error, ClientError::PlaceholderSecretManager))
//     } else {
//         panic!("expected PlaceholderSecretManager")
//     }

//     tear_down(storage_path)
// }

#[tokio::test]
async fn wallet_address_generation_custom_secret_manager() -> Result<()> {
    let storage_path = "test-storage/wallet_address_generation_custom_secret_manager";
    setup(storage_path)?;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct CustomSecretManager {
        pub public_key: String,
        // Will obviously be invalid, just to have something
        pub signature: String,
    }

    #[async_trait::async_trait]
    impl iota_sdk::client::secret::Generate<crypto::signatures::ed25519::PublicKey> for CustomSecretManager {
        type Options = ();

        async fn generate(
            &self,
            _options: &Self::Options,
        ) -> iota_sdk::client::Result<crypto::signatures::ed25519::PublicKey> {
            Ok(crypto::signatures::ed25519::PublicKey::try_from_bytes(
                prefix_hex::decode(&self.public_key)?,
            )?)
        }
    }

    #[async_trait::async_trait]
    impl iota_sdk::client::secret::Sign<iota_sdk::types::block::signature::Ed25519Signature> for CustomSecretManager {
        type Options = ();

        async fn sign(
            &self,
            _msg: &[u8],
            _options: &Self::Options,
        ) -> iota_sdk::client::Result<iota_sdk::types::block::signature::Ed25519Signature> {
            Ok(iota_sdk::types::block::signature::Ed25519Signature::try_from_bytes(
                prefix_hex::decode(&self.public_key)?,
                prefix_hex::decode(&self.signature)?,
            )?)
        }
    }
    impl iota_sdk::client::secret::SignTransaction for CustomSecretManager {}

    impl iota_sdk::client::secret::SecretManagerConfig for CustomSecretManager {
        type Config = String;

        fn to_config(&self) -> Option<Self::Config> {
            Some(serde_json::to_string(self).unwrap())
        }

        fn from_config(config: &Self::Config) -> iota_sdk::client::Result<Self> {
            Ok(serde_json::from_str(config)?)
        }
    }

    let custom_secret_manager = CustomSecretManager {
        public_key: "0x503b258b32c586e2c66c99d3af45086d1c96fbcd86b3d04f464081589d1a51b2".to_string(),
        signature: "0xbb36dc62c92d35175b6ccee15341a776d188a71c50fed86204ca01555cd344303611a836c546c7fcfa983af75fe941ae1533a10d692ccd0008578b351b170f03".to_string(),
    };

    assert_eq!(
        Ed25519Address::from_public_key_bytes(
            custom_secret_manager
                .generate::<crypto::signatures::ed25519::PublicKey>(&())
                .await?
                .to_bytes()
        ),
        <Ed25519Address as std::str::FromStr>::from_str(
            "0x69da7d3cf43670a6585763eb05d4a9272d424bcc921d550fd726a183501a8539"
        )
        .unwrap()
    );

    let client_options = ClientOptions::new().with_node(NODE_LOCAL)?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::<()>::builder()
        .with_secret_manager(custom_secret_manager)
        .with_public_key_options(()) // TODO: Should we have a default bound somewhere?
        .with_signing_options(())
        .with_client_options(client_options);

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }
    let wallet = wallet_builder.finish().await?;

    assert_eq!(
        *wallet.address().await.inner().as_ed25519(),
        <Ed25519Address as std::str::FromStr>::from_str(
            "0x69da7d3cf43670a6585763eb05d4a9272d424bcc921d550fd726a183501a8539"
        )
        .unwrap()
    );

    tear_down(storage_path)
}
