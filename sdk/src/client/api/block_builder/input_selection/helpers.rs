// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Helper functions used in the input selection

use crate::{
    client::Result,
    types::block::{
        address::{Address, Ed25519Address},
        output::{
            unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokens, Output, Rent, RentStructure,
        },
    },
};

/// Computes the minimum storage deposit amount that a basic output needs to have with an [AddressUnlockCondition] and
/// optional [NativeTokens].
pub fn minimum_storage_deposit_basic_output(
    config: &RentStructure,
    native_tokens: &Option<NativeTokens>,
    token_supply: u64,
) -> Result<u64> {
    let mut basic_output_builder = BasicOutputBuilder::new_with_amount(Output::AMOUNT_MIN);
    if let Some(native_tokens) = native_tokens {
        basic_output_builder = basic_output_builder.with_native_tokens(native_tokens.clone());
    }
    let basic_output = basic_output_builder
        // Null address because we only care about the size and ed25519, account and nft addresses have the same size.
        .add_unlock_condition(AddressUnlockCondition::new(Address::from(Ed25519Address::from(
            [0; Ed25519Address::LENGTH],
        ))))
        .finish_output(token_supply)?;

    Ok(basic_output.rent_cost(config))
}
