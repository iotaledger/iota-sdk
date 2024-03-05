// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{
        api::{options::TransactionOptions, PreparedTransactionData},
        secret::SecretManage,
        ClientError,
    },
    types::block::output::{AccountId, AccountOutputBuilder},
    wallet::{types::TransactionWithMetadata, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    pub async fn end_staking(
        &self,
        account_id: AccountId,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let options = options.into();
        let prepared = self.prepare_end_staking(account_id, options.clone()).await?;

        self.sign_and_submit_transaction(prepared, options).await
    }

    /// Prepares the transaction for [Wallet::end_staking()].
    pub async fn prepare_end_staking(
        &self,
        account_id: AccountId,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] prepare_end_staking");

        let account_output_data = self
            .ledger()
            .await
            .unspent_account_output(&account_id)
            .cloned()
            .ok_or_else(|| WalletError::AccountNotFound)?;

        let staking_feature = account_output_data
            .output
            .features()
            .and_then(|f| f.staking())
            .ok_or_else(|| WalletError::StakingFailed(format!("account id {account_id} is not staking")))?;

        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let slot_commitment_id = self.client().get_issuance().await?.latest_commitment.id();
        let future_bounded_epoch = protocol_parameters.future_bounded_epoch(slot_commitment_id);

        if future_bounded_epoch <= staking_feature.end_epoch() {
            let end_epoch = protocol_parameters.epoch_index_of(slot_commitment_id.slot_index())
                + (staking_feature.end_epoch() - future_bounded_epoch);
            return Err(WalletError::StakingFailed(format!(
                "account id {account_id} cannot end staking until {end_epoch}"
            )));
        }

        let features = account_output_data
            .output
            .features()
            .map(|f| f.iter().filter(|f| !f.is_staking()))
            .into_iter()
            .flatten()
            .cloned();

        let output = AccountOutputBuilder::from(account_output_data.output.as_account())
            .with_account_id(account_id)
            .with_features(features)
            .finish_output()?;

        let transaction = self.prepare_send_outputs([output], options).await?;

        Ok(transaction)
    }
}
