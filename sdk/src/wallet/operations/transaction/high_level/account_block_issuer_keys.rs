// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{
        api::{
            transaction_builder::{transition::AccountChange, Transitions},
            PreparedTransactionData,
        },
        secret::SecretManage,
        ClientError,
    },
    types::block::output::{feature::BlockIssuerKey, AccountId},
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet, WalletError},
};

/// Params for `modify_account_output_block_issuer_keys()`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyAccountBlockIssuerKey {
    pub account_id: AccountId,
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

        let change = AccountChange::ModifyBlockIssuerKeys {
            keys_to_add: params.keys_to_add,
            keys_to_remove: params.keys_to_remove,
        };

        let account_id = params.account_id;

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

        self.prepare_send_outputs(None, options).await
    }
}
