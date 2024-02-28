// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{
            feature::MetadataFeature, unlock_condition::AddressUnlockCondition, AccountId, AccountOutputBuilder, Output,
        },
    },
    wallet::{
        operations::transaction::TransactionOptions,
        types::{OutputData, TransactionWithMetadata},
        Wallet,
    },
};

/// Params `create_account_output()`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountParams {
    /// Bech32 encoded address which will control the account. Default will use the
    /// ed25519 wallet address
    pub address: Option<Bech32Address>,
    /// Immutable account metadata
    pub immutable_metadata: Option<MetadataFeature>,
    /// Account metadata
    pub metadata: Option<MetadataFeature>,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Creates an account output.
    /// ```ignore
    /// let params = CreateAccountParams {
    ///     address: None,
    ///     immutable_metadata: Some(b"some immutable account metadata".to_vec()),
    ///     metadata: Some(b"some account metadata".to_vec()),
    /// };
    ///
    /// let transaction = account.create_account_output(params, None).await?;
    /// println!(
    ///     "Transaction sent: {}/transaction/{}",
    ///     std::env::var("EXPLORER_URL").unwrap(),
    ///     transaction.transaction_id
    /// );
    /// ```
    pub async fn create_account_output(
        &self,
        params: Option<CreateAccountParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let options = options.into();
        let prepared_transaction = self.prepare_create_account_output(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for [Wallet::create_account_output()].
    pub async fn prepare_create_account_output(
        &self,
        params: Option<CreateAccountParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_create_account_output");
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let address = match params.as_ref().and_then(|options| options.address.as_ref()) {
            Some(bech32_address) => {
                self.client().bech32_hrp_matches(bech32_address.hrp()).await?;
                bech32_address.inner().clone()
            }
            None => self.address().await.inner().clone(),
        };

        let mut account_output_builder =
            AccountOutputBuilder::new_with_minimum_amount(storage_score_params, AccountId::null())
                .with_foundry_counter(0)
                .add_unlock_condition(AddressUnlockCondition::new(address.clone()));

        if let Some(CreateAccountParams {
            immutable_metadata,
            metadata,
            ..
        }) = params
        {
            if let Some(immutable_metadata) = immutable_metadata {
                account_output_builder = account_output_builder.add_immutable_feature(immutable_metadata);
            }
            if let Some(metadata) = metadata {
                account_output_builder = account_output_builder.add_feature(metadata);
            }
        }

        let outputs = [account_output_builder.finish_output()?];

        self.prepare_transaction(outputs, None, options).await
    }

    /// Gets an existing account output.
    pub(crate) async fn get_account_output(
        &self,
        account_id: impl Into<Option<AccountId>> + Send,
    ) -> Option<(AccountId, OutputData)> {
        log::debug!("[get_account_output]");
        let account_id = account_id.into();
        self.ledger()
            .await
            .unspent_outputs
            .values()
            .find_map(|output_data| match &output_data.output {
                Output::Account(account_output) => {
                    let output_account_id = account_output.account_id_non_null(&output_data.output_id);

                    account_id.map_or_else(
                        || Some((output_account_id, output_data.clone())),
                        |account_id| {
                            if output_account_id == account_id {
                                Some((output_account_id, output_data.clone()))
                            } else {
                                None
                            }
                        },
                    )
                }
                _ => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn create_account_params_serde() {
        let params_none_1 = CreateAccountParams {
            address: None,
            immutable_metadata: None,
            metadata: None,
        };
        let json_none = serde_json::to_string(&params_none_1).unwrap();
        let params_none_2 = serde_json::from_str(&json_none).unwrap();

        assert_eq!(params_none_1, params_none_2);

        let params_some_1 = CreateAccountParams {
            address: None,
            immutable_metadata: Some(
                MetadataFeature::new([("data".to_owned(), b"immutable_metadata".to_vec())]).unwrap(),
            ),
            metadata: Some(MetadataFeature::new([("data".to_owned(), b"metadata".to_vec())]).unwrap()),
        };
        let json_some = serde_json::to_string(&params_some_1).unwrap();
        let params_some_2 = serde_json::from_str(&json_some).unwrap();

        assert_eq!(params_some_1, params_some_2);
    }
}
