// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::collections::HashMap;

use super::{Error, InputSelection};
use crate::{
    client::{
        api::input_selection::{MinManaAllotment, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        input::{Input, UtxoInput},
        mana::ManaAllotment,
        output::{AccountOutputBuilder, Output},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        signature::Ed25519Signature,
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        Error as BlockError,
    },
};

impl InputSelection {
    pub(crate) fn fulfill_mana_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let Some(MinManaAllotment {
            issuer_id,
            reference_mana_cost,
            ..
        }) = self.min_mana_allotment
        else {
            // If there is no min allotment calculation needed, just check mana
            self.get_inputs_for_mana_balance()?;
            return Ok(Vec::new());
        };

        if !self.selected_inputs.is_empty() && self.all_outputs().next().is_some() {
            self.selected_inputs = Self::sort_input_signing_data(
                std::mem::take(&mut self.selected_inputs),
                self.latest_slot_commitment_id.slot_index(),
                self.protocol_parameters.committable_age_range(),
            )?;

            let inputs = self
                .selected_inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_id())));

            let outputs = self.all_outputs().cloned();

            let mut builder = Transaction::builder(self.protocol_parameters.network_id())
                .with_inputs(inputs)
                .with_outputs(outputs);

            if let Some(payload) = &self.payload {
                builder = builder.with_payload(payload.clone());
            }

            // Add the empty allotment so the work score includes it
            self.mana_allotments.entry(issuer_id).or_default();

            // If the transaction fails to build, just keep going in case another requirement helps
            let transaction = builder
                .with_context_inputs(self.context_inputs.clone())
                .with_mana_allotments(
                    self.mana_allotments
                        .iter()
                        .map(|(&account_id, &mana)| ManaAllotment { account_id, mana }),
                )
                .finish_with_params(&self.protocol_parameters)?;

            let signed_transaction = SignedTransactionPayload::new(transaction, self.null_transaction_unlocks()?)?;

            let block_work_score = self.protocol_parameters.work_score(&signed_transaction)
                + self.protocol_parameters.work_score_parameters().block();

            let required_allotment_mana = block_work_score as u64 * reference_mana_cost;

            let MinManaAllotment {
                issuer_id,
                allotment_debt,
                allow_allotting_from_account_mana,
                ..
            } = self
                .min_mana_allotment
                .as_mut()
                .ok_or(Error::UnfulfillableRequirement(Requirement::Mana))?;

            // Add the required allotment to the issuing allotment
            if required_allotment_mana > self.mana_allotments[issuer_id] {
                log::debug!("Allotting at least {required_allotment_mana} mana to account ID {issuer_id}");
                let additional_allotment = required_allotment_mana - self.mana_allotments[issuer_id];
                log::debug!("{additional_allotment} additional mana required to meet minimum allotment");
                // Unwrap: safe because we always add the record above
                *self.mana_allotments.get_mut(issuer_id).unwrap() = required_allotment_mana;
                log::debug!("Adding {additional_allotment} to allotment debt {allotment_debt}");
                *allotment_debt += additional_allotment;
            } else {
                log::debug!("Setting allotment debt to {}", self.mana_allotments[issuer_id]);
                *allotment_debt = self.mana_allotments[issuer_id];
            }

            if *allow_allotting_from_account_mana {
                self.reduce_account_output()?;
            }
        } else if !self.requirements.contains(&Requirement::Mana) {
            self.requirements.push(Requirement::Mana);
        }

        // Remainders can only be calculated when the input mana is >= the output mana
        let (input_mana, output_mana) = self.mana_sums(false)?;
        if input_mana >= output_mana {
            self.update_remainders()?;
        }

        let additional_inputs = self.get_inputs_for_mana_balance()?;
        // If we needed more inputs to cover the additional allotment mana
        // then update remainders and re-run this requirement
        if additional_inputs && !self.requirements.contains(&Requirement::Mana) {
            self.requirements.push(Requirement::Mana);
        }

        Ok(Vec::new())
    }

    fn reduce_account_output(&mut self) -> Result<(), Error> {
        let MinManaAllotment {
            issuer_id,
            allotment_debt,
            ..
        } = self
            .min_mana_allotment
            .as_mut()
            .ok_or(Error::UnfulfillableRequirement(Requirement::Mana))?;
        if let Some(output) = self
            .provided_outputs
            .iter_mut()
            .chain(&mut self.added_outputs)
            .filter(|o| o.is_account() && o.mana() != 0)
            .find(|o| o.as_account().account_id() == issuer_id)
        {
            log::debug!(
                "Reducing account mana of {} by {} for allotment",
                output.as_account().account_id(),
                allotment_debt
            );
            let output_mana = output.mana();
            *output = AccountOutputBuilder::from(output.as_account())
                .with_mana(output_mana.saturating_sub(*allotment_debt))
                .finish_output()?;
            *allotment_debt = allotment_debt.saturating_sub(output_mana);
            log::debug!("Allotment debt after reduction: {}", allotment_debt);
        }
        Ok(())
    }

    pub(crate) fn null_transaction_unlocks(&self) -> Result<Unlocks, Error> {
        let mut blocks = Vec::new();
        let mut block_indexes = HashMap::<Address, usize>::new();

        // Assuming inputs_data is ordered by address type
        for (current_block_index, input) in self.selected_inputs.iter().enumerate() {
            // Get the address that is required to unlock the input
            let required_address = input
                .output
                .required_address(
                    self.latest_slot_commitment_id.slot_index(),
                    self.protocol_parameters.committable_age_range(),
                )?
                .expect("expiration deadzone");

            // Convert restricted and implicit addresses to Ed25519 address, so they're the same entry in
            // `block_indexes`.
            let required_address = match required_address {
                Address::ImplicitAccountCreation(implicit) => Address::Ed25519(*implicit.ed25519_address()),
                Address::Restricted(restricted) => restricted.address().clone(),
                _ => required_address,
            };

            // Check if we already added an [Unlock] for this address
            match block_indexes.get(&required_address) {
                // If we already have an [Unlock] for this address, add a [Unlock] based on the address type
                Some(block_index) => match required_address {
                    Address::Ed25519(_) | Address::ImplicitAccountCreation(_) => {
                        blocks.push(Unlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
                    }
                    Address::Account(_) => blocks.push(Unlock::Account(AccountUnlock::new(*block_index as u16)?)),
                    Address::Nft(_) => blocks.push(Unlock::Nft(NftUnlock::new(*block_index as u16)?)),
                    _ => Err(BlockError::UnsupportedAddressKind(required_address.kind()))?,
                },
                None => {
                    // We can only sign ed25519 addresses and block_indexes needs to contain the account or nft
                    // address already at this point, because the reference index needs to be lower
                    // than the current block index
                    match &required_address {
                        Address::Ed25519(_) | Address::ImplicitAccountCreation(_) => {}
                        _ => Err(Error::MissingInputWithEd25519Address)?,
                    }

                    let block = SignatureUnlock::new(
                        Ed25519Signature::from_bytes(
                            [0; Ed25519Signature::PUBLIC_KEY_LENGTH],
                            [0; Ed25519Signature::SIGNATURE_LENGTH],
                        )
                        .into(),
                    )
                    .into();
                    blocks.push(block);

                    // Add the ed25519 address to the block_indexes, so it gets referenced if further inputs have
                    // the same address in their unlock condition
                    block_indexes.insert(required_address.clone(), current_block_index);
                }
            }

            // When we have an account or Nft output, we will add their account or nft address to block_indexes,
            // because they can be used to unlock outputs via [Unlock::Account] or [Unlock::Nft],
            // that have the corresponding account or nft address in their unlock condition
            match &input.output {
                Output::Account(account_output) => block_indexes.insert(
                    Address::Account(account_output.account_address(input.output_id())),
                    current_block_index,
                ),
                Output::Nft(nft_output) => block_indexes.insert(
                    Address::Nft(nft_output.nft_address(input.output_id())),
                    current_block_index,
                ),
                _ => None,
            };
        }

        Ok(Unlocks::new(blocks)?)
    }

    pub(crate) fn get_inputs_for_mana_balance(&mut self) -> Result<bool, Error> {
        let (mut selected_mana, mut required_mana) = self.mana_sums(true)?;

        log::debug!("Mana requirement selected mana: {selected_mana}, required mana: {required_mana}");

        let mut added_inputs = false;
        if selected_mana >= required_mana {
            log::debug!("Mana requirement already fulfilled");
        } else {
            if !self.allow_additional_input_selection {
                return Err(Error::AdditionalInputsRequired(Requirement::Mana));
            }
            // TODO we should do as for the amount and have preferences on which inputs to pick.
            while let Some(input) = self.available_inputs.pop() {
                selected_mana += self.total_mana(&input)?;
                if let Some(output) = self.select_input(input)? {
                    required_mana += output.mana();
                }
                added_inputs = true;

                if selected_mana >= required_mana {
                    break;
                }
            }
        }
        Ok(added_inputs)
    }

    pub(crate) fn mana_sums(&self, include_remainders: bool) -> Result<(u64, u64), Error> {
        let mut required_mana =
            self.non_remainder_outputs().map(|o| o.mana()).sum::<u64>() + self.mana_allotments.values().sum::<u64>();
        if include_remainders {
            // Add the remainder outputs mana as well as the excess mana we've allocated to add to existing outputs
            // later.
            required_mana += self.remainder_outputs().map(|o| o.mana()).sum::<u64>() + self.remainders.added_mana;
        }

        let mut selected_mana = 0;

        for input in &self.selected_inputs {
            selected_mana += self.total_mana(input)?;
        }
        Ok((selected_mana, required_mana))
    }

    fn total_mana(&self, input: &InputSigningData) -> Result<u64, Error> {
        Ok(self.mana_rewards.get(input.output_id()).copied().unwrap_or_default()
            + input.output.available_mana(
                &self.protocol_parameters,
                input.output_id().transaction_id().slot_index(),
                self.creation_slot,
            )?)
    }
}
