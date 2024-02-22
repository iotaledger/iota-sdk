// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{AddressUnlockCondition, StorageDepositReturnUnlockCondition},
            BasicOutputBuilder,
        },
    },
    utils::ConvertTo,
    wallet::{
        operations::transaction::{prepare_output::ReturnStrategy, TransactionOptions, TransactionWithMetadata},
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
        return_strategy: impl Into<Option<ReturnStrategy>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let options = options.into();
        let prepared_transaction = self
            .prepare_send_mana(mana, address, return_strategy, options.clone())
            .await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    pub async fn prepare_send_mana(
        &self,
        mana: u64,
        address: impl ConvertTo<Bech32Address>,
        return_strategy: impl Into<Option<ReturnStrategy>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_send_mana");
        let return_strategy = return_strategy.into().unwrap_or_default();
        let options = options.into();
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let mut output_builder = BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
            .with_mana(mana)
            .add_unlock_condition(AddressUnlockCondition::new(address.convert()?));

        if let ReturnStrategy::Return = return_strategy {
            output_builder = output_builder.add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                self.address().await.inner().clone(),
                1,
            )?);
            let return_amount = output_builder.clone().finish()?.amount();
            output_builder = output_builder.replace_unlock_condition(StorageDepositReturnUnlockCondition::new(
                self.address().await.inner().clone(),
                return_amount,
            )?);
        }

        let output = output_builder.finish_output()?;

        self.prepare_transaction(vec![output], options).await
    }
}
