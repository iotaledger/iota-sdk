// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    signatures::ed25519::PublicKey,
};
use iota_sdk::types::block::{
    address::{
        Address, CapabilityFlag, Ed25519Address, ImplicitAccountCreationAddress, RestrictedAddress, ToBech32Ext,
    },
    rand::address::rand_ed25519_address,
};
use packable::PackableExt;

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
fn restricted_ed25519() {
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
    // Ed25519 Address (Plain)
    assert_eq!(
        address.to_bech32_unchecked("iota"),
        "iota1qrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqgyzyx"
    );

    // Restricted Ed25519 Address (Every Capability Disallowed)
    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq3l9hek"
    );

    // Restricted Ed25519 Address (Every Capability Allowed)
    address.set_allowed_capabilities([CapabilityFlag::ALL]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcplupydhwt"
    );

    address.set_allowed_capabilities([CapabilityFlag::NONE]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq3l9hek"
    );

    // Restricted Ed25519 Address (Can receive Native Tokens)
    address
        .set_allowed_capabilities([CapabilityFlag::NATIVE_TOKENS])
        .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcpqytmqxr4"
    );

    let address = ImplicitAccountCreationAddress::from(*address.address().as_ed25519());
    assert_eq!(
        address.to_bech32_unchecked("iota"),
        "iota1rrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xg4ad2d"
    );
}

#[test]
fn restricted_account() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
    let address = Address::unpack_verified(
        hex::decode("08f1c011fb54df4a4e5b07462536fbacc779bf80cc45e03bc3410836587b4efc98").unwrap(),
        &(),
    )
    .unwrap();
    // Account Address (Plain)
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1prcuqy0m2n055njmqarz2dhm4nrhn0uqe3z7qw7rgyyrvkrmfm7fsnwyxu6"
    );

    // Restricted Account Address (Every Capability Disallowed)
    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqqdyjudm"
    );

    // Restricted Account Address (Every Capability Allowed)
    address.set_allowed_capabilities([CapabilityFlag::ALL]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqplurds6sq"
    );

    address.set_allowed_capabilities([CapabilityFlag::NONE]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqqdyjudm"
    );

    // Restricted Account Address (Can receive Native Tokens)
    address
        .set_allowed_capabilities([CapabilityFlag::NATIVE_TOKENS])
        .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqpqyfjata7"
    );
}

#[test]
fn restricted_nft() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
    let address = Address::unpack_verified(
        hex::decode("10c72a65ae53d70b99a57f72637bfd1d5ea7baa2b4ba095c989b667d38558087db").unwrap(),
        &(),
    )
    .unwrap();
    // NFT Address (Plain)
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1zrrj5edw20tshxd90aexx7lar4020w4zkjaqjhycndn86wz4szrak44cs6h"
    );

    // Restricted NFT Address (Every Capability Disallowed)
    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcqek0lex"
    );

    // Restricted NFT Address (Every Capability Allowed)
    address.set_allowed_capabilities([CapabilityFlag::ALL]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcpluts738a"
    );

    address.set_allowed_capabilities([CapabilityFlag::NONE]).unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcqek0lex"
    );

    // Restricted NFT Address (Can receive Native Tokens)
    address
        .set_allowed_capabilities([CapabilityFlag::NATIVE_TOKENS])
        .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcpqyp0nq2r"
    );
}
