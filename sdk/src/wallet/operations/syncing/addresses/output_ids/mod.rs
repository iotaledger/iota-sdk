// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account_foundry;
mod basic;
mod nft;

use std::collections::HashSet;

#[cfg(not(target_family = "wasm"))]
use futures::FutureExt;
use instant::Instant;

use crate::{
    client::{
        node_api::indexer::query_parameters::{FoundryOutputQueryParameters, OutputQueryParameters},
        secret::SecretManage,
    },
    types::block::{address::Bech32Address, output::OutputId},
    wallet::{operations::syncing::SyncOptions, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Returns output ids for outputs that are directly (Ed25519 address in AddressUnlockCondition) or indirectly
    /// (account/nft address in AddressUnlockCondition and the account/nft output is controlled with the Ed25519
    /// address) connected to
    pub(crate) async fn get_output_ids_for_address(
        &self,
        address: &Bech32Address,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        if sync_options.sync_only_most_basic_outputs {
            let output_ids = self
                .get_basic_output_ids_with_address_unlock_condition_only(address.clone())
                .await?;
            return Ok(output_ids);
        }

        // If interested in account, basic, NFT and foundry outputs, get them all at once
        if (address.is_ed25519() && sync_options.wallet.all_outputs())
            || (address.is_nft() && sync_options.nft.all_outputs())
            || (address.is_account() && sync_options.account.all_outputs())
        {
            return Ok(self
                .client()
                .output_ids(OutputQueryParameters::new().unlockable_by_address(address.clone()))
                .await?
                .items);
        }

        #[cfg(target_family = "wasm")]
        let mut results = Vec::new();

        #[cfg(not(target_family = "wasm"))]
        let mut tasks = Vec::new();

        if (address.is_ed25519() && sync_options.wallet.basic_outputs)
            || (address.is_nft() && sync_options.nft.basic_outputs)
            || (address.is_account() && sync_options.account.basic_outputs)
        {
            // basic outputs
            #[cfg(target_family = "wasm")]
            {
                results.push(
                    self.get_basic_output_ids_with_any_unlock_condition(bech32_address.clone())
                        .await,
                )
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async {
                        let bech32_address = address.clone();
                        let account = self.clone();
                        tokio::spawn(async move {
                            account
                                .get_basic_output_ids_with_any_unlock_condition(bech32_address)
                                .await
                        })
                        .await
                    }
                    .boxed(),
                );
            }
        }

        if (address.is_ed25519() && sync_options.wallet.nft_outputs)
            || (address.is_nft() && sync_options.nft.nft_outputs)
            || (address.is_account() && sync_options.account.nft_outputs)
        {
            // nfts
            #[cfg(target_family = "wasm")]
            {
                results.push(self.get_nft_output_ids_with_any_unlock_condition(address.clone()).await)
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async {
                        let bech32_address = address.clone();
                        let account = self.clone();
                        tokio::spawn(async move {
                            account
                                .get_nft_output_ids_with_any_unlock_condition(bech32_address)
                                .await
                        })
                        .await
                    }
                    .boxed(),
                );
            }
        }

        if (address.is_ed25519() && sync_options.wallet.account_outputs)
            || (address.is_nft() && sync_options.nft.account_outputs)
            || (address.is_account() && sync_options.account.account_outputs)
        {
            // accounts and foundries
            #[cfg(target_family = "wasm")]
            {
                results.push(
                    self.get_account_and_foundry_output_ids(address.clone(), sync_options)
                        .await,
                )
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async {
                        let bech32_address = address.clone();
                        let sync_options = sync_options.clone();
                        let account = self.clone();
                        tokio::spawn(async move {
                            account
                                .get_account_and_foundry_output_ids(bech32_address, &sync_options)
                                .await
                        })
                        .await
                    }
                    .boxed(),
                );
            }
        } else if address.is_account() && sync_options.account.foundry_outputs {
            // foundries
            #[cfg(target_family = "wasm")]
            {
                results.push(Ok(self
                    .client()
                    .foundry_output_ids(FoundryOutputQueryParameters::new().account(address))
                    .await?
                    .items))
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async {
                        let bech32_address = address.clone();
                        let client = self.client().clone();
                        tokio::spawn(async move {
                            Ok(client
                                .foundry_output_ids(FoundryOutputQueryParameters::new().account(bech32_address))
                                .await?
                                .items)
                        })
                        .await
                    }
                    .boxed(),
                );
            }
        }

        #[cfg(not(target_family = "wasm"))]
        let results = futures::future::try_join_all(tasks).await?;

        // Get all results
        let mut output_ids = HashSet::new();
        for res in results {
            let found_output_ids = res?;
            output_ids.extend(found_output_ids);
        }

        Ok(output_ids.into_iter().collect())
    }

    /// Get the current output ids and returns spent or not synced outputs separated
    pub(crate) async fn get_output_ids_for_addresses(
        &self,
        address: Bech32Address,
        options: &SyncOptions,
    ) -> crate::wallet::Result<(Vec<OutputId>, Vec<OutputId>)> {
        log::debug!("[SYNC] start get_output_ids_for_addresses");
        let address_output_ids_start_time = Instant::now();

        let mut addresses_with_outputs = Vec::new();
        // spent outputs or account/nft/foundries that don't get synced anymore, because of other sync options
        let mut spent_or_not_anymore_synced_outputs = Vec::new();

        let current_output_ids = &self.data.read().await.unspent_outputs;

        let new_synced_output_ids = self.get_output_ids_for_address(&address, &options).await?;

        if !new_synced_output_ids.is_empty() {
            // outputs we had before, but now not anymore, got spent or are account/nft/foundries that don't
            // get synced anymore because of other sync options
            for output_id in &new_synced_output_ids {
                if !current_output_ids.contains_key(&output_id) {
                    spent_or_not_anymore_synced_outputs.push(*output_id);
                }
            }
            addresses_with_outputs.push(address);
        } else {
            // outputs we had before, but now not anymore, got spent or are account/nft/foundries that don't
            // get synced anymore because of other sync options
            spent_or_not_anymore_synced_outputs.extend(current_output_ids.keys().cloned().collect::<Vec<OutputId>>());
        }

        log::debug!(
            "[SYNC] spent or not anymore synced account/nft/foundries outputs: {:?}",
            spent_or_not_anymore_synced_outputs
        );
        log::debug!(
            "[SYNC] finished get_output_ids_for_addresses in {:.2?}",
            address_output_ids_start_time.elapsed()
        );
        Ok((new_synced_output_ids, spent_or_not_anymore_synced_outputs))
    }
}
