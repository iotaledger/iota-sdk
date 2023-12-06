// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::{SecretManage, SignBlock},
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{Error, Result, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    pub(crate) async fn submit_basic_block(
        &self,
        payload: Option<Payload>,
        issuer_id: impl Into<Option<AccountId>> + Send,
    ) -> Result<BlockId> {
        log::debug!("submit_basic_block");

        // TODO https://github.com/iotaledger/iota-sdk/issues/1741

        let issuer_id = match issuer_id.into() {
            Some(issuer_id) => Some(issuer_id),
            None => self
                .data()
                .await
                .accounts()
                .next()
                .map(|o| *o.output.as_account().account_id()),
        }
        .unwrap();

        let block = self
            .client()
            .build_basic_block(issuer_id, payload)
            .await?
            .sign_ed25519(
                &*self.get_secret_manager().read().await,
                self.bip_path().await.ok_or(Error::MissingBipPath)?,
            )
            .await?;

        let block_id = self.client().post_block(&block).await?;

        log::debug!("submitted block {}", block_id);

        Ok(block_id)
    }
}
