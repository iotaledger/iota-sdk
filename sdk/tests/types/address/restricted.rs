// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    address::{Address, AddressCapabilities, AddressCapabilityFlag, RestrictedAddress, ToBech32Ext},
    capabilities::CapabilityFlag,
    rand::address::rand_ed25519_address,
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

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
    // Test from https://github.com/iotaledger/tips/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings

    // Ed25519 Address (Plain)
    let address = Address::unpack_bytes_verified(
        prefix_hex::decode::<Vec<_>>("0x00efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a3").unwrap(),
        &(),
    )
    .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1qrhacyfwlcnzkvzteumekfkrrwks98mpdm37cj4xx3drvmjvnep6xqgyzyx"
    );

    // Restricted Ed25519 Address (Every Capability Disallowed)
    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3000efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a300"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq8mnjgf"
    );

    // Restricted Ed25519 Address (Every Capability Allowed)
    address.set_allowed_capabilities(AddressCapabilities::all());
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3000efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a302ff01"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gczluqs97eene"
    );

    // Restricted Ed25519 Address (Every Capability Disallowed Reset)
    address.set_allowed_capabilities(AddressCapabilities::none());
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3000efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a300"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcq8mnjgf"
    );

    // Restricted Ed25519 Address (Can receive Native Tokens)
    address.set_allowed_capabilities([AddressCapabilityFlag::OutputsWithNativeTokens]);
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3000efdc112efe262b304bcf379b26c31bad029f616ee3ec4aa6345a366e4c9e43a30101"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqqwlhq39mlzv2esf08n0xexcvd66q5lv9hw8mz25c695dnwfj0y8gcpqyla70tq"
    );
}

#[test]
fn restricted_account() {
    // Test from https://github.com/iotaledger/tips/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings

    // Account Address (Plain)
    let address = Address::unpack_bytes_verified(
        prefix_hex::decode::<Vec<_>>("0x0860441c013b400f402c317833366f48730610296a09243636343e7b1b7e115409").unwrap(),
        &(),
    )
    .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1ppsyg8qp8dqq7spvx9urxdn0fpesvypfdgyjgd3kxsl8kxm7z92qj2lln86"
    );

    // Restricted Account Address (Every Capability Disallowed)
    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x300860441c013b400f402c317833366f48730610296a09243636343e7b1b7e11540900"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqyxq3quqya5qr6q9schsvekday8xpss994qjfpkxc6ru7cm0cg4gzgq9nu0d0"
    );

    // Restricted Account Address (Every Capability Allowed)
    address.set_allowed_capabilities(AddressCapabilities::all());
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x300860441c013b400f402c317833366f48730610296a09243636343e7b1b7e11540902ff01"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqyxq3quqya5qr6q9schsvekday8xpss994qjfpkxc6ru7cm0cg4gzgzluqs9xmye3"
    );

    // Restricted Account Address (Every Capability Disallowed Reset)
    address.set_allowed_capabilities(AddressCapabilities::none());
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x300860441c013b400f402c317833366f48730610296a09243636343e7b1b7e11540900"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqyxq3quqya5qr6q9schsvekday8xpss994qjfpkxc6ru7cm0cg4gzgq9nu0d0"
    );

    // Restricted Account Address (Can receive Native Tokens)
    address.set_allowed_capabilities([AddressCapabilityFlag::OutputsWithNativeTokens]);
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x300860441c013b400f402c317833366f48730610296a09243636343e7b1b7e1154090101"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqyxq3quqya5qr6q9schsvekday8xpss994qjfpkxc6ru7cm0cg4gzgpqys8pcr3"
    );
}

#[test]
fn restricted_nft() {
    // Test from https://github.com/iotaledger/tips/blob/tip50/tips/TIP-0050/tip-0050.md#bech32-strings

    // NFT Address (Plain)
    let address = Address::unpack_bytes_verified(
        prefix_hex::decode::<Vec<_>>("0x10140f39267a343f0d650a751250445e40600d133522085d210a2b5f3f69445139").unwrap(),
        &(),
    )
    .unwrap();
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1zq2q7wfx0g6r7rt9pf63y5zyteqxqrgnx53qshfppg4470mfg3gnjfmvts0"
    );

    // Restricted NFT Address (Every Capability Disallowed)
    let mut address = RestrictedAddress::new(address).unwrap();
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3010140f39267a343f0d650a751250445e40600d133522085d210a2b5f3f6944513900"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqgpgreeyearg0cdv5982yjsg30yqcqdzv6jyzzayy9zkheld9z9zwgqjt4fkk"
    );

    // Restricted NFT Address (Every Capability Allowed)
    address.set_allowed_capabilities(AddressCapabilities::all());
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3010140f39267a343f0d650a751250445e40600d133522085d210a2b5f3f6944513902ff01"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqgpgreeyearg0cdv5982yjsg30yqcqdzv6jyzzayy9zkheld9z9zwgzluqs3ctnc5"
    );

    // Restricted NFT Address (Every Capability Disallowed Reset)
    address.set_allowed_capabilities(AddressCapabilities::none());
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3010140f39267a343f0d650a751250445e40600d133522085d210a2b5f3f6944513900"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqgpgreeyearg0cdv5982yjsg30yqcqdzv6jyzzayy9zkheld9z9zwgqjt4fkk"
    );

    // Restricted NFT Address (Can receive Native Tokens)
    address.set_allowed_capabilities([AddressCapabilityFlag::OutputsWithNativeTokens]);
    assert_eq!(
        prefix_hex::encode(Address::from(address.clone()).pack_to_vec()),
        "0x3010140f39267a343f0d650a751250445e40600d133522085d210a2b5f3f694451390101"
    );
    assert_eq!(
        address.clone().to_bech32_unchecked("iota"),
        "iota1xqgpgreeyearg0cdv5982yjsg30yqcqdzv6jyzzayy9zkheld9z9zwgpqysq5lyk"
    );
}
