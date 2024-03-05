// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;
use std::collections::HashMap;

use crypto::keys::bip44::Bip44;
use primitive_types::U256;

use super::{Error, InputSelection};
use crate::{
    client::api::{input_selection::requirement::native_tokens::get_native_tokens_diff, RemainderData},
    types::block::{
        address::{Address, Ed25519Address},
        output::{
            unlock_condition::AddressUnlockCondition, AccountOutput, BasicOutput, BasicOutputBuilder, NativeToken,
            NftOutput, Output, StorageScoreParameters, TokenId,
        },
    },
};

impl InputSelection {
    /// Updates the remainders, overwriting old values.
    pub(crate) fn update_remainders(&mut self) -> Result<(), Error> {
        let (storage_deposit_returns, remainders) = self.storage_deposit_returns_and_remainders()?;

        self.remainders.storage_deposit_returns = storage_deposit_returns;
        self.remainders.data = remainders;

        Ok(())
    }

    /// Gets the remainder address from configuration of finds one from the inputs.
    pub(crate) fn get_remainder_address(&self) -> Result<Option<(Address, Option<Bip44>)>, Error> {
        if let Some(remainder_address) = &self.remainders.address {
            // Search in inputs for the Bip44 chain for the remainder address, so the ledger can regenerate it
            for input in self.available_inputs.iter().chain(self.selected_inputs.iter()) {
                let required_address = input
                    .output
                    .required_address(
                        self.latest_slot_commitment_id.slot_index(),
                        self.protocol_parameters.committable_age_range(),
                    )?
                    .expect("expiration unlockable outputs already filtered out");

                if &required_address == remainder_address {
                    return Ok(Some((remainder_address.clone(), input.chain)));
                }
            }
            return Ok(Some((remainder_address.clone(), None)));
        }

        for input in &self.selected_inputs {
            let required_address = input
                .output
                .required_address(
                    self.latest_slot_commitment_id.slot_index(),
                    self.protocol_parameters.committable_age_range(),
                )?
                .expect("expiration unlockable outputs already filtered out");

            if let Some(&required_address) = required_address.backing_ed25519() {
                return Ok(Some((required_address.into(), input.chain)));
            }
        }

        Ok(None)
    }

    pub(crate) fn storage_deposit_returns_and_remainders(
        &mut self,
    ) -> Result<(Vec<Output>, Vec<RemainderData>), Error> {
        let (input_amount, output_amount, inputs_sdr, outputs_sdr) = self.amount_sums();
        let mut storage_deposit_returns = Vec::new();

        for (address, amount) in inputs_sdr {
            let output_sdr_amount = *outputs_sdr.get(&address).unwrap_or(&0);

            if amount > output_sdr_amount {
                let diff = amount - output_sdr_amount;
                let srd_output = BasicOutputBuilder::new_with_amount(diff)
                    .with_unlock_conditions([AddressUnlockCondition::new(address.clone())])
                    .finish_output()?;

                // TODO verify_storage_deposit ?

                log::debug!("Created storage deposit return output of {diff} for {address:?}");

                storage_deposit_returns.push(srd_output);
            }
        }

        let (input_nts, output_nts) = self.get_input_output_native_tokens();
        log::debug!("input_nts: {input_nts:#?}");
        log::debug!("output_nts: {output_nts:#?}");
        let native_tokens_diff = get_native_tokens_diff(input_nts, output_nts);

        let (input_mana, output_mana) = self.mana_sums(false)?;

        let amount_diff = input_amount.checked_sub(output_amount).expect("amount underflow");
        let mut mana_diff = input_mana.checked_sub(output_mana).expect("mana underflow");

        // If we are burning mana, then we can subtract out the burned amount.
        if self.burn.as_ref().map_or(false, |b| b.mana()) {
            mana_diff = mana_diff.saturating_sub(self.initial_mana_excess()?);
        }

        let (remainder_address, chain) = self
            .get_remainder_address()?
            .ok_or(Error::MissingInputWithEd25519Address)?;

        // If there is a mana remainder, try to fit it in an existing output
        if mana_diff > 0 && self.output_for_added_mana_exists(&remainder_address) {
            log::debug!("Allocating {mana_diff} excess input mana for output with address {remainder_address}");
            self.remainders.added_mana = std::mem::take(&mut mana_diff);
        }

        if input_amount == output_amount && mana_diff == 0 && native_tokens_diff.is_empty() {
            log::debug!("No remainder required");
            return Ok((storage_deposit_returns, Vec::new()));
        }

        let remainder_outputs = create_remainder_outputs(
            amount_diff,
            mana_diff,
            native_tokens_diff,
            remainder_address,
            chain,
            self.protocol_parameters.storage_score_parameters(),
        )?;

        Ok((storage_deposit_returns, remainder_outputs))
    }

    fn output_for_added_mana_exists(&self, remainder_address: &Address) -> bool {
        // Find the first value that matches the remainder address
        self.non_remainder_outputs().any(|o| {
            (o.is_basic() || o.is_account() || o.is_anchor() || o.is_nft())
                && o.unlock_conditions()
                    .map_or(true, |uc| uc.expiration().is_none() && uc.timelock().is_none())
                && matches!(o.required_address(
                    self.latest_slot_commitment_id.slot_index(),
                    self.protocol_parameters.committable_age_range(),
                ), Ok(Some(address)) if &address == remainder_address)
        })
    }

    pub(crate) fn get_output_for_added_mana(&mut self, remainder_address: &Address) -> Option<&mut Output> {
        // Establish the order in which we want to pick an output
        let sort_order = [AccountOutput::KIND, BasicOutput::KIND, NftOutput::KIND]
            .into_iter()
            .zip(0..)
            .collect::<HashMap<_, _>>();
        // Remove those that do not have an ordering and sort
        let ordered_outputs = self
            .provided_outputs
            .iter_mut()
            .chain(&mut self.added_outputs)
            .filter(|o| {
                o.unlock_conditions()
                    .map_or(true, |uc| uc.expiration().is_none() && uc.timelock().is_none())
            })
            .filter_map(|o| sort_order.get(&o.kind()).map(|order| (*order, o)))
            .collect::<BTreeMap<_, _>>();

        // Find the first value that matches the remainder address
        ordered_outputs.into_values().find(|o| {
            matches!(o.required_address(
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.committable_age_range(),
            ), Ok(Some(address)) if &address == remainder_address)
        })
    }

    /// Calculates the required amount for required remainder outputs (multiple outputs are required if multiple native
    /// tokens are remaining) and returns if there are native tokens as remainder.
    pub(crate) fn required_remainder_amount(&self) -> Result<(u64, bool, bool), Error> {
        let (input_nts, output_nts) = self.get_input_output_native_tokens();
        let remainder_native_tokens = get_native_tokens_diff(input_nts, output_nts);

        let remainder_builder =
            BasicOutputBuilder::new_with_minimum_amount(self.protocol_parameters.storage_score_parameters())
                .add_unlock_condition(AddressUnlockCondition::new(Address::from(Ed25519Address::from(
                    [0; 32],
                ))));

        let remainder_amount = if !remainder_native_tokens.is_empty() {
            let nt_remainder_amount = remainder_builder
                .with_native_token(
                    remainder_native_tokens
                        .first_key_value()
                        .map(|(token_id, amount)| NativeToken::new(*token_id, amount))
                        .unwrap()?,
                )
                .finish_output()?
                .amount();
            // Amount can be just multiplied, because all remainder outputs with a native token have the same storage
            // cost.
            nt_remainder_amount * remainder_native_tokens.len() as u64
        } else {
            remainder_builder.finish_output()?.amount()
        };

        let (selected_mana, required_mana) = self.mana_sums(false)?;

        let remainder_address = self.get_remainder_address()?.map(|v| v.0);

        // Mana can potentially be added to an appropriate existing output instead of a new remainder output
        let mut mana_remainder = selected_mana > required_mana
            && remainder_address.map_or(true, |remainder_address| {
                !self.output_for_added_mana_exists(&remainder_address)
            });
        // If we are burning mana, we may not need a mana remainder
        if self.burn.as_ref().map_or(false, |b| b.mana()) {
            let initial_excess = self.initial_mana_excess()?;
            mana_remainder &= selected_mana > required_mana + initial_excess;
        }

        Ok((remainder_amount, !remainder_native_tokens.is_empty(), mana_remainder))
    }
}

fn create_remainder_outputs(
    amount_diff: u64,
    mana_diff: u64,
    mut native_tokens: BTreeMap<TokenId, U256>,
    remainder_address: Address,
    remainder_address_chain: Option<Bip44>,
    storage_score_parameters: StorageScoreParameters,
) -> Result<Vec<RemainderData>, Error> {
    let mut remainder_outputs = Vec::new();
    let mut remaining_amount = amount_diff;
    let mut catchall_native_token = None;

    // Start with the native tokens
    if let Some((token_id, amount)) = native_tokens.pop_last() {
        // Save this one for the catchall
        catchall_native_token.replace(NativeToken::new(token_id, amount)?);
        // Create remainder outputs with min amount
        for (token_id, amount) in native_tokens {
            let output = BasicOutputBuilder::new_with_minimum_amount(storage_score_parameters)
                .add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()))
                .with_native_token(NativeToken::new(token_id, amount)?)
                .finish_output()?;
            log::debug!(
                "Created remainder output of amount {}, mana {} and native token ({token_id}: {amount}) for {remainder_address:?}",
                output.amount(),
                output.mana()
            );
            remaining_amount = remaining_amount.saturating_sub(output.amount());
            remainder_outputs.push(output);
        }
    }
    let mut catchall = BasicOutputBuilder::new_with_amount(remaining_amount)
        .with_mana(mana_diff)
        .add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()));
    if let Some(native_token) = catchall_native_token {
        catchall = catchall.with_native_token(native_token);
    }
    let catchall = catchall.finish_output()?;
    catchall.verify_storage_deposit(storage_score_parameters)?;
    log::debug!(
        "Created remainder output of amount {}, mana {} and native token {:?} for {remainder_address:?}",
        catchall.amount(),
        catchall.mana(),
        catchall.native_token(),
    );
    remainder_outputs.push(catchall);

    Ok(remainder_outputs
        .into_iter()
        .map(|o| RemainderData {
            output: o,
            chain: remainder_address_chain,
            address: remainder_address.clone(),
        })
        .collect())
}
