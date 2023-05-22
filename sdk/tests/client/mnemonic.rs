// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::client::{Client, Result, secret::types::Mnemonic};

#[tokio::test]
async fn mnemonic() -> Result<()> {
    let mnemonic = Client::generate_mnemonic()?;
    assert!(!Client::mnemonic_to_hex_seed(&mnemonic).is_empty());
    // assert!(Mnemonic::try_from("until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket".to_owned()).is_ok());
    // assert!(Mnemonic::try_from("fire until hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket".to_owned()).is_err());
    // assert!(Mnemonic::try_from("invalid mnemonic".to_owned()).is_err());
    // mnemonic with space at the beginning or end should return the same as without
    let mnemonic = "until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket".to_owned();
    let mnemonic_with_spaces = " until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket ".to_owned();
    assert_eq!(
        Mnemonic::from(mnemonic),
        Mnemonic::from(mnemonic_with_spaces)
    );
    Ok(())
}
