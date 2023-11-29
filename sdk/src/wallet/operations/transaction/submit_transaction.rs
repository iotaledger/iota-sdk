// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::SecretManage,
    types::block::{payload::Payload, BlockId},
    wallet::{operations::transaction::SignedTransactionPayload, Wallet},
};

impl Wallet {
    /// Submits a signed transaction in a block.
    pub(crate) async fn submit_signed_transaction<S: 'static + SecretManage>(
        &self,
        secret_manager: &S,
        payload: SignedTransactionPayload,
    ) -> crate::wallet::Result<BlockId>
    where
        crate::client::Error: From<S::Error>,
    {
        log::debug!("[TRANSACTION] submit_signed_transaction");

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting))
            .await;

        self.submit_basic_block(secret_manager, Some(Payload::from(payload)))
            .await
    }
}
