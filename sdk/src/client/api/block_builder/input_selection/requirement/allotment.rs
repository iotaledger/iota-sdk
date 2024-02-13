// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use super::{Error, InputSelection};
use crate::{
    client::{
        api::input_selection::{AutoManaAllotment, Requirement},
        secret::types::InputSigningData,
    },
    types::block::{
        address::Address,
        input::{Input, UtxoInput},
        mana::ManaAllotment,
        output::{AccountOutputBuilder, ChainId, NftOutputBuilder, Output},
        payload::{signed_transaction::Transaction, SignedTransactionPayload},
        signature::Ed25519Signature,
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        Error as BlockError,
    },
};

impl InputSelection {
    pub(crate) fn fulfill_allotment_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        let AutoManaAllotment {
            issuer_id,
            reference_mana_cost,
        } = self
            .auto_mana_allotment
            .ok_or(Error::UnfulfillableRequirement(Requirement::Allotment))?;

        self.selected_inputs = Self::sort_input_signing_data(
            std::mem::take(&mut self.selected_inputs),
            self.creation_slot_index,
            self.protocol_parameters.committable_age_range(),
        )?;

        let mut inputs = Vec::new();
        for input in &self.selected_inputs {
            inputs.push(Input::Utxo(UtxoInput::from(*input.output_id())));
        }

        let mut builder = Transaction::builder(self.protocol_parameters.network_id())
            .with_inputs(inputs)
            .with_outputs(self.outputs.clone());

        if let Some(payload) = &self.payload {
            builder = builder.with_payload(payload.clone());
        }

        // Add the empty allotment so the work score includes it
        self.mana_allotments.entry(issuer_id).or_default();

        let transaction = builder
            .with_context_inputs(self.context_inputs.clone())
            .with_mana_allotments(self.mana_allotments.iter().map(|(&account_id, &mana)| ManaAllotment {
                account_id,
                mana: mana.0,
            }))
            .finish_with_params(&self.protocol_parameters)?;

        let signed_transaction = SignedTransactionPayload::new(transaction, self.null_transaction_unlocks()?)?;

        let block_work_score = self.protocol_parameters.work_score(&signed_transaction)
            + self.protocol_parameters.work_score_parameters().block();
        let required_allotment_mana = block_work_score as u64 * reference_mana_cost;

        // Add the required allotment to the issuing allotment
        if self.mana_allotments[&issuer_id].0 < required_allotment_mana {
            let (selected_mana, required_mana) = self.mana_sums_without_allotments()?;
            let available_mana = selected_mana.saturating_sub(required_mana);
            log::debug!("available_mana {available_mana}");

            let mut previously_added_mana = self.mana_allotments[&issuer_id].1;
            log::debug!("previously_added_mana {previously_added_mana}");

            let mut updated_required_allotment_mana = required_allotment_mana
                .saturating_sub(previously_added_mana)
                .saturating_sub(available_mana);
            let mut new_added_mana = 0;

            // Reduce mana amount of automatically transitioned outputs
            for output in self.outputs.iter_mut().filter(|o| match o {
                Output::Account(account) => {
                    self.automatically_transitioned
                        .contains(&ChainId::Account(*account.account_id()))
                        && *account.account_id() == issuer_id
                }
                Output::Nft(nft) => self.automatically_transitioned.contains(&ChainId::Nft(*nft.nft_id())),
                _ => false,
            }) {
                if updated_required_allotment_mana == 0 {
                    break;
                }
                match output {
                    Output::Account(account) => {
                        let account_id = *account.account_id();

                        let new_mana = if output.mana() >= updated_required_allotment_mana {
                            let new_mana = output.mana() - updated_required_allotment_mana;
                            updated_required_allotment_mana = 0;
                            new_mana
                        } else {
                            updated_required_allotment_mana -= output.mana();
                            0
                        };

                        new_added_mana += output.mana() - new_mana;
                        log::debug!(
                            "Reducing account mana of {} by {} for allotment",
                            account_id,
                            output.mana() - new_mana
                        );
                        *output = match output {
                            Output::Account(a) => {
                                AccountOutputBuilder::from(&*a).with_mana(new_mana).finish_output()?
                            }
                            _ => unreachable!(),
                        };
                    }
                    Output::Nft(nft) => {
                        let nft_id = *nft.nft_id();
                        let new_mana = if output.mana() >= updated_required_allotment_mana {
                            let new_mana = output.mana() - updated_required_allotment_mana;
                            updated_required_allotment_mana = 0;
                            new_mana
                        } else {
                            updated_required_allotment_mana -= output.mana();
                            0
                        };

                        new_added_mana += output.mana() - new_mana;
                        log::debug!(
                            "Reducing nft mana of {} by {} for allotment",
                            nft_id,
                            output.mana() - new_mana
                        );
                        *output = match output {
                            Output::Nft(a) => NftOutputBuilder::from(&*a).with_mana(new_mana).finish_output()?,
                            _ => unreachable!(),
                        };
                    }
                    _ => unreachable!(),
                }
            }

            log::debug!("Allotting {required_allotment_mana} to account ID {issuer_id}");
            previously_added_mana += new_added_mana;

            if updated_required_allotment_mana == 0 {
                // needs to be set to 0 if enough was found, otherwise would try to create a remainder for it
                previously_added_mana = 0
            }

            log::debug!("setting previously_added_mana {}", previously_added_mana);
            self.mana_allotments
                .insert(issuer_id, (required_allotment_mana, previously_added_mana));

            log::debug!("Checking mana requirement again with added allotment");
            let additional_inputs = self.fulfill_mana_requirement()?;
            // If we needed more inputs to cover the additional allotment mana
            // then re-add this requirement so we try again
            if !additional_inputs.is_empty() {
                self.requirements.push(Requirement::Allotment);
                return Ok(additional_inputs);
            }
        }

        Ok(Vec::new())
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
                    self.creation_slot_index,
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
}
