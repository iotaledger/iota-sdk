// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use std::{cmp::Ordering, collections::HashSet};

use primitive_types::U256;

use super::{TransactionBuilder, TransactionBuilderError};
use crate::{
    client::api::transaction_builder::Requirement,
    types::block::{
        output::{Output, TokenId, TokenScheme},
        payload::signed_transaction::TransactionCapabilityFlag,
    },
};

impl TransactionBuilder {
    pub(crate) fn fulfill_native_tokens_requirement(&mut self) -> Result<(), TransactionBuilderError> {
        let (input_nts, output_nts) = self.get_input_output_native_tokens();
        let diffs = get_native_tokens_diff(output_nts, input_nts);
        if self.burn.as_ref().map_or(false, |burn| !burn.native_tokens.is_empty()) {
            self.transaction_capabilities
                .add_capability(TransactionCapabilityFlag::BurnNativeTokens);
        }
        if diffs.is_empty() {
            log::debug!("Native tokens requirement already fulfilled");

            return Ok(());
        }

        if !self.allow_additional_input_selection {
            return Err(TransactionBuilderError::AdditionalInputsRequired(
                Requirement::NativeTokens,
            ));
        }

        log::debug!("Fulfilling native tokens requirement");

        let mut newly_selected_inputs = Vec::new();
        let mut newly_selected_ids = HashSet::new();

        for (&token_id, &amount) in diffs.iter() {
            let mut input_amount = U256::zero();
            // TODO sort ?
            let inputs = self.available_inputs.iter().filter(|input| {
                input
                    .output
                    .native_token()
                    .is_some_and(|native_token| native_token.token_id() == &token_id)
            });

            for input in inputs {
                input_amount += input
                    .output
                    .native_token()
                    // PANIC: safe to unwrap as the filter guarantees inputs containing this native token.
                    .unwrap()
                    .amount();

                if newly_selected_ids.insert(*input.output_id()) {
                    newly_selected_inputs.push(input.clone());
                }

                if input_amount >= amount {
                    break;
                }
            }

            if input_amount < amount {
                return Err(TransactionBuilderError::InsufficientNativeTokenAmount {
                    token_id,
                    found: input_amount,
                    required: amount,
                });
            }
        }

        log::debug!("Outputs {newly_selected_ids:?} selected to fulfill the native tokens requirement");

        self.available_inputs
            .retain(|input| !newly_selected_ids.contains(input.output_id()));

        for input in newly_selected_inputs {
            self.select_input(input)?;
        }

        Ok(())
    }

    pub(crate) fn get_input_output_native_tokens(&self) -> (BTreeMap<TokenId, U256>, BTreeMap<TokenId, U256>) {
        let mut input_native_tokens = self
            .selected_inputs
            .iter()
            .filter_map(|i| i.output.native_token().map(|t| (*t.token_id(), t.amount())))
            .fold(BTreeMap::new(), |mut nts, (token_id, amount)| {
                *nts.entry(token_id).or_default() += amount;
                nts
            });
        let mut output_native_tokens = self
            .non_remainder_outputs()
            .filter_map(|output| output.native_token().map(|t| (*t.token_id(), t.amount())))
            .fold(BTreeMap::new(), |mut nts, (token_id, amount)| {
                *nts.entry(token_id).or_default() += amount;
                nts
            });
        let (minted_native_tokens, melted_native_tokens) = self.get_minted_and_melted_native_tokens();

        minted_native_tokens
            .into_iter()
            .fold(&mut input_native_tokens, |nts, (token_id, amount)| {
                *nts.entry(token_id).or_default() += amount;
                nts
            });
        melted_native_tokens
            .into_iter()
            .fold(&mut output_native_tokens, |nts, (token_id, amount)| {
                *nts.entry(token_id).or_default() += amount;
                nts
            });

        if let Some(burn) = self.burn.as_ref() {
            burn.native_tokens
                .iter()
                .fold(&mut output_native_tokens, |nts, (token_id, amount)| {
                    *nts.entry(*token_id).or_default() += *amount;
                    nts
                });
        }
        (input_native_tokens, output_native_tokens)
    }

    pub(crate) fn get_minted_and_melted_native_tokens(&self) -> (BTreeMap<TokenId, U256>, BTreeMap<TokenId, U256>) {
        let mut minted_native_tokens = BTreeMap::new();
        let mut melted_native_tokens = BTreeMap::new();

        for output in self.non_remainder_outputs() {
            if let Output::Foundry(output_foundry) = output {
                let TokenScheme::Simple(output_foundry_simple_ts) = output_foundry.token_scheme();
                let mut initial_creation = true;

                for input in &self.selected_inputs {
                    if let Output::Foundry(input_foundry) = &input.output {
                        let token_id = output_foundry.token_id();

                        if output_foundry.id() == input_foundry.id() {
                            initial_creation = false;
                            let TokenScheme::Simple(input_foundry_simple_ts) = input_foundry.token_scheme();

                            match output_foundry_simple_ts
                                .circulating_supply()
                                .cmp(&input_foundry_simple_ts.circulating_supply())
                            {
                                Ordering::Greater => {
                                    let minted_native_token_amount = output_foundry_simple_ts.circulating_supply()
                                        - input_foundry_simple_ts.circulating_supply();

                                    *minted_native_tokens.entry(token_id).or_default() += minted_native_token_amount;
                                }
                                Ordering::Less => {
                                    let melted_native_token_amount = input_foundry_simple_ts.circulating_supply()
                                        - output_foundry_simple_ts.circulating_supply();

                                    *melted_native_tokens.entry(token_id).or_default() += melted_native_token_amount;
                                }
                                Ordering::Equal => {}
                            }
                        }
                    }
                }

                // If we created the foundry with this transaction, then we need to add the circulating supply as minted
                // tokens
                if initial_creation {
                    *minted_native_tokens.entry(output_foundry.token_id()).or_default() +=
                        output_foundry_simple_ts.circulating_supply();
                }
            }
        }

        (minted_native_tokens, melted_native_tokens)
    }
}

pub(crate) fn get_native_tokens_diff(
    first: BTreeMap<TokenId, U256>,
    second: BTreeMap<TokenId, U256>,
) -> BTreeMap<TokenId, U256> {
    first
        .into_iter()
        .filter_map(|(id, in_amount)| {
            let out_amount = second.get(&id).copied().unwrap_or_default();
            (in_amount > out_amount).then_some((id, in_amount.saturating_sub(out_amount)))
        })
        .collect()
}
