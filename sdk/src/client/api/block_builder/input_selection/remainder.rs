// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;

use super::{
    requirement::{
        amount::amount_sums,
        native_tokens::{get_minted_and_melted_native_tokens, get_native_tokens, get_native_tokens_diff},
    },
    Error, InputSelection,
};
use crate::{
    client::api::RemainderData,
    types::block::{
        address::{Address, Ed25519Address},
        output::{
            unlock_condition::AddressUnlockCondition, AccountOutputBuilder, BasicOutputBuilder, NativeTokens,
            NativeTokensBuilder, NftOutputBuilder, Output, StorageScoreParameters,
        },
        Error as BlockError,
    },
};

impl InputSelection {
    // Gets the remainder address from configuration of finds one from the inputs.
    fn get_remainder_address(&self) -> Result<Option<(Address, Option<Bip44>)>, Error> {
        if let Some(remainder_address) = &self.remainder_address {
            // Search in inputs for the Bip44 chain for the remainder address, so the ledger can regenerate it
            for input in self.available_inputs.iter().chain(self.selected_inputs.iter()) {
                let required_address = input
                    .output
                    .required_address(self.slot_index, self.protocol_parameters.committable_age_range())?
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
                .required_address(self.slot_index, self.protocol_parameters.committable_age_range())?
                .expect("expiration unlockable outputs already filtered out");

            if required_address.is_ed25519_backed() {
                return Ok(Some((required_address, input.chain)));
            }
        }

        Ok(None)
    }

    pub(crate) fn remainder_amount(&self) -> Result<(u64, bool), Error> {
        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.outputs.iter())?;
        let (minted_native_tokens, melted_native_tokens) =
            get_minted_and_melted_native_tokens(&self.selected_inputs, self.outputs.as_slice())?;

        input_native_tokens.merge(minted_native_tokens)?;
        output_native_tokens.merge(melted_native_tokens)?;

        if let Some(burn) = self.burn.as_ref() {
            output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
        }

        let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;

        required_remainder_amount(native_tokens_diff, self.protocol_parameters.storage_score_parameters())
    }

    pub(crate) fn remainder_and_storage_deposit_return_outputs(
        &mut self,
    ) -> Result<(Vec<RemainderData>, Vec<Output>), Error> {
        let (input_amount, output_amount, inputs_sdr, outputs_sdr) =
            amount_sums(&self.selected_inputs, &self.outputs, self.slot_index);
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

        let mut input_native_tokens = get_native_tokens(self.selected_inputs.iter().map(|input| &input.output))?;
        let mut output_native_tokens = get_native_tokens(self.outputs.iter())?;
        let (minted_native_tokens, melted_native_tokens) =
            get_minted_and_melted_native_tokens(&self.selected_inputs, &self.outputs)?;

        input_native_tokens.merge(minted_native_tokens)?;
        output_native_tokens.merge(melted_native_tokens)?;

        if let Some(burn) = self.burn.as_ref() {
            output_native_tokens.merge(NativeTokensBuilder::from(burn.native_tokens.clone()))?;
        }

        let native_tokens_diff = get_native_tokens_diff(&input_native_tokens, &output_native_tokens)?;

        let mut input_mana = 0;

        for input in &self.selected_inputs {
            input_mana += input.output.available_mana(
                &self.protocol_parameters,
                input.output_id().transaction_id().slot_index(),
                self.slot_index,
            )?;
            // TODO rewards https://github.com/iotaledger/iota-sdk/issues/1310
        }

        let output_mana = self.outputs.iter().map(|o| o.mana()).sum::<u64>() + self.mana_allotments;

        if input_amount == output_amount && input_mana == output_mana && native_tokens_diff.is_none() {
            log::debug!("No remainder required");
            return Ok((Vec::new(), storage_deposit_returns));
        }

        let amount_diff = input_amount
            .checked_sub(output_amount)
            .ok_or(BlockError::ConsumedAmountOverflow)?;
        let mana_diff = input_mana
            .checked_sub(output_mana)
            .ok_or(BlockError::ConsumedManaOverflow)?;

        // If there is only a mana remainder, try to fit it in an automatically transitioned output.
        if input_amount == output_amount && input_mana != output_mana && native_tokens_diff.is_none() {
            let filter = |output: &Output| {
                output
                    .chain_id()
                    .as_ref()
                    .map(|chain_id| self.automatically_transitioned.contains(chain_id))
                    .unwrap_or(false)
                    // Foundries can't hold mana so they are not considered here.
                    && !output.is_foundry()
            };
            let index = self
                .outputs
                .iter()
                .position(|output| filter(output) && output.is_account())
                .or_else(|| self.outputs.iter().position(filter));

            if let Some(index) = index {
                self.outputs[index] = match &self.outputs[index] {
                    Output::Account(output) => AccountOutputBuilder::from(output)
                        .with_mana(output.mana() + mana_diff)
                        .finish_output()?,
                    Output::Nft(output) => NftOutputBuilder::from(output)
                        .with_mana(output.mana() + mana_diff)
                        .finish_output()?,
                    _ => panic!("only account, nft can be automatically created and can hold mana"),
                };

                return Ok((Vec::new(), storage_deposit_returns));
            }
        }

        let Some((remainder_address, chain)) = self.get_remainder_address()? else {
            return Err(Error::MissingInputWithEd25519Address);
        };

        let remainder_outputs = create_remainder_outputs(
            amount_diff,
            mana_diff,
            native_tokens_diff,
            remainder_address,
            chain,
            self.protocol_parameters.storage_score_parameters(),
        )?;

        Ok((remainder_outputs, storage_deposit_returns))
    }
}

/// Calculates the required amount for required remainder outputs (multiple outputs are required if multiple native
/// tokens are remaining) and returns if there are native tokens as remainder.
pub(crate) fn required_remainder_amount(
    remainder_native_tokens: Option<NativeTokens>,
    storage_score_parameters: StorageScoreParameters,
) -> Result<(u64, bool), Error> {
    let native_tokens_remainder = remainder_native_tokens.is_some();

    let remainder_builder = BasicOutputBuilder::new_with_minimum_amount(storage_score_parameters).add_unlock_condition(
        AddressUnlockCondition::new(Address::from(Ed25519Address::from([0; 32]))),
    );

    let remainder_amount = if let Some(native_tokens) = remainder_native_tokens {
        let nt_remainder_amount = remainder_builder
            .with_native_token(*native_tokens.first().unwrap())
            .finish_output()?
            .amount();
        // Amount can be just multiplied, because all remainder outputs with a native token have the same storage
        // cost.
        nt_remainder_amount * native_tokens.len() as u64
    } else {
        remainder_builder.finish_output()?.amount()
    };

    Ok((remainder_amount, native_tokens_remainder))
}

fn create_remainder_outputs(
    amount_diff: u64,
    mana_diff: u64,
    native_tokens_diff: Option<NativeTokens>,
    remainder_address: Address,
    remainder_address_chain: Option<Bip44>,
    storage_score_parameters: StorageScoreParameters,
) -> Result<Vec<RemainderData>, Error> {
    let mut remainder_outputs = Vec::new();

    if let Some(native_tokens) = native_tokens_diff {
        let native_tokens_len = native_tokens.len();
        let mut remaining_amount = amount_diff;
        // Create a remainder output with minimum amount for each native token and put remaining amount + mana in
        // the last output.
        for (n, native_token) in native_tokens.into_iter().enumerate() {
            let remainder_builder = if n + 1 < native_tokens_len {
                BasicOutputBuilder::new_with_minimum_amount(storage_score_parameters)
            } else {
                // All remainder mana in the last remainder output which also gets all remaining amount.
                BasicOutputBuilder::new_with_amount(remaining_amount).with_mana(mana_diff)
            };

            let remainder = remainder_builder
                .add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()))
                .with_native_token(native_token)
                .finish_output()?;

            if n + 1 < native_tokens_len {
                remaining_amount = remaining_amount.saturating_sub(remainder.amount());
            } else {
                // Only last output uses amount diff and needs to be validated
                remainder.verify_storage_deposit(storage_score_parameters)?;
            };
            log::debug!(
                "Created remainder output of amount {}, mana {} and native token {native_token:?} for {remainder_address:?}",
                remainder.amount(),
                remainder.mana()
            );
            remainder_outputs.push(remainder);
        }
    } else {
        // No native token, just put all amount and mana in a single output.
        let remainder = BasicOutputBuilder::new_with_amount(amount_diff)
            .with_mana(mana_diff)
            .add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()))
            .finish_output()?;
        remainder.verify_storage_deposit(storage_score_parameters)?;
        log::debug!("Created remainder output of amount {amount_diff} and mana {mana_diff} for {remainder_address:?}");
        remainder_outputs.push(remainder);
    }

    Ok(remainder_outputs
        .into_iter()
        .map(|o| RemainderData {
            output: o,
            chain: remainder_address_chain,
            address: remainder_address.clone(),
        })
        .collect())
}
