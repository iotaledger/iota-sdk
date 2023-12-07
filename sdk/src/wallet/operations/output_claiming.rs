// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::{Address, Ed25519Address},
        output::{
            unlock_condition::{AddressUnlockCondition, StorageDepositReturnUnlockCondition},
            BasicOutput, BasicOutputBuilder, NativeTokensBuilder, NftOutputBuilder, Output, OutputId,
        },
        slot::SlotIndex,
    },
    wallet::{
        core::WalletData,
        operations::{helpers::time::can_output_be_unlocked_now, transaction::TransactionOptions},
        types::{OutputData, TransactionWithMetadata},
        Wallet,
    },
};

/// Enum to specify which outputs should be claimed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OutputsToClaim {
    MicroTransactions,
    NativeTokens,
    Nfts,
    Amount,
    All,
}

impl WalletData {
    /// Get basic and nft outputs that have
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition),
    /// [`StorageDepositReturnUnlockCondition`] or
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) and can be
    /// unlocked now and also get basic outputs with only an [`AddressUnlockCondition`] unlock condition, for
    /// additional inputs
    pub(crate) fn claimable_outputs(
        &self,
        outputs_to_claim: OutputsToClaim,
        slot_index: SlotIndex,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[OUTPUT_CLAIMING] claimable_outputs");

        // Get outputs for the claim
        let mut output_ids_to_claim: HashSet<OutputId> = HashSet::new();
        for (output_id, output_data) in self
            .unspent_outputs
            .iter()
            .filter(|(_, o)| o.output.is_basic() || o.output.is_nft())
        {
            // Don't use outputs that are locked for other transactions
            if !self.locked_outputs.contains(output_id) && self.outputs.contains_key(output_id) {
                if let Some(unlock_conditions) = output_data.output.unlock_conditions() {
                    // If there is a single [UnlockCondition], then it's an
                    // [AddressUnlockCondition] and we own it already without
                    // further restrictions
                    if unlock_conditions.len() != 1
                        && can_output_be_unlocked_now(
                            // We use the addresses with unspent outputs, because other addresses of the
                            // account without unspent outputs can't be related to this output
                            self.address.inner(),
                            output_data,
                            slot_index,
                        )?
                    {
                        match outputs_to_claim {
                            OutputsToClaim::MicroTransactions => {
                                if let Some(sdr) = unlock_conditions.storage_deposit_return() {
                                    // If expired, it's not a micro transaction anymore
                                    if unlock_conditions.is_expired(slot_index) {
                                        continue;
                                    }
                                    // Only micro transaction if not the same
                                    if sdr.amount() != output_data.output.amount() {
                                        output_ids_to_claim.insert(output_data.output_id);
                                    }
                                }
                            }
                            OutputsToClaim::NativeTokens => {
                                // TODO https://github.com/iotaledger/iota-sdk/issues/1633
                                // if !output_data.output.native_tokens().map(|n| n.is_empty()).unwrap_or(true) {
                                //     output_ids_to_claim.insert(output_data.output_id);
                                // }
                            }
                            OutputsToClaim::Nfts => {
                                if output_data.output.is_nft() {
                                    output_ids_to_claim.insert(output_data.output_id);
                                }
                            }
                            OutputsToClaim::Amount => {
                                let mut claimable_amount = output_data.output.amount();
                                if !unlock_conditions.is_expired(slot_index) {
                                    claimable_amount -= unlock_conditions
                                        .storage_deposit_return()
                                        .map(|s| s.amount())
                                        .unwrap_or_default()
                                };
                                if claimable_amount > 0 {
                                    output_ids_to_claim.insert(output_data.output_id);
                                }
                            }
                            OutputsToClaim::All => {
                                output_ids_to_claim.insert(output_data.output_id);
                            }
                        }
                    }
                }
            }
        }
        log::debug!(
            "[OUTPUT_CLAIMING] available outputs to claim: {}",
            output_ids_to_claim.len()
        );
        Ok(output_ids_to_claim.into_iter().collect())
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Get basic and nft outputs that have
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition),
    /// [`StorageDepositReturnUnlockCondition`] or
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) and can be
    /// unlocked now and also get basic outputs with only an [`AddressUnlockCondition`] unlock condition, for
    /// additional inputs
    pub async fn claimable_outputs(&self, outputs_to_claim: OutputsToClaim) -> crate::wallet::Result<Vec<OutputId>> {
        let wallet_data = self.data().await;

        let slot_index = self.client().get_slot_index().await?;

        wallet_data.claimable_outputs(outputs_to_claim, slot_index)
    }

    /// Get basic outputs that have only one unlock condition which is [AddressUnlockCondition], so they can be used as
    /// additional inputs
    pub(crate) async fn get_basic_outputs_for_additional_inputs(&self) -> crate::wallet::Result<Vec<OutputData>> {
        log::debug!("[OUTPUT_CLAIMING] get_basic_outputs_for_additional_inputs");
        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await?;
        let wallet_data = self.data().await;

        // Get basic outputs only with AddressUnlockCondition and no other unlock condition
        let mut basic_outputs: Vec<OutputData> = Vec::new();
        for (output_id, output_data) in &wallet_data.unspent_outputs {
            #[cfg(feature = "participation")]
            if let Some(ref voting_output) = voting_output {
                // Remove voting output from inputs, because we don't want to spent it to claim something else.
                if output_data.output_id == voting_output.output_id {
                    continue;
                }
            }
            // Don't use outputs that are locked for other transactions
            if !wallet_data.locked_outputs.contains(output_id) {
                if let Some(output) = wallet_data.outputs.get(output_id) {
                    if let Output::Basic(basic_output) = &output.output {
                        if basic_output.unlock_conditions().len() == 1 {
                            // Store outputs with [`AddressUnlockCondition`] alone, because they could be used as
                            // additional input, if required
                            basic_outputs.push(output_data.clone());
                        }
                    }
                }
            }
        }
        log::debug!("[OUTPUT_CLAIMING] available basic outputs: {}", basic_outputs.len());
        Ok(basic_outputs)
    }

    /// Try to claim basic or nft outputs that have additional unlock conditions to their [AddressUnlockCondition]
    /// from [`Wallet::claimable_outputs()`].
    pub async fn claim_outputs<I: IntoIterator<Item = OutputId> + Send>(
        &self,
        output_ids_to_claim: I,
    ) -> crate::wallet::Result<TransactionWithMetadata>
    where
        I::IntoIter: Send,
    {
        log::debug!("[OUTPUT_CLAIMING] claim_outputs");
        let prepared_transaction = self.prepare_claim_outputs(output_ids_to_claim).await.map_err(|error| {
            // Map InsufficientStorageDepositAmount error here because it's the result of InsufficientFunds in this
            // case and then easier to handle
            match error {
                crate::wallet::Error::Block(block_error) => match *block_error {
                    crate::types::block::Error::InsufficientStorageDepositAmount { amount, required } => {
                        crate::wallet::Error::InsufficientFunds {
                            available: amount,
                            required,
                        }
                    }
                    _ => crate::wallet::Error::Block(block_error),
                },
                _ => error,
            }
        })?;

        let claim_tx = self
            .sign_and_submit_transaction(prepared_transaction, None, None)
            .await?;

        log::debug!(
            "[OUTPUT_CLAIMING] Claiming transaction created: block_id: {:?} tx_id: {:?}",
            claim_tx.block_id,
            claim_tx.transaction_id
        );
        Ok(claim_tx)
    }

    /// Try to claim basic outputs that have additional unlock conditions to their [AddressUnlockCondition].
    pub async fn prepare_claim_outputs<I: IntoIterator<Item = OutputId> + Send>(
        &self,
        output_ids_to_claim: I,
    ) -> crate::wallet::Result<PreparedTransactionData>
    where
        I::IntoIter: Send,
    {
        log::debug!("[OUTPUT_CLAIMING] prepare_claim_outputs");

        let mut possible_additional_inputs = self.get_basic_outputs_for_additional_inputs().await?;

        let slot_index = self.client().get_slot_index().await?;
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let wallet_data = self.data().await;

        let mut outputs_to_claim = Vec::new();
        for output_id in output_ids_to_claim {
            if let Some(output_data) = wallet_data.unspent_outputs.get(&output_id) {
                if !wallet_data.locked_outputs.contains(&output_id) {
                    outputs_to_claim.push(output_data.clone());
                }
            }
        }

        if outputs_to_claim.is_empty() {
            return Err(crate::wallet::Error::CustomInput(
                "provided outputs can't be claimed".to_string(),
            ));
        }

        let wallet_address = wallet_data.address.clone();
        drop(wallet_data);

        let mut additional_inputs_used = HashSet::new();

        // Outputs with expiration and storage deposit return might require two outputs if there is a storage deposit
        // return unlock condition Maybe also more additional inputs are required for the storage deposit, if we
        // have to send the storage deposit back.

        let mut outputs_to_send = Vec::new();
        // Keep track of the outputs to return, so we only create one output per address
        let mut required_address_returns: HashMap<Address, u64> = HashMap::new();
        // Amount we get with the storage deposit return amounts already subtracted
        let mut available_amount = 0;
        let mut required_amount_for_nfts = 0;
        let mut new_native_tokens = NativeTokensBuilder::new();

        // There can be outputs with less amount than min required storage deposit, so we have to check that we
        // have enough amount to create a new basic output
        let enough_amount_for_basic_output = possible_additional_inputs
            .iter()
            .map(|i| i.output.amount())
            .sum::<u64>()
            >= BasicOutput::minimum_amount(&Address::from(Ed25519Address::null()), storage_score_params);

        // check native tokens
        for output_data in &outputs_to_claim {
            if let Some(native_token) = output_data.output.native_token() {
                new_native_tokens.add_native_token(*native_token)?;
            }
            if let Some(sdr) = sdr_not_expired(&output_data.output, slot_index) {
                // for own output subtract the return amount
                available_amount += output_data.output.amount() - sdr.amount();

                // Insert for return output
                *required_address_returns
                    .entry(sdr.return_address().clone())
                    .or_default() += sdr.amount();
            } else {
                available_amount += output_data.output.amount();
            }

            if let Output::Nft(nft_output) = &output_data.output {
                // build new output with same amount, nft_id, immutable/feature blocks and native tokens, just
                // updated address unlock conditions

                let nft_output = if !enough_amount_for_basic_output {
                    // Only update address and nft id if we have no additional inputs which can provide the storage
                    // deposit for the remaining amount and possible native tokens
                    NftOutputBuilder::from(nft_output)
                        .with_nft_id(nft_output.nft_id_non_null(&output_data.output_id))
                        .with_unlock_conditions([AddressUnlockCondition::new(wallet_address.clone())])
                        .finish_output()?
                } else {
                    NftOutputBuilder::from(nft_output)
                        .with_minimum_amount(storage_score_params)
                        .with_nft_id(nft_output.nft_id_non_null(&output_data.output_id))
                        .with_unlock_conditions([AddressUnlockCondition::new(wallet_address.clone())])
                        .finish_output()?
                };

                // Add required amount for the new output
                required_amount_for_nfts += nft_output.amount();
                outputs_to_send.push(nft_output);
            }
        }

        // TODO: rework native tokens
        let option_native_token = if new_native_tokens.is_empty() {
            None
        } else {
            Some(new_native_tokens.clone().finish()?)
        };

        // Check if the new amount is enough for the storage deposit, otherwise increase it with a minimal basic output
        // amount
        let mut required_amount = if !enough_amount_for_basic_output {
            required_amount_for_nfts
        } else {
            required_amount_for_nfts
                + BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
                    .add_unlock_condition(AddressUnlockCondition::new(Ed25519Address::null()))
                    // TODO https://github.com/iotaledger/iota-sdk/issues/1633
                    // .with_native_tokens(option_native_token.into_iter().flatten())
                    .finish()?
                    .amount()
        };

        let mut additional_inputs = Vec::new();
        if available_amount < required_amount {
            // Sort by amount so we use as little as possible
            possible_additional_inputs.sort_by_key(|o| o.output.amount());

            // add more inputs
            for output_data in &possible_additional_inputs {
                let option_native_token = if new_native_tokens.is_empty() {
                    None
                } else {
                    Some(new_native_tokens.clone().finish()?)
                };
                // Recalculate every time, because new inputs can also add more native tokens, which would increase
                // the required storage deposit
                required_amount = required_amount_for_nfts
                    + BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
                        .add_unlock_condition(AddressUnlockCondition::new(Ed25519Address::null()))
                        // TODO https://github.com/iotaledger/iota-sdk/issues/1633
                        // .with_native_token(option_native_token)
                        .finish()?
                        .amount();

                if available_amount < required_amount {
                    if !additional_inputs_used.contains(&output_data.output_id) {
                        if let Some(native_token) = output_data.output.native_token() {
                            new_native_tokens.add_native_token(*native_token)?;
                        }
                        available_amount += output_data.output.amount();
                        additional_inputs.push(output_data.output_id);
                        additional_inputs_used.insert(output_data.output_id);
                    }
                } else {
                    // Break if we have enough inputs
                    break;
                }
            }
        }

        // If we still don't have enough amount we can't create the output
        if available_amount < required_amount {
            return Err(crate::wallet::Error::InsufficientFunds {
                available: available_amount,
                required: required_amount,
            });
        }

        for (return_address, return_amount) in required_address_returns {
            outputs_to_send.push(
                BasicOutputBuilder::new_with_amount(return_amount)
                    .add_unlock_condition(AddressUnlockCondition::new(return_address))
                    .finish_output()?,
            );
        }

        // Create output with claimed values
        if available_amount - required_amount_for_nfts > 0 {
            outputs_to_send.push(
                BasicOutputBuilder::new_with_amount(available_amount - required_amount_for_nfts)
                    .add_unlock_condition(AddressUnlockCondition::new(wallet_address))
                    // TODO https://github.com/iotaledger/iota-sdk/issues/1633
                    // .with_native_tokens(new_native_tokens.finish()?)
                    .finish_output()?,
            );
        } else if !new_native_tokens.finish()?.is_empty() {
            return Err(crate::client::api::input_selection::Error::InsufficientAmount {
                found: available_amount,
                required: required_amount_for_nfts,
            })?;
        }

        self.prepare_transaction(
            outputs_to_send,
            Some(TransactionOptions {
                custom_inputs: Some(
                    outputs_to_claim
                        .iter()
                        .map(|o| o.output_id)
                        // add additional inputs
                        .chain(additional_inputs)
                        .collect::<Vec<OutputId>>(),
                ),
                ..Default::default()
            }),
        )
        .await
    }
}

/// Get the `StorageDepositReturnUnlockCondition`, if not expired
pub(crate) fn sdr_not_expired(output: &Output, slot_index: SlotIndex) -> Option<&StorageDepositReturnUnlockCondition> {
    output.unlock_conditions().and_then(|unlock_conditions| {
        unlock_conditions.storage_deposit_return().and_then(|sdr| {
            let expired = unlock_conditions
                .expiration()
                .map_or(false, |expiration| slot_index >= expiration.slot_index());

            // We only have to send the storage deposit return back if the output is not expired
            (!expired).then_some(sdr)
        })
    })
}
