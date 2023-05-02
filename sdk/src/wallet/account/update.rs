// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::{
    client::Client,
    types::block::output::{dto::OutputMetadataDto, OutputId},
    wallet::account::{
        operations::syncing::options::SyncOptions,
        types::{address::AddressWithUnspentOutputs, InclusionState, OutputData, Transaction},
        Account, AccountAddress,
    },
};
#[cfg(feature = "events")]
use crate::{
    types::{api::core::response::OutputWithMetadataResponse, block::payload::transaction::dto::TransactionPayloadDto},
    wallet::{
        account::types::OutputDataDto,
        events::types::{NewOutputEvent, SpentOutputEvent, TransactionInclusionEvent, WalletEvent},
    },
};

impl Account {
    /// Set the alias for the account
    pub async fn set_alias(&self, alias: &str) -> crate::wallet::Result<()> {
        let mut account_details = self.write().await;
        account_details.alias = alias.to_string();
        #[cfg(feature = "storage")]
        self.save(Some(&account_details)).await?;
        Ok(())
    }

    /// Update account with newly synced data and emit events for outputs
    pub(crate) async fn update_account(
        &self,
        addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputs>,
        unspent_outputs: Vec<OutputData>,
        spent_or_unsynced_output_metadata_map: HashMap<OutputId, Option<OutputMetadataDto>>,
        options: &SyncOptions,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] Update account with new synced transactions");

        let network_id = self.client.get_network_id().await?;
        let mut account_details = self.write().await;
        #[cfg(feature = "events")]
        let account_index = account_details.index;

        // update used field of the addresses
        for address_with_unspent_outputs in addresses_with_unspent_outputs.iter() {
            if address_with_unspent_outputs.internal {
                let position = account_details
                    .internal_addresses
                    .binary_search_by_key(
                        &(
                            address_with_unspent_outputs.key_index,
                            address_with_unspent_outputs.internal,
                        ),
                        |a| (a.key_index, a.internal),
                    )
                    .map_err(|_| {
                        crate::wallet::Error::AddressNotFoundInAccount(address_with_unspent_outputs.address.clone())
                    })?;
                account_details.internal_addresses[position].used = true;
            } else {
                let position = account_details
                    .public_addresses
                    .binary_search_by_key(
                        &(
                            address_with_unspent_outputs.key_index,
                            address_with_unspent_outputs.internal,
                        ),
                        |a| (a.key_index, a.internal),
                    )
                    .map_err(|_| {
                        crate::wallet::Error::AddressNotFoundInAccount(address_with_unspent_outputs.address.clone())
                    })?;
                account_details.public_addresses[position].used = true;
            }
        }

        // Update addresses_with_unspent_outputs
        // only keep addresses below the address start index, because we synced the addresses above and will update them
        account_details.addresses_with_unspent_outputs.retain(|a| {
            if a.internal {
                a.key_index < options.address_start_index_internal
            } else {
                a.key_index < options.address_start_index
            }
        });
        // then add all synced addresses with balance, all other addresses that had balance before will then be removed
        // from this list
        account_details
            .addresses_with_unspent_outputs
            .extend(addresses_with_unspent_outputs);

        // Update spent outputs
        for (output_id, output_metadata_response_opt) in spent_or_unsynced_output_metadata_map {
            // If we got the output response and it's still unspent, skip it
            if let Some(output_metadata_response) = output_metadata_response_opt {
                if output_metadata_response.is_spent {
                    account_details.unspent_outputs.remove(&output_id);
                    if let Some(output_data) = account_details.outputs.get_mut(&output_id) {
                        output_data.metadata = output_metadata_response;
                    }
                } else {
                    // not spent, just not synced, skip
                    continue;
                }
            }

            if let Some(output) = account_details.outputs.get(&output_id) {
                // Could also be outputs from other networks after we switched the node, so we check that first
                if output.network_id == network_id {
                    log::debug!("[SYNC] Spent output {}", output_id);
                    account_details.locked_outputs.remove(&output_id);
                    account_details.unspent_outputs.remove(&output_id);
                    // Update spent data fields
                    if let Some(output_data) = account_details.outputs.get_mut(&output_id) {
                        output_data.metadata.is_spent = true;
                        output_data.is_spent = true;
                        #[cfg(feature = "events")]
                        {
                            self.event_emitter.lock().await.emit(
                                account_index,
                                WalletEvent::SpentOutput(Box::new(SpentOutputEvent {
                                    output: OutputDataDto::from(&*output_data),
                                })),
                            );
                        }
                    }
                }
            }
        }

        // Add new synced outputs
        for output_data in unspent_outputs {
            // Insert output, if it's unknown emit the NewOutputEvent
            if account_details
                .outputs
                .insert(output_data.output_id, output_data.clone())
                .is_none()
            {
                #[cfg(feature = "events")]
                {
                    let transaction = account_details
                        .incoming_transactions
                        .get(output_data.output_id.transaction_id());
                    self.event_emitter.lock().await.emit(
                        account_index,
                        WalletEvent::NewOutput(Box::new(NewOutputEvent {
                            output: OutputDataDto::from(&output_data),
                            transaction: transaction.as_ref().map(|tx| TransactionPayloadDto::from(&tx.payload)),
                            transaction_inputs: transaction.as_ref().map(|tx| {
                                tx.inputs
                                    .clone()
                                    .into_iter()
                                    .map(OutputWithMetadataResponse::from)
                                    .collect()
                            }),
                        })),
                    );
                }
            };
            if !output_data.is_spent {
                account_details
                    .unspent_outputs
                    .insert(output_data.output_id, output_data);
            }
        }

        #[cfg(feature = "storage")]
        {
            log::debug!(
                "[SYNC] storing account {} with new synced data",
                account_details.alias()
            );
            self.save(Some(&account_details)).await?;
        }
        Ok(())
    }

    /// Update account with newly synced transactions
    pub(crate) async fn update_account_with_transactions(
        &self,
        updated_transactions: Vec<Transaction>,
        spent_output_ids: Vec<OutputId>,
        output_ids_to_unlock: Vec<OutputId>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] Update account with new synced transactions");

        let mut account_details = self.write().await;

        for transaction in updated_transactions {
            match transaction.inclusion_state {
                InclusionState::Confirmed | InclusionState::Conflicting | InclusionState::UnknownPruned => {
                    let transaction_id = transaction.payload.id();
                    account_details.pending_transactions.remove(&transaction_id);
                    log::debug!(
                        "[SYNC] inclusion_state of {transaction_id} changed to {:?}",
                        transaction.inclusion_state
                    );
                    #[cfg(feature = "events")]
                    {
                        self.event_emitter.lock().await.emit(
                            account_details.index,
                            WalletEvent::TransactionInclusion(TransactionInclusionEvent {
                                transaction_id,
                                inclusion_state: transaction.inclusion_state,
                            }),
                        );
                    }
                }
                _ => {}
            }
            account_details
                .transactions
                .insert(transaction.payload.id(), transaction.clone());
        }

        for output_to_unlock in &spent_output_ids {
            if let Some(output) = account_details.outputs.get_mut(output_to_unlock) {
                output.is_spent = true;
            }
            account_details.locked_outputs.remove(output_to_unlock);
            account_details.unspent_outputs.remove(output_to_unlock);
            log::debug!("[SYNC] Unlocked spent output {}", output_to_unlock);
        }

        for output_to_unlock in &output_ids_to_unlock {
            account_details.locked_outputs.remove(output_to_unlock);
            log::debug!(
                "[SYNC] Unlocked unspent output {} because of a conflicting transaction",
                output_to_unlock
            );
        }

        #[cfg(feature = "storage")]
        {
            log::debug!(
                "[SYNC] storing account {} with new synced transactions",
                account_details.alias()
            );
            self.save(Some(&account_details)).await?;
        }
        Ok(())
    }

    /// Update account with newly generated addresses
    pub(crate) async fn update_account_addresses(
        &self,
        internal: bool,
        new_addresses: Vec<AccountAddress>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[update_account_addresses]");

        let mut account_details = self.write().await;

        // add addresses to the account
        if internal {
            account_details.internal_addresses.extend(new_addresses);
        } else {
            account_details.public_addresses.extend(new_addresses);
        };

        #[cfg(feature = "storage")]
        {
            log::debug!("[update_account_addresses] storing account {}", account_details.index());
            self.save(Some(&account_details)).await?;
        }
        Ok(())
    }

    // Should only be called from the Wallet so all accounts are on the same state
    // Will update the addresses with a possible new Bech32 HRP and clear the inaccessible_incoming_transactions.
    pub(crate) async fn update_account_with_new_client(&mut self, client: Client) -> crate::wallet::Result<()> {
        self.client = client;
        let bech32_hrp = self.client.get_bech32_hrp().await?;
        log::debug!("[UPDATE ACCOUNT WITH NEW CLIENT] new bech32_hrp: {}", bech32_hrp);
        let mut account_details = self.write().await;
        for address in &mut account_details.addresses_with_unspent_outputs {
            address.address.hrp = bech32_hrp.clone();
        }
        for address in &mut account_details.public_addresses {
            address.address.hrp = bech32_hrp.clone();
        }
        for address in &mut account_details.internal_addresses {
            address.address.hrp = bech32_hrp.clone();
        }

        account_details.inaccessible_incoming_transactions.clear();

        #[cfg(feature = "storage")]
        {
            log::debug!(
                "[SYNC] storing account {} after updating it with new client options",
                account_details.alias()
            );
            self.save(Some(&account_details)).await?;
        }

        Ok(())
    }
}
