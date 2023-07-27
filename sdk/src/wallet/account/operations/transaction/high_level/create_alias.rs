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
            AliasId, AliasOutputBuilder, Output,
        },
    },
    wallet::account::{types::Transaction, Account, OutputData, TransactionOptions},
};

/// Params `create_alias_output()`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAliasParams {
    /// Bech32 encoded address which will control the alias. Default will use the first
    /// address of the account
    pub address: Option<Bech32Address>,
    /// Immutable alias metadata
    #[serde(default, with = "crate::utils::serde::option_prefix_hex_bytes")]
    pub immutable_metadata: Option<Vec<u8>>,
    /// Alias metadata
    #[serde(default, with = "crate::utils::serde::option_prefix_hex_bytes")]
    pub metadata: Option<Vec<u8>>,
    /// Alias state metadata
    #[serde(default, with = "crate::utils::serde::option_prefix_hex_bytes")]
    pub state_metadata: Option<Vec<u8>>,
}

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Creates an alias output.
    /// ```ignore
    /// let params = CreateAliasParams {
    ///     address: None,
    ///     immutable_metadata: Some(b"some immutable alias metadata".to_vec()),
    ///     metadata: Some(b"some alias metadata".to_vec()),
    ///     state_metadata: Some(b"some alias state metadata".to_vec()),
    /// };
    ///
    /// let transaction = account.create_alias_output(params, None).await?;
    /// println!(
    ///     "Transaction sent: {}/transaction/{}",
    ///     std::env::var("EXPLORER_URL").unwrap(),
    ///     transaction.transaction_id
    /// );
    /// ```
    pub async fn create_alias_output(
        &self,
        params: Option<CreateAliasParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let options = options.into();
        let prepared_transaction = self.prepare_create_alias_output(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for
    /// [Account::create_alias_output()](crate::wallet::Account::create_alias_output).
    pub async fn prepare_create_alias_output(
        &self,
        params: Option<CreateAliasParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_create_alias_output");
        let rent_structure = self.client().get_rent_structure().await?;
        let token_supply = self.client().get_token_supply().await?;

        let controller_address = match params.as_ref().and_then(|options| options.address.as_ref()) {
            Some(bech32_address) => {
                self.client().bech32_hrp_matches(bech32_address.hrp()).await?;
                *bech32_address.inner()
            }
            None => {
                self.public_addresses()
                    .await
                    .first()
                    .expect("first address is generated during account creation")
                    .address
                    .inner
            }
        };

        let mut alias_output_builder =
            AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
                .with_state_index(0)
                .with_foundry_counter(0)
                .add_unlock_condition(StateControllerAddressUnlockCondition::new(controller_address))
                .add_unlock_condition(GovernorAddressUnlockCondition::new(controller_address));
        if let Some(CreateAliasParams {
            immutable_metadata,
            metadata,
            state_metadata,
            ..
        }) = params
        {
            if let Some(immutable_metadata) = immutable_metadata {
                alias_output_builder =
                    alias_output_builder.add_immutable_feature(MetadataFeature::new(immutable_metadata)?);
            }
            if let Some(metadata) = metadata {
                alias_output_builder = alias_output_builder.add_feature(MetadataFeature::new(metadata)?);
            }
            if let Some(state_metadata) = state_metadata {
                alias_output_builder = alias_output_builder.with_state_metadata(state_metadata);
            }
        }

        let outputs = [alias_output_builder.finish_output(token_supply)?];

        self.prepare_transaction(outputs, options).await
    }

    /// Gets an existing alias output.
    pub(crate) async fn get_alias_output(&self, alias_id: Option<AliasId>) -> Option<(AliasId, OutputData)> {
        log::debug!("[get_alias_output]");
        self.details()
            .await
            .unspent_outputs()
            .values()
            .find_map(|output_data| match &output_data.output {
                Output::Alias(alias_output) => {
                    let output_alias_id = alias_output.alias_id_non_null(&output_data.output_id);

                    alias_id.map_or_else(
                        || Some((output_alias_id, output_data.clone())),
                        |alias_id| {
                            if output_alias_id == alias_id {
                                Some((output_alias_id, output_data.clone()))
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
    fn create_alias_params_serde() {
        let params_none_1 = CreateAliasParams {
            address: None,
            immutable_metadata: None,
            metadata: None,
            state_metadata: None,
        };
        let json_none = serde_json::to_string(&params_none_1).unwrap();
        let params_none_2 = serde_json::from_str(&json_none).unwrap();

        assert_eq!(params_none_1, params_none_2);

        let params_some_1 = CreateAliasParams {
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
