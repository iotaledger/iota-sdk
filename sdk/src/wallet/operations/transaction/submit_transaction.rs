// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::{SecretManage, SignBlock},
    types::block::{payload::Payload, BlockId, IssuerId},
    wallet::{operations::transaction::SignedTransactionPayload, Error, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Submits a payload in a block
    pub(crate) async fn submit_transaction_payload(
        &self,
        transaction_payload: SignedTransactionPayload,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[TRANSACTION] send_payload");

        let block = self
            .client()
            // TODO https://github.com/iotaledger/iota-sdk/issues/1665 to set IssuerId
            .build_basic_block(IssuerId::null(), Some(Payload::from(transaction_payload)))
            .await?
            .sign_ed25519(
                &*self.get_secret_manager().read().await,
                self.bip_path().await.ok_or(Error::MissingBipPath)?,
            )
            .await?;

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting))
            .await;
        let block_id = self.client().post_block(&block).await?;

        log::debug!("[TRANSACTION] submitted block {}", block_id);

        Ok(block_id)
    }
}
