// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::{Address, Ed25519Address},
        context_input::CommitmentContextInput,
        output::{
            unlock_condition::AddressUnlockCondition, BasicOutput, NftOutputBuilder, Output, OutputId, UnlockCondition,
        },
        protocol::ProtocolParameters,
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
    /// [`StorageDepositReturnUnlockCondition`](crate::types::block::output::unlock_condition::StorageDepositReturnUnlockCondition) or
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) and can be
    /// unlocked now and also get basic outputs with only an [`AddressUnlockCondition`] unlock condition, for
    /// additional inputs
    pub(crate) fn claimable_outputs(
        &self,
        outputs_to_claim: OutputsToClaim,
        slot_index: SlotIndex,
        protocol_parameters: &ProtocolParameters,
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
                            protocol_parameters.committable_age_range(),
                        )?
                    {
                        match outputs_to_claim {
                            OutputsToClaim::MicroTransactions => {
                                if let Some(sdr) = unlock_conditions.storage_deposit_return() {
                                    // If expired, it's not a micro transaction anymore
                                    match unlock_conditions
                                        .is_expired(slot_index, protocol_parameters.committable_age_range())
                                    {
                                        Some(false) => {
                                            // Only micro transaction if not the same amount needs to be returned
                                            // (resulting in 0 amount to claim)
                                            if sdr.amount() != output_data.output.amount() {
                                                output_ids_to_claim.insert(output_data.output_id);
                                            }
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            OutputsToClaim::NativeTokens => {
                                if output_data.output.native_token().is_some() {
                                    output_ids_to_claim.insert(output_data.output_id);
                                }
                            }
                            OutputsToClaim::Nfts => {
                                if output_data.output.is_nft() {
                                    output_ids_to_claim.insert(output_data.output_id);
                                }
                            }
                            OutputsToClaim::Amount => {
                                let mut claimable_amount = output_data.output.amount();
                                if unlock_conditions.is_expired(slot_index, protocol_parameters.committable_age_range())
                                    == Some(false)
                                {
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
    /// [`StorageDepositReturnUnlockCondition`](crate::types::block::output::unlock_condition::StorageDepositReturnUnlockCondition) or
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) and can be
    /// unlocked now and also get basic outputs with only an [`AddressUnlockCondition`] unlock condition, for
    /// additional inputs
    pub async fn claimable_outputs(&self, outputs_to_claim: OutputsToClaim) -> crate::wallet::Result<Vec<OutputId>> {
        let wallet_data = self.data().await;

        let slot_index = self.client().get_slot_index().await?;
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        wallet_data.claimable_outputs(outputs_to_claim, slot_index, &protocol_parameters)
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
                        if let [UnlockCondition::Address(a)] = basic_output.unlock_conditions().as_ref() {
                            // Implicit accounts can't be used
                            if !a.address().is_implicit_account_creation() {
                                // Store outputs with [`AddressUnlockCondition`] alone, because they could be used as
                                // additional input, if required
                                basic_outputs.push(output_data.clone());
                            }
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

        let possible_additional_inputs = self.get_basic_outputs_for_additional_inputs().await?;

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

        let mut nft_outputs_to_send = Vec::new();

        // There can be outputs with less amount than min required storage deposit, so we have to check that we
        // have enough amount to create a new basic output
        let enough_amount_for_basic_output = possible_additional_inputs
            .iter()
            .map(|i| i.output.amount())
            .sum::<u64>()
            >= BasicOutput::minimum_amount(&Address::from(Ed25519Address::null()), storage_score_params);

        for output_data in &outputs_to_claim {
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

                nft_outputs_to_send.push(nft_output);
            }
        }

        // CommitmentContextInput is required for inputs with expiration or storage_deposit_return unlock condition
        let commitment_context_input_required = outputs_to_claim.iter().any(|o| {
            o.output.unlock_conditions().map_or(false, |uc| {
                uc.expiration().is_some() || uc.storage_deposit_return().is_some()
            })
        });
        let context_inputs = if commitment_context_input_required {
            Some(vec![
                CommitmentContextInput::new(self.client().get_issuance().await?.latest_commitment.id()).into(),
            ])
        } else {
            None
        };

        self.prepare_transaction(
            // We only need to provide the NFT outputs, ISA automatically creates basic outputs as remainder outputs
            nft_outputs_to_send,
            Some(TransactionOptions {
                custom_inputs: Some(
                    outputs_to_claim
                        .iter()
                        .map(|o| o.output_id)
                        // add additional inputs
                        .chain(possible_additional_inputs.iter().map(|o| o.output_id))
                        .collect::<Vec<OutputId>>(),
                ),
                context_inputs,
                ..Default::default()
            }),
        )
        .await
    }
}
