// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
use iota_sdk::client::{Client, Result};

#[tokio::test]
async fn mnemonic() -> Result<()> {
    let mnemonic = Client::generate_mnemonic()?;
    assert!(Client::mnemonic_to_hex_seed(mnemonic).is_ok());
    assert!(Client::mnemonic_to_hex_seed(Mnemonic::from("until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket".to_owned())).is_ok());
    assert!(Client::mnemonic_to_hex_seed(Mnemonic::from("fire until hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket".to_owned())).is_err());
    assert!(Client::mnemonic_to_hex_seed(Mnemonic::from("invalid mnemonic".to_owned())).is_err());
    Ok(())
}
