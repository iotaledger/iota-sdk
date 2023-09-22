// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::{
    client::secret::SecretManage,
    types::block::output::{OutputId, OutputMetadata},
    wallet::{
        account::{
            operations::syncing::options::SyncOptions,
            types::{address::AddressWithUnspentOutputs, InclusionState, OutputData, Transaction},
            Bip44Address,
        },
        Wallet,
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

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Set the alias for the wallet.
    pub async fn set_alias(&self, alias: &str) -> crate::wallet::Result<()> {
        let mut wallet_data = self.data_mut().await;
        wallet_data.alias = alias.to_string();
        #[cfg(feature = "storage")]
        self.save(Some(&wallet_data)).await?;
        Ok(())
    }

    /// Update wallet with newly synced data and emit events for outputs.
    pub(crate) async fn update(
        &self,
        unspent_outputs: Vec<OutputData>,
        spent_or_unsynced_output_metadata_map: HashMap<OutputId, Option<OutputMetadata>>,
        options: &SyncOptions,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] Update wallet with new synced transactions");

        let network_id = self.client().get_network_id().await?;
        let mut wallet_data = self.data_mut().await;

        // TODO: remove

        // // Update addresses_with_unspent_outputs
        // // only keep addresses below the address start index, because we synced the addresses above and will update
        // // them
        // wallet_data.addresses_with_unspent_outputs.retain(|a| {
        //     if a.internal {
        //         a.key_index < options.address_start_index_internal
        //     } else {
        //         a.key_index < options.address_start_index
        //     }
        // });

        // // then add all synced addresses with balance, all other addresses that had balance before will then be
        // removed // from this list
        // wallet_data
        //     .addresses_with_unspent_outputs
        //     .extend(addresses_with_unspent_outputs);

        // Update spent outputs
        for (output_id, output_metadata_response_opt) in spent_or_unsynced_output_metadata_map {
            // If we got the output response and it's still unspent, skip it
            if let Some(output_metadata_response) = output_metadata_response_opt {
                if output_metadata_response.is_spent() {
                    wallet_data.unspent_outputs.remove(&output_id);
                    if let Some(output_data) = wallet_data.outputs.get_mut(&output_id) {
                        output_data.metadata = output_metadata_response;
                    }
                } else {
                    // not spent, just not synced, skip
                    continue;
                }
            }

            if let Some(output) = wallet_data.outputs.get(&output_id) {
                // Could also be outputs from other networks after we switched the node, so we check that first
                if output.network_id == network_id {
                    log::debug!("[SYNC] Spent output {}", output_id);
                    wallet_data.locked_outputs.remove(&output_id);
                    wallet_data.unspent_outputs.remove(&output_id);
                    // Update spent data fields
                    if let Some(output_data) = wallet_data.outputs.get_mut(&output_id) {
                        output_data.metadata.set_spent(true);
                        output_data.is_spent = true;
                        #[cfg(feature = "events")]
                        {
                            self.emit(
                                self.index().await,
                                WalletEvent::SpentOutput(Box::new(SpentOutputEvent {
                                    output: OutputDataDto::from(&*output_data),
                                })),
                            )
                            .await;
                        }
                    }
                }
            }
        }

        // Add new synced outputs
        for output_data in unspent_outputs {
            // Insert output, if it's unknown emit the NewOutputEvent
            if wallet_data
                .outputs
                .insert(output_data.output_id, output_data.clone())
                .is_none()
            {
                #[cfg(feature = "events")]
                {
                    let transaction = wallet_data
                        .incoming_transactions
                        .get(output_data.output_id.transaction_id());
                    self.emit(
                        todo!("account_index"),
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
                    )
                    .await;
                }
            };
            if !output_data.is_spent {
                wallet_data.unspent_outputs.insert(output_data.output_id, output_data);
            }
        }

        #[cfg(feature = "storage")]
        {
            log::debug!("[SYNC] storing account {} with new synced data", wallet_data.alias);
            self.save(Some(&wallet_data)).await?;
        }
        Ok(())
    }

    /// Update wallet with newly synced transactions.
    pub(crate) async fn update_with_transactions(
        &self,
        updated_transactions: Vec<Transaction>,
        spent_output_ids: Vec<OutputId>,
        output_ids_to_unlock: Vec<OutputId>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] Update wallet with new synced transactions");

        let mut wallet_data = self.data_mut().await;

        for transaction in updated_transactions {
            match transaction.inclusion_state {
                InclusionState::Confirmed | InclusionState::Conflicting | InclusionState::UnknownPruned => {
                    let transaction_id = transaction.payload.id();
                    wallet_data.pending_transactions.remove(&transaction_id);
                    log::debug!(
                        "[SYNC] inclusion_state of {transaction_id} changed to {:?}",
                        transaction.inclusion_state
                    );
                    #[cfg(feature = "events")]
                    {
                        self.emit(
                            todo!("wallet_data.index"),
                            WalletEvent::TransactionInclusion(TransactionInclusionEvent {
                                transaction_id,
                                inclusion_state: transaction.inclusion_state,
                            }),
                        )
                        .await;
                    }
                }
                _ => {}
            }
            wallet_data
                .transactions
                .insert(transaction.payload.id(), transaction.clone());
        }

        for output_to_unlock in &spent_output_ids {
            if let Some(output) = wallet_data.outputs.get_mut(output_to_unlock) {
                output.is_spent = true;
            }
            wallet_data.locked_outputs.remove(output_to_unlock);
            wallet_data.unspent_outputs.remove(output_to_unlock);
            log::debug!("[SYNC] Unlocked spent output {}", output_to_unlock);
        }

        for output_to_unlock in &output_ids_to_unlock {
            wallet_data.locked_outputs.remove(output_to_unlock);
            log::debug!(
                "[SYNC] Unlocked unspent output {} because of a conflicting transaction",
                output_to_unlock
            );
        }

        #[cfg(feature = "storage")]
        {
            log::debug!(
                "[SYNC] storing account {} with new synced transactions",
                wallet_data.alias
            );
            self.save(Some(&wallet_data)).await?;
        }
        Ok(())
    }

    // TODO: remove

    // /// Update wallet with newly generated addresses.
    // pub(crate) async fn update_wallet_addresses(
    //     &self,
    //     internal: bool,
    //     new_addresses: Vec<Bip44Address>,
    // ) -> crate::wallet::Result<()> { log::debug!("[update_account_addresses]");

    //     let mut wallet_data = self.data_mut().await;

    //     // add addresses to the account
    //     if internal {
    //         wallet_data.internal_addresses.extend(new_addresses);
    //     } else {
    //         wallet_data.public_addresses.extend(new_addresses);
    //     };

    //     #[cfg(feature = "storage")]
    //     {
    //         log::debug!("[update_wallet_addresses] storing account: {}", wallet_data.alias);
    //         self.save(Some(&wallet_data)).await?;
    //     }
    //     Ok(())
    // }

    // TODO: remove?

    /// Update the wallet address with a possible new Bech32 HRP and clear the inaccessible_incoming_transactions.
    pub(crate) async fn update_bech32_hrp(&self) -> crate::wallet::Result<()> {
        let bech32_hrp = self.client().get_bech32_hrp().await?;
        log::debug!("[UPDATE WALLET WITH BECH32 HRP] new bech32_hrp: {}", bech32_hrp);
        let mut wallet_data = self.data_mut().await;
        wallet_data.bech32_hrp = bech32_hrp;

        // TODO: remove

        // for address in &mut wallet_data.addresses_with_unspent_outputs {
        //     address.address.hrp = bech32_hrp;
        // }
        // for address in &mut wallet_data.public_addresses {
        //     address.address.hrp = bech32_hrp;
        // }
        // for address in &mut wallet_data.internal_addresses {
        //     address.address.hrp = bech32_hrp;
        // }

        wallet_data.inaccessible_incoming_transactions.clear();

        #[cfg(feature = "storage")]
        {
            log::debug!(
                "[SYNC] storing wallet {} after updating it with new bech32 hrp",
                wallet_data.alias
            );
            self.save(Some(&wallet_data)).await?;
        }

        Ok(())
    }
}
