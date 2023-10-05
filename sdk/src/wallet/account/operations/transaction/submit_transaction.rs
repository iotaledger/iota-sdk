// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    types::block::{payload::Payload, BlockId},
    wallet::account::{operations::transaction::TransactionPayload, Account},
};

impl Account {
    /// Submits a payload in a block
    pub(crate) async fn submit_transaction_payload(
        &self,
        transaction_payload: TransactionPayload,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[TRANSACTION] send_payload");
        #[cfg(feature = "events")]
        let account_index = self.details().await.index;

        let block = self
            .client()
            .build_basic_block(
                todo!("issuer id"),
                todo!("issuing time"),
                None,
                Some(Payload::from(transaction_payload)),
                self.get_secret_manager().read().await.as_ref(),
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
