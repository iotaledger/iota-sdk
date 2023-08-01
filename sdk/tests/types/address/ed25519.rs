// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::address::{Address, Bech32Address, Ed25519Address, ToBech32Ext};
use packable::PackableExt;
use serde_json::json;

const ED25519_ADDRESS: &str = "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655";
const ED25519_BECH32: &str = "rms1qr47gz3xxjqpjrwd0yu5glhqrth6w0t08npney8000ust2lcw2r92j5a8rt";
const ED25519_ADDRESS_INVALID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64";

#[test]
fn kind() {
    assert_eq!(Ed25519Address::KIND, 0);

    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(address.kind(), Ed25519Address::KIND);
}

#[test]
fn length() {
    assert_eq!(Ed25519Address::LENGTH, 32);
}

#[test]
fn is_methods() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert!(address.is_ed25519());
    assert!(!address.is_account());
    assert!(!address.is_nft());
}

#[test]
fn as_methods() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let address = Address::from(ed25519_address);

    assert_eq!(address.as_ed25519(), &ed25519_address);
    assert!(std::panic::catch_unwind(|| address.as_account()).is_err());
    assert!(std::panic::catch_unwind(|| address.as_nft()).is_err());
}

#[test]
fn new_bytes() {
    let bytes = prefix_hex::decode(ED25519_ADDRESS).unwrap();
    let ed25519_address = Ed25519Address::new(bytes);

    assert_eq!(ed25519_address.as_ref(), &bytes);
}

#[test]
fn from_str_to_str() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();

    assert_eq!(ed25519_address.to_string(), ED25519_ADDRESS);
}

#[test]
fn debug() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();

    assert_eq!(
        format!("{ed25519_address:?}"),
        "Ed25519Address(0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655)"
    );
}

#[test]
fn bech32() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(address.to_bech32_unchecked("rms"), ED25519_BECH32);
}

#[test]
fn bech32_roundtrip() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());
    let bech32 = address.to_bech32_unchecked("rms").to_string();

    assert_eq!(
        Bech32Address::try_from_str(bech32),
        Bech32Address::try_new("rms", address)
    );
}

#[test]
fn serde_fields() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let ed25519_address_ser = serde_json::to_value(ed25519_address).unwrap();

    assert_eq!(
        ed25519_address_ser,
        json!({
            "type": Ed25519Address::KIND,
            "pubKeyHash": ED25519_ADDRESS,
        })
    );

    let address = Address::from(ed25519_address);
    let address_ser = serde_json::to_value(address).unwrap();

    assert_eq!(address_ser, ed25519_address_ser);
}

#[test]
fn serde_roundtrip() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let ed25519_address_ser = serde_json::to_string(&ed25519_address).unwrap();

    assert_eq!(
        serde_json::from_str::<Ed25519Address>(&ed25519_address_ser).unwrap(),
        ed25519_address
    );

    let address = Address::from(ed25519_address);
    let address_ser = serde_json::to_string(&address).unwrap();

    assert_eq!(serde_json::from_str::<Address>(&address_ser).unwrap(), address);
}

#[test]
fn serde_invalid_ed25519_address() {
    let ed25519_address_ser = json!({
        "type": Ed25519Address::KIND,
        "pubKeyHash": ED25519_ADDRESS_INVALID,
    });

    assert!(matches!(
        serde_json::from_value::<Ed25519Address>(ed25519_address_ser),
        Err(e) if e.to_string() == "hex error: Invalid hex string length for slice: expected 64 got 63"
    ));
}

#[test]
fn packed_len() {
    let address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();

    assert_eq!(address.packed_len(), Ed25519Address::LENGTH);
    assert_eq!(address.pack_to_vec().len(), Ed25519Address::LENGTH);

    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(address.packed_len(), 1 + Ed25519Address::LENGTH);
    assert_eq!(address.pack_to_vec().len(), 1 + Ed25519Address::LENGTH);
}

#[test]
fn pack_unpack() {
    let address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        Ed25519Address::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );

    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        Address::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );
}
