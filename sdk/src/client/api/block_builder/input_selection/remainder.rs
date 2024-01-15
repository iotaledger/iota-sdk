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
            unlock_condition::AddressUnlockCondition, AccountOutputBuilder, BasicOutputBuilder, MinimumOutputAmount,
            NativeTokensBuilder, NftOutputBuilder, Output,
        },
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
        let native_tokens_remainder = native_tokens_diff.is_some();

        let mut remainder_builder =
            BasicOutputBuilder::new_with_minimum_amount(self.protocol_parameters.storage_score_parameters())
                .add_unlock_condition(AddressUnlockCondition::new(Address::from(Ed25519Address::from(
                    [0; 32],
                ))));

        // TODO https://github.com/iotaledger/iota-sdk/issues/1631
        // TODO Only putting one in remainder atm so we can at least create foundries
        if let Some(native_tokens) = native_tokens_diff {
            remainder_builder = remainder_builder.with_native_token(*native_tokens.first().unwrap());
        }

        Ok((remainder_builder.finish_output()?.amount(), native_tokens_remainder))
    }

    pub(crate) fn remainder_and_storage_deposit_return_outputs(
        &mut self,
    ) -> Result<(Option<RemainderData>, Vec<Output>), Error> {
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
            let potential_mana = {
                let min_deposit = input
                    .output
                    .minimum_amount(self.protocol_parameters.storage_score_parameters());
                let generation_amount = input.output.amount().saturating_sub(min_deposit);

                self.protocol_parameters.generate_mana_with_decay(
                    generation_amount,
                    input.output_id().transaction_id().slot_index(),
                    self.slot_index,
                )
            }?;
            let stored_mana = self.protocol_parameters.mana_with_decay(
                input.output.mana(),
                input.output_id().transaction_id().slot_index(),
                self.slot_index,
            )?;

            input_mana += potential_mana + stored_mana;
            // TODO rewards
        }

        let output_mana = self.outputs.iter().map(|o| o.mana()).sum::<u64>() + self.mana_allotments;

        if input_amount == output_amount && input_mana == output_mana && native_tokens_diff.is_none() {
            log::debug!("No remainder required");
            return Ok((None, storage_deposit_returns));
        }

        // TODO underflows?
        let amount_diff = input_amount - output_amount;
        let mana_diff = input_mana - output_mana;

        if input_amount == output_amount && input_mana != output_mana && native_tokens_diff.is_none() {
            let output = self
                .outputs
                .iter_mut()
                .filter(|output| {
                    output
                        .chain_id()
                        .as_ref()
                        .map(|chain_id| self.automatically_transitioned.contains(chain_id))
                        .unwrap_or(false)
                })
                .next();
            if let Some(output) = output {
                *output = match output {
                    Output::Account(output) => AccountOutputBuilder::from(&*output)
                        .with_mana(output.mana() + mana_diff)
                        .finish_output()?,
                    Output::Foundry(_) => panic!(),
                    Output::Nft(output) => NftOutputBuilder::from(&*output)
                        .with_mana(output.mana() + mana_diff)
                        .finish_output()?,
                    _ => panic!("only account, nft and foundry can be automatically created"),
                };
            }

            return Ok((None, storage_deposit_returns));
        }

        let Some((remainder_address, chain)) = self.get_remainder_address()? else {
            return Err(Error::MissingInputWithEd25519Address);
        };

        let mut remainder_builder = BasicOutputBuilder::new_with_amount(amount_diff).with_mana(mana_diff);

        remainder_builder =
            remainder_builder.add_unlock_condition(AddressUnlockCondition::new(remainder_address.clone()));

        // TODO https://github.com/iotaledger/iota-sdk/issues/1631
        // TODO Only putting one in remainder atm so we can at least create foundries
        if let Some(native_tokens) = native_tokens_diff {
            log::debug!("Adding {native_tokens:?} to remainder output for {remainder_address:?}");
            remainder_builder = remainder_builder.with_native_token(*native_tokens.first().unwrap());
        }

        let remainder = remainder_builder.finish_output()?;

        // TODO add log
        log::debug!("Created remainder output of {amount_diff} for {remainder_address:?}");

        remainder.verify_storage_deposit(self.protocol_parameters.storage_score_parameters())?;

        Ok((
            Some(RemainderData {
                output: remainder,
                chain,
                address: remainder_address,
            }),
            storage_deposit_returns,
        ))
    }
}
