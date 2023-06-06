// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use crate::{
    types::block::output::{FoundryId, Output},
    wallet::{task, Account},
};

impl Account {
    pub(crate) async fn request_and_store_foundry_outputs(
        &self,
        foundry_ids: HashSet<FoundryId>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] request_and_store_foundry_outputs");

        let mut foundries = self.details().await.native_token_foundries().clone();
        let results =
            futures::future::try_join_all(foundry_ids.into_iter().filter(|f| !foundries.contains_key(f)).map(
                |foundry_id| {
                    let client = self.client().clone();
                    async move {
                        task::spawn(async move {
                            match client.foundry_output_id(foundry_id).await {
                                Ok(output_id) => Ok(Some(client.get_output(&output_id).await?)),
                                Err(crate::client::Error::NoOutput(_)) => Ok(None),
                                Err(e) => Err(crate::wallet::Error::Client(e.into())),
                            }
                        })
                        .await?
                    }
                },
            ))
            .await?;

        // Update account with new foundries.
        for foundry_output_with_metadata in results.into_iter().flatten() {
            if let Output::Foundry(foundry) = foundry_output_with_metadata.output() {
                foundries.insert(foundry.id(), foundry.to_owned());
            }
        }

        let mut account_details = self.details_mut().await;
        account_details.native_token_foundries = foundries;

        Ok(())
    }
}
