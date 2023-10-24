// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::SecretManage,
    types::block::{payload::Payload, BlockId},
    wallet::{operations::transaction::SignedTransactionPayload, Wallet},
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
            .build_basic_block(
                todo!("issuer id"),
                todo!("issuing time"),
                None,
                Some(Payload::from(transaction_payload)),
                &*self.get_secret_manager().read().await,
                self.bip_path().await,
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
