// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Range;

#[cfg(feature = "stronghold")]
use crypto::keys::bip39::Mnemonic;
use crypto::signatures::secp256k1_ecdsa::EvmAddress;
#[cfg(feature = "stronghold")]
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::{
    client::{
        constants::{IOTA_BECH32_HRP, IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP, SHIMMER_BECH32_HRP, SHIMMER_COIN_TYPE},
        generate_mnemonic,
        secret::{mnemonic::MnemonicSecretManager, MultiKeyOptions, PublicKeyOptions, SecretManageExt},
        Client, Result,
    },
    types::block::address::{Address, Ed25519Address, Hrp, ToBech32Ext},
};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn ed25519_addresses() {
    let secret_manager = crate::client::node_api::setup_secret_manager();

    let public = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await
        .unwrap()
        .to_bech32(IOTA_TESTNET_BECH32_HRP);
    let internal = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE).with_internal(true))
        .await
        .unwrap()
        .to_bech32(IOTA_TESTNET_BECH32_HRP);

    assert_eq!(
        public,
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
    );
    assert_eq!(
        internal,
        "atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak"
    );
}

#[tokio::test]
async fn evm_addresses() {
    let secret_manager = crate::client::node_api::setup_secret_manager();

    let public = secret_manager
        .generate::<EvmAddress>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await
        .unwrap();
    let internal = secret_manager
        .generate::<EvmAddress>(&PublicKeyOptions::new(IOTA_COIN_TYPE).with_internal(true))
        .await
        .unwrap();

    // Address generated with bip32 path: [44, 4218, 0, 0, 0].
    // This address was generated with a MnemonicSecretManager and verified with an outside source.
    // Seed: 0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2.
    assert_eq!(
        prefix_hex::encode(public.as_ref()),
        "0xb23e784f0464a30d536c961e414925eab6b3107d"
    );
    assert_eq!(
        prefix_hex::encode(internal.as_ref()),
        "0x98d8833ec4b82587d66207eb9c578fd0134c51b6"
    );
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
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

    // account 0, address 0 and 1
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(&MultiKeyOptions::new(IOTA_COIN_TYPE).with_address_range(0..2))
        .await
        .unwrap()
        .into_iter()
        .map(|a| a.to_bech32(IOTA_BECH32_HRP))
        .collect::<Vec<_>>();

    assert_eq!(
        addresses[0],
        "iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg"
    );
    assert_eq!(
        addresses[1],
        "iota1qpswqe4v8z2cdtgc7sfj0hfneqh37lhmjgnth36mfndwcxkjrakcvpmm727"
    );

    // account 1
    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await
        .unwrap()
        .to_bech32(IOTA_BECH32_HRP);

    assert_eq!(
        address,
        "iota1qr43g007shcd7zx3xe7s4lu2c9fr33w7tfjppyy0swlhrxx247szqhuaeaa"
    );
}

#[tokio::test]
async fn mnemonic_address_generation_shimmer() {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_owned();
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

    // account 0, address 0 and 1
    let addresses = secret_manager
        .generate::<Vec<Ed25519Address>>(&MultiKeyOptions::new(SHIMMER_COIN_TYPE).with_address_range(0..2))
        .await
        .unwrap()
        .into_iter()
        .map(|a| a.to_bech32(SHIMMER_BECH32_HRP))
        .collect::<Vec<_>>();

    assert_eq!(
        addresses[0],
        "smr1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6ckj80y"
    );
    assert_eq!(
        addresses[1],
        "smr1qznujl7m240za4pf6p0p8rdtqdca6tq7z44heqec8e57xsf429tvz0wt4w3"
    );

    // account 1
    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(SHIMMER_COIN_TYPE))
        .await
        .unwrap()
        .to_bech32(SHIMMER_BECH32_HRP);

    assert_eq!(
        address,
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

    for address_data in &addresses_data {
        let secret_manager = MnemonicSecretManager::try_from_mnemonic(address_data.mnemonic.clone()).unwrap();
        let address = secret_manager
            .generate::<Ed25519Address>(
                &PublicKeyOptions::new(address_data.coin_type)
                    .with_account_index(address_data.account_index)
                    .with_address_index(address_data.address_index)
                    .with_internal(address_data.internal),
            )
            .await
            .unwrap()
            .to_bech32(address_data.bech32_hrp);

        assert_eq!(address, address_data.bech32_address);
        if let Address::Ed25519(ed25519_address) = address.inner() {
            assert_eq!(ed25519_address.to_string(), address_data.ed25519_address);
        } else {
            panic!("Invalid address type")
        }
    }

    #[cfg(feature = "stronghold")]
    {
        iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();
        for address_data in &addresses_data {
            let stronghold_filename = format!("{}.stronghold", address_data.bech32_address);
            let stronghold_secret_manager = StrongholdSecretManager::builder()
                .password("some_hopefully_secure_password".to_owned())
                .build(&stronghold_filename)
                .unwrap();

            stronghold_secret_manager
                .store_mnemonic(Mnemonic::from(address_data.mnemonic.as_str()))
                .await
                .unwrap();

            let address = stronghold_secret_manager
                .generate::<Ed25519Address>(
                    &PublicKeyOptions::new(address_data.coin_type)
                        .with_account_index(address_data.account_index)
                        .with_address_index(address_data.address_index)
                        .with_internal(address_data.internal),
                )
                .await
                .unwrap()
                .to_bech32(address_data.bech32_hrp);

            assert_eq!(address, address_data.bech32_address);
            if let Address::Ed25519(ed25519_address) = address.inner() {
                assert_eq!(ed25519_address.to_string(), address_data.ed25519_address);
            } else {
                panic!("Invalid address type")
            }
            std::fs::remove_file(stronghold_filename).unwrap();
        }
    }
}

#[tokio::test]
async fn address_search() -> Result<()> {
    let client = Client::builder().finish().await.unwrap();

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(generate_mnemonic()?)?;

    // Public
    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE).with_address_index(9))
        .await
        .unwrap();

    let res = search_address(&secret_manager, IOTA_BECH32_HRP, IOTA_COIN_TYPE, 0, 0..10, &address).await?;

    assert_eq!(res, (9, false));

    // Internal
    let address = secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE).with_address_index(9))
        .await?;

    let res = search_address(&secret_manager, IOTA_BECH32_HRP, IOTA_COIN_TYPE, 0, 0..10, &address).await?;

    assert_eq!(res, (9, true));

    // not in range
    let res = search_address(&secret_manager, IOTA_BECH32_HRP, IOTA_COIN_TYPE, 0, 0..9, &address).await;

    match res {
        Err(iota_sdk::client::Error::InputAddressNotFound { .. }) => {}
        _ => panic!("should not have found search address range & public"),
    }

    Ok(())
}

pub async fn search_address<S: iota_sdk::client::secret::Generate<Vec<Ed25519Address>, Options = MultiKeyOptions>>(
    secret_manager: &S,
    bech32_hrp: Hrp,
    coin_type: u32,
    account_index: u32,
    range: Range<u32>,
    address: &Ed25519Address,
) -> Result<(u32, bool)> {
    use iota_sdk::client::secret::Generate;
    let mut opts = MultiKeyOptions::new(coin_type)
        .with_account_index(account_index)
        .with_address_range(range.clone());
    let public = Generate::<Vec<Ed25519Address>>::generate(secret_manager, &opts).await?;
    opts = opts.with_internal(true);
    let internal = Generate::<Vec<Ed25519Address>>::generate(secret_manager, &opts).await?;
    for index in 0..public.len() {
        if &public[index] == address {
            return Ok((range.start + index as u32, false));
        }
        if &internal[index] == address {
            return Ok((range.start + index as u32, true));
        }
    }
    Err(iota_sdk::client::Error::InputAddressNotFound {
        address: address.clone().to_bech32(bech32_hrp).to_string(),
        range: format!("{range:?}"),
    })
}
