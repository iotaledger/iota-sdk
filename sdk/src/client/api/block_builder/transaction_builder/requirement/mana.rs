// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, sync::OnceLock};

use super::{TransactionBuilder, TransactionBuilderError};
use crate::{
    client::{
        api::transaction_builder::{requirement::PriorityMap, MinManaAllotment, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        input::{Input, UtxoInput},
        mana::ManaAllotment,
        output::{AccountOutput, AccountOutputBuilder, BasicOutput, FoundryOutput, NftOutput, Output},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        signature::Ed25519Signature,
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        BlockError,
    },
};

impl TransactionBuilder {
    pub(crate) fn fulfill_mana_requirement(&mut self) -> Result<(), TransactionBuilderError> {
        let Some(MinManaAllotment {
            issuer_id,
            reference_mana_cost,
            ..
        }) = self.min_mana_allotment
        else {
            // If there is no min allotment calculation needed, just check mana
            self.get_inputs_for_mana_balance()?;
            return Ok(());
        };

        let mut should_recalculate = false;

        if !self.selected_inputs.is_empty() && self.all_outputs().next().is_some() {
            let inputs = self
                .selected_inputs
                .iter_sorted()
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

            let transaction = builder
                .with_context_inputs(self.context_inputs())
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
                ..
            } = self
                .min_mana_allotment
                .as_mut()
                .ok_or(TransactionBuilderError::UnfulfillableRequirement(Requirement::Mana))?;

            // Add the required allotment to the issuing allotment
            if required_allotment_mana > self.mana_allotments[issuer_id] {
                log::debug!("Allotting at least {required_allotment_mana} mana to account ID {issuer_id}");
                let additional_allotment = required_allotment_mana - self.mana_allotments[issuer_id];
                log::debug!("{additional_allotment} additional mana required to meet minimum allotment");
                // Unwrap: safe because we always add the record above
                *self.mana_allotments.get_mut(issuer_id).unwrap() = required_allotment_mana;
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
        } else {
            should_recalculate = true;
        }

        // Remainders can only be calculated when the input mana is >= the output mana
        let (input_mana, output_mana) = self.mana_sums(false)?;
        if input_mana >= output_mana {
            self.update_remainders()?;
        }

        should_recalculate |= self.get_inputs_for_mana_balance()?;

        if should_recalculate && !self.requirements.contains(&Requirement::Mana) {
            self.requirements.push(Requirement::Mana);
        }

        Ok(())
    }

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
            return Ok(true);
        }
        Ok(false)
    }

    pub(crate) fn null_transaction_unlocks(&self) -> Result<Unlocks, TransactionBuilderError> {
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
        } else {
            if !self.allow_additional_input_selection {
                return Err(TransactionBuilderError::AdditionalInputsRequired(Requirement::Mana));
            }
            let include_generated = self.burn.as_ref().map_or(true, |b| !b.generated_mana());
            let mut priority_map = PriorityMap::<ManaPriority>::generate(&mut self.available_inputs);
            loop {
                let Some(input) = priority_map.next(required_mana - selected_mana) else {
                    break;
                };
                selected_mana += self.total_mana(&input, include_generated)?;
                if let Some(output) = self.select_input(input)? {
                    required_mana += output.mana();
                }
                added_inputs = true;

                if selected_mana >= required_mana {
                    break;
                }
            }
            // Return unselected inputs to the available list
            for input in priority_map.into_inputs() {
                self.available_inputs.push(input);
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

    pub(crate) fn mana_sums(&self, include_remainders: bool) -> Result<(u64, u64), TransactionBuilderError> {
        let mut required_mana =
            self.non_remainder_outputs().map(|o| o.mana()).sum::<u64>() + self.mana_allotments.values().sum::<u64>();
        if include_remainders {
            // Add the remainder outputs mana as well as the excess mana we've allocated to add to existing outputs
            // later.
            required_mana += self.remainder_outputs().map(|o| o.mana()).sum::<u64>() + self.remainders.added_mana;
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

    fn total_mana(&self, input: &InputSigningData, include_generated: bool) -> Result<u64, TransactionBuilderError> {
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ManaPriority {
    kind_priority: usize,
    has_native_token: bool,
}

impl PartialOrd for ManaPriority {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ManaPriority {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.kind_priority, self.has_native_token).cmp(&(other.kind_priority, other.has_native_token))
    }
}

impl From<&InputSigningData> for Option<ManaPriority> {
    fn from(value: &InputSigningData) -> Self {
        sort_order_type()
            .get(&value.output.kind())
            .map(|&kind_priority| ManaPriority {
                kind_priority,
                has_native_token: value.output.native_token().is_some(),
            })
    }
}

/// Establish the order in which we want to pick an input
pub fn sort_order_type() -> &'static HashMap<u8, usize> {
    static MAP: OnceLock<HashMap<u8, usize>> = OnceLock::new();
    MAP.get_or_init(|| {
        [
            BasicOutput::KIND,
            NftOutput::KIND,
            AccountOutput::KIND,
            FoundryOutput::KIND,
        ]
        .into_iter()
        .zip(0_usize..)
        .collect::<HashMap<_, _>>()
    })
}

impl PriorityMap<ManaPriority> {
    fn next(&mut self, missing_mana: u64) -> Option<InputSigningData> {
        let mana_sort = |mana: u64| {
            // If the mana is greater than the missing mana, we want the smallest ones first
            if mana >= missing_mana {
                (false, mana)
            // Otherwise, we want the biggest first
            } else {
                (true, u64::MAX - mana)
            }
        };
        if let Some((priority, mut inputs)) = self.0.pop_first() {
            // Sort in reverse so we can pop from the back
            inputs.sort_unstable_by(|i1, i2| mana_sort(i2.output.mana()).cmp(&mana_sort(i1.output.mana())));
            let input = inputs.pop();
            if !inputs.is_empty() {
                self.0.insert(priority, inputs);
            }
            return input;
        }
        None
    }
}
