// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[cfg(feature = "ledger_nano")]
use crate::client::secret::{ledger_nano::LedgerSecretManager, DowncastSecretManager};
use crate::{
    client::{
        api::{
            options::{RemainderValueStrategy, TransactionOptions},
            PreparedTransactionData,
        },
        secret::SecretManage,
        ClientError,
    },
    types::block::{
        address::{Address, Bech32Address},
        input::INPUT_COUNT_MAX,
        output::{MinimumOutputAmount, Output},
        slot::SlotIndex,
    },
    wallet::{
        constants::DEFAULT_OUTPUT_CONSOLIDATION_THRESHOLD,
        operations::helpers::time::can_output_be_unlocked_now,
        types::{OutputData, TransactionWithMetadata},
        Wallet, WalletError,
    },
};

// Constants for the calculation of the amount of inputs we can use with a ledger nano
#[cfg(feature = "ledger_nano")]
const ESSENCE_SIZE_WITHOUT_IN_AND_OUTPUTS: usize = 49;
#[cfg(feature = "ledger_nano")]
// Input size in essence (35) + LedgerBIP32Index (8)
const INPUT_SIZE: usize = 43;
#[cfg(feature = "ledger_nano")]
const MIN_OUTPUT_SIZE_IN_ESSENCE: usize = 46;

#[cfg(feature = "ledger_nano")]
use crate::wallet::constants::DEFAULT_LEDGER_OUTPUT_CONSOLIDATION_THRESHOLD;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsolidationParams {
    /// Ignores the output_threshold if set to `true`.
    force: bool,
    /// Consolidates if the output number is >= the output_threshold.
    output_threshold: Option<usize>,
    /// Address to which the consolidated output should be sent.
    target_address: Option<Bech32Address>,
}

impl ConsolidationParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn with_output_threshold(mut self, output_threshold: impl Into<Option<usize>>) -> Self {
        self.output_threshold = output_threshold.into();
        self
    }

    pub fn with_target_address(mut self, target_address: impl Into<Option<Bech32Address>>) -> Self {
        self.target_address = target_address.into();
        self
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Consolidates basic outputs from an account by sending them to a provided address or to an own address again if
    /// the output amount is >= the output_threshold. When `force` is set to `true`, the threshold is ignored. Only
    /// consolidates the amount of outputs that fit into a single transaction.
    pub async fn consolidate_outputs(
        &self,
        params: ConsolidationParams,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let prepared_transaction = self.prepare_consolidate_outputs(params).await?;
        let consolidation_tx = self.sign_and_submit_transaction(prepared_transaction, None).await?;

        log::debug!(
            "[OUTPUT_CONSOLIDATION] consolidation transaction created: block_id: {:?} tx_id: {:?}",
            consolidation_tx.block_id,
            consolidation_tx.transaction_id
        );

        Ok(consolidation_tx)
    }

    /// Prepares the transaction for [Wallet::consolidate_outputs()].
    pub async fn prepare_consolidate_outputs(
        &self,
        params: ConsolidationParams,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[OUTPUT_CONSOLIDATION] prepare consolidating outputs if needed");
        let wallet_address = self.address().await;

        let outputs_to_consolidate = self.get_outputs_to_consolidate(&params).await?;

        let options = Some(TransactionOptions {
            required_inputs: outputs_to_consolidate.into_iter().map(|o| o.output_id).collect(),
            remainder_value_strategy: RemainderValueStrategy::CustomAddress(
                params
                    .target_address
                    .map(|bech32| bech32.into_inner())
                    .unwrap_or_else(|| wallet_address.into_inner()),
            ),
            ..Default::default()
        });

        self.prepare_send_outputs([], options).await
    }

    /// Determines whether an output should be consolidated or not.
    async fn should_consolidate_output(
        &self,
        output_data: &OutputData,
        slot_index: SlotIndex,
        controlled_addresses: &HashSet<Address>,
    ) -> Result<bool, WalletError> {
        Ok(if let Output::Basic(basic_output) = &output_data.output {
            let protocol_parameters = self.client().get_protocol_parameters().await?;
            let unlock_conditions = basic_output.unlock_conditions();

            // Implicit account creation outputs shouldn't be consolidated.
            if basic_output.address().is_implicit_account_creation() {
                return Ok(false);
            }

            let is_time_locked = unlock_conditions.is_timelocked(slot_index, protocol_parameters.min_committable_age());
            if is_time_locked {
                // If the output is timelocked, then it cannot be consolidated.
                return Ok(false);
            }

            let has_storage_deposit_return = unlock_conditions.storage_deposit_return().is_some();
            let is_expired = unlock_conditions.is_expired(slot_index, protocol_parameters.committable_age_range());
            if is_expired.is_none() {
                // If the output is in a deadzone because of expiration, then it cannot be consolidated.
                return Ok(false);
            }
            if has_storage_deposit_return && is_expired == Some(false) {
                // If the output has not expired and must return a storage deposit, then it cannot be consolidated.
                return Ok(false);
            }

            can_output_be_unlocked_now(
                controlled_addresses,
                output_data,
                slot_index,
                protocol_parameters.committable_age_range(),
            )?
        } else {
            false
        })
    }

    /// Returns all outputs that should be consolidated.
    async fn get_outputs_to_consolidate(&self, params: &ConsolidationParams) -> Result<Vec<OutputData>, WalletError> {
        // #[cfg(feature = "participation")]
        // let voting_output = self.get_voting_output().await?;
        let slot_index = self.client().get_slot_index().await?;
        let storage_score_parameters = self.client().get_protocol_parameters().await?.storage_score_parameters;
        let wallet_ledger = self.ledger().await;
        let wallet_address = self.address().await;

        let mut outputs_to_consolidate = Vec::new();
        let mut native_token_inputs = HashMap::new();

        let controlled_addresses = wallet_ledger.controlled_addresses(wallet_address.inner().clone());

        for (output_id, output_data) in &wallet_ledger.unspent_outputs {
            // #[cfg(feature = "participation")]
            // if let Some(ref voting_output) = voting_output {
            //     // Remove voting output from inputs, because we want to keep its features and not consolidate it.
            //     if output_data.output_id == voting_output.output_id {
            //         continue;
            //     }.await
            // }

            let is_locked_output = wallet_ledger.locked_outputs.contains(output_id);
            let should_consolidate_output = self
                .should_consolidate_output(output_data, slot_index, &controlled_addresses)
                .await?;
            if !is_locked_output && should_consolidate_output {
                outputs_to_consolidate.push(output_data.clone());

                // Keep track of inputs with native tokens.
                if let Some(nt) = &output_data.output.native_token() {
                    native_token_inputs
                        .entry(*nt.token_id())
                        .or_insert_with(HashSet::new)
                        .insert(output_data.output_id);
                }
            }
        }

        // Remove outputs if they have a native token, <= minimum amount and there are no other outputs with the same
        // native token.
        outputs_to_consolidate.retain(|output_data| {
            output_data.output.native_token().as_ref().map_or(true, |nt| {
                // `<=` because outputs in genesis snapshot can have a lower amount than min amount.
                if output_data.output.amount() <= output_data.output.minimum_amount(storage_score_parameters) {
                    // If there is only a single output with this native token, then it shouldn't be consolidated,
                    // because no amount will be made available, since we need to create a remainder output with the
                    // native token again.
                    native_token_inputs
                        .get(nt.token_id())
                        .map_or_else(|| false, |ids| ids.len() > 1)
                } else {
                    true
                }
            })
        });

        drop(wallet_ledger);

        let output_threshold = self.get_output_consolidation_threshold(params).await?;

        // only consolidate if the unlocked outputs are >= output_threshold
        if outputs_to_consolidate.is_empty() || (!params.force && outputs_to_consolidate.len() < output_threshold) {
            log::debug!(
                "[OUTPUT_CONSOLIDATION] no consolidation needed, available_outputs: {}, output_threshold: {}",
                outputs_to_consolidate.len(),
                output_threshold
            );
            return Err(WalletError::NoOutputsToConsolidate {
                available_outputs: outputs_to_consolidate.len(),
                consolidation_threshold: output_threshold,
            });
        }

        let max_inputs = self.get_max_inputs().await?;
        outputs_to_consolidate.truncate(max_inputs.into());

        log::debug!(
            "outputs_to_consolidate: {:?}",
            outputs_to_consolidate.iter().map(|o| o.output_id).collect::<Vec<_>>()
        );

        Ok(outputs_to_consolidate)
    }

    /// Returns the max amount of inputs that can be used in a consolidation transaction. For Ledger Nano it's more
    /// limited.
    async fn get_max_inputs(&self) -> Result<u16, WalletError> {
        #[cfg(feature = "ledger_nano")]
        let max_inputs = {
            use crate::client::secret::SecretManager;
            let secret_manager = self.secret_manager.read().await;
            if let Some(ledger) = secret_manager.downcast::<LedgerSecretManager>().or_else(|| {
                secret_manager.downcast::<SecretManager>().and_then(|s| {
                    if let SecretManager::LedgerNano(n) = s {
                        Some(n)
                    } else {
                        None
                    }
                })
            }) {
                let ledger_nano_status = ledger.get_ledger_nano_status().await;
                // With blind signing we are only limited by the protocol
                if ledger_nano_status.blind_signing_enabled() {
                    INPUT_COUNT_MAX
                } else {
                    ledger_nano_status
                        .buffer_size()
                        .map(|buffer_size| {
                            // Calculate how many inputs we can have with this ledger, buffer size is different for
                            // different ledger types
                            let available_buffer_size_for_inputs =
                                buffer_size - ESSENCE_SIZE_WITHOUT_IN_AND_OUTPUTS - MIN_OUTPUT_SIZE_IN_ESSENCE;
                            (available_buffer_size_for_inputs / INPUT_SIZE) as u16
                        })
                        .unwrap_or(INPUT_COUNT_MAX)
                }
            } else {
                INPUT_COUNT_MAX
            }
        };
        #[cfg(not(feature = "ledger_nano"))]
        let max_inputs = INPUT_COUNT_MAX;
        Ok(max_inputs)
    }

    /// Returns the threshold value above which outputs should be consolidated. Lower for ledger nano secret manager, as
    /// their memory size is limited.
    async fn get_output_consolidation_threshold(&self, params: &ConsolidationParams) -> Result<usize, WalletError> {
        #[allow(clippy::option_if_let_else)]
        let output_threshold = match params.output_threshold {
            Some(t) => t,
            None => {
                #[cfg(feature = "ledger_nano")]
                {
                    use crate::client::secret::SecretManager;
                    let secret_manager = self.secret_manager.read().await;
                    if secret_manager
                        .downcast::<LedgerSecretManager>()
                        .or_else(|| {
                            secret_manager.downcast::<SecretManager>().and_then(|s| {
                                if let SecretManager::LedgerNano(n) = s {
                                    Some(n)
                                } else {
                                    None
                                }
                            })
                        })
                        .is_some()
                    {
                        DEFAULT_LEDGER_OUTPUT_CONSOLIDATION_THRESHOLD
                    } else {
                        DEFAULT_OUTPUT_CONSOLIDATION_THRESHOLD
                    }
                }
                #[cfg(not(feature = "ledger_nano"))]
                DEFAULT_OUTPUT_CONSOLIDATION_THRESHOLD
            }
        };

        Ok(output_threshold)
    }
}
