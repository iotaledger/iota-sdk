// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    client::{
        api::{options::TransactionOptions, PreparedTransactionData},
        secret::SecretManage,
        ClientError,
    },
    types::block::{
        address::{Address, Ed25519Address},
        output::{
            unlock_condition::AddressUnlockCondition, BasicOutput, NftOutputBuilder, Output, OutputId, UnlockCondition,
        },
        protocol::ProtocolParameters,
        slot::SlotIndex,
    },
    wallet::{
        core::WalletLedger,
        operations::helpers::time::can_output_be_unlocked_now,
        types::{OutputData, TransactionWithMetadata},
        Wallet, WalletError,
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

impl WalletLedger {
    /// Get basic and nft outputs that have
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition),
    /// [`StorageDepositReturnUnlockCondition`](crate::types::block::output::unlock_condition::StorageDepositReturnUnlockCondition) or
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) and can be
    /// unlocked now and also get basic outputs with only an [`AddressUnlockCondition`] unlock condition, for
    /// additional inputs
    pub(crate) fn claimable_outputs(
        &self,
        wallet_address: Address,
        outputs_to_claim: OutputsToClaim,
        slot_index: SlotIndex,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Vec<OutputId>, WalletError> {
        log::debug!("[OUTPUT_CLAIMING] claimable_outputs");

        let controlled_addresses = self.controlled_addresses(wallet_address);

        // Get outputs for the claim
        let mut output_ids_to_claim: HashSet<OutputId> = HashSet::new();
        for (output_id, output_data) in self
            .unspent_outputs
            .iter()
            .filter(|(_, o)| o.output.is_basic() || o.output.is_nft())
        {
            // Don't use outputs that are locked for other transactions
            if !self.locked_outputs.contains(output_id) && self.outputs.contains_key(output_id) {
                // If there is a single [UnlockCondition], then it's an
                // [AddressUnlockCondition] and we own it already without
                // further restrictions
                if output_data.output.unlock_conditions().len() != 1
                    && can_output_be_unlocked_now(
                        // We use the addresses with unspent outputs, because other addresses of the
                        // account without unspent outputs can't be related to this output
                        &controlled_addresses,
                        output_data,
                        slot_index,
                        protocol_parameters.committable_age_range(),
                    )?
                {
                    match outputs_to_claim {
                        OutputsToClaim::MicroTransactions => {
                            if let Some(sdr) = output_data.output.unlock_conditions().storage_deposit_return() {
                                // If expired, it's not a micro transaction anymore
                                match output_data
                                    .output
                                    .unlock_conditions()
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
                            if output_data
                                .output
                                .unlock_conditions()
                                .is_expired(slot_index, protocol_parameters.committable_age_range())
                                == Some(false)
                            {
                                claimable_amount -= output_data
                                    .output
                                    .unlock_conditions()
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
        log::debug!(
            "[OUTPUT_CLAIMING] available outputs to claim: {}",
            output_ids_to_claim.len()
        );
        Ok(output_ids_to_claim.into_iter().collect())
    }

    // Returns the wallet address together with account and nft addresses that only have the address unlock condition
    pub(crate) fn controlled_addresses(&self, wallet_address: Address) -> HashSet<Address> {
        let mut controlled_addresses = HashSet::from([wallet_address]);
        for o in self.unspent_outputs().values() {
            match &o.output {
                Output::Account(account) => {
                    controlled_addresses.insert(Address::Account(account.account_address(&o.output_id)));
                }
                Output::Nft(nft) => {
                    // Only consider addresses of NFTs with a single (address) unlock condition
                    if nft.unlock_conditions().len() == 1 {
                        controlled_addresses.insert(Address::Nft(nft.nft_address(&o.output_id)));
                    }
                }
                _ => {} // not interested in other outputs here
            }
        }
        controlled_addresses
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Get basic and nft outputs that have
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition),
    /// [`StorageDepositReturnUnlockCondition`](crate::types::block::output::unlock_condition::StorageDepositReturnUnlockCondition) or
    /// [`TimelockUnlockCondition`](crate::types::block::output::unlock_condition::TimelockUnlockCondition) and can be
    /// unlocked now and also get basic outputs with only an [`AddressUnlockCondition`] unlock condition, for
    /// additional inputs
    pub async fn claimable_outputs(&self, outputs_to_claim: OutputsToClaim) -> Result<Vec<OutputId>, WalletError> {
        let wallet_ledger = self.ledger().await;

        let slot_index = self.client().get_slot_index().await?;
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        wallet_ledger.claimable_outputs(
            self.address().await.into_inner(),
            outputs_to_claim,
            slot_index,
            &protocol_parameters,
        )
    }

    /// Get basic outputs that have only one unlock condition which is [AddressUnlockCondition], so they can be used as
    /// additional inputs
    pub(crate) async fn get_basic_outputs_for_additional_inputs(&self) -> Result<Vec<OutputData>, WalletError> {
        log::debug!("[OUTPUT_CLAIMING] get_basic_outputs_for_additional_inputs");
        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await;
        let wallet_ledger = self.ledger().await;

        // Get basic outputs only with AddressUnlockCondition and no other unlock condition
        let mut basic_outputs: Vec<OutputData> = Vec::new();
        for (output_id, output_data) in &wallet_ledger.unspent_outputs {
            #[cfg(feature = "participation")]
            if let Some(ref voting_output) = voting_output {
                // Remove voting output from inputs, because we don't want to spent it to claim something else.
                if output_data.output_id == voting_output.output_id {
                    continue;
                }
            }
            // Don't use outputs that are locked for other transactions
            if !wallet_ledger.locked_outputs.contains(output_id) {
                if let Some(output) = wallet_ledger.outputs.get(output_id) {
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
        transaction_options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError>
    where
        I::IntoIter: Send,
    {
        log::debug!("[OUTPUT_CLAIMING] claim_outputs");
        let prepared_transaction = self
            .prepare_claim_outputs(output_ids_to_claim, transaction_options)
            .await?;

        let claim_tx = self.sign_and_submit_transaction(prepared_transaction, None).await?;

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
        transaction_options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError>
    where
        I::IntoIter: Send,
    {
        log::debug!("[OUTPUT_CLAIMING] prepare_claim_outputs");

        let possible_additional_inputs = self.get_basic_outputs_for_additional_inputs().await?;

        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let wallet_ledger = self.ledger().await;

        let mut outputs_to_claim = Vec::new();
        for output_id in output_ids_to_claim {
            if let Some(output_data) = wallet_ledger.unspent_outputs.get(&output_id) {
                if !wallet_ledger.locked_outputs.contains(&output_id) {
                    outputs_to_claim.push(output_data.clone());
                }
            }
        }

        if outputs_to_claim.is_empty() {
            return Err(WalletError::CustomInput(
                "provided outputs can't be claimed".to_string(),
            ));
        }

        let transaction_options = transaction_options.into();
        let remainder_address = self.get_remainder_address(transaction_options.clone()).await?;
        drop(wallet_ledger);

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
                        .with_unlock_conditions([AddressUnlockCondition::new(remainder_address.clone())])
                        .finish_output()?
                } else {
                    NftOutputBuilder::from(nft_output)
                        .with_minimum_amount(storage_score_params)
                        .with_nft_id(nft_output.nft_id_non_null(&output_data.output_id))
                        .with_unlock_conditions([AddressUnlockCondition::new(remainder_address.clone())])
                        .finish_output()?
                };

                nft_outputs_to_send.push(nft_output);
            }
        }

        let required_inputs = outputs_to_claim
            .iter()
            .map(|o| o.output_id)
            // add additional inputs
            .chain(possible_additional_inputs.iter().map(|o| o.output_id))
            .collect();

        self.prepare_send_outputs(
            // We only need to provide the NFT outputs, ISA automatically creates basic outputs as remainder outputs
            nft_outputs_to_send,
            match transaction_options {
                Some(mut tx_options) => {
                    tx_options.required_inputs = required_inputs;
                    tx_options
                }
                None => TransactionOptions {
                    required_inputs,
                    ..Default::default()
                },
            },
        )
        .await
    }
}
