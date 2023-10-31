// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use crate::{
    client::node_api::indexer::query_parameters::{AccountOutputQueryParameters, FoundryOutputQueryParameters},
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

impl Account {
    /// Returns output ids of account outputs
    pub(crate) async fn get_account_and_foundry_output_ids(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[SYNC] get_alias_and_foundry_output_ids");
        let bech32_address = bech32_address.convert()?;

        let mut output_ids = self
            .client()
            .account_output_ids(AccountOutputQueryParameters::new().unlockable_by_address(bech32_address))
            .await?
            .items;

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
        account_output_ids: &[OutputId],
    ) -> crate::wallet::Result<Vec<OutputId>> {
        log::debug!("[SYNC] get_foundry_output_ids");
        // Get account outputs, so we can then get the foundry outputs with the account addresses
        let account_outputs_with_meta = self.get_outputs(account_output_ids.to_vec()).await?;

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
                        .foundry_output_ids(FoundryOutputQueryParameters::new().account_address(account_bech32_address))
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
