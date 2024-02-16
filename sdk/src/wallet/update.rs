// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::{
    client::secret::SecretManage,
    types::block::{
        output::{OutputConsumptionMetadata, OutputId, OutputMetadata},
        payload::signed_transaction::TransactionId,
    },
    wallet::{
        core::WalletLedgerDto,
        types::{InclusionState, OutputData, TransactionWithMetadata},
        Wallet,
    },
};
#[cfg(feature = "events")]
use crate::{
    types::block::payload::signed_transaction::dto::SignedTransactionPayloadDto,
    wallet::events::types::{NewOutputEvent, SpentOutputEvent, TransactionInclusionEvent, WalletEvent},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Update the wallet address with a possible new Bech32 HRP and clear the inaccessible incoming transactions.
    pub(crate) async fn update_wallet_address_hrp(&self) -> crate::wallet::Result<()> {
        let bech32_hrp = self.client().get_bech32_hrp().await?;
        log::debug!("updating wallet with new bech32 hrp: {}", bech32_hrp);

        self.address_mut().await.hrp = bech32_hrp;

        #[cfg(feature = "storage")]
        {
            let wallet_ledger = {
                let mut wallet_ledger = self.ledger_mut().await;
                wallet_ledger.inaccessible_incoming_transactions.clear();
                WalletLedgerDto::from(&*wallet_ledger)
            };

            log::debug!("[save] wallet ledger with updated bech32 hrp",);
            self.storage_manager().save_wallet_ledger(&wallet_ledger).await?;
        }

        Ok(())
    }

    /// Update the alias for the wallet.
    pub async fn update_wallet_alias(&self, alias: &str) -> crate::wallet::Result<()> {
        log::debug!("updating wallet with new alias: {}", alias);

        *self.alias_mut().await = Some(alias.to_string());

        Ok(())
    }

    /// Update wallet with newly synced data and emit events for outputs.
    pub(crate) async fn update_after_sync(
        &self,
        unspent_outputs: Vec<OutputData>,
        spent_or_unsynced_output_metadata_map: HashMap<OutputId, Option<OutputMetadata>>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] Update wallet ledger with new synced transactions");

        let network_id = self.client().get_network_id().await?;
        let mut wallet_ledger = self.ledger_mut().await;

        // Update spent outputs
        for (output_id, output_metadata_response_opt) in spent_or_unsynced_output_metadata_map {
            // If we got the output response and it's still unspent, skip it
            if let Some(output_metadata_response) = output_metadata_response_opt {
                if output_metadata_response.is_spent() {
                    wallet_ledger.unspent_outputs.remove(&output_id);
                    if let Some(output_data) = wallet_ledger.outputs.get_mut(&output_id) {
                        output_data.metadata = output_metadata_response;
                    }
                } else {
                    // not spent, just not synced, skip
                    continue;
                }
            }

            if let Some(output) = wallet_ledger.outputs.get(&output_id) {
                // Could also be outputs from other networks after we switched the node, so we check that first
                if output.network_id == network_id {
                    log::debug!("[SYNC] Spent output {}", output_id);
                    wallet_ledger.locked_outputs.remove(&output_id);
                    wallet_ledger.unspent_outputs.remove(&output_id);
                    // Update spent data fields
                    if let Some(output_data) = wallet_ledger.outputs.get_mut(&output_id) {
                        if !output_data.is_spent() {
                            log::warn!(
                                "[SYNC] Setting output {} as spent without having the OutputConsumptionMetadata",
                                output_id
                            );
                            // Set 0 values because we don't have the actual metadata and also couldn't get it, probably
                            // because it got pruned.
                            output_data.metadata.spent = Some(OutputConsumptionMetadata::new(
                                0.into(),
                                TransactionId::new([0u8; TransactionId::LENGTH]),
                                None,
                            ));
                        }

                        #[cfg(feature = "events")]
                        {
                            self.emit(WalletEvent::SpentOutput(Box::new(SpentOutputEvent {
                                output: output_data.clone(),
                            })))
                            .await;
                        }
                    }
                }
            }
        }

        // Add new synced outputs
        for output_data in unspent_outputs {
            // Insert output, if it's unknown emit the NewOutputEvent
            if wallet_ledger
                .outputs
                .insert(output_data.output_id, output_data.clone())
                .is_none()
            {
                #[cfg(feature = "events")]
                {
                    let transaction = wallet_ledger
                        .incoming_transactions
                        .get(output_data.output_id.transaction_id());
                    self.emit(WalletEvent::NewOutput(Box::new(NewOutputEvent {
                        output: output_data.clone(),
                        transaction: transaction
                            .as_ref()
                            .map(|tx| SignedTransactionPayloadDto::from(&tx.payload)),
                        transaction_inputs: transaction.as_ref().map(|tx| tx.inputs.clone()),
                    })))
                    .await;
                }
            };
            if !output_data.is_spent() {
                wallet_ledger.unspent_outputs.insert(output_data.output_id, output_data);
            }
        }

        #[cfg(feature = "storage")]
        {
            log::debug!("[SYNC] storing wallet ledger with new synced data");
            self.storage_manager()
                .save_wallet_ledger(&WalletLedgerDto::from(&*wallet_ledger))
                .await?;
        }
        Ok(())
    }

    /// Update wallet with newly synced transactions.
    pub(crate) async fn update_with_transactions(
        &self,
        updated_transactions: Vec<TransactionWithMetadata>,
        spent_output_ids: Vec<OutputId>,
        output_ids_to_unlock: Vec<OutputId>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] Update wallet with new synced transactions");

        let mut wallet_ledger = self.ledger_mut().await;

        for transaction in updated_transactions {
            match transaction.inclusion_state {
                InclusionState::Confirmed | InclusionState::Conflicting | InclusionState::UnknownPruned => {
                    let transaction_id = transaction.payload.transaction().id();
                    wallet_ledger.pending_transactions.remove(&transaction_id);
                    log::debug!(
                        "[SYNC] inclusion_state of {transaction_id} changed to {:?}",
                        transaction.inclusion_state
                    );
                    #[cfg(feature = "events")]
                    {
                        self.emit(WalletEvent::TransactionInclusion(TransactionInclusionEvent {
                            transaction_id,
                            inclusion_state: transaction.inclusion_state,
                        }))
                        .await;
                    }
                }
                _ => {}
            }
            wallet_ledger
                .transactions
                .insert(transaction.payload.transaction().id(), transaction.clone());
        }

        for output_to_unlock in &spent_output_ids {
            if let Some(output_data) = wallet_ledger.outputs.get_mut(output_to_unlock) {
                if !output_data.is_spent() {
                    log::warn!(
                        "[SYNC] Setting output {} as spent without having the OutputConsumptionMetadata",
                        output_data.output_id
                    );
                    // Set 0 values because we don't have the actual metadata and also couldn't get it, probably because
                    // it got pruned.
                    output_data.metadata.spent = Some(OutputConsumptionMetadata::new(
                        0.into(),
                        TransactionId::new([0u8; TransactionId::LENGTH]),
                        None,
                    ));
                }
            }
            wallet_ledger.locked_outputs.remove(output_to_unlock);
            wallet_ledger.unspent_outputs.remove(output_to_unlock);
            log::debug!("[SYNC] Unlocked spent output {}", output_to_unlock);
        }

        for output_to_unlock in &output_ids_to_unlock {
            wallet_ledger.locked_outputs.remove(output_to_unlock);
            log::debug!(
                "[SYNC] Unlocked unspent output {} because of a conflicting transaction",
                output_to_unlock
            );
        }

        #[cfg(feature = "storage")]
        {
            log::debug!("[SYNC] storing wallet ledger with new synced transactions");
            self.storage_manager()
                .save_wallet_ledger(&WalletLedgerDto::from(&*wallet_ledger))
                .await?;
        }
        Ok(())
    }
}
