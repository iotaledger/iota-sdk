// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will migrate a snapshot from V2 to V3.
//!
//! ```sh
//! cargo run --release --all-features --example migrate_stronghold_snapshot_v2_to_v3
//! ```

use iota_sdk::{
    client::{
        constants::{IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP},
        secret::{stronghold::StrongholdSecretManager, PublicKeyOptions, SecretManageExt},
        stronghold::StrongholdAdapter,
        Result,
    },
    types::block::address::{Ed25519Address, ToBech32Ext},
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
    let address = stronghold_secret_manager
        .generate::<Ed25519Address>(&PublicKeyOptions::new(IOTA_COIN_TYPE))
        .await
        .unwrap()
        .to_bech32(IOTA_TESTNET_BECH32_HRP);

    println!("First public address: {address}");

    Ok(())
}
