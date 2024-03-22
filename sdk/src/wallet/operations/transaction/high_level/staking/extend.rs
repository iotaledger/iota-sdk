// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{
        api::{
            options::TransactionOptions,
            transaction_builder::transition::{AccountChange, Transitions},
            PreparedTransactionData,
        },
        secret::SecretManage,
        ClientError,
    },
    types::block::output::AccountId,
    wallet::{types::TransactionWithMetadata, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    pub async fn extend_staking(
        &self,
        account_id: AccountId,
        additional_epochs: u32,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let options = options.into();
        let prepared = self
            .prepare_extend_staking(account_id, additional_epochs, options.clone())
            .await?;

        self.sign_and_submit_transaction(prepared, options).await
    }

    /// Prepares the transaction for [Wallet::extend_staking()].
    pub async fn prepare_extend_staking(
        &self,
        account_id: AccountId,
        additional_epochs: u32,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] prepare_extend_staking");

        let change = AccountChange::ExtendStaking { additional_epochs };

        let mut options = options.into();
        if let Some(options) = options.as_mut() {
            if let Some(transitions) = options.transitions.take() {
                options.transitions = Some(transitions.add_account(account_id, change));
            }
        } else {
            options.replace(TransactionOptions {
                transitions: Some(Transitions::new().add_account(account_id, change)),
                ..Default::default()
            });
        }

        let transaction = self.prepare_send_outputs(None, options).await?;

        Ok(transaction)
    }
}
