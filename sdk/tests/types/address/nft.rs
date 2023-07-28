// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::types::block::{
    address::{Address, Bech32Address, NftAddress, ToBech32Ext},
    output::NftId,
};
use packable::PackableExt;
use serde_json::json;

const NFT_ID: &str = "0xa9ede98a7f0223fa7a49fbc586f7a88bb4f0d152f282b19bcebd05c9e8a02370";
const NFT_BECH32: &str = "rms1zz57m6v20upz87n6f8autphh4z9mfux32teg9vvme67stj0g5q3hqd6l53z";
const NFT_ID_INVALID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86e";

#[test]
fn kind() {
    assert_eq!(NftAddress::KIND, 16);

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(address.kind(), NftAddress::KIND);
}

#[test]
fn length() {
    assert_eq!(NftAddress::LENGTH, 32);
}

#[test]
fn is_methods() {
    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert!(!address.is_ed25519());
    assert!(!address.is_account());
    assert!(address.is_nft());
}

#[test]
fn as_methods() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();
    let address = Address::from(nft_address);

    assert!(std::panic::catch_unwind(|| address.as_ed25519()).is_err());
    assert!(std::panic::catch_unwind(|| address.as_account()).is_err());
    assert_eq!(address.as_nft(), &nft_address);
}

#[test]
fn new_nft_id() {
    let nft_id = NftId::from_str(NFT_ID).unwrap();
    let nft_address = NftAddress::new(nft_id);

    assert_eq!(nft_address.nft_id(), &nft_id);
}

#[test]
fn new_into_nft_id() {
    let nft_id = NftId::from_str(NFT_ID).unwrap();
    let nft_address = NftAddress::new(nft_id);

    assert_eq!(nft_address.into_nft_id(), nft_id);
}

#[test]
fn from_str_to_str() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();

    assert_eq!(nft_address.to_string(), NFT_ID);
}

#[test]
fn debug() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();

    assert_eq!(
        format!("{nft_address:?}"),
        "NftAddress(0xa9ede98a7f0223fa7a49fbc586f7a88bb4f0d152f282b19bcebd05c9e8a02370)"
    );
}

#[test]
fn bech32() {
    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(address.to_bech32_unchecked("rms"), NFT_BECH32);
}

#[test]
fn bech32_roundtrip() {
    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());
    let bech32 = address.to_bech32_unchecked("rms").to_string();

    assert_eq!(
        Bech32Address::try_from_str(bech32),
        Bech32Address::try_new("rms", address)
    );
}

#[test]
fn serde_fields() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();
    let nft_address_ser = serde_json::to_value(&nft_address).unwrap();

    assert_eq!(
        nft_address_ser,
        json!({
            "type": NftAddress::KIND,
            "nftId": NFT_ID,
        })
    );

    let address = Address::from(nft_address);
    let address_ser = serde_json::to_value(&address).unwrap();

    assert_eq!(address_ser, nft_address_ser);
}

#[test]
fn serde_roundtrip() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();
    let nft_address_ser = serde_json::to_string(&nft_address).unwrap();

    assert_eq!(
        serde_json::from_str::<NftAddress>(&nft_address_ser).unwrap(),
        nft_address
    );

    let address = Address::from(nft_address);
    let address_ser = serde_json::to_string(&address).unwrap();

    assert_eq!(serde_json::from_str::<Address>(&address_ser).unwrap(), address);
}

#[test]
fn serde_invalid_account_id() {
    let nft_address_ser = json!({
        "type": NftAddress::KIND,
        "nftId": NFT_ID_INVALID,
    });

    assert!(matches!(
        serde_json::from_value::<NftAddress>(nft_address_ser),
        Err(e) if e.to_string() == "hex error: Invalid hex string length for slice: expected 64 got 61"
    ));
}

#[test]
fn packed_len() {
    let address = NftAddress::from_str(NFT_ID).unwrap();

    assert_eq!(address.packed_len(), NftAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), NftAddress::LENGTH);

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(address.packed_len(), 1 + NftAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), 1 + NftAddress::LENGTH);
}

#[test]
fn pack_unpack() {
    let address = NftAddress::from_str(NFT_ID).unwrap();
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );
}
