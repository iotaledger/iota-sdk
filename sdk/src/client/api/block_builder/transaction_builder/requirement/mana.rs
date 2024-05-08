// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use super::{TransactionBuilder, TransactionBuilderError};
use crate::{
    client::{
        api::transaction_builder::{MinManaAllotment, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        input::{Input, UtxoInput, INPUT_COUNT_MAX},
        mana::ManaAllotment,
        output::{AccountOutput, AccountOutputBuilder, BasicOutput, ChainId, FoundryOutput, NftOutput, Output},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        signature::Ed25519Signature,
        slot::{SlotCommitmentId, SlotIndex},
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        BlockError,
    },
};

impl TransactionBuilder {
    pub(crate) fn fulfill_mana_requirement(&mut self) -> Result<(), TransactionBuilderError> {
        if self.min_mana_allotment.is_none() {
            // If there is no min allotment calculation needed, just check mana
            self.get_inputs_for_mana_balance()?;
            return Ok(());
        }

        let mut should_recalculate = false;

        if let Some(required_allotment) = self.required_allotment()? {
            let MinManaAllotment {
                issuer_id,
                allotment_debt,
                ..
            } = self.min_mana_allotment.as_mut().unwrap();
            // Add the required allotment to the issuing allotment
            if required_allotment > self.mana_allotments[issuer_id] {
                log::debug!("Allotting at least {required_allotment} mana to account ID {issuer_id}");
                let additional_allotment = required_allotment - self.mana_allotments[issuer_id];
                log::debug!("{additional_allotment} additional mana required to meet minimum allotment");
                // Unwrap: safe because we always add the record above
                *self.mana_allotments.get_mut(issuer_id).unwrap() = required_allotment;
                log::debug!("Adding {additional_allotment} to allotment debt {allotment_debt}");
                *allotment_debt += additional_allotment;
                should_recalculate = true;
            } else {
                log::debug!("Setting allotment debt to {}", self.mana_allotments[issuer_id]);
                *allotment_debt = self.mana_allotments[issuer_id];
                // Since the allotment is fine, check if the mana balance is good because
                // we can exit early in that case.
                let (input_mana, output_mana) = self.mana_sums(true)?;
                if input_mana == output_mana {
                    log::debug!("allotments and mana are both correct, no further action needed");
                    return Ok(());
                }
            }

            should_recalculate |= self.reduce_account_output()?;
        }

        // Remainders can only be calculated when the input mana is >= the output mana
        let (input_mana, output_mana) = self.mana_sums(false)?;
        if input_mana >= output_mana {
            should_recalculate |= self.update_remainders()?;
        }

        should_recalculate |= self.get_inputs_for_mana_balance()?;

        if should_recalculate && !self.requirements.contains(&Requirement::Mana) {
            self.requirements.insert(0, Requirement::Mana);
        }

        Ok(())
    }

    pub(crate) fn required_allotment(&mut self) -> Result<Option<u64>, TransactionBuilderError> {
        let Some(MinManaAllotment {
            issuer_id,
            reference_mana_cost,
            required_allotment,
            ..
        }) = self.min_mana_allotment
        else {
            return Ok(None);
        };

        if !self.selected_inputs.is_empty() && self.all_outputs().next().is_some() {
            let inputs = self
                .selected_inputs
                .sorted_iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_id())));

            let mut builder = Transaction::builder(self.protocol_parameters.network_id())
                .with_inputs(inputs)
                .with_outputs(self.all_outputs().cloned());

            if let Some(payload) = &self.payload {
                builder = builder.with_payload(payload.clone());
            }

            // Add the empty allotment so the work score includes it
            self.mana_allotments.entry(issuer_id).or_default();

            let transaction = builder
                .with_context_inputs(self.context_inputs())
                .with_mana_allotments(
                    self.mana_allotments
                        .iter()
                        .map(|(&account_id, &mana)| ManaAllotment { account_id, mana }),
                )
                .with_capabilities(self.transaction_capabilities.clone())
                .finish_with_params(&self.protocol_parameters)?;

            let signed_transaction = SignedTransactionPayload::new(transaction, self.null_transaction_unlocks()?)?;

            let block_work_score = self.protocol_parameters.work_score(&signed_transaction)
                + self.protocol_parameters.work_score_parameters().block();

            let MinManaAllotment { required_allotment, .. } = self.min_mana_allotment.as_mut().unwrap();
            required_allotment.replace(block_work_score as u64 * reference_mana_cost);

            return Ok(*required_allotment);
        }
        Ok(required_allotment)
    }

    /// Reduce an account output by an allotment value, if one exists.
    /// This will only affect automatically transitioned accounts.
    fn reduce_account_output(&mut self) -> Result<bool, TransactionBuilderError> {
        let MinManaAllotment {
            issuer_id,
            allotment_debt,
            ..
        } = self
            .min_mana_allotment
            .as_mut()
            .ok_or(TransactionBuilderError::UnfulfillableRequirement(Requirement::Mana))?;
        if let Some(output) = self
            .added_outputs
            .iter_mut()
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
            return Ok(true);
        }
        Ok(false)
    }

    pub(crate) fn null_transaction_unlocks(&self) -> Result<Unlocks, TransactionBuilderError> {
        let mut blocks = Vec::new();
        let mut block_indexes = HashMap::<Address, usize>::new();

        // Assuming inputs_data is ordered by address type
        for (current_block_index, input) in self.selected_inputs.sorted_iter().enumerate() {
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
                        _ => Err(TransactionBuilderError::MissingInputWithEd25519Address)?,
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

    pub(crate) fn get_inputs_for_mana_balance(&mut self) -> Result<bool, TransactionBuilderError> {
        let (mut selected_mana, mut required_mana) = self.mana_sums(true)?;

        log::debug!("Mana requirement selected mana: {selected_mana}, required mana: {required_mana}");

        let mut added_inputs = false;
        if selected_mana >= required_mana {
            log::debug!("Mana requirement already fulfilled");
            return Ok(false);
        }
        if !self.allow_additional_input_selection {
            return Err(TransactionBuilderError::AdditionalInputsRequired(Requirement::Mana));
        }
        let include_generated = self.burn.as_ref().map_or(true, |b| !b.generated_mana());
        while let Some(input) = self.next_input_for_mana(
            required_mana - selected_mana,
            include_generated,
            self.latest_slot_commitment_id,
        ) {
            selected_mana += self.total_mana(&input, include_generated)?;
            if self.select_input(input)? {
                let output = self.added_outputs.last().unwrap();
                // If we're allotting, it's possible the added output should be reduced, so just exit early and
                // We will re-calculate the allotment.
                if let Some(MinManaAllotment { issuer_id, .. }) = &self.min_mana_allotment {
                    if output
                        .as_account_opt()
                        .is_some_and(|account| account.account_id() == issuer_id)
                    {
                        return Ok(true);
                    }
                }
                required_mana += output.mana();
            }
            added_inputs = true;

            if selected_mana >= required_mana {
                break;
            }
        }

        Ok(added_inputs)
    }

    pub(crate) fn initial_mana_excess(&self) -> Result<u64, TransactionBuilderError> {
        let output_mana = self.provided_outputs.iter().map(|o| o.mana()).sum::<u64>();
        let mut input_mana = 0;
        let include_generated = self.burn.as_ref().map_or(true, |b| !b.generated_mana());

        for input in self
            .selected_inputs
            .iter()
            .filter(|i| self.required_inputs.contains(i.output_id()))
        {
            input_mana += self.total_mana(input, include_generated)?;
        }

        Ok(input_mana.saturating_sub(output_mana))
    }

    pub(crate) fn mana_sums(&mut self, include_remainders: bool) -> Result<(u64, u64), TransactionBuilderError> {
        let allotments_sum = if let Some(MinManaAllotment { issuer_id, .. }) = self.min_mana_allotment {
            let mut required_allotment = self.min_mana_allotment.and_then(|a| a.required_allotment);
            if required_allotment.is_none() {
                required_allotment = self.required_allotment()?;
            }
            self.mana_allotments
                .iter()
                .filter_map(|(id, value)| (id != &issuer_id).then_some(value))
                .sum::<u64>()
                + self
                    .mana_allotments
                    .get(&issuer_id)
                    .copied()
                    .unwrap_or_default()
                    .max(required_allotment.unwrap_or_default())
        } else {
            self.mana_allotments.values().sum::<u64>()
        };
        let mut required_mana = self.non_remainder_outputs().map(|o| o.mana()).sum::<u64>() + allotments_sum;
        if include_remainders {
            // Add the remainder outputs mana as well as the excess mana we've allocated to add to existing outputs
            // later.
            required_mana += self.remainder_outputs().map(|o| o.mana()).sum::<u64>()
                + self.remainders.added_mana.values().sum::<u64>();
        }

        Ok((self.total_selected_mana(None)?, required_mana))
    }

    pub(crate) fn total_selected_mana(
        &self,
        include_generated: impl Into<Option<bool>> + Copy,
    ) -> Result<u64, TransactionBuilderError> {
        let mut selected_mana = 0;
        let include_generated = include_generated
            .into()
            .unwrap_or_else(|| self.burn.as_ref().map_or(true, |b| !b.generated_mana()));

        for input in self.selected_inputs.iter() {
            selected_mana += self.total_mana(input, include_generated)?;
        }

        Ok(selected_mana)
    }

    pub(crate) fn total_mana(
        &self,
        input: &InputSigningData,
        include_generated: bool,
    ) -> Result<u64, TransactionBuilderError> {
        Ok(self.mana_rewards.get(input.output_id()).copied().unwrap_or_default()
            + if include_generated {
                input.output.available_mana(
                    &self.protocol_parameters,
                    input.output_id().transaction_id().slot_index(),
                    self.creation_slot,
                )?
            } else {
                input.output.mana()
            })
    }

    pub(crate) fn mana_chains(&self) -> Result<HashMap<ChainId, (u64, u64)>, TransactionBuilderError> {
        let include_generated = self.burn.as_ref().map_or(true, |b| !b.generated_mana());
        let mut res = self
            .non_remainder_outputs()
            .filter_map(|o| o.chain_id().map(|id| (id, (0, o.mana()))))
            .collect::<HashMap<_, _>>();
        for input in self.selected_inputs.iter() {
            if let Some(chain_id) = input
                .output
                .chain_id()
                .map(|id| id.or_from_output_id(input.output_id()))
            {
                res.entry(chain_id).or_default().0 += self.total_mana(input, include_generated)?;
            }
        }
        Ok(res)
    }

    fn next_input_for_mana(
        &mut self,
        missing_mana: u64,
        include_generated: bool,
        slot_commitment_id: SlotCommitmentId,
    ) -> Option<InputSigningData> {
        self.available_inputs
            .iter()
            .enumerate()
            .filter_map(|(idx, input)| {
                self.score_for_mana(input, missing_mana, include_generated, slot_commitment_id.slot_index())
                    .map(|score| (score, idx))
            })
            .max_by_key(|(score, _)| *score)
            .map(|(_, idx)| self.available_inputs.swap_remove(idx))
    }

    // Score an input based on how desirable it is.
    fn score_for_mana(
        &self,
        input: &InputSigningData,
        missing_mana: u64,
        include_generated: bool,
        slot_index: SlotIndex,
    ) -> Option<usize> {
        ([
            BasicOutput::KIND,
            NftOutput::KIND,
            AccountOutput::KIND,
            FoundryOutput::KIND,
        ]
        .contains(&input.output.kind()))
        .then(|| {
            let mut work_score = self
                .protocol_parameters
                .work_score(&UtxoInput::from(*input.output_id()));
            let mut mana_gained = self.total_mana(input, include_generated).unwrap_or_default();
            let mut remainder_work_score = 0;
            if super::amount::sdruc_not_expired(&input.output, slot_index).is_some() {
                remainder_work_score = self.protocol_parameters.work_score(self.basic_remainder())
            }

            if let Ok(Some(output)) = self.transition_input(input) {
                work_score += self.protocol_parameters.work_score(&output);
                if let Some(allotment) = self.min_mana_allotment {
                    // If we're allotting to this output account, we will have more mana from the reduction.
                    if output.as_account_opt().is_some_and(|account| {
                        account.is_block_issuer() && account.account_id() == &allotment.issuer_id
                    }) {
                        // We can regain as much as the full account mana value
                        // by reducing the mana on the account.
                        let new_required_allotment = allotment.required_allotment.unwrap_or_default()
                            + (work_score as u64 * allotment.reference_mana_cost);
                        mana_gained += new_required_allotment.min(output.mana());
                    }
                }
                mana_gained = mana_gained.saturating_sub(output.mana());
            } else if input.output.native_token().is_some() {
                remainder_work_score += self.protocol_parameters.work_score(self.native_token_remainder())
            } else if mana_gained > missing_mana {
                remainder_work_score = self.protocol_parameters.work_score(self.basic_remainder())
            }
            work_score += remainder_work_score;

            // The gained mana is reduced by the amount we'll need to allot later.
            if let Some(allotment) = self.min_mana_allotment {
                mana_gained = mana_gained.saturating_sub(work_score as u64 * allotment.reference_mana_cost);
            }

            if mana_gained == 0
                && !(input.output.as_account_opt().is_some_and(|account| {
                    account.is_block_issuer()
                        && self
                            .min_mana_allotment
                            .is_some_and(|allotment| *account.account_id() == allotment.issuer_id)
                }))
            {
                return None;
            }

            let mana_diff = mana_gained.abs_diff(missing_mana) as f64;
            // Exp(-x) creates a curve which is 1 when x is 0, and approaches 0 as x increases
            // If the mana is insufficient, the score will decrease the more inputs are selected
            let mana_score = if mana_gained >= missing_mana {
                (-mana_diff / u64::MAX as f64).exp()
            } else {
                (-mana_diff / missing_mana as f64).exp()
                    * ((INPUT_COUNT_MAX as f64 - self.selected_inputs.len() as f64) / INPUT_COUNT_MAX as f64)
            };
            let work_score = (-(work_score as f64) / u32::MAX as f64).exp();
            // Normalize scores between 0..1 with 1 being desirable
            Some((mana_score * work_score * usize::MAX as f64).round() as _)
        })
        .flatten()
    }
}
