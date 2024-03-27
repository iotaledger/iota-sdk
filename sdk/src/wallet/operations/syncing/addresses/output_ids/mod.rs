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
        node_api::indexer::query_parameters::{
            DelegationOutputQueryParameters, FoundryOutputQueryParameters, OutputQueryParameters,
        },
        secret::SecretManage,
    },
    types::block::{address::Bech32Address, output::OutputId},
    wallet::{
        constants::PARALLEL_REQUESTS_AMOUNT,
        operations::syncing::SyncOptions,
        types::address::{AddressWithUnspentOutputIds, SpentOutputId},
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Returns output ids for outputs that are directly (Ed25519 address in AddressUnlockCondition) or indirectly
    /// (account/nft address in AddressUnlockCondition and the account/nft output is controlled with the Ed25519
    /// address) connected to
    pub(crate) async fn get_output_ids_for_address(
        &self,
        address: &Bech32Address,
        sync_options: &SyncOptions,
    ) -> Result<Vec<OutputId>, WalletError> {
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
            || (address.is_implicit_account_creation() && sync_options.sync_implicit_accounts)
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
            || (address.is_implicit_account_creation() && sync_options.sync_implicit_accounts)
        {
            // basic outputs
            #[cfg(target_family = "wasm")]
            {
                results.push(
                    self.get_basic_output_ids_with_any_unlock_condition(address.clone())
                        .await,
                )
            }

            #[cfg(not(target_family = "wasm"))]
            {
                tasks.push(
                    async {
                        let bech32_address = address.clone();
                        let wallet = self.clone();
                        tokio::spawn(async move {
                            wallet
                                .get_basic_output_ids_with_any_unlock_condition(bech32_address)
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
                        let wallet = self.clone();
                        tokio::spawn(async move {
                            wallet
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
                    .foundry_output_ids(FoundryOutputQueryParameters::new().account(address.clone()))
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
                        let wallet = self.clone();
                        tokio::spawn(async move {
                            wallet
                                .get_nft_output_ids_with_any_unlock_condition(bech32_address)
                                .await
                        })
                        .await
                    }
                    .boxed(),
                );
            }
        }

        if (address.is_ed25519() && sync_options.wallet.delegation_outputs)
            || (address.is_nft() && sync_options.nft.delegation_outputs)
            || (address.is_account() && sync_options.account.delegation_outputs)
        {
            // delegations
            #[cfg(target_family = "wasm")]
            {
                results.push(Ok(self
                    .client()
                    .delegation_output_ids(DelegationOutputQueryParameters::new().address(address.clone()))
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
                                .delegation_output_ids(DelegationOutputQueryParameters::new().address(bech32_address))
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
        let output_ids = results.into_iter().collect::<Result<Vec<_>, _>>()?;
        let output_ids: HashSet<OutputId> = HashSet::from_iter(output_ids.into_iter().flat_map(|v| v.into_iter()));

        Ok(output_ids.into_iter().collect())
    }

    /// Get the current output ids and only returns addresses that have unspent outputs and
    /// return spent outputs separated
    pub(crate) async fn get_output_ids_for_addresses(
        &self,
        addresses: &[AddressWithUnspentOutputIds],
        options: &SyncOptions,
    ) -> Result<(Vec<AddressWithUnspentOutputIds>, Vec<SpentOutputId>), WalletError> {
        log::debug!("[SYNC] start get_output_ids_for_addresses");
        let address_output_ids_start_time = Instant::now();

        let mut addresses_with_unspent_outputs = Vec::new();
        // spent outputs or account/nft/foundries that don't get synced anymore, because of other sync options
        let mut spent_or_ignored_outputs = Vec::new();

        // We split the addresses into chunks so we don't get timeouts if we have thousands
        for addresses_chunk in addresses
            .chunks(PARALLEL_REQUESTS_AMOUNT)
            .map(|x: &[AddressWithUnspentOutputIds]| x.to_vec())
        {
            let results: Vec<Result<_, WalletError>>;
            #[cfg(target_family = "wasm")]
            {
                let mut tasks = Vec::new();
                for address in addresses_chunk {
                    let output_ids = self.get_output_ids_for_address(&address.address, options).await?;
                    tasks.push(Ok((address, output_ids)));
                }
                results = tasks;
            }

            #[cfg(not(target_family = "wasm"))]
            {
                let mut tasks = Vec::new();
                for address in addresses_chunk {
                    let wallet = self.clone();
                    let sync_options = options.clone();
                    tasks.push(async move {
                        tokio::spawn(async move {
                            let output_ids = wallet
                                .get_output_ids_for_address(&address.address, &sync_options)
                                .await?;
                            Ok((address, output_ids))
                        })
                        .await
                    });
                }

                results = futures::future::try_join_all(tasks).await?;
            }

            let addresses_with_new_unspent_output_ids = results.into_iter().collect::<Result<Vec<_>, _>>()?;

            for (mut address, new_unspent_output_ids) in addresses_with_new_unspent_output_ids {
                // only return addresses with outputs
                if !new_unspent_output_ids.is_empty() {
                    // outputs we had before, but now not anymore, got spent or are account/nft/foundries that don't
                    // get synced anymore because of other sync options
                    for output_id in address.unspent_output_ids {
                        if !new_unspent_output_ids.contains(&output_id) {
                            spent_or_ignored_outputs.push(output_id);
                        }
                    }
                    address.unspent_output_ids = new_unspent_output_ids;
                    addresses_with_unspent_outputs.push(address);
                } else {
                    // outputs we had before, but now not anymore, got spent or are account/nft/foundries that don't
                    // get synced anymore because of other sync options
                    spent_or_ignored_outputs.extend(address.unspent_output_ids);
                }
            }
        }

        log::debug!(
            "[SYNC] spent or ignored account/nft/foundries outputs: {:?}",
            spent_or_ignored_outputs
        );
        log::debug!(
            "[SYNC] finished get_output_ids_for_addresses in {:.2?}",
            address_output_ids_start_time.elapsed()
        );
        Ok((addresses_with_unspent_outputs, spent_or_ignored_outputs))
    }
}
