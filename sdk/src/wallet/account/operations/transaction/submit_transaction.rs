// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::secret::{SecretManage, SignBlock},
    types::block::{payload::Payload, BlockId},
    wallet::account::{operations::transaction::SignedTransactionPayload, Account},
};

impl<S: 'static + SecretManage> Account<S>
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
        #[cfg(feature = "events")]
        let account_index = self.details().await.index;

        let block = self
            .client()
            .build_basic_block(todo!("issuer id"), Some(Payload::from(transaction_payload)))
            .await?
            .sign_ed25519(
                &*self.get_secret_manager().read().await,
                Bip44::new(self.wallet.coin_type()),
            )
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
