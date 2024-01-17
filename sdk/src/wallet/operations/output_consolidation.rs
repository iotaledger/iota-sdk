// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[cfg(feature = "ledger_nano")]
use crate::client::secret::{ledger_nano::LedgerSecretManager, DowncastSecretManager};
use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::{Address, Bech32Address},
        input::INPUT_COUNT_MAX,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder, Output},
        slot::SlotIndex,
    },
    wallet::{
        constants::DEFAULT_OUTPUT_CONSOLIDATION_THRESHOLD,
        operations::{helpers::time::can_output_be_unlocked_now, transaction::TransactionOptions},
        types::{OutputData, TransactionWithMetadata},
        Result, Wallet,
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
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    async fn should_consolidate_output(
        &self,
        output_data: &OutputData,
        slot_index: SlotIndex,
        wallet_address: &Address,
    ) -> Result<bool> {
        Ok(if let Output::Basic(basic_output) = &output_data.output {
            let protocol_parameters = self.client().get_protocol_parameters().await?;
            let unlock_conditions = basic_output.unlock_conditions();

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
                wallet_address,
                output_data,
                slot_index,
                protocol_parameters.committable_age_range(),
            )?
        } else {
            false
        })
    }

    /// Consolidates basic outputs with only an [AddressUnlockCondition] from an account by sending them to a provided
    /// address or to an own address again if the output amount is >= the output_threshold. When `force`
    /// is set to `true`, the threshold is ignored. Only consolidates the amount of outputs that fit into a single
    /// transaction.
    pub async fn consolidate_outputs(&self, params: ConsolidationParams) -> Result<TransactionWithMetadata> {
        let prepared_transaction = self.prepare_consolidate_outputs(params).await?;
        let consolidation_tx = self
            .sign_and_submit_transaction(prepared_transaction, None, None)
            .await?;

        log::debug!(
            "[OUTPUT_CONSOLIDATION] consolidation transaction created: block_id: {:?} tx_id: {:?}",
            consolidation_tx.block_id,
            consolidation_tx.transaction_id
        );

        Ok(consolidation_tx)
    }

    /// Prepares the transaction for [Wallet::consolidate_outputs()].
    pub async fn prepare_consolidate_outputs(&self, params: ConsolidationParams) -> Result<PreparedTransactionData> {
        log::debug!("[OUTPUT_CONSOLIDATION] prepare consolidating outputs if needed");
        // #[cfg(feature = "participation")]
        // let voting_output = self.get_voting_output().await?;
        let slot_index = self.client().get_slot_index().await?;
        let mut outputs_to_consolidate = Vec::new();
        let wallet_data = self.data().await;

        let wallet_address = wallet_data.address.clone();

        for (output_id, output_data) in &wallet_data.unspent_outputs {
            // #[cfg(feature = "participation")]
            // if let Some(ref voting_output) = voting_output {
            //     // Remove voting output from inputs, because we want to keep its features and not consolidate it.
            //     if output_data.output_id == voting_output.output_id {
            //         continue;
            //     }
            // }

            let is_locked_output = wallet_data.locked_outputs.contains(output_id);
            let should_consolidate_output = self
                .should_consolidate_output(output_data, slot_index, &wallet_address)
                .await?;
            if !is_locked_output && should_consolidate_output {
                outputs_to_consolidate.push(output_data.clone());
            }
        }

        drop(wallet_data);

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

        // only consolidate if the unlocked outputs are >= output_threshold
        if outputs_to_consolidate.is_empty() || (!params.force && outputs_to_consolidate.len() < output_threshold) {
            log::debug!(
                "[OUTPUT_CONSOLIDATION] no consolidation needed, available_outputs: {}, output_threshold: {}",
                outputs_to_consolidate.len(),
                output_threshold
            );
            return Err(crate::wallet::Error::NoOutputsToConsolidate {
                available_outputs: outputs_to_consolidate.len(),
                consolidation_threshold: output_threshold,
            });
        }

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

        let mut total_amount = 0;
        let mut custom_inputs = Vec::with_capacity(max_inputs.into());
        let mut total_native_tokens = NativeTokensBuilder::new();

        for output_data in outputs_to_consolidate.iter().take(max_inputs.into()) {
            if let Some(native_token) = output_data.output.native_token() {
                total_native_tokens.add_native_token(*native_token)?;
            };
            total_amount += output_data.output.amount();

            custom_inputs.push(output_data.output_id);
        }

        let consolidation_output = [BasicOutputBuilder::new_with_amount(total_amount)
            .add_unlock_condition(AddressUnlockCondition::new(
                params
                    .target_address
                    .map(|bech32| bech32.into_inner())
                    .unwrap_or_else(|| wallet_address.into_inner()),
            ))
            // TODO https://github.com/iotaledger/iota-sdk/issues/1632
            // .with_native_tokens(total_native_tokens.finish()?)
            .finish_output()?];

        let options = Some(TransactionOptions {
            custom_inputs: Some(custom_inputs),
            ..Default::default()
        });

        self.prepare_transaction(consolidation_output, options).await
    }
}
