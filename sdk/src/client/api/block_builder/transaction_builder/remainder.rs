// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;

use crypto::keys::bip44::Bip44;
use primitive_types::U256;

use super::{TransactionBuilder, TransactionBuilderError};
use crate::{
    client::api::{
        transaction_builder::{requirement::native_tokens::get_native_tokens_diff, Remainders},
        RemainderData,
    },
    types::block::{
        address::{Address, Ed25519Address},
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, ChainId, NativeToken, Output, TokenId},
    },
};

impl TransactionBuilder {
    /// Updates the remainders, overwriting old values.
    pub(crate) fn update_remainders(&mut self) -> Result<(), TransactionBuilderError> {
        self.remainders = Remainders {
            address: self.remainders.address.take(),
            ..Default::default()
        };
        let (input_amount, output_amount, inputs_sdr, outputs_sdr) = self.amount_sums();

        for (address, amount) in inputs_sdr {
            let output_sdr_amount = *outputs_sdr.get(&address).unwrap_or(&0);

            if amount > output_sdr_amount {
                let diff = amount - output_sdr_amount;
                let srd_output = BasicOutputBuilder::new_with_amount(diff)
                    .with_unlock_conditions([AddressUnlockCondition::new(address.clone())])
                    .finish_output()?;

                // TODO verify_storage_deposit ?

                log::debug!("Created storage deposit return output of {diff} for {address:?}");

                self.remainders.storage_deposit_returns.push(srd_output);
            }
        }

        let (input_nts, output_nts) = self.get_input_output_native_tokens();
        log::debug!("input_nts: {input_nts:#?}");
        log::debug!("output_nts: {output_nts:#?}");
        let native_tokens_diff = get_native_tokens_diff(input_nts, output_nts);

        let (input_mana, output_mana) = self.mana_sums(false)?;

        let mut amount_diff = input_amount.checked_sub(output_amount).expect("amount underflow");
        let mut mana_diff = input_mana.checked_sub(output_mana).expect("mana underflow");

        // If we are burning mana, then we can subtract out the burned amount.
        if self.burn.as_ref().map_or(false, |b| b.mana()) {
            mana_diff = mana_diff.saturating_sub(self.initial_mana_excess()?);
        }

        let (remainder_address, chain) = self
            .get_remainder_address()?
            .ok_or(TransactionBuilderError::MissingInputWithEd25519Address)?;

        let remainder_builder =
            BasicOutputBuilder::new_with_minimum_amount(self.protocol_parameters.storage_score_parameters())
                .add_unlock_condition(AddressUnlockCondition::new(Ed25519Address::null()));

        let nt_min_amount = if !native_tokens_diff.is_empty() {
            let nt_remainder_amount = remainder_builder
                .with_native_token(
                    native_tokens_diff
                        .first_key_value()
                        .map(|(token_id, amount)| NativeToken::new(*token_id, amount))
                        .unwrap()?,
                )
                .finish_output()?
                .amount();
            // Amount can be just multiplied, because all remainder outputs with a native token have the same storage
            // cost.
            nt_remainder_amount * native_tokens_diff.len() as u64
        } else {
            0
        };

        // If there is an amount remainder (above any nt min amount), try to fit it in an existing output
        if amount_diff > nt_min_amount {
            for (chain_id, (input_amount, output_amount)) in self.amount_chains()? {
                if input_amount > output_amount
                    && (self.output_for_remainder_exists(Some(chain_id), &remainder_address)
                        || self.output_for_remainder_exists(None, &remainder_address))
                {
                    // Get the lowest of the total diff or the diff for this chain
                    let amount_to_add = (amount_diff - nt_min_amount).min(input_amount - output_amount);
                    log::debug!(
                        "Allocating {amount_to_add} excess input amount for output with address {remainder_address} and chain id {chain_id}"
                    );
                    amount_diff -= amount_to_add;
                    self.remainders.added_amount.insert(Some(chain_id), amount_to_add);
                }
            }
            // Any leftover amount diff can go in any output with the right address
            if amount_diff > nt_min_amount && self.output_for_remainder_exists(None, &remainder_address) {
                let amount_to_add = amount_diff - nt_min_amount;
                log::debug!(
                    "Allocating {amount_to_add} excess input amount for output with address {remainder_address}"
                );
                amount_diff = nt_min_amount;
                self.remainders.added_amount.insert(None, amount_to_add);
            }
        }

        // If there is a mana remainder, try to fit it in an existing output
        if mana_diff > 0 {
            for (chain_id, (input_mana, output_mana)) in self.mana_chains()? {
                if input_mana > output_mana
                    && (self.output_for_remainder_exists(Some(chain_id), &remainder_address)
                        || self.output_for_remainder_exists(None, &remainder_address))
                {
                    // Get the lowest of the total diff or the diff for this chain
                    let mana_to_add = mana_diff.min(input_mana - output_mana);
                    log::debug!(
                        "Allocating {mana_to_add} excess input mana for output with address {remainder_address} and chain id {chain_id}"
                    );
                    mana_diff -= mana_to_add;
                    self.remainders.added_mana.insert(Some(chain_id), mana_to_add);
                }
            }
            // Any leftover mana diff can go in any output with the right address
            if mana_diff > 0 && self.output_for_remainder_exists(None, &remainder_address) {
                log::debug!("Allocating {mana_diff} excess input mana for output with address {remainder_address}");
                self.remainders.added_mana.insert(None, std::mem::take(&mut mana_diff));
            }
        }

        if amount_diff == 0 && mana_diff == 0 && native_tokens_diff.is_empty() {
            log::debug!("No remainder required");
            return Ok(());
        }

        self.create_remainder_outputs(amount_diff, mana_diff, native_tokens_diff, remainder_address, chain)?;

        Ok(())
    }

    /// Gets the remainder address from configuration of finds one from the inputs.
    pub(crate) fn get_remainder_address(&self) -> Result<Option<(Address, Option<Bip44>)>, TransactionBuilderError> {
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

    fn output_for_remainder_exists(&self, chain_id: Option<ChainId>, remainder_address: &Address) -> bool {
        // Find the first value that matches the remainder address
        self.added_outputs.iter().any(|o| {
            (o.chain_id() == chain_id || chain_id.is_none() && (o.is_basic() || o.is_account() || o.is_nft()))
                && o.unlock_conditions()
                    .map_or(true, |uc| uc.expiration().is_none() && uc.timelock().is_none())
                && matches!(o.required_address(
                    self.latest_slot_commitment_id.slot_index(),
                    self.protocol_parameters.committable_age_range(),
                ), Ok(Some(address)) if &address == remainder_address)
        })
    }

    pub(crate) fn get_output_for_remainder(
        &mut self,
        chain_id: Option<ChainId>,
        remainder_address: &Address,
    ) -> Option<&mut Output> {
        self.added_outputs.iter_mut().find(|o| {
            (o.chain_id() == chain_id || chain_id.is_none() && (o.is_basic() || o.is_account() || o.is_nft()))
                && o.unlock_conditions()
                    .map_or(true, |uc| uc.expiration().is_none() && uc.timelock().is_none())
                && matches!(o.required_address(
                            self.latest_slot_commitment_id.slot_index(),
                            self.protocol_parameters.committable_age_range(),
                        ), Ok(Some(address)) if &address == remainder_address)
        })
    }

    /// Calculates the required amount for required remainder outputs (multiple outputs are required if multiple native
    /// tokens are remaining) and returns if there are native tokens as remainder.
    pub(crate) fn required_remainder_amount(&self) -> Result<(u64, bool, bool), TransactionBuilderError> {
        let (input_nts, output_nts) = self.get_input_output_native_tokens();
        let remainder_native_tokens = get_native_tokens_diff(input_nts, output_nts);

        let remainder_builder =
            BasicOutputBuilder::new_with_minimum_amount(self.protocol_parameters.storage_score_parameters())
                .add_unlock_condition(AddressUnlockCondition::new(Ed25519Address::null()));

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

        log::debug!("selected mana: {selected_mana}, required: {required_mana}");

        let remainder_address = self.get_remainder_address()?.map(|v| v.0);

        let mana_chains = self.mana_chains()?;

        // Mana can potentially be added to an appropriate existing output instead of a new remainder output
        let mut mana_remainder = selected_mana > required_mana
            && remainder_address.map_or(true, |remainder_address| {
                let mut mana_diff = selected_mana - required_mana;
                for (chain_id, (mana_in, mana_out)) in mana_chains {
                    if mana_in > mana_out && self.output_for_remainder_exists(Some(chain_id), &remainder_address) {
                        mana_diff -= mana_diff.min(mana_in - mana_out);
                        if mana_diff == 0 {
                            return false;
                        }
                    }
                }
                mana_diff > 0 && !self.output_for_remainder_exists(None, &remainder_address)
            });
        // If we are burning mana, we may not need a mana remainder
        if self.burn.as_ref().map_or(false, |b| b.mana()) {
            let initial_excess = self.initial_mana_excess()?;
            log::debug!("initial_mana_excess: {initial_excess}");
            mana_remainder &= selected_mana > required_mana + initial_excess;
        }

        Ok((remainder_amount, !remainder_native_tokens.is_empty(), mana_remainder))
    }

    fn create_remainder_outputs(
        &mut self,
        amount_diff: u64,
        mana_diff: u64,
        mut native_tokens: BTreeMap<TokenId, U256>,
        remainder_address: Address,
        remainder_address_chain: Option<Bip44>,
    ) -> Result<(), TransactionBuilderError> {
        let mut remaining_amount = amount_diff;
        let mut catchall_native_token = None;

        // Start with the native tokens
        if let Some((token_id, amount)) = native_tokens.pop_last() {
            // Save this one for the catchall
            catchall_native_token.replace(NativeToken::new(token_id, amount)?);
            // Create remainder outputs with min amount
            for (token_id, amount) in native_tokens {
                let output =
                    BasicOutputBuilder::new_with_minimum_amount(self.protocol_parameters.storage_score_parameters())
                        .add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()))
                        .with_native_token(NativeToken::new(token_id, amount)?)
                        .finish_output()?;
                log::debug!(
                    "Created remainder output of amount {}, mana {} and native token ({token_id}: {amount}) for {remainder_address:?}",
                    output.amount(),
                    output.mana()
                );
                remaining_amount = remaining_amount.saturating_sub(output.amount());
                self.remainders.data.push(RemainderData {
                    output,
                    chain: remainder_address_chain,
                    address: remainder_address.clone(),
                });
            }
        }
        let mut catchall = BasicOutputBuilder::new_with_amount(remaining_amount)
            .with_mana(mana_diff)
            .add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()));
        if let Some(native_token) = catchall_native_token {
            catchall = catchall.with_native_token(native_token);
        }
        let catchall = catchall.finish_output()?;
        catchall.verify_storage_deposit(self.protocol_parameters.storage_score_parameters())?;
        log::debug!(
            "Created remainder output of amount {}, mana {} and native token {:?} for {remainder_address:?}",
            catchall.amount(),
            catchall.mana(),
            catchall.native_token(),
        );
        self.remainders.data.push(RemainderData {
            output: catchall,
            chain: remainder_address_chain,
            address: remainder_address.clone(),
        });

        Ok(())
    }
}
