// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias_foundry;
mod basic;
mod nft;

use std::collections::HashSet;

#[cfg(not(target_family = "wasm"))]
use futures::FutureExt;
use instant::Instant;

use crate::{
    client::{node_api::indexer::QueryParameter, secret::SecretManage},
    types::block::{
        address::{Address, Bech32Address},
        output::OutputId,
    },
    wallet::account::{
        constants::PARALLEL_REQUESTS_AMOUNT, operations::syncing::SyncOptions,
        types::address::AddressWithUnspentOutputs, Account,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Returns output ids for outputs that are directly (Ed25519 address in AddressUnlockCondition) or indirectly
    /// (alias/nft address in AddressUnlockCondition and the alias/nft output is controlled with the Ed25519 address)
    /// connected to
    pub(crate) async fn get_output_ids_for_address(
        &self,
        address: Address,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        let bech32_address = Bech32Address::new(self.client().get_bech32_hrp().await?, address);

        if sync_options.sync_only_most_basic_outputs {
            let output_ids = self
                .get_basic_output_ids_with_address_unlock_condition_only(bech32_address)
                .await?;
            return Ok(output_ids);
        }

        // If interested in alias, basic and NFT outputs, get them all at once
        if (address.is_ed25519()
            && sync_options.account.alias_outputs
            && sync_options.account.basic_outputs
            && sync_options.account.nft_outputs)
            || (address.is_nft()
                && sync_options.nft.alias_outputs
                && sync_options.nft.basic_outputs
                && sync_options.nft.nft_outputs)
            || (address.is_alias()
                && sync_options.alias.alias_outputs
                && sync_options.alias.basic_outputs
                && sync_options.alias.nft_outputs)
        {
            let mut output_ids = self
                .client()
                .output_ids([QueryParameter::UnlockableByAddress(bech32_address)])
                .await?
                .items;

            if address.is_alias() && sync_options.alias.foundry_outputs {
                output_ids.extend(
                    self.client()
                        .foundry_output_ids([QueryParameter::AliasAddress(bech32_address)])
                        .await?
                        .items,
                );
            }

            return Ok(output_ids);
        }

        #[cfg(target_family = "wasm")]
        let mut results = Vec::new();

        #[cfg(not(target_family = "wasm"))]
        let mut tasks = Vec::new();

        if (address.is_ed25519() && sync_options.account.basic_outputs)
            || (address.is_nft() && sync_options.nft.basic_outputs)
            || (address.is_alias() && sync_options.alias.basic_outputs)
        {
            // basic outputs
            #[cfg(target_family = "wasm")]
            {
                results.push(
                    self.get_basic_output_ids_with_any_unlock_condition(bech32_address)
                        .await,
                )
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async move {
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

        if (address.is_ed25519() && sync_options.account.nft_outputs)
            || (address.is_nft() && sync_options.nft.nft_outputs)
            || (address.is_alias() && sync_options.alias.nft_outputs)
        {
            // nfts
            #[cfg(target_family = "wasm")]
            {
                results.push(self.get_nft_output_ids_with_any_unlock_condition(bech32_address).await)
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async move {
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

        if (address.is_ed25519() && sync_options.account.alias_outputs)
            || (address.is_nft() && sync_options.nft.alias_outputs)
            || (address.is_alias() && sync_options.alias.alias_outputs)
        {
            // aliases and foundries
            #[cfg(target_family = "wasm")]
            {
                results.push(
                    self.get_alias_and_foundry_output_ids(bech32_address, sync_options)
                        .await,
                )
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async move {
                        let sync_options = sync_options.clone();
                        let account = self.clone();
                        tokio::spawn(async move {
                            account
                                .get_alias_and_foundry_output_ids(bech32_address, &sync_options)
                                .await
                        })
                        .await
                    }
                    .boxed(),
                );
            }
        } else if address.is_alias() && sync_options.alias.foundry_outputs {
            // foundries
            #[cfg(target_family = "wasm")]
            {
                results.push(Ok(self
                    .client()
                    .foundry_output_ids([QueryParameter::AliasAddress(bech32_address)])
                    .await?
                    .items))
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async move {
                        let client = self.client().clone();
                        tokio::spawn(async move {
                            Ok(client
                                .foundry_output_ids([QueryParameter::AliasAddress(bech32_address)])
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

    /// Get the current output ids for provided addresses and only returns addresses that have unspent outputs and
    /// return spent outputs separated
    pub(crate) async fn get_output_ids_for_addresses(
        &self,
        options: &SyncOptions,
        addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputs>,
    ) -> crate::wallet::Result<(Vec<AddressWithUnspentOutputs>, Vec<OutputId>)> {
        log::debug!("[SYNC] start get_output_ids_for_addresses");
        let address_output_ids_start_time = Instant::now();

        let mut addresses_with_outputs = Vec::new();
        // spent outputs or alias/nft/foundries that don't get synced anymore, because of other sync options
        let mut spent_or_not_anymore_synced_outputs = Vec::new();
        // We split the addresses into chunks so we don't get timeouts if we have thousands
        for addresses_chunk in &mut addresses_with_unspent_outputs
            .chunks(PARALLEL_REQUESTS_AMOUNT)
            .map(|x: &[AddressWithUnspentOutputs]| x.to_vec())
        {
            let results;
            #[cfg(target_family = "wasm")]
            {
                let mut tasks = Vec::new();
                for address in addresses_chunk {
                    let output_ids = self.get_output_ids_for_address(address.address.inner, &options).await?;
                    tasks.push(crate::wallet::Result::Ok((address, output_ids)));
                }
                results = tasks;
            }

            #[cfg(not(target_family = "wasm"))]
            {
                let mut tasks = Vec::new();
                for address in addresses_chunk {
                    let account = self.clone();
                    let sync_options = options.clone();
                    tasks.push(async move {
                        tokio::spawn(async move {
                            let output_ids = account
                                .get_output_ids_for_address(address.address.inner, &sync_options)
                                .await?;
                            crate::wallet::Result::Ok((address, output_ids))
                        })
                        .await
                    });
                }

                results = futures::future::try_join_all(tasks).await?;
            }

            for res in results {
                let (mut address, output_ids): (AddressWithUnspentOutputs, Vec<OutputId>) = res?;
                // only return addresses with outputs
                if !output_ids.is_empty() {
                    // outputs we had before, but now not anymore, got spent or are alias/nft/foundries that don't get
                    // synced anymore because of other sync options
                    for output_id in address.output_ids {
                        if !output_ids.contains(&output_id) {
                            spent_or_not_anymore_synced_outputs.push(output_id);
                        }
                    }
                    address.output_ids = output_ids;
                    addresses_with_outputs.push(address);
                } else {
                    // outputs we had before, but now not anymore, got spent or are alias/nft/foundries that don't get
                    // synced anymore because of other sync options
                    spent_or_not_anymore_synced_outputs.extend(address.output_ids);
                }
            }
        }

        log::debug!(
            "[SYNC] spent or not anymore synced alias/nft/foundries outputs: {:?}",
            spent_or_not_anymore_synced_outputs
        );
        log::debug!(
            "[SYNC] finished get_output_ids_for_addresses in {:.2?}",
            address_output_ids_start_time.elapsed()
        );
        Ok((addresses_with_outputs, spent_or_not_anymore_synced_outputs))
    }
}
