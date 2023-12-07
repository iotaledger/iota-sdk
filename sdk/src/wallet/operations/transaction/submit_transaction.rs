// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::SecretManage,
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{core::SecretData, operations::transaction::SignedTransactionPayload, Wallet},
};

impl<S: SecretManage> Wallet<SecretData<S>> {
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

        self.submit_basic_block(Some(Payload::from(payload)), issuer_id).await
    }
}
