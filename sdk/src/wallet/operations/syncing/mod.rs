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
    client::{secret::SecretManage, ClientError},
    types::block::{
        address::{AccountAddress, Address, Bech32Address, NftAddress},
        output::{FoundryId, Output, OutputId, OutputMetadata},
    },
    wallet::{
        constants::MIN_SYNC_INTERVAL,
        types::{
            address::{AddressWithUnspentOutputIds, AddressWithUnspentOutputs, SpentOutput},
            Balance, OutputWithExtendedMetadata,
        },
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S> {
    // TODO: can we rename this to `set_fallback_sync_options`? It's no biggie, but if I read `default` in Rust
    // if feels like it would use `SyncOptions::default()`, but that's not what we mean here, right?
    /// Set the default SyncOptions for account syncing.
    /// If storage is enabled, will persist during restarts.
    pub async fn set_default_sync_options(&self, options: SyncOptions) -> Result<(), WalletError> {
        #[cfg(feature = "storage")]
        {
            self.storage_manager().set_default_sync_options(&options).await?;
        }

        *self.default_sync_options.lock().await = options;
        Ok(())
    }

    // Get the default sync options we use when none are provided.
    pub async fn default_sync_options(&self) -> SyncOptions {
        self.default_sync_options.lock().await.clone()
    }

    // First request all outputs directly related to the wallet address, then for each nft and account output we got,
    // request all outputs that are related to their account/nft addresses in a loop until no new account or nft outputs
    // are found.
    async fn request_outputs_recursively(
        &self,
        addresses: &[AddressWithUnspentOutputIds],
        options: &SyncOptions,
    ) -> Result<
        (
            Vec<AddressWithUnspentOutputIds>,
            Vec<SpentOutput>,
            Vec<OutputWithExtendedMetadata>,
        ),
        WalletError,
    > {
        let bech32_hrp = self.client().get_bech32_hrp().await?;
        let network_id = self.client().get_network_id().await?;

        // Get the unspent and spent/not-synced output ids per address to sync
        let (addresses_with_unspent_output_ids, mut spent_or_not_synced_output_ids) =
            self.get_output_ids_for_addresses(addresses, options).await?;

        // Get the corresponding unspent outputs
        let mut addresses_with_unspent_outputs = self
            .get_outputs_from_address_output_ids(&addresses_with_unspent_output_ids)
            .await?;

        // Cache account and nft addresses with the related Ed25519 address, so we can update the account
        // address with the new output ids.
        let mut addresses_to_scan: HashMap<Address, Address> = HashMap::new();
        let mut new_addresses_with_unspent_output_ids = Vec::new();
        let mut unspent_outputs_with_extra_metadata = Vec::new();

        loop {
            // Try to discover new addresses and collect them in `addresses_to_scan`.
            // See https://github.com/rust-lang/rust-clippy/issues/8539 regarding this lint.
            #[allow(clippy::iter_with_drain)]
            for AddressWithUnspentOutputs {
                address,
                unspent_output_ids,
                unspent_outputs,
            } in addresses_with_unspent_outputs.drain(..)
            {
                for unspent_output in &unspent_outputs {
                    match &unspent_output.output {
                        Output::Account(account) => {
                            addresses_to_scan.insert(
                                AccountAddress::from(account.account_id_non_null(&unspent_output.output_id)).into(),
                                address.inner().clone(),
                            );
                        }
                        Output::Nft(nft) => {
                            addresses_to_scan.insert(
                                NftAddress::from(nft.nft_id_non_null(&unspent_output.output_id)).into(),
                                address.inner().clone(),
                            );
                        }
                        _ => {}
                    }
                }
                new_addresses_with_unspent_output_ids.push(AddressWithUnspentOutputIds {
                    address,
                    unspent_output_ids,
                });
                unspent_outputs_with_extra_metadata.extend(unspent_outputs);
            }

            log::debug!("[SYNC] new_addresses: {addresses_to_scan:?}");

            // If there are no new addresses to scan, we are finished
            if addresses_to_scan.is_empty() {
                break;
            }

            // Get the unspent outputs of the new addresses
            for (account_or_nft_address, output_address) in addresses_to_scan.drain() {
                let address_with_unspent_output_ids = new_addresses_with_unspent_output_ids
                    .iter_mut()
                    .find(|address| address.address.inner() == &output_address)
                    // Panic: can't happen because one is a superset of the other
                    .unwrap();

                let account_or_nft_output_ids = self
                    .get_output_ids_for_address(&Bech32Address::new(bech32_hrp, account_or_nft_address), options)
                    .await?;

                // Update address with new associated unspent outputs
                address_with_unspent_output_ids
                    .unspent_output_ids
                    .extend(account_or_nft_output_ids.clone());

                let account_or_nft_outputs_with_metadata =
                    self.get_outputs_request_unknown(&account_or_nft_output_ids).await?;
                let account_or_nft_outputs_with_extra_metadata = self
                    .output_response_to_output_with_extended_metadata(account_or_nft_outputs_with_metadata, network_id)
                    .await?;

                let AddressWithUnspentOutputIds {
                    address,
                    unspent_output_ids,
                } = address_with_unspent_output_ids.clone();
                addresses_with_unspent_outputs.push(AddressWithUnspentOutputs {
                    address,
                    unspent_output_ids,
                    unspent_outputs: account_or_nft_outputs_with_extra_metadata,
                });
            }
        }

        // get_output_ids_for_addresses() will return recursively owned outputs not anymore, sine they will only get
        // synced afterwards, so we filter these unspent outputs here. Maybe the spent_or_not_synced_output_ids can be
        // calculated more efficient in the future, by comparing the new and old outputs only at this point. Then this
        // retain isn't needed anymore.
        let unspent_output_ids_all: HashSet<OutputId> =
            HashSet::from_iter(unspent_outputs_with_extra_metadata.iter().map(|o| o.output_id));
        spent_or_not_synced_output_ids.retain(|o| !unspent_output_ids_all.contains(o));

        Ok((
            new_addresses_with_unspent_output_ids,
            spent_or_not_synced_output_ids,
            unspent_outputs_with_extra_metadata,
        ))
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Sync the wallet by fetching new information from the nodes. Will also reissue pending transactions
    /// if necessary. A custom default can be set using set_default_sync_options.
    pub async fn sync(&self, options: impl Into<Option<SyncOptions>> + Send) -> Result<Balance, WalletError> {
        let options = match options.into() {
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

    async fn sync_internal(&self, options: &SyncOptions) -> Result<(), WalletError> {
        log::debug!("[SYNC] sync_internal");

        let wallet_address_with_unspent_outputs = AddressWithUnspentOutputIds {
            address: self.address().await,
            unspent_output_ids: self.ledger().await.unspent_outputs().keys().copied().collect(),
        };

        let mut addresses_to_sync = vec![wallet_address_with_unspent_outputs];

        if options.sync_implicit_accounts {
            if let Ok(implicit_account_creation_address) = self.implicit_account_creation_address().await {
                addresses_to_sync.push(AddressWithUnspentOutputIds {
                    unspent_output_ids: self
                        .ledger()
                        .await
                        .implicit_accounts()
                        .filter_map(|output_with_ext_metadata| {
                            if output_with_ext_metadata.output.as_basic().address()
                                == implicit_account_creation_address.inner()
                            {
                                Some(output_with_ext_metadata.output_id)
                            } else {
                                None
                            }
                        })
                        .collect(),
                    address: implicit_account_creation_address,
                });
            }
        }

        let (_addresses_with_unspent_outputs, spent_or_not_synced_output_ids, outputs_data) =
            self.request_outputs_recursively(&addresses_to_sync, options).await?;

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
}
