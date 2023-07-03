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
            address::{AccountAddress, Bech32Address, ToBech32Ext},
            output::{Output, OutputId},
            ConvertTo,
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
    /// Returns output ids of account outputs
    pub(crate) async fn get_account_and_foundry_output_ids(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[SYNC] get_account_and_foundry_output_ids");
        let client = self.client();
        let bech32_address = bech32_address.convert()?;

        let mut output_ids = HashSet::new();

        #[cfg(target_family = "wasm")]
        {
            output_ids.extend(
                client
                    .account_output_ids([QueryParameter::Governor(bech32_address)])
                    .await?
                    .items,
            );
            output_ids.extend(
                client
                    .account_output_ids([QueryParameter::StateController(bech32_address)])
                    .await?
                    .items,
            );
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let tasks = [
                // Get outputs where the address is in the governor address unlock condition
                async move {
                    let client = client.clone();
                    task::spawn(async move {
                        client
                            .account_output_ids([QueryParameter::Governor(bech32_address)])
                            .await
                            .map_err(From::from)
                    })
                    .await
                }
                .boxed(),
                // Get outputs where the address is in the state controller unlock condition
                async move {
                    let client = client.clone();
                    task::spawn(async move {
                        client
                            .account_output_ids([QueryParameter::StateController(bech32_address)])
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

    /// Returns output ids of foundries controlled by the provided accounts
    pub(crate) async fn get_foundry_output_ids(
        &self,
        account_output_ids: &HashSet<OutputId>,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[SYNC] get_foundry_output_ids");
        // Get account outputs, so we can then get the foundry outputs with the account addresses
        let account_outputs_with_meta = self.get_outputs(account_output_ids.iter().copied().collect()).await?;

        let bech32_hrp = self.client().get_bech32_hrp().await?;

        let mut tasks = Vec::new();

        for account_output_with_meta in account_outputs_with_meta {
            if let Output::Account(account_output) = account_output_with_meta.output() {
                let account_address = AccountAddress::from(
                    account_output.account_id_non_null(account_output_with_meta.metadata().output_id()),
                );
                let account_bech32_address = account_address.to_bech32(bech32_hrp);
                let client = self.client().clone();
                tasks.push(Box::pin(task::spawn(async move {
                    client
                        .foundry_output_ids([QueryParameter::AccountAddress(account_bech32_address)])
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
