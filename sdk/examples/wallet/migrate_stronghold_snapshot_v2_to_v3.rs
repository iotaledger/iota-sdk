// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --release --all-features --example migrate_stronghold_snapshot_v2_to_v3

use iota_sdk::client::{
    api::GetAddressesOptions,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{stronghold::StrongholdSecretManager, SecretManager},
    stronghold::StrongholdAdapter,
    Result,
};

const V2_PATH: &str = "./tests/wallet/fixtures/v2.stronghold";
const V3_PATH: &str = "./v3.stronghold";

#[tokio::main]
async fn main() -> Result<()> {
    // This should fail with error, migration required.
    let error = if let Err(e) = StrongholdSecretManager::builder()
        .password("current_password".to_owned())
        .build(V2_PATH)
    {
        e
    } else {
        panic!("should be an error");
    };
    println!("Creating a stronghold failed with error: {error:?}");

    println!("Migrating snapshot from v2 to v3");
    StrongholdAdapter::migrate_snapshot_v2_to_v3(
        V2_PATH,
        "current_password".to_owned().into(),
        "wallet.rs",
        100,
        Some(V3_PATH),
        Some("new_password".to_owned().into()),
    )
    .unwrap();

    // This shouldn't fail anymore as snapshot has been migrated.
    let stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password".to_owned())
        .build(V3_PATH)?;

    // Generate addresses with custom account index and range
    let addresses = SecretManager::Stronghold(stronghold_secret_manager)
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
                .with_coin_type(SHIMMER_COIN_TYPE)
                .with_account_index(0)
                .with_range(0..1),
        )
        .await
        .unwrap();

    println!("First public address: {}", addresses[0]);

    Ok(())
}
