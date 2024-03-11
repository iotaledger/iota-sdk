// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use crate::{
    client::{secret::SecretManage, ClientError},
    types::block::output::{FoundryId, Output},
    wallet::{task, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S> {
    pub(crate) async fn request_and_store_foundry_outputs(
        &self,
        foundry_ids: HashSet<FoundryId>,
    ) -> Result<(), WalletError> {
        log::debug!("[SYNC] request_and_store_foundry_outputs");

        let mut foundries = self.ledger().await.native_token_foundries.clone();
        let results =
            futures::future::try_join_all(foundry_ids.into_iter().filter(|f| !foundries.contains_key(f)).map(
                |foundry_id| {
                    let client = self.client().clone();
                    async move {
                        task::spawn(async move {
                            match client.foundry_output_id(foundry_id).await {
                                Ok(output_id) => Ok(Some(client.get_output(&output_id).await?)),
                                Err(ClientError::NoOutput(_)) => Ok(None),
                                Err(e) => Err(WalletError::Client(e)),
                            }
                        })
                        .await?
                    }
                },
            ))
            .await?;

        // Update account with new foundries.
        for foundry in results.into_iter().flatten() {
            if let Output::Foundry(foundry) = foundry.output {
                foundries.insert(foundry.id(), foundry);
            }
        }

        let mut wallet_ledger = self.ledger_mut().await;
        wallet_ledger.native_token_foundries = foundries;

        Ok(())
    }
}
