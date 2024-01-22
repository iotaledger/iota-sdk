// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::{BlockSignExt, SecretManage},
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{core::SecretData, Error, Result, Wallet},
};

impl<S: SecretManage> Wallet<SecretData<S>> {
    pub(crate) async fn submit_basic_block(
        &self,
        payload: Option<Payload>,
        issuer_id: impl Into<Option<AccountId>> + Send,
    ) -> Result<BlockId> {
        log::debug!("submit_basic_block");
        let wallet_data = self.data().await;

        // If an issuer ID is provided, use it; otherwise, use the first available account or implicit account.
        let issuer_id = issuer_id
            .into()
            .or_else(|| wallet_data.first_account_id())
            .ok_or(Error::AccountNotFound)?;
        drop(wallet_data);

        let block = self
            .client()
            .build_basic_block(issuer_id, payload)
            .await?
            .sign_ed25519(&*self.secret_manager().read().await, &self.signing_options())
            .await?;

        let block_id = self.client().post_block(&block).await?;

        log::debug!("submitted block {}", block_id);

        Ok(block_id)
    }
}
