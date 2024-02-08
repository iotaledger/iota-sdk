// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::SecretManage,
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{operations::transaction::SignedTransactionPayload, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Submits a signed transaction in a block.
    pub(crate) async fn submit_signed_transaction(
        &self,
        payload: SignedTransactionPayload,
        issuer_id: impl Into<Option<AccountId>> + Send,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[TRANSACTION] submit_signed_transaction");

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting))
            .await;

        self.submit_basic_block(Some(Payload::from(payload)), issuer_id, true)
            .await
    }
}
