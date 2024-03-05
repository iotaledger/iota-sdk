// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::mana::ManaAllotment,
    wallet::{
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    pub async fn allot_mana(
        &self,
        allotments: impl IntoIterator<Item = impl Into<ManaAllotment>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let options = options.into();
        let prepared_transaction = self.prepare_allot_mana(allotments, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    pub async fn prepare_allot_mana(
        &self,
        allotments: impl IntoIterator<Item = impl Into<ManaAllotment>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] prepare_allot_mana");

        let mut options = options.into().unwrap_or_default();

        for allotment in allotments {
            let ManaAllotment { account_id, mana } = allotment.into();

            *options.mana_allotments.entry(account_id).or_default() += mana;
        }

        self.prepare_send_outputs([], options).await
    }
}
