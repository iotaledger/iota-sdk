// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    address::AccountAddress,
    output::{AccountId, FoundryId, SimpleTokenScheme, TokenScheme},
};
use pretty_assertions::assert_eq;

#[test]
fn getters() {
    let account_address = AccountAddress::from(
        AccountId::from_str("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649").unwrap(),
    );
    let serial_number = 42;
    let token_scheme = TokenScheme::from(SimpleTokenScheme::new(100, 0, 100).unwrap());
    let foundry_id = FoundryId::build(&account_address, serial_number, token_scheme.kind());

    assert_eq!(foundry_id.account_address(), account_address);
    assert_eq!(foundry_id.serial_number(), serial_number);
    assert_eq!(foundry_id.token_scheme_kind(), token_scheme.kind());
    assert_eq!(
        foundry_id,
        FoundryId::from_str("0x0852fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00000000").unwrap()
    );
}
