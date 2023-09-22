// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{
            feature::MetadataFeature,
            unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
            AccountId, AccountOutputBuilder, Output,
        },
    },
    utils::serde::option_prefix_hex_bytes,
    wallet::{
        account::{types::Transaction, OutputData, TransactionOptions},
        Wallet,
    },
};

/// Params `create_account_output()`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountParams {
    /// Bech32 encoded address which will control the account. Default will use the first
    /// ed25519 address of the wallet account
    pub address: Option<Bech32Address>,
    /// Immutable account metadata
    #[serde(default, with = "option_prefix_hex_bytes")]
    pub immutable_metadata: Option<Vec<u8>>,
    /// Account metadata
    #[serde(default, with = "option_prefix_hex_bytes")]
    pub metadata: Option<Vec<u8>>,
    /// Account state metadata
    #[serde(default, with = "option_prefix_hex_bytes")]
    pub state_metadata: Option<Vec<u8>>,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Creates an account output.
    /// ```ignore
    /// let params = CreateAccountParams {
    ///     address: None,
    ///     immutable_metadata: Some(b"some immutable account metadata".to_vec()),
    ///     metadata: Some(b"some account metadata".to_vec()),
    ///     state_metadata: Some(b"some account state metadata".to_vec()),
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
    ) -> crate::wallet::Result<Transaction> {
        let options = options.into();
        let prepared_transaction = self.prepare_create_account_output(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for
    /// [Account::create_account_output()](crate::wallet::Account::create_account_output).
    pub async fn prepare_create_account_output(
        &self,
        params: Option<CreateAccountParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_create_account_output");
        let rent_structure = self.client().get_rent_structure().await?;
        let token_supply = self.client().get_token_supply().await?;

        let controller_address = match params.as_ref().and_then(|options| options.address.as_ref()) {
            Some(bech32_address) => {
                self.client().bech32_hrp_matches(bech32_address.hrp()).await?;
                *bech32_address.inner()
            }
            None => self.address().await,
        };

        let mut account_output_builder =
            AccountOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AccountId::null())
                .with_state_index(0)
                .with_foundry_counter(0)
                .add_unlock_condition(StateControllerAddressUnlockCondition::new(controller_address))
                .add_unlock_condition(GovernorAddressUnlockCondition::new(controller_address));
        if let Some(CreateAccountParams {
            immutable_metadata,
            metadata,
            state_metadata,
            ..
        }) = params
        {
            if let Some(immutable_metadata) = immutable_metadata {
                account_output_builder =
                    account_output_builder.add_immutable_feature(MetadataFeature::new(immutable_metadata)?);
            }
            if let Some(metadata) = metadata {
                account_output_builder = account_output_builder.add_feature(MetadataFeature::new(metadata)?);
            }
            if let Some(state_metadata) = state_metadata {
                account_output_builder = account_output_builder.with_state_metadata(state_metadata);
            }
        }

        let outputs = [account_output_builder.finish_output(token_supply)?];

        self.prepare_transaction(outputs, options).await
    }

    /// Gets an existing account output.
    pub(crate) async fn get_account_output(&self, account_id: Option<AccountId>) -> Option<(AccountId, OutputData)> {
        log::debug!("[get_account_output]");
        self.data()
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
    use super::*;

    #[test]
    fn create_account_params_serde() {
        let params_none_1 = CreateAccountParams {
            address: None,
            immutable_metadata: None,
            metadata: None,
            state_metadata: None,
        };
        let json_none = serde_json::to_string(&params_none_1).unwrap();
        let params_none_2 = serde_json::from_str(&json_none).unwrap();

        assert_eq!(params_none_1, params_none_2);

        let params_some_1 = CreateAccountParams {
            address: None,
            immutable_metadata: Some(b"immutable_metadata".to_vec()),
            metadata: Some(b"metadata".to_vec()),
            state_metadata: Some(b"state_metadata".to_vec()),
        };
        let json_some = serde_json::to_string(&params_some_1).unwrap();
        let params_some_2 = serde_json::from_str(&json_some).unwrap();

        assert_eq!(params_some_1, params_some_2);
    }
}
