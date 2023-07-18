// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::{
    client::{
        api::GetAddressesOptions,
        constants::{IOTA_BECH32_HRP, IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP, SHIMMER_BECH32_HRP, SHIMMER_COIN_TYPE},
        generate_mnemonic,
        secret::{GenerateAddressOptions, SecretManager},
        Client, Result,
    },
    types::block::address::{Address, Hrp},
};
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn ed25519_addresses() {
    let secret_manager = crate::client::node_api::setup_secret_manager();

    let opts = GetAddressesOptions::default()
        .with_bech32_hrp(IOTA_TESTNET_BECH32_HRP)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_range(0..1);
    let public = secret_manager.generate_ed25519_addresses(opts.clone()).await.unwrap();
    let internal = secret_manager
        .generate_ed25519_addresses(opts.internal())
        .await
        .unwrap();

    assert_eq!(
        public[0],
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
    );
    assert_eq!(
        internal[0],
        "atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak"
    );
}

#[tokio::test]
async fn evm_addresses() {
    let secret_manager = crate::client::node_api::setup_secret_manager();

    let opts = GetAddressesOptions::default()
        .with_bech32_hrp(IOTA_TESTNET_BECH32_HRP)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_range(0..1);
    let public = secret_manager.generate_evm_addresses(opts.clone()).await.unwrap();
    let internal = secret_manager.generate_evm_addresses(opts.internal()).await.unwrap();

    // Address generated with bip32 path: [44, 4218, 0, 0, 0].
    // This address was generated with a MnemonicSecretManager and verified with an outside source.
    // Seed: 0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2.
    assert_eq!(public[0], "0xb23e784f0464a30d536c961e414925eab6b3107d");
    assert_eq!(internal[0], "0x98d8833ec4b82587d66207eb9c578fd0134c51b6");
}

#[tokio::test]
async fn public_key_to_address() {
    let client = Client::builder().finish().await.unwrap();
    let hex_public_key = "0x2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a";

    let public_key_address = client
        .hex_public_key_to_bech32_address(hex_public_key, Some("atoi"))
        .await
        .unwrap();

    assert_eq!(
        public_key_address,
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
    );
}

#[tokio::test]
async fn mnemonic_address_generation_iota() {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_owned();
    let secret_manager = SecretManager::try_from_mnemonic(mnemonic).unwrap();

    // account 0, address 0 and 1
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(IOTA_BECH32_HRP)
                .with_coin_type(IOTA_COIN_TYPE)
                .with_range(0..2)
                .with_account_index(0),
        )
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg"
    );
    assert_eq!(
        addresses[1],
        "iota1qpswqe4v8z2cdtgc7sfj0hfneqh37lhmjgnth36mfndwcxkjrakcvpmm727"
    );

    // account 1
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(IOTA_BECH32_HRP)
                .with_coin_type(IOTA_COIN_TYPE)
                .with_range(0..1)
                .with_account_index(1),
        )
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "iota1qr43g007shcd7zx3xe7s4lu2c9fr33w7tfjppyy0swlhrxx247szqhuaeaa"
    );
}

#[tokio::test]
async fn mnemonic_address_generation_shimmer() {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_owned();
    let secret_manager = SecretManager::try_from_mnemonic(mnemonic).unwrap();

    // account 0, address 0 and 1
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..2)
                .with_account_index(0),
        )
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "smr1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6ckj80y"
    );
    assert_eq!(
        addresses[1],
        "smr1qznujl7m240za4pf6p0p8rdtqdca6tq7z44heqec8e57xsf429tvz0wt4w3"
    );

    // account 1
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_range(0..1)
                .with_account_index(1),
        )
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "smr1qrexl2g0m74v57y4kl6kfwqz7zrlrkvjt8m30av0cxgxlu92kyzc5npslm8"
    );
}

#[tokio::test]
async fn address_generation() {
    #[derive(Serialize, Deserialize)]
    struct AddressData {
        mnemonic: String,
        bech32_hrp: Hrp,
        coin_type: u32,
        account_index: u32,
        internal: bool,
        address_index: u32,
        ed25519_address: String,
        bech32_address: String,
    }

    let file = std::fs::File::open("./tests/client/fixtures/test_vectors.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let general = json.get("general").unwrap();
    let addresses_data: Vec<AddressData> =
        serde_json::from_value(general.get("address_generations").unwrap().clone()).unwrap();

    for address in &addresses_data {
        let secret_manager = SecretManager::try_from_mnemonic(address.mnemonic.clone()).unwrap();
        let addresses = secret_manager
            .generate_ed25519_addresses(
                GetAddressesOptions::default()
                    .with_bech32_hrp(address.bech32_hrp)
                    .with_coin_type(address.coin_type)
                    .with_range(address.address_index..address.address_index + 1)
                    .with_account_index(address.account_index)
                    .with_options(GenerateAddressOptions {
                        internal: address.internal,
                        ..Default::default()
                    }),
            )
            .await
            .unwrap();

        assert_eq!(addresses[0], address.bech32_address);
        if let Address::Ed25519(ed25519_address) = addresses[0].inner() {
            assert_eq!(ed25519_address.to_string(), address.ed25519_address);
        } else {
            panic!("Invalid address type")
        }
    }

    #[cfg(feature = "stronghold")]
    {
        iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
        for address in &addresses_data {
            let stronghold_filename = format!("{}.stronghold", address.bech32_address);
            let stronghold_secret_manager = StrongholdSecretManager::builder()
                .password("some_hopefully_secure_password".to_owned())
                .build(&stronghold_filename)
                .unwrap();

            stronghold_secret_manager
                .store_mnemonic(Mnemonic::from(address.mnemonic.as_str()))
                .await
                .unwrap();

            let addresses = SecretManager::Stronghold(stronghold_secret_manager)
                .generate_ed25519_addresses(
                    GetAddressesOptions::default()
                        .with_bech32_hrp(address.bech32_hrp)
                        .with_coin_type(address.coin_type)
                        .with_range(address.address_index..address.address_index + 1)
                        .with_account_index(address.account_index)
                        .with_options(GenerateAddressOptions {
                            internal: address.internal,
                            ..Default::default()
                        }),
                )
                .await
                .unwrap();

            assert_eq!(addresses[0], address.bech32_address);
            if let Address::Ed25519(ed25519_address) = addresses[0].inner() {
                assert_eq!(ed25519_address.to_string(), address.ed25519_address);
            } else {
                panic!("Invalid address type")
            }
            std::fs::remove_file(stronghold_filename).unwrap();
        }
    }
}

#[tokio::test]
async fn search_address() -> Result<()> {
    let client = Client::builder().finish().await.unwrap();

    let secret_manager = SecretManager::try_from_mnemonic(generate_mnemonic()?)?;

    // Public
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .with_coin_type(IOTA_COIN_TYPE)
                .with_account_index(0)
                .with_range(9..10)
                .with_bech32_hrp(IOTA_BECH32_HRP),
        )
        .await?;

    let res = iota_sdk::client::api::search_address(
        &secret_manager,
        IOTA_BECH32_HRP,
        IOTA_COIN_TYPE,
        0,
        0..10,
        &addresses[0],
    )
    .await?;

    assert_eq!(res, (9, false));

    // Internal
    let addresses = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::from_client(&client)
                .await?
                .internal()
                .with_coin_type(IOTA_COIN_TYPE)
                .with_account_index(0)
                .with_range(9..10)
                .with_bech32_hrp(IOTA_BECH32_HRP),
        )
        .await?;

    let res = iota_sdk::client::api::search_address(
        &secret_manager,
        IOTA_BECH32_HRP,
        IOTA_COIN_TYPE,
        0,
        0..10,
        &addresses[0],
    )
    .await?;

    assert_eq!(res, (9, true));

    // not in range
    let res =
        iota_sdk::client::api::search_address(&secret_manager, IOTA_BECH32_HRP, IOTA_COIN_TYPE, 0, 0..9, &addresses[0])
            .await;

    match res {
        Err(iota_sdk::client::Error::InputAddressNotFound { .. }) => {}
        _ => panic!("should not have found search address range & public"),
    }

    Ok(())
}
