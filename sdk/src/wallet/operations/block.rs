// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::{SecretManage, SignBlock},
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{Error, Result, Wallet},
};

impl Wallet {
    pub(crate) async fn submit_basic_block<S: SecretManage>(
        &self,
        secret_manager: &S,
        payload: Option<Payload>,
    ) -> Result<BlockId>
    where
        crate::client::Error: From<S::Error>,
    {
        log::debug!("submit_basic_block");

        let block = self
            .client()
            // TODO https://github.com/iotaledger/iota-sdk/issues/1665 to set AccountId
            .build_basic_block(AccountId::null(), payload)
            .await?
            .sign_ed25519(secret_manager, self.bip_path().await.ok_or(Error::MissingBipPath)?)
            .await?;

        let block_id = self.client().post_block(&block).await?;

        log::debug!("submitted block {}", block_id);

        Ok(block_id)
    }
}
