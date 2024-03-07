// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    output::{
        unlock_condition::ImmutableAccountAddressUnlockCondition, FoundryId, FoundryOutput, MinimumOutputAmount,
        NativeToken, SimpleTokenScheme, TokenId,
    },
    protocol::iota_mainnet_protocol_parameters,
    rand::{
        address::rand_account_address,
        output::{feature::rand_metadata_feature, rand_foundry_output, rand_token_scheme},
    },
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn builder() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);
    let account_1 = ImmutableAccountAddressUnlockCondition::new(rand_account_address());
    let account_2 = ImmutableAccountAddressUnlockCondition::new(rand_account_address());
    let metadata_1 = rand_metadata_feature();
    let metadata_2 = rand_metadata_feature();
    let amount = 500_000;

    let mut builder = FoundryOutput::build_with_amount(amount, 234, rand_token_scheme())
        .with_serial_number(85)
        .with_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
        .with_unlock_conditions([account_1.clone()])
        .add_feature(metadata_1.clone())
        .replace_feature(metadata_2.clone())
        .with_immutable_features([metadata_2.clone()])
        .replace_immutable_feature(metadata_1.clone());

    let output = builder.clone().finish().unwrap();
    assert_eq!(output.amount(), amount);
    assert_eq!(output.serial_number(), 85);
    assert_eq!(output.unlock_conditions().immutable_account_address(), Some(&account_1));
    assert_eq!(output.features().metadata(), Some(&metadata_2));
    assert_eq!(output.immutable_features().metadata(), Some(&metadata_1));

    builder = builder
        .clear_unlock_conditions()
        .clear_features()
        .clear_immutable_features()
        .replace_unlock_condition(account_2.clone());
    let output = builder.clone().finish().unwrap();
    assert_eq!(output.unlock_conditions().immutable_account_address(), Some(&account_2));
    assert!(output.features().is_empty());
    assert!(output.immutable_features().is_empty());

    let output = builder
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(rand_account_address()))
        .finish()
        .unwrap();

    assert_eq!(
        output.amount(),
        output.minimum_amount(protocol_parameters.storage_score_parameters())
    );
}

#[test]
fn pack_unpack() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let output = rand_foundry_output(protocol_parameters.token_supply());
    let bytes = output.pack_to_vec();
    let output_unpacked = FoundryOutput::unpack_bytes_verified(bytes, protocol_parameters).unwrap();

    assert_eq!(output, output_unpacked);
}
