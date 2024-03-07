// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    output::{AccountOutput, Feature, MinimumOutputAmount},
    protocol::iota_mainnet_protocol_parameters,
    rand::output::{
        feature::{rand_issuer_feature, rand_metadata_feature, rand_sender_feature},
        rand_account_id, rand_account_output,
        unlock_condition::rand_address_unlock_condition_different_from_account_id,
    },
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn builder() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let account_id = rand_account_id();
    let address_1 = rand_address_unlock_condition_different_from_account_id(&account_id);
    let address_2 = rand_address_unlock_condition_different_from_account_id(&account_id);
    let sender_1 = rand_sender_feature();
    let sender_2 = rand_sender_feature();
    let issuer_1 = rand_issuer_feature();
    let issuer_2 = rand_issuer_feature();
    let amount = 500_000;

    let mut builder = AccountOutput::build_with_amount(amount, account_id)
        .add_unlock_condition(address_1.clone())
        .add_feature(sender_1.clone())
        .replace_feature(sender_2.clone())
        .replace_immutable_feature(issuer_1.clone())
        .add_immutable_feature(issuer_2);

    let output = builder.clone().finish().unwrap();
    assert_eq!(output.amount(), amount);
    assert_eq!(output.unlock_conditions().address(), Some(&address_1));
    assert_eq!(output.features().sender(), Some(&sender_2));
    assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));

    builder = builder
        .clear_unlock_conditions()
        .clear_features()
        .clear_immutable_features()
        .replace_unlock_condition(address_2.clone());
    let output = builder.clone().finish().unwrap();
    assert_eq!(output.unlock_conditions().address(), Some(&address_2));
    assert!(output.features().is_empty());
    assert!(output.immutable_features().is_empty());

    let metadata = rand_metadata_feature();

    let output = builder
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .add_unlock_condition(rand_address_unlock_condition_different_from_account_id(&account_id))
        .with_features([Feature::from(metadata.clone()), sender_1.clone().into()])
        .with_immutable_features([Feature::from(metadata.clone()), issuer_1.clone().into()])
        .finish()
        .unwrap();

    assert_eq!(
        output.amount(),
        output.minimum_amount(protocol_parameters.storage_score_parameters())
    );
    assert_eq!(output.features().metadata(), Some(&metadata));
    assert_eq!(output.features().sender(), Some(&sender_1));
    assert_eq!(output.immutable_features().metadata(), Some(&metadata));
    assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));
}

#[test]
fn pack_unpack() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let output = rand_account_output(protocol_parameters.token_supply());
    let bytes = output.pack_to_vec();
    let output_unpacked = AccountOutput::unpack_bytes_verified(bytes, protocol_parameters).unwrap();

    assert_eq!(output, output_unpacked);
}
