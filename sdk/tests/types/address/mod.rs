// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod bech32;
mod ed25519;
mod nft;

use core::str::FromStr;

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    signatures::ed25519::PublicKey,
};
use iota_sdk::types::block::{
    address::{
        AccountAddress, Address, CapabilityFlag, Ed25519Address, ImplicitAccountCreationAddress, NftAddress,
        RestrictedAddress, ToBech32Ext,
    },
    rand::address::rand_ed25519_address,
    Error,
};
use packable::PackableExt;

const ED25519_ADDRESS: &str = "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655";
const ALIAS_ID: &str = "0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee";
const NFT_ID: &str = "0xa9ede98a7f0223fa7a49fbc586f7a88bb4f0d152f282b19bcebd05c9e8a02370";
const ED25519_ADDRESS_INVALID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64x";

#[test]
fn invalid_bech32() {
    let address = Address::try_from_bech32(ED25519_ADDRESS_INVALID);

    assert!(matches!(address, Err(Error::InvalidAddress)));
}

#[test]
fn debug() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(
        format!("{address:?}"),
        "Ed25519Address(0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655)"
    );

    let address = Address::from(AccountAddress::from_str(ALIAS_ID).unwrap());

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
fn capabilities() {
    let address = RestrictedAddress::new(rand_ed25519_address())
        .unwrap()
        .with_allowed_capabilities([0])
        .unwrap();
    let mut capabilities = address.allowed_capabilities()[0];

    assert!(!capabilities.has_capabilities(CapabilityFlag::NATIVE_TOKENS));
    capabilities.add_capabilities(CapabilityFlag::NATIVE_TOKENS);
    assert!(capabilities.has_capabilities(CapabilityFlag::NATIVE_TOKENS));

    assert!(!capabilities.has_capabilities(CapabilityFlag::MANA));
    capabilities.set_capabilities(CapabilityFlag::MANA | CapabilityFlag::DELEGATION_OUTPUTS);
    assert!(capabilities.has_capabilities(CapabilityFlag::MANA));
    assert!(capabilities.has_capabilities(CapabilityFlag::DELEGATION_OUTPUTS));
    assert!(!capabilities.has_capabilities(CapabilityFlag::NATIVE_TOKENS));
}

#[test]
fn ed25519_bech32() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
    let address = Ed25519Address::new(
        Blake2b256::digest(
            PublicKey::try_from_bytes(
                hex::decode("6f1581709bb7b1ef030d210db18e3b0ba1c776fba65d8cdaad05415142d189f8")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .unwrap()
            .to_bytes(),
        )
        .try_into()
        .unwrap(),
    );
    assert_eq!(
        hex::encode(address),
        "efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a3"
    );
    assert_eq!(
        address.to_bech32_unchecked("iota").to_string(),
        "iota1qrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqgyzyx"
    );

    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq3l9hek"
    );
    address.set_allowed_capabilities([CapabilityFlag::ALL]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcplupydhwt"
    );
    address.set_allowed_capabilities([CapabilityFlag::NONE]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq3l9hek"
    );
    address
        .set_allowed_capabilities([CapabilityFlag::NATIVE_TOKENS])
        .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcpqytmqxr4"
    );

    let address = ImplicitAccountCreationAddress::from(address.address().as_ed25519().clone());
    assert_eq!(
        address.to_bech32_unchecked("iota").to_string(),
        "iota1rrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xg4ad2d"
    );
}

#[test]
fn account_bech32() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
    let address = Address::unpack_verified(
        hex::decode("08f1c011fb54df4a4e5b07462536fbacc779bf80cc45e03bc3410836587b4efc98").unwrap(),
        &(),
    )
    .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota1prcuqy0m2n055njmqarz2dhm4nrhn0uqe3z7qw7rgyyrvkrmfm7fsnwyxu6"
    );

    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqqdyjudm"
    );
    address.set_allowed_capabilities([CapabilityFlag::ALL]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqplurds6sq"
    );
    address.set_allowed_capabilities([CapabilityFlag::NONE]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqqdyjudm"
    );
    address
        .set_allowed_capabilities([CapabilityFlag::NATIVE_TOKENS])
        .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqpqyfjata7"
    );
}

#[test]
fn nft_bech32() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
    let address = Address::unpack_verified(
        hex::decode("10c72a65ae53d70b99a57f72637bfd1d5ea7baa2b4ba095c989b667d38558087db").unwrap(),
        &(),
    )
    .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota1zrrj5edw20tshxd90aexx7lar4020w4zkjaqjhycndn86wz4szrak44cs6h"
    );

    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcqek0lex"
    );
    address.set_allowed_capabilities([CapabilityFlag::ALL]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcpluts738a"
    );
    address.set_allowed_capabilities([CapabilityFlag::NONE]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcqek0lex"
    );
    address
        .set_allowed_capabilities([CapabilityFlag::NATIVE_TOKENS])
        .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota").to_string(),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcpqyp0nq2r"
    );
}
