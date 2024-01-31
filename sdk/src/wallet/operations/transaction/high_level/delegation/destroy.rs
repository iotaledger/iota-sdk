// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::output::DelegationId,
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    pub async fn destroy_delegation(
        &self,
        delegation_id: DelegationId,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let prepared_transaction = self.prepare_destroy_delegation(delegation_id).await?;

        self.sign_and_submit_transaction(prepared_transaction, None, None).await
    }

    pub async fn prepare_destroy_delegation(
        &self,
        delegation_id: DelegationId,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        let delegation_output = self
            .data()
            .await
            .unspent_delegation_output(&delegation_id)
            .ok_or(crate::wallet::Error::MissingDelegation(delegation_id))?
            .clone();
        self.prepare_transaction(
            [],
            TransactionOptions {
                custom_inputs: Some(vec![delegation_output.output_id]),
                ..Default::default()
            },
        )
        .await
    }
}
