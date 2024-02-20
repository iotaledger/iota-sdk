// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    },
    utils::ConvertTo,
    wallet::{
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    pub async fn send_mana(
        &self,
        mana: u64,
        address: impl ConvertTo<Bech32Address>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let options = options.into();
        let prepared_transaction = self.prepare_send_mana(mana, address, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    pub async fn prepare_send_mana(
        &self,
        mana: u64,
        address: impl ConvertTo<Bech32Address>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_send_mana");
        let options = options.into();
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let output = BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
            .with_mana(mana)
            .add_unlock_condition(AddressUnlockCondition::new(address.convert()?))
            .finish_output()?;

        self.prepare_transaction(vec![output], options).await
    }
}
