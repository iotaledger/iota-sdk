// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    output::{MinimumOutputAmount, NftId, NftOutput},
    protocol::iota_mainnet_protocol_parameters,
    rand::output::{
        feature::{rand_issuer_feature, rand_sender_feature},
        rand_nft_output,
        unlock_condition::rand_address_unlock_condition,
    },
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

#[test]
fn builder() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let address_1 = rand_address_unlock_condition();
    let address_2 = rand_address_unlock_condition();
    let sender_1 = rand_sender_feature();
    let sender_2 = rand_sender_feature();
    let issuer_1 = rand_issuer_feature();
    let issuer_2 = rand_issuer_feature();
    let amount = 500_000;

    let mut builder = NftOutput::build_with_amount(amount, NftId::null())
        .add_unlock_condition(address_1.clone())
        .add_feature(sender_1)
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

    let output = builder
        .with_minimum_amount(protocol_parameters.storage_score_parameters())
        .add_unlock_condition(rand_address_unlock_condition())
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
    let output = rand_nft_output(protocol_parameters.token_supply());
    let bytes = output.pack_to_vec();
    let output_unpacked = NftOutput::unpack_bytes_verified(bytes, protocol_parameters).unwrap();

    assert_eq!(output, output_unpacked);
}
