// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use futures::FutureExt;

use crate::{
    client::{node_api::indexer::query_parameters::BasicOutputsQueryParameters, secret::SecretManage},
    types::{
        api::plugins::indexer::OutputIdsResponse,
        block::{address::Bech32Address, output::OutputId, ConvertTo},
    },
    wallet::Account,
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Returns output ids of basic outputs that have only the address unlock condition
    pub(crate) async fn get_basic_output_ids_with_address_unlock_condition_only(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> crate::client::Result<Vec<OutputId>> {
        let bech32_address = bech32_address.convert()?;
        // Only request basic outputs with `AddressUnlockCondition` only
        Ok(self
            .client()
            .basic_output_ids(BasicOutputsQueryParameters {
                address: Some(bech32_address),
                has_expiration: Some(false),
                has_timelock: Some(false),
                has_storage_deposit_return: Some(false),
                ..Default::default()
            })
            .await?
            .items)
    }

    /// Returns output ids of basic outputs that have the address in the `AddressUnlockCondition`,
    /// `ExpirationUnlockCondition` or `StorageDepositReturnUnlockCondition`
    pub(crate) async fn get_basic_output_ids_with_any_unlock_condition(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        let bech32_address = bech32_address.convert()?;
        #[cfg(target_family = "wasm")]
        {
            let mut output_ids = Vec::new();
            output_ids.extend(
                self.client()
                    .basic_output_ids(BasicOutputsQueryParameters {
                        unlockable_by_address: Some(bech32_address.clone()),
                        ..Default::default()
                    })
                    .await?
                    .items,
            );
            output_ids.extend(
                self.client()
                    .basic_output_ids(BasicOutputsQueryParameters {
                        storage_deposit_return_address: Some(bech32_address.clone()),
                        ..Default::default()
                    })
                    .await?
                    .items,
            );

            Ok(output_ids)
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let client = self.client();
            let tasks = [
                // Get basic outputs
                async {
                    let bech32_address = bech32_address.clone();
                    let client = client.clone();
                    tokio::spawn(async move {
                        client
                            .basic_output_ids(BasicOutputsQueryParameters {
                                unlockable_by_address: Some(bech32_address),
                                ..Default::default()
                            })
                            .await
                            .map_err(From::from)
                    })
                    .await
                }
                .boxed(),
                // Get outputs where the address is in the storage deposit return unlock condition
                async {
                    let bech32_address = bech32_address.clone();
                    let client = client.clone();
                    tokio::spawn(async move {
                        client
                            .basic_output_ids(BasicOutputsQueryParameters {
                                storage_deposit_return_address: Some(bech32_address),
                                ..Default::default()
                            })
                            .await
                            .map_err(From::from)
                    })
                    .await
                }
                .boxed(),
            ];

            // Get all results
            let mut output_ids = HashSet::new();
            let results: Vec<crate::wallet::Result<OutputIdsResponse>> = futures::future::try_join_all(tasks).await?;

            for res in results {
                let found_output_ids = res?;
                output_ids.extend(found_output_ids.items);
            }

            Ok(output_ids.into_iter().collect())
        }
    }
}
