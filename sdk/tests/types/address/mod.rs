// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod bech32;
mod ed25519;
mod multi;
mod nft;
mod restricted;

use core::str::FromStr;

use iota_sdk::types::block::{
    address::{AccountAddress, Address, AddressError, Ed25519Address, NftAddress},
    rand::address::{
        rand_account_address, rand_anchor_address, rand_ed25519_address, rand_implicit_address, rand_multi_address,
        rand_nft_address, rand_restricted_address,
    },
};
use pretty_assertions::assert_eq;

const ED25519_ADDRESS: &str = "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655";
const ACCOUNT_ID: &str = "0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee";
const NFT_ID: &str = "0xa9ede98a7f0223fa7a49fbc586f7a88bb4f0d152f282b19bcebd05c9e8a02370";
const ED25519_ADDRESS_INVALID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64x";

#[test]
fn invalid_bech32() {
    let address = Address::try_from_bech32(ED25519_ADDRESS_INVALID).unwrap_err();

    assert!(matches!(address, AddressError::Bech32Encoding(_)));
}

#[test]
fn debug() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(
        format!("{address:?}"),
        "Ed25519Address(0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655)"
    );

    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());

    assert_eq!(
        format!("{address:?}"),
        "AccountAddress(0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee)"
    );

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(
        format!("{address:?}"),
        "NftAddress(0xa9ede98a7f0223fa7a49fbc586f7a88bb4f0d152f282b19bcebd05c9e8a02370)"
    );
}

#[test]
fn is_valid_bech32() {
    assert!(Address::is_valid_bech32(
        "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy"
    ));

    assert!(!Address::is_valid_bech32(
        "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zY"
    ));
}

#[test]
fn address_display_similar() {
    let addresses: Vec<Address> = vec![
        rand_ed25519_address().into(),
        rand_account_address().into(),
        rand_nft_address().into(),
        rand_anchor_address().into(),
        rand_implicit_address().into(),
        rand_multi_address().into(),
        rand_restricted_address().into(),
    ];
    // Restricted address is 72 length, the rest 64.
    let regex_pattern = regex::Regex::new(r"^0x[0-9a-fA-F]{64,72}$").unwrap();

    // Check if all addresses match the regex pattern.
    assert!(
        addresses
            .iter()
            .all(|address| { regex_pattern.is_match(&address.to_string()) })
    );
}
