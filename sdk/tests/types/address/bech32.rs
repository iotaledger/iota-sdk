// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::address::{Address, Bech32Address, Ed25519Address};

const ED25519_ADDRESS: &str = "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655";
const ED25519_BECH32: &str = "rms1qr47gz3xxjqpjrwd0yu5glhqrth6w0t08npney8000ust2lcw2r92j5a8rt";

#[test]
fn ctors() {
    let ed25519_address = ED25519_ADDRESS.parse::<Ed25519Address>().unwrap();
    let bech32_address_1 = Bech32Address::new("rms".to_string(), Address::Ed25519(ed25519_address)).unwrap();
    assert_eq!(bech32_address_1.inner().kind(), Ed25519Address::KIND);
    assert_eq!(bech32_address_1.hrp(), "rms");

    let bech32_address_test = ED25519_BECH32.parse::<Bech32Address>().unwrap();
    assert_eq!(bech32_address_1, bech32_address_test);

    let bech32_address_2 = Bech32Address::try_from_str(ED25519_BECH32).unwrap();
    assert_eq!(bech32_address_1, bech32_address_2);
}
