// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    address::{Address, AddressCapabilities, AddressCapabilityFlag, Ed25519Address, RestrictedAddress, ToBech32Ext},
    capabilities::CapabilityFlag,
    rand::address::rand_ed25519_address,
};
use packable::PackableExt;

#[test]
fn capabilities() {
    use AddressCapabilityFlag as Flag;
    let address = RestrictedAddress::new(rand_ed25519_address()).unwrap();
    let mut capabilities = address.allowed_capabilities().clone();

    assert!(!capabilities.has_capability(Flag::OutputsWithNativeTokens));
    capabilities.add_capability(Flag::OutputsWithNativeTokens);
    assert!(capabilities.has_capabilities([Flag::OutputsWithNativeTokens]));
    assert!(!capabilities.has_capabilities(AddressCapabilities::all().capabilities_iter()));
    assert!(!capabilities.is_none());
    assert!(!capabilities.is_all());
    capabilities.set_all();
    assert!(capabilities.is_all());
    assert!(capabilities.has_capabilities(Flag::all()));
    capabilities.set_none();

    assert!(!capabilities.has_capability(Flag::OutputsWithMana));
    capabilities.set_capabilities([Flag::OutputsWithMana, Flag::DelegationOutputs]);
    assert!(capabilities.has_capability(Flag::OutputsWithMana));
    assert!(capabilities.has_capabilities([Flag::OutputsWithMana, Flag::DelegationOutputs]));
    assert!(!capabilities.has_capability(Flag::OutputsWithNativeTokens));
    assert!(!capabilities.has_capabilities([
        Flag::OutputsWithMana,
        Flag::DelegationOutputs,
        Flag::OutputsWithNativeTokens
    ]));
}

#[test]
fn restricted_ed25519() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings
    let address = Ed25519Address::from_public_key_bytes(
        hex::decode("6f1581709bb7b1ef030d210db18e3b0ba1c776fba65d8cdaad05415142d189f8")
            .unwrap()
            .try_into()
            .unwrap(),
    )
    .unwrap();
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

    // TODO reenable when TIP is updated
    // // Restricted Ed25519 Address (Every Capability Allowed)
    // address.set_allowed_capabilities(AddressCapabilities::all());
    // assert_eq!(
    //     address.clone().to_bech32_unchecked("iota"),
    //     "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcplupydhwt"
    // );

    address.set_allowed_capabilities(AddressCapabilities::none());
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq3l9hek"
    );

    // Restricted Ed25519 Address (Can receive Native Tokens)
    address.set_allowed_capabilities([AddressCapabilityFlag::OutputsWithNativeTokens]);
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcpqytmqxr4"
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

    // TODO reenable when TIP is updated
    // // Restricted Account Address (Every Capability Allowed)
    // address.set_allowed_capabilities(AddressCapabilities::all());
    // assert_eq!(
    //     address.clone().to_bech32_unchecked("iota"),
    //     "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqplurds6sq"
    // );

    address.set_allowed_capabilities(AddressCapabilities::none());
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qy0rsq3ld2d7jjwtvr5vffklwkvw7dlsrxytcpmcdqssdjc0d80exqqdyjudm"
    );

    // Restricted Account Address (Can receive Native Tokens)
    address.set_allowed_capabilities([AddressCapabilityFlag::OutputsWithNativeTokens]);
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

    // TODO reenable when TIP is updated
    // // Restricted NFT Address (Every Capability Allowed)
    // address.set_allowed_capabilities(AddressCapabilities::all());
    // assert_eq!(
    //     address.clone().to_bech32_unchecked("iota"),
    //     "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcpluts738a"
    // );

    address.set_allowed_capabilities(AddressCapabilities::none());
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcqek0lex"
    );

    // Restricted NFT Address (Can receive Native Tokens)
    address.set_allowed_capabilities([AddressCapabilityFlag::OutputsWithNativeTokens]);
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota19qgvw2n94efawzue54lhycmml5w4afa6526t5z2unzdkvlfc2kqg0kcpqyp0nq2r"
    );
}
