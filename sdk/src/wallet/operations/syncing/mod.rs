// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod addresses;
pub(crate) mod foundries;
pub(crate) mod options;
pub(crate) mod outputs;
pub(crate) mod transactions;

use std::collections::{HashMap, HashSet};

pub use self::options::SyncOptions;
use crate::{
    client::secret::SecretManage,
    types::block::{
        address::{AccountAddress, Address, Bech32Address, NftAddress, ToBech32Ext},
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
                .collect::<Vec<_>>(),
            internal: false,
            key_index: 0,
        };

        let address_to_sync = vec![wallet_address_with_unspent_outputs];

        let (addresses_with_unspent_outputs, spent_or_not_synced_output_ids, outputs_data): (
            Vec<AddressWithUnspentOutputs>,
            Vec<OutputId>,
            Vec<OutputData>,
        ) = self.request_outputs_recursively(address_to_sync, options).await?;

        // Request possible spent outputs
        log::debug!("[SYNC] spent_or_not_synced_outputs: {spent_or_not_synced_output_ids:?}");
        let spent_or_unsynced_output_metadata_responses = self
            .client()
            .get_outputs_metadata_ignore_errors(&spent_or_not_synced_output_ids)
            .await?;

        // Add the output response to the output ids, the output response is optional, because an output could be
        // pruned and then we can't get the metadata
        let mut spent_or_unsynced_output_metadata_map: HashMap<OutputId, Option<OutputMetadata>> =
            spent_or_not_synced_output_ids.into_iter().map(|o| (o, None)).collect();
        for output_metadata_response in spent_or_unsynced_output_metadata_responses {
            let output_id = output_metadata_response.output_id();
            spent_or_unsynced_output_metadata_map.insert(*output_id, Some(output_metadata_response));
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
                .filter_map(|output| output.output.native_tokens())
                .flat_map(|native_tokens| {
                    native_tokens
                        .iter()
                        .map(|native_token| FoundryId::from(*native_token.token_id()))
                })
                .collect::<HashSet<_>>();

            // Request and store foundry outputs
            self.request_and_store_foundry_outputs(native_token_foundry_ids).await?;
        }

        // Updates wallet with balances, output ids, outputs
        self.update_after_sync(outputs_data, spent_or_unsynced_output_metadata_map)
            .await
    }

    // First request all outputs directly related to the wallet address, then for each nft and account output we got,
    // request all outputs that are related to their account/nft addresses in a loop until no new account or nft outputs
    // are found.
    async fn request_outputs_recursively(
        &self,
        addresses_to_sync: Vec<AddressWithUnspentOutputs>,
        options: &SyncOptions,
    ) -> crate::wallet::Result<(Vec<AddressWithUnspentOutputs>, Vec<OutputId>, Vec<OutputData>)> {
        // Cache the account and nft address with the related ed2559 address, so we can update the account address with
        // the new output ids

        let mut new_account_and_nft_addresses: HashMap<Address, Address> = HashMap::new();
        let mut spent_or_not_synced_output_ids = Vec::new();
        let mut addresses_with_unspent_outputs = Vec::new();
        let mut outputs_data = Vec::new();

        let bech32_hrp = self.client().get_bech32_hrp().await?;

        loop {
            let new_outputs_data = if new_account_and_nft_addresses.is_empty() {
                // Get outputs for the  addresses and add them also the the addresses_with_unspent_outputs
                let (unspent_output_ids, spent_or_not_synced_output_ids_inner) = self
                    .get_output_ids_for_addresses(addresses_to_sync.clone(), options)
                    .await?;

                spent_or_not_synced_output_ids = spent_or_not_synced_output_ids_inner;

                // Get outputs for addresses and add them also the the addresses_with_unspent_outputs
                let (addresses_with_unspent_outputs_inner, outputs_data_inner) =
                    self.get_outputs_from_address_output_ids(unspent_output_ids).await?;

                addresses_with_unspent_outputs = addresses_with_unspent_outputs_inner;
                outputs_data.extend(outputs_data_inner.clone());
                outputs_data_inner
            } else {
                let mut new_outputs_data = Vec::new();
                for (account_or_nft_address, output_address) in &new_account_and_nft_addresses {
                    let output_ids = self
                        .get_output_ids_for_address(
                            &Bech32Address::new(bech32_hrp, account_or_nft_address.clone()),
                            options,
                        )
                        .await?;

                    // Update address with unspent outputs
                    let address_with_unspent_outputs = addresses_with_unspent_outputs
                        .iter_mut()
                        .find(|address| address.address.inner() == output_address)
                        .ok_or_else(|| {
                            crate::wallet::Error::WalletAddressMismatch(output_address.clone().to_bech32(bech32_hrp))
                        })?;
                    address_with_unspent_outputs.output_ids.extend(output_ids.clone());

                    let new_outputs_data_inner = self.get_outputs(output_ids).await?;

                    let outputs_data_inner = self
                        .output_response_to_output_data(new_outputs_data_inner, address_with_unspent_outputs)
                        .await?;

                    outputs_data.extend(outputs_data_inner.clone());
                    new_outputs_data.extend(outputs_data_inner);
                }
                new_outputs_data
            };

            // Clear, so we only get new addresses
            new_account_and_nft_addresses.clear();

            // Add new account and nft addresses
            for output_data in new_outputs_data {
                match output_data.output {
                    Output::Account(account_output) => {
                        let account_address =
                            AccountAddress::from(account_output.account_id_non_null(&output_data.output_id));

                        new_account_and_nft_addresses.insert(Address::Account(account_address), output_data.address);
                    }
                    Output::Nft(nft_output) => {
                        let nft_address = NftAddress::from(nft_output.nft_id_non_null(&output_data.output_id));

                        new_account_and_nft_addresses.insert(Address::Nft(nft_address), output_data.address);
                    }
                    _ => {}
                }
            }

            log::debug!("[SYNC] new_account_and_nft_addresses: {new_account_and_nft_addresses:?}");
            if new_account_and_nft_addresses.is_empty() {
                break;
            }
        }

        // get_output_ids_for_addresses() will return recursively owned outputs not anymore, sine they will only get
        // synced afterwards, so we filter these unspent outputs here. Maybe the spent_or_not_synced_output_ids can be
        // calculated more efficient in the future, by comparing the new and old outputs only at this point. Then this
        // retain isn't needed anymore.

        let unspent_output_ids: HashSet<OutputId> = HashSet::from_iter(outputs_data.iter().map(|o| o.output_id));
        spent_or_not_synced_output_ids.retain(|o| !unspent_output_ids.contains(o));

        Ok((
            addresses_with_unspent_outputs,
            spent_or_not_synced_output_ids,
            outputs_data,
        ))
    }
}
