// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::types::block::{
    address::{
        dto::{AccountAddressDto, AddressDto},
        AccountAddress, Address, Bech32Address, ToBech32Ext,
    },
    output::AccountId,
    Error,
};
use packable::PackableExt;

const ACCOUNT_ID: &str = "0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee";
const ALIAS_BECH32: &str = "rms1pr5m4q9dz4s7gdakvwslrmal4025fvxhmfamx0s2vt5ekg8wg597um6lcnn";
const ACCOUNT_ID_INVALID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86e";

#[test]
fn kind() {
    assert_eq!(AccountAddress::KIND, 8);

    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());

    assert_eq!(address.kind(), AccountAddress::KIND);
}

#[test]
fn length() {
    assert_eq!(AccountAddress::LENGTH, 32);
}

#[test]
fn is_methods() {
    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());

    assert!(!address.is_ed25519());
    assert!(address.is_alias());
    assert!(!address.is_nft());
}

#[test]
fn as_methods() {
    let account_address = AccountAddress::from_str(ACCOUNT_ID).unwrap();
    let address = Address::from(account_address);

    assert!(std::panic::catch_unwind(|| address.as_ed25519()).is_err());
    assert_eq!(address.as_alias(), &account_address);
    assert!(std::panic::catch_unwind(|| address.as_nft()).is_err());
}

#[test]
fn new_account_id() {
    let account_id = AccountId::from_str(ACCOUNT_ID).unwrap();
    let account_address = AccountAddress::new(account_id);

    assert_eq!(account_address.account_id(), &account_id);
}

#[test]
fn new_into_account_id() {
    let account_id = AccountId::from_str(ACCOUNT_ID).unwrap();
    let account_address = AccountAddress::new(account_id);

    assert_eq!(account_address.into_account_id(), account_id);
}

#[test]
fn from_str_to_str() {
    let account_address = AccountAddress::from_str(ACCOUNT_ID).unwrap();

    assert_eq!(account_address.to_string(), ACCOUNT_ID);
}

#[test]
fn debug() {
    let account_address = AccountAddress::from_str(ACCOUNT_ID).unwrap();

    assert_eq!(
        format!("{account_address:?}"),
        "AccountAddress(0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee)"
    );
}

#[test]
fn bech32() {
    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());

    assert_eq!(address.to_bech32_unchecked("rms"), ALIAS_BECH32);
}

#[test]
fn bech32_roundtrip() {
    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());
    let bech32 = address.to_bech32_unchecked("rms").to_string();

    assert_eq!(
        Bech32Address::try_from_str(bech32),
        Bech32Address::try_new("rms", address)
    );
}

#[test]
fn dto_fields() {
    let account_address = AccountAddress::from_str(ACCOUNT_ID).unwrap();
    let alias_dto = AccountAddressDto::from(&account_address);

    assert_eq!(alias_dto.kind, AccountAddress::KIND);
    assert_eq!(alias_dto.account_id, ACCOUNT_ID.to_string());

    let address = Address::from(account_address);
    let dto = AddressDto::from(&address);

    assert_eq!(dto, AddressDto::Account(alias_dto));
}

#[test]
fn dto_roundtrip() {
    let account_address = AccountAddress::from_str(ACCOUNT_ID).unwrap();
    let alias_dto = AccountAddressDto::from(&account_address);

    assert_eq!(AccountAddress::try_from(alias_dto).unwrap(), account_address);

    let address = Address::from(account_address);
    let dto = AddressDto::from(&address);

    assert_eq!(Address::try_from(dto).unwrap(), address);
}

#[test]
fn dto_invalid_account_id() {
    let dto = AccountAddressDto {
        kind: AccountAddress::KIND,
        account_id: ACCOUNT_ID_INVALID.to_string(),
    };

    assert!(matches!(
        AccountAddress::try_from(dto),
        Err(Error::InvalidField("accountId"))
    ));
}

#[test]
fn packed_len() {
    let address = AccountAddress::from_str(ACCOUNT_ID).unwrap();

    assert_eq!(address.packed_len(), AccountAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), AccountAddress::LENGTH);

    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());

    assert_eq!(address.packed_len(), 1 + AccountAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), 1 + AccountAddress::LENGTH);
}

#[test]
fn pack_unpack() {
    let address = AccountAddress::from_str(ACCOUNT_ID).unwrap();
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );

    let address = Address::from(AccountAddress::from_str(ACCOUNT_ID).unwrap());
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );
}
