// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

#[cfg(not(target_family = "wasm"))]
use futures::FutureExt;

use crate::{
    client::{node_api::indexer::query_parameters::QueryParameter, secret::SecretManage},
    types::{
        api::plugins::indexer::OutputIdsResponse,
        block::{
            address::{Address, AliasAddress},
            output::{Output, OutputId},
        },
    },
    wallet::{
        account::{Account, SyncOptions},
        task,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Returns output ids of alias outputs
    pub(crate) async fn get_alias_and_foundry_output_ids(
        &self,
        bech32_address: &str,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[SYNC] get_alias_and_foundry_output_ids");
        let client = self.client();

        let mut output_ids = HashSet::new();

        #[cfg(target_family = "wasm")]
        {
            output_ids.extend(
                client
                    .alias_output_ids(vec![QueryParameter::Governor(bech32_address.to_string())])
                    .await?
                    .items,
            );
            output_ids.extend(
                client
                    .alias_output_ids(vec![QueryParameter::StateController(bech32_address.to_string())])
                    .await?
                    .items,
            );
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let tasks = vec![
                // Get outputs where the address is in the governor address unlock condition
                async move {
                    let bech32_address_ = bech32_address.to_string();
                    let client = client.clone();
                    task::spawn(async move {
                        client
                            .alias_output_ids(vec![QueryParameter::Governor(bech32_address_)])
                            .await
                            .map_err(From::from)
                    })
                    .await
                }
                .boxed(),
                // Get outputs where the address is in the state controller unlock condition
                async move {
                    let bech32_address_ = bech32_address.to_string();
                    let client = client.clone();
                    task::spawn(async move {
                        client
                            .alias_output_ids(vec![QueryParameter::StateController(bech32_address_)])
                            .await
                            .map_err(From::from)
                    })
                    .await
                }
                .boxed(),
            ];

            let results: Vec<crate::wallet::Result<OutputIdsResponse>> = futures::future::try_join_all(tasks).await?;

            for res in results {
                let found_output_ids = res?;
                output_ids.extend(found_output_ids.items);
            }
        }

        // Get all results
        if sync_options.alias.foundry_outputs {
            let foundry_output_ids = self.get_foundry_output_ids(&output_ids).await?;
            output_ids.extend(foundry_output_ids);
        }

        Ok(output_ids.into_iter().collect())
    }

    /// Returns output ids of foundries controlled by the provided aliases
    pub(crate) async fn get_foundry_output_ids(
        &self,
        alias_output_ids: &HashSet<OutputId>,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[SYNC] get_foundry_output_ids");
        // Get alias outputs, so we can then get the foundry outputs with the alias addresses
        let alias_outputs_with_meta = self.get_outputs(alias_output_ids.iter().copied().collect()).await?;

        let bech32_hrp = self.client().get_bech32_hrp().await?;

        let mut tasks = vec![];

        for alias_output_with_meta in alias_outputs_with_meta {
            if let Output::Alias(alias_output) = alias_output_with_meta.output() {
                let alias_address =
                    AliasAddress::from(alias_output.alias_id_non_null(alias_output_with_meta.metadata().output_id()));
                let alias_bech32_address = Address::Alias(alias_address).to_bech32(bech32_hrp.clone());
                let client = self.client().clone();
                tasks.push(Box::pin(task::spawn(async move {
                    client
                        .foundry_output_ids(vec![QueryParameter::AliasAddress(alias_bech32_address)])
                        .await
                        .map_err(From::from)
                })));
            }
        }

        let mut output_ids = HashSet::new();
        let results: Vec<crate::wallet::Result<OutputIdsResponse>> = futures::future::try_join_all(tasks).await?;

        for res in results {
            let foundry_output_ids = res?;
            output_ids.extend(foundry_output_ids.items);
        }

        Ok(output_ids.into_iter().collect())
    }
}
