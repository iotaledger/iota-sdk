// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManage, ClientError},
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{operations::transaction::SignedTransactionPayload, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Submits a signed transaction in a block.
    pub(crate) async fn submit_signed_transaction(
        &self,
        payload: SignedTransactionPayload,
        issuer_id: impl Into<Option<AccountId>> + Send,
    ) -> Result<BlockId, WalletError> {
        log::debug!("[TRANSACTION] submit_signed_transaction");

        self.submit_basic_block(Some(Payload::from(payload)), issuer_id, true)
            .await
    }
}
