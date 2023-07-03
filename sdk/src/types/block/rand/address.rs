// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::{AccountAddress, Address, Ed25519Address, NftAddress},
    output::{AccountId, NftId},
    rand::{bytes::rand_bytes_array, number::rand_number},
};

/// Generates a random Ed25519 address.
pub fn rand_ed25519_address() -> Ed25519Address {
    Ed25519Address::new(rand_bytes_array())
}

/// Generates a random account address.
pub fn rand_account_address() -> AccountAddress {
    AccountAddress::new(AccountId::from(rand_bytes_array()))
}

/// Generates a random NFT address.
pub fn rand_nft_address() -> NftAddress {
    NftAddress::new(NftId::from(rand_bytes_array()))
}

/// Generates a random address.
pub fn rand_address() -> Address {
    match rand_number::<u64>() % 3 {
        0 => rand_ed25519_address().into(),
        1 => rand_account_address().into(),
        2 => rand_nft_address().into(),
        _ => unreachable!(),
    }
}
