// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
};

use primitive_types::U256;

use super::{Error, InputSelection};
use crate::{
    client::secret::types::InputSigningData,
    types::block::output::{Output, TokenId, TokenScheme},
};

pub(crate) fn get_native_tokens<'a>(
    outputs: impl Iterator<Item = &'a Output>,
) -> Result<BTreeMap<TokenId, U256>, Error> {
    let mut required_native_tokens = BTreeMap::<TokenId, U256>::new();

    for output in outputs {
        if let Some(native_token) = output.native_token() {
            (*required_native_tokens.entry(*native_token.token_id()).or_default()) += native_token.amount();
        }
    }

    Ok(required_native_tokens)
}

// TODO only handles one side
pub(crate) fn get_native_tokens_diff(
    inputs: &BTreeMap<TokenId, U256>,
    outputs: &BTreeMap<TokenId, U256>,
) -> Result<Option<BTreeMap<TokenId, U256>>, Error> {
    let mut native_tokens_diff = BTreeMap::<TokenId, U256>::new();

    for (token_id, input_amount) in inputs.iter() {
        match outputs.get(token_id) {
            None => {
                native_tokens_diff.insert(*token_id, *input_amount);
            }
            Some(output_amount) => {
                if input_amount > output_amount {
                    native_tokens_diff.insert(*token_id, input_amount - output_amount);
                }
            }
        }
    }

    if native_tokens_diff.is_empty() {
        Ok(None)
    } else {
        Ok(Some(native_tokens_diff))
    }
}

impl InputSelection {
    pub(crate) fn fulfill_native_tokens_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.non_remainder_outputs())?;
        let (minted_native_tokens, melted_native_tokens) = self.get_minted_and_melted_native_tokens()?;

        for (minted_native_token_id, minted_native_token_amount) in minted_native_tokens {
            (*input_native_tokens.entry(minted_native_token_id).or_default()) += minted_native_token_amount;
        }

        for (melted_native_token_id, melted_native_token_amount) in melted_native_tokens {
            (*output_native_tokens.entry(melted_native_token_id).or_default()) += melted_native_token_amount;
        }

        if let Some(burn) = self.burn.as_ref() {
            for (burnt_native_token_id, burnt_native_token_amount) in burn.native_tokens {
                (*output_native_tokens.entry(burnt_native_token_id).or_default()) += burnt_native_token_amount;
            }
        }

        // TODO weird that it happens in this direction?
        if let Some(diffs) = get_native_tokens_diff(&output_native_tokens, &input_native_tokens)? {
            log::debug!(
                "Fulfilling native tokens requirement with input {input_native_tokens:?} and output {output_native_tokens:?}"
            );

            let mut newly_selected_inputs = Vec::new();
            let mut newly_selected_ids = HashSet::new();

            for (token_id, diff) in diffs {
                let mut amount = U256::zero();
                // TODO sort ?
                let inputs = self.available_inputs.iter().filter(|input| {
                    input
                        .output
                        .native_token()
                        .is_some_and(|native_token| native_token.token_id() == &token_id)
                });

                for input in inputs {
                    amount += input
                        .output
                        .native_token()
                        // PANIC: safe to unwrap as the filter guarantees inputs containing this native token.
                        .unwrap()
                        .amount();

                    if newly_selected_ids.insert(*input.output_id()) {
                        newly_selected_inputs.push(input.clone());
                    }

                    if amount >= diff {
                        break;
                    }
                }

                if amount < diff {
                    return Err(Error::InsufficientNativeTokenAmount {
                        token_id,
                        found: amount,
                        required: diff,
                    });
                }
            }

            log::debug!("Outputs {newly_selected_ids:?} selected to fulfill the native tokens requirement");

            self.available_inputs
                .retain(|input| !newly_selected_ids.contains(input.output_id()));

            Ok(newly_selected_inputs)
        } else {
            log::debug!("Native tokens requirement already fulfilled");

            Ok(Vec::new())
        }
    }

    pub(crate) fn get_minted_and_melted_native_tokens(
        &self,
    ) -> Result<(BTreeMap<TokenId, U256>, BTreeMap<TokenId, U256>), Error> {
        let mut minted_native_tokens = BTreeMap::<TokenId, U256>::new();
        let mut melted_native_tokens = BTreeMap::<TokenId, U256>::new();

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

                                    (*minted_native_tokens.entry(token_id).or_default()) += minted_native_token_amount;
                                }
                                Ordering::Less => {
                                    let melted_native_token_amount = input_foundry_simple_ts.circulating_supply()
                                        - output_foundry_simple_ts.circulating_supply();

                                    (*melted_native_tokens.entry(token_id).or_default()) += melted_native_token_amount;
                                }
                                Ordering::Equal => {}
                            }
                        }
                    }
                }

                // If we created the foundry with this transaction, then we need to add the circulating supply as minted
                // tokens
                if initial_creation {
                    let circulating_supply = output_foundry_simple_ts.circulating_supply();

                    if !circulating_supply.is_zero() {
                        (*minted_native_tokens.entry(output_foundry.token_id()).or_default()) += circulating_supply;
                    }
                }
            }
        }

        Ok((minted_native_tokens, melted_native_tokens))
    }
}
