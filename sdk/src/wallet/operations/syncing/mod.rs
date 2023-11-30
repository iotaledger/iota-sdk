// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod addresses;
pub(crate) mod foundries;
pub(crate) mod options;
pub(crate) mod outputs;
pub(crate) mod transactions;

use std::collections::{HashMap, HashSet};

use futures::{future::BoxFuture, FutureExt};

pub use self::options::SyncOptions;
use crate::{
    client::secret::SecretManage,
    types::block::{
        address::{AccountAddress, Address, Hrp, NftAddress, ToBech32Ext},
        output::{FoundryId, Output, OutputId, OutputMetadata},
    },
    wallet::{
        constants::MIN_SYNC_INTERVAL,
        types::{AddressWithUnspentOutputs, Balance, OutputData},
        Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Set the fallback SyncOptions for account syncing.
    /// If storage is enabled, will persist during restarts.
    pub async fn set_default_sync_options(&self, options: SyncOptions) -> crate::wallet::Result<()> {
        #[cfg(feature = "storage")]
        {
            let storage_manager = self.storage_manager.read().await;
            storage_manager.set_default_sync_options(&options).await?;
        }

        *self.default_sync_options.lock().await = options;
        Ok(())
    }

    // Get the default sync options we use when none are provided.
    pub async fn default_sync_options(&self) -> SyncOptions {
        self.default_sync_options.lock().await.clone()
    }

    /// Sync the wallet by fetching new information from the nodes. Will also reissue pending transactions
    /// if necessary. A custom default can be set using set_default_sync_options.
    pub async fn sync(&self, options: Option<SyncOptions>) -> crate::wallet::Result<Balance> {
        let options = match options {
            Some(opt) => opt,
            None => self.default_sync_options().await,
        };

        log::debug!("[SYNC] start syncing with {:?}", options);
        let syc_start_time = instant::Instant::now();

        // Prevent syncing the account multiple times simultaneously
        let time_now = crate::client::unix_timestamp_now().as_millis();
        let mut last_synced = self.last_synced.lock().await;
        log::debug!("[SYNC] last time synced before {}ms", time_now - *last_synced);
        if !options.force_syncing && time_now - *last_synced < MIN_SYNC_INTERVAL {
            log::debug!(
                "[SYNC] synced within the latest {} ms, only calculating balance",
                MIN_SYNC_INTERVAL
            );
            // Calculate the balance because if we created a transaction in the meantime, the amount for the inputs
            // is not available anymore
            return self.balance().await;
        }

        self.sync_internal(&options).await?;

        // Sync transactions after updating account with outputs, so we can use them to check the transaction
        // status
        if options.sync_pending_transactions {
            let confirmed_tx_with_unknown_output = self.sync_pending_transactions().await?;
            // Sync again if we don't know the output yet, to prevent having no unspent outputs after syncing
            if confirmed_tx_with_unknown_output {
                log::debug!("[SYNC] a transaction for which no output is known got confirmed, syncing outputs again");
                self.sync_internal(&options).await?;
            }
        };

        let balance = self.balance().await?;
        // Update last_synced mutex
        let time_now = crate::client::unix_timestamp_now().as_millis();
        *last_synced = time_now;
        log::debug!("[SYNC] finished syncing in {:.2?}", syc_start_time.elapsed());
        Ok(balance)
    }

    async fn sync_internal(&self, options: &SyncOptions) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] sync_internal");

        let wallet_address_with_unspent_outputs = AddressWithUnspentOutputs {
            address: self.address().await,
            output_ids: self
                .unspent_outputs(None)
                .await
                .into_iter()
                .map(|data| data.output_id)
                .collect(),
            internal: false,
            key_index: 0,
        };

        let address_to_sync = vec![
            wallet_address_with_unspent_outputs,
            AddressWithUnspentOutputs {
                address: self.implicit_account_creation_address().await?,
                output_ids: HashSet::new(),
                internal: false,
                key_index: 0,
            },
        ];

        let (_addresses_with_unspent_outputs, spent_or_not_synced_output_ids, outputs_data) =
            self.request_outputs_recursively(address_to_sync, options).await?;

        // Request possible spent outputs
        log::debug!("[SYNC] spent_or_not_synced_outputs: {spent_or_not_synced_output_ids:?}");
        let spent_or_unsynced_output_metadata_responses = self
            .client()
            .get_outputs_metadata_ignore_not_found(&spent_or_not_synced_output_ids)
            .await?;

        // Add the output response to the output ids, the output response is optional, because an output could be
        // pruned and then we can't get the metadata
        let mut spent_or_unsynced_output_metadata: HashMap<OutputId, Option<OutputMetadata>> =
            spent_or_not_synced_output_ids.into_iter().map(|o| (o, None)).collect();
        for output_metadata_response in spent_or_unsynced_output_metadata_responses {
            let output_id = output_metadata_response.output_id();
            spent_or_unsynced_output_metadata.insert(*output_id, Some(output_metadata_response));
        }

        if options.sync_incoming_transactions {
            let transaction_ids = outputs_data
                .iter()
                .map(|output| *output.output_id.transaction_id())
                .collect();
            // Request and store transaction payload for newly received unspent outputs
            self.request_incoming_transaction_data(transaction_ids).await?;
        }

        if options.sync_native_token_foundries {
            let native_token_foundry_ids = outputs_data
                .iter()
                .filter_map(|output| {
                    output
                        .output
                        .native_token()
                        .map(|native_token| FoundryId::from(*native_token.token_id()))
                })
                .collect::<HashSet<_>>();

            // Request and store foundry outputs
            self.request_and_store_foundry_outputs(native_token_foundry_ids).await?;
        }

        // Updates wallet with balances, output ids, outputs
        self.update_after_sync(outputs_data, spent_or_unsynced_output_metadata)
            .await
    }

    /// Recursively scans the given address for associated outputs.
    /// Adds to the unspent_outputs_data and removes unspent data from spent_output_ids.
    fn scan_address<'a>(
        &'a self,
        bech32_hrp: Hrp,
        address_with_unspent_outputs: &'a mut AddressWithUnspentOutputs,
        unspent_outputs_data: &'a mut Vec<OutputData>,
        unspent_outputs: &'a [OutputData],
        spent_output_ids: &'a mut HashSet<OutputId>,
        options: &'a SyncOptions,
    ) -> BoxFuture<'a, crate::wallet::Result<()>> {
        async move {
            for unspent_output in unspent_outputs {
                // TODO: is this really necessary?
                spent_output_ids.remove(&unspent_output.output_id);
                if let Some(account_or_nft_address) = match &unspent_output.output {
                    Output::Account(account) => {
                        Some(AccountAddress::from(account.account_id_non_null(&unspent_output.output_id)).into())
                    }
                    Output::Nft(nft) => Some(NftAddress::from(nft.nft_id_non_null(&unspent_output.output_id)).into()),
                    _ => None,
                }
                .map(|a| Address::to_bech32(a, bech32_hrp))
                {
                    let account_or_nft_output_ids = self
                        .get_output_ids_for_address(&account_or_nft_address, options)
                        .await?;

                    let account_or_nft_outputs_with_metadata = self.get_outputs(&account_or_nft_output_ids).await?;

                    // Update address with new associated unspent outputs
                    address_with_unspent_outputs
                        .output_ids
                        .extend(account_or_nft_output_ids);

                    let output_data = self
                        .output_response_to_output_data(
                            account_or_nft_outputs_with_metadata,
                            address_with_unspent_outputs,
                        )
                        .await?;

                    self.scan_address(
                        bech32_hrp,
                        address_with_unspent_outputs,
                        unspent_outputs_data,
                        &output_data,
                        spent_output_ids,
                        options,
                    )
                    .await?;

                    unspent_outputs_data.extend(output_data);
                }
            }
            Ok(())
        }
        .boxed()
    }

    // First request all outputs directly related to the wallet address, then for each nft and account output we got,
    // request all outputs that are related to their account/nft addresses in a loop until no new account or nft outputs
    // are found.
    async fn request_outputs_recursively(
        &self,
        addresses_to_sync: Vec<AddressWithUnspentOutputs>,
        options: &SyncOptions,
    ) -> crate::wallet::Result<(Vec<AddressWithUnspentOutputs>, HashSet<OutputId>, Vec<OutputData>)> {
        let bech32_hrp = self.client().get_bech32_hrp().await?;

        // Get the unspent and spent/not-synced output ids per address to sync
        let (addresses_to_sync_with_unspent_output_ids, mut spent_output_ids) =
            self.get_output_ids_for_addresses(addresses_to_sync, options).await?;

        // Get the corresponding unspent output data
        let unspent_outputs_map = self
            .get_outputs_from_address_output_ids(addresses_to_sync_with_unspent_output_ids)
            .await?;

        let mut addresses_with_unspent_outputs = Vec::with_capacity(unspent_outputs_map.len());
        let mut unspent_outputs_data = Vec::new();
        for (_, (mut address_with_unspent_outputs, unspent_outputs)) in unspent_outputs_map {
            self.scan_address(
                bech32_hrp,
                &mut address_with_unspent_outputs,
                &mut unspent_outputs_data,
                &unspent_outputs,
                &mut spent_output_ids,
                options,
            )
            .await?;
            addresses_with_unspent_outputs.push(address_with_unspent_outputs);
            unspent_outputs_data.extend(unspent_outputs);
        }

        Ok((addresses_with_unspent_outputs, spent_output_ids, unspent_outputs_data))
    }
}
