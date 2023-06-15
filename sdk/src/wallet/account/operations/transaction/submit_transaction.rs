// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::SecretManage,
    types::block::{payload::Payload, BlockId},
    wallet::account::{operations::transaction::TransactionPayload, Account},
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Submits a payload in a block
    pub(crate) async fn submit_transaction_payload(
        &self,
        transaction_payload: TransactionPayload,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[TRANSACTION] send_payload");
        #[cfg(feature = "events")]
        let account_index = self.details().await.index;

        let local_pow = self.client().get_local_pow().await;
        if local_pow {
            log::debug!("[TRANSACTION] doing local pow");
            #[cfg(feature = "events")]
            self.emit(
                account_index,
                WalletEvent::TransactionProgress(TransactionProgressEvent::PerformingPow),
            )
            .await;
        }
        let block = self
            .client()
            .finish_block_builder(None, Some(Payload::from(transaction_payload)))
            .await?;

        #[cfg(feature = "events")]
        self.emit(
            account_index,
            WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting),
        )
        .await;
        let block_id = self.client().post_block(&block).await?;
        log::debug!("[TRANSACTION] submitted block {}", block_id);
        Ok(block_id)
    }
}
