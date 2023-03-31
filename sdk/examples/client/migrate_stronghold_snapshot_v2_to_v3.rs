// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migrate_stronghold_snapshot_v2_to_v3 --features=stronghold --release

use iota_sdk::client::{
    api::GetAddressesBuilder,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{stronghold::StrongholdSecretManager, SecretManager},
    stronghold::StrongholdAdapter,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let error = if let Err(e) = StrongholdSecretManager::builder()
        .password("current_password")
        .build("test.stronghold")
    {
        e
    } else {
        panic!("should be an error");
    };
    println!("Creating a stronghold failed with error: {error:?}");

    println!("Migrating snapshot from v2 to v3");
    StrongholdAdapter::migrate_v2_to_v3("test.stronghold", "current_password", None, Some("new_password")).unwrap();

    let stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("new_password")
        .build("test.stronghold")?;

    // Generate addresses with custom account index and range
    let addresses = GetAddressesBuilder::new(&SecretManager::Stronghold(stronghold_secret_manager))
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await?;

    println!("First public address: {}", addresses[0]);

    Ok(())
}
