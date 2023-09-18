// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::{
    block::{
        address::AccountAddress,
        output::{AccountOutput, Feature, FoundryId, NativeToken, Output, Rent, SimpleTokenScheme, TokenId},
        protocol::protocol_parameters,
        rand::output::{
            feature::{rand_issuer_feature, rand_metadata_feature, rand_sender_feature},
            rand_account_id, rand_account_output,
            unlock_condition::{
                rand_governor_address_unlock_condition_different_from,
                rand_state_controller_address_unlock_condition_different_from,
            },
        },
    },
    ValidationParams,
};
use packable::PackableExt;

#[test]
fn builder() {
    let protocol_parameters = protocol_parameters();
    let account_id = rand_account_id();
    let foundry_id = FoundryId::build(&AccountAddress::from(account_id), 0, SimpleTokenScheme::KIND);
    let gov_address_1 = rand_governor_address_unlock_condition_different_from(&account_id);
    let gov_address_2 = rand_governor_address_unlock_condition_different_from(&account_id);
    let state_address_1 = rand_state_controller_address_unlock_condition_different_from(&account_id);
    let state_address_2 = rand_state_controller_address_unlock_condition_different_from(&account_id);
    let sender_1 = rand_sender_feature();
    let sender_2 = rand_sender_feature();
    let issuer_1 = rand_issuer_feature();
    let issuer_2 = rand_issuer_feature();
    let amount = 500_000;

    let mut builder = AccountOutput::build_with_amount(amount, account_id)
        .add_native_token(NativeToken::new(TokenId::from(foundry_id), 1000).unwrap())
        .add_unlock_condition(gov_address_1)
        .add_unlock_condition(state_address_1)
        .add_feature(sender_1)
        .replace_feature(sender_2)
        .replace_immutable_feature(issuer_1)
        .add_immutable_feature(issuer_2);

    let output = builder.clone().finish().unwrap();
    assert_eq!(output.amount(), amount);
    assert_eq!(output.unlock_conditions().governor_address(), Some(&gov_address_1));
    assert_eq!(
        output.unlock_conditions().state_controller_address(),
        Some(&state_address_1)
    );
    assert_eq!(output.features().sender(), Some(&sender_2));
    assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));

    builder = builder
        .clear_unlock_conditions()
        .clear_features()
        .clear_immutable_features()
        .replace_unlock_condition(gov_address_2)
        .replace_unlock_condition(state_address_2);
    let output = builder.clone().finish().unwrap();
    assert_eq!(output.unlock_conditions().governor_address(), Some(&gov_address_2));
    assert_eq!(
        output.unlock_conditions().state_controller_address(),
        Some(&state_address_2)
    );
    assert!(output.features().is_empty());
    assert!(output.immutable_features().is_empty());

    let metadata = rand_metadata_feature();

    let output = builder
        .with_minimum_amount(protocol_parameters.rent_structure())
        .add_unlock_condition(rand_state_controller_address_unlock_condition_different_from(
            &account_id,
        ))
        .add_unlock_condition(rand_governor_address_unlock_condition_different_from(&account_id))
        .with_features([Feature::from(metadata.clone()), sender_1.into()])
        .with_immutable_features([Feature::from(metadata.clone()), issuer_1.into()])
        .finish_with_params(ValidationParams::default().with_protocol_parameters(protocol_parameters.clone()))
        .unwrap();

    assert_eq!(
        output.amount(),
        Output::Account(output.clone()).rent_cost(protocol_parameters.rent_structure())
    );
    assert_eq!(output.features().metadata(), Some(&metadata));
    assert_eq!(output.features().sender(), Some(&sender_1));
    assert_eq!(output.immutable_features().metadata(), Some(&metadata));
    assert_eq!(output.immutable_features().issuer(), Some(&issuer_1));
}

#[test]
fn pack_unpack() {
    let protocol_parameters = protocol_parameters();
    let output = rand_account_output(protocol_parameters.token_supply());
    let bytes = output.pack_to_vec();
    let output_unpacked = AccountOutput::unpack_verified(bytes, &protocol_parameters).unwrap();

    assert_eq!(output, output_unpacked);
}
