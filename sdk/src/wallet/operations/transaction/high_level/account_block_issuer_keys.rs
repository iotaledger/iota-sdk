// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::output::{
        feature::{BlockIssuerFeature, BlockIssuerKey},
        AccountId, AccountOutput, AccountOutputBuilder,
    },
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet, WalletError},
};

/// Params `modify_account_block_issuer_key()`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyAccountBlockIssuerKey {
    pub account: AccountId,
    /// The keys that will be added.
    pub keys_to_add: Vec<BlockIssuerKey>,
    /// The keys that will be removed.
    pub keys_to_remove: Vec<BlockIssuerKey>,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    pub async fn modify_account_output_block_issuer_keys(
        &self,
        params: ModifyAccountBlockIssuerKey,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let options = options.into();
        let prepared_transaction = self
            .prepare_modify_account_output_block_issuer_keys(params, options.clone())
            .await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for [Wallet::create_account_output()].
    pub async fn prepare_modify_account_output_block_issuer_keys(
        &self,
        params: ModifyAccountBlockIssuerKey,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] prepare_modify_account_output_block_issuer_keys");
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let (_, output_data) = self
            .get_account_output(params.account)
            .await
            .ok_or(WalletError::AccountNotFound)?;

        let previous_account: &AccountOutput = output_data.output.as_account();

        if !previous_account.is_block_issuer() {
            return Err(WalletError::InvalidParameter(
                "block issuer keys can only be modified on an account with an existing block issuer feature",
            ));
        }

        let previous_block_issuer_feature = previous_account
            .features()
            .block_issuer()
            .expect("we should not support adding a new block issuer feature in this method for now");
        let mut block_issuer_keys = previous_block_issuer_feature.block_issuer_keys().to_vec();

        block_issuer_keys.extend(params.keys_to_add);
        params.keys_to_remove.iter().for_each(|key_to_remove| {
            if let Ok(index) = block_issuer_keys.binary_search(key_to_remove) {
                block_issuer_keys.remove(index);
            }
        });

        let updated_block_issuer_feature =
            BlockIssuerFeature::new(previous_block_issuer_feature.expiry_slot(), block_issuer_keys)?;

        let account_output_builder = AccountOutputBuilder::from(previous_account)
            .with_amount_or_minimum(previous_account.amount(), storage_score_params)
            .replace_feature(updated_block_issuer_feature);

        let outputs = [account_output_builder.finish_output()?];

        self.prepare_send_outputs(outputs, options).await
    }
}
