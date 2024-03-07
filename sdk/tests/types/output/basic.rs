// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    output::{BasicOutput, Feature, FoundryId, MinimumOutputAmount, NativeToken, SimpleTokenScheme, TokenId},
    protocol::iota_mainnet_protocol_parameters,
    rand::{
        address::rand_account_address,
        output::{
            feature::{rand_metadata_feature, rand_sender_feature},
            rand_basic_output,
            unlock_condition::rand_address_unlock_condition,
        },
    },
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn builder() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let foundry_id = FoundryId::build(&rand_account_address(), 0, SimpleTokenScheme::KIND);
    let address_1 = rand_address_unlock_condition();
    let address_2 = rand_address_unlock_condition();
    let sender_1 = rand_sender_feature();
    let sender_2 = rand_sender_feature();
    let amount = 500_000;

    let mut builder = BasicOutput::build_with_amount(amount)
        .with_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
        .add_unlock_condition(address_1.clone())
        .add_feature(sender_1.clone())
        .replace_feature(sender_2.clone());

    let output = builder.clone().finish().unwrap();
    assert_eq!(output.amount(), amount);
    assert_eq!(output.unlock_conditions().address(), Some(&address_1));
    assert_eq!(output.features().sender(), Some(&sender_2));

    builder = builder
        .clear_unlock_conditions()
        .clear_features()
        .replace_unlock_condition(address_2.clone());
    let output = builder.clone().finish().unwrap();
    assert_eq!(output.unlock_conditions().address(), Some(&address_2));
    assert!(output.features().is_empty());

    let metadata = rand_metadata_feature();

    let output = builder
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .add_unlock_condition(rand_address_unlock_condition())
        .with_features([Feature::from(metadata.clone()), sender_1.clone().into()])
        .finish()
        .unwrap();

    assert_eq!(
        output.amount(),
        output.minimum_amount(protocol_parameters.storage_score_parameters())
    );
    assert_eq!(output.features().metadata(), Some(&metadata));
    assert_eq!(output.features().sender(), Some(&sender_1));
}

#[test]
fn pack_unpack() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let output = rand_basic_output(protocol_parameters.token_supply());
    let bytes = output.pack_to_vec();
    let output_unpacked = BasicOutput::unpack_bytes_verified(bytes, protocol_parameters).unwrap();

    assert_eq!(output, output_unpacked);
}
