// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Module providing random feature generation utilities.
pub mod feature;
/// Module providing random output metadata generation utilities.
pub mod metadata;
/// Module providing random native token generation utilities.
pub mod native_token;
/// Module providing random unlock condition generation utilities.
pub mod unlock_condition;

use primitive_types::U256;

pub use self::{
    metadata::{rand_output_metadata, rand_output_metadata_with_id},
    native_token::rand_native_token,
};
use crate::types::block::{
    output::{
        unlock_condition::ImmutableAccountAddressUnlockCondition, AccountId, AccountOutput, AnchorId, AnchorOutput,
        BasicOutput, DelegationId, FoundryOutput, NftId, NftOutput, Output, OutputId, SimpleTokenScheme, TokenScheme,
        OUTPUT_INDEX_RANGE,
    },
    rand::{
        address::rand_account_address,
        bytes::rand_bytes_array,
        number::{rand_number, rand_number_range},
        output::{
            feature::rand_allowed_features,
            unlock_condition::{
                rand_address_unlock_condition, rand_address_unlock_condition_different_from,
                rand_address_unlock_condition_different_from_account_id,
                rand_governor_address_unlock_condition_different_from,
                rand_state_controller_address_unlock_condition_different_from,
            },
        },
        transaction::rand_transaction_id_with_slot_index,
    },
    slot::SlotIndex,
};

/// Generates a random output id with a given slot index.
pub fn rand_output_id_with_slot_index(slot_index: impl Into<SlotIndex>) -> OutputId {
    OutputId::new(
        rand_transaction_id_with_slot_index(slot_index),
        rand_number_range(OUTPUT_INDEX_RANGE),
    )
}

/// Generates a random [`OutputId`].
pub fn rand_output_id() -> OutputId {
    rand_output_id_with_slot_index(rand_number::<u32>())
}

/// Generates a random [`BasicOutput`].
pub fn rand_basic_output(token_supply: u64) -> BasicOutput {
    let amount = rand_number_range(0..token_supply);

    BasicOutput::build_with_amount(amount)
        .with_features(rand_allowed_features(amount, BasicOutput::ALLOWED_FEATURES))
        .add_unlock_condition(rand_address_unlock_condition())
        .finish()
        .unwrap()
}

/// Generates a random [`AccountId`].
pub fn rand_account_id() -> AccountId {
    AccountId::from(rand_bytes_array())
}

/// Generates a random [`AnchorId`].
pub fn rand_anchor_id() -> AnchorId {
    AnchorId::from(rand_bytes_array())
}

/// Generates a random [`DelegationId`].
pub fn rand_delegation_id() -> DelegationId {
    DelegationId::from(rand_bytes_array())
}

/// Generates a random [`AccountOutput`].
pub fn rand_account_output(token_supply: u64) -> AccountOutput {
    // We need to make sure that `AccountId` and `Address` don't match.
    let account_id = rand_account_id();
    let amount = rand_number_range(0..token_supply);

    AccountOutput::build_with_amount(amount, account_id)
        .with_features(rand_allowed_features(amount, AccountOutput::ALLOWED_FEATURES))
        .add_unlock_condition(rand_address_unlock_condition_different_from_account_id(&account_id))
        .finish()
        .unwrap()
}

/// Generates a random [`AnchorOutput`].
pub fn rand_anchor_output(token_supply: u64) -> AnchorOutput {
    // We need to make sure that `AnchorId` and `Address` don't match.
    let anchor_id = rand_anchor_id();
    let amount = rand_number_range(0..token_supply);

    AnchorOutput::build_with_amount(amount, anchor_id)
        .with_features(rand_allowed_features(amount, AnchorOutput::ALLOWED_FEATURES))
        .add_unlock_condition(rand_state_controller_address_unlock_condition_different_from(
            &anchor_id,
        ))
        .add_unlock_condition(rand_governor_address_unlock_condition_different_from(&anchor_id))
        .add_unlock_condition(rand_state_controller_address_unlock_condition_different_from(
            &anchor_id,
        ))
        .finish()
        .unwrap()
}

/// Generates a random [`TokenScheme`].
pub fn rand_token_scheme() -> TokenScheme {
    let max = U256::from(rand_bytes_array()).saturating_add(U256::one());
    let minted = U256::from(rand_bytes_array()) % max.saturating_add(U256::one());
    let melted = U256::from(rand_bytes_array()) % minted.saturating_add(U256::one());

    TokenScheme::Simple(SimpleTokenScheme::new(minted, melted, max).unwrap())
}

/// Generates a random [`FoundryOutput`].
pub fn rand_foundry_output(token_supply: u64) -> FoundryOutput {
    let amount = rand_number_range(0..token_supply);

    FoundryOutput::build_with_amount(amount, rand_number(), rand_token_scheme())
        .with_features(rand_allowed_features(amount, FoundryOutput::ALLOWED_FEATURES))
        .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(rand_account_address()))
        .finish()
        .unwrap()
}

/// Generates a random [`NftOutput`].
pub fn rand_nft_output(token_supply: u64) -> NftOutput {
    // We need to make sure that `NftId` and `Address` don't match.
    let nft_id = NftId::from(rand_bytes_array());
    let amount = rand_number_range(0..token_supply);

    NftOutput::build_with_amount(amount, nft_id)
        .with_features(rand_allowed_features(amount, NftOutput::ALLOWED_FEATURES))
        .add_unlock_condition(rand_address_unlock_condition_different_from(&nft_id))
        .finish()
        .unwrap()
}

/// Generates a random [`Output`].
pub fn rand_output(token_supply: u64) -> Output {
    match rand_number::<u64>() % 5 {
        1 => rand_basic_output(token_supply).into(),
        2 => rand_account_output(token_supply).into(),
        3 => rand_foundry_output(token_supply).into(),
        4 => rand_nft_output(token_supply).into(),
        _ => unreachable!(),
    }
}
