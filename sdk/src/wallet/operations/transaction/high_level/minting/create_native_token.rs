// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::AccountAddress,
        output::{
            feature::MetadataFeature, unlock_condition::ImmutableAccountAddressUnlockCondition, AccountId,
            AccountOutputBuilder, FoundryId, FoundryOutputBuilder, Output, SimpleTokenScheme, TokenId, TokenScheme,
        },
    },
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet},
};

/// Address and foundry data for `create_native_token()`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNativeTokenParams {
    /// The account id which should be used to create the foundry.
    pub account_id: Option<AccountId>,
    /// Circulating supply
    pub circulating_supply: U256,
    /// Maximum supply
    pub maximum_supply: U256,
    /// Foundry metadata
    #[serde(default)]
    pub foundry_metadata: Option<MetadataFeature>,
}

/// The result of a transaction to create a native token
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNativeTokenTransaction {
    pub token_id: TokenId,
    pub transaction: TransactionWithMetadata,
}

/// The result of preparing a transaction to create a native token
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedCreateNativeTokenTransaction {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionData,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Creates a new foundry output with minted native tokens.
    ///
    /// Calls [Wallet::prepare_transaction()](crate::wallet::Wallet::prepare_transaction) internally, the options may
    /// define the remainder value strategy or custom inputs.
    /// ```ignore
    /// let params = CreateNativeTokenParams {
    ///     account_id: None,
    ///     circulating_supply: U256::from(100),
    ///     maximum_supply: U256::from(100),
    ///     foundry_metadata: None
    /// };
    ///
    /// let tx = account.create_native_token(params, None).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn create_native_token(
        &self,
        params: CreateNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<CreateNativeTokenTransaction> {
        let options = options.into();
        let prepared = self.prepare_create_native_token(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared.transaction, options)
            .await
            .map(|transaction| CreateNativeTokenTransaction {
                token_id: prepared.token_id,
                transaction,
            })
    }

    /// Prepares the transaction for [Wallet::create_native_token()].
    pub async fn prepare_create_native_token(
        &self,
        params: CreateNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedCreateNativeTokenTransaction> {
        log::debug!("[TRANSACTION] create_native_token");
        let protocol_parameters = self.client().get_protocol_parameters().await?;
        let storage_score_params = protocol_parameters.storage_score_parameters();

        let (account_id, account_output_data) = self
            .get_account_output(params.account_id)
            .await
            .ok_or_else(|| crate::wallet::Error::MintingFailed("Missing account output".to_string()))?;

        if let Output::Account(account_output) = &account_output_data.output {
            // Create the new account output with the same features, just updated mana and foundry_counter.
            let new_account_output_builder = AccountOutputBuilder::from(account_output)
                .with_account_id(account_id)
                .with_foundry_counter(account_output.foundry_counter() + 1);

            // create foundry output with minted native tokens
            let foundry_id = FoundryId::build(
                &AccountAddress::new(account_id),
                account_output.foundry_counter() + 1,
                SimpleTokenScheme::KIND,
            );
            let token_id = TokenId::from(foundry_id);

            let outputs = [
                new_account_output_builder.finish_output()?,
                {
                    let mut foundry_builder = FoundryOutputBuilder::new_with_minimum_amount(
                        storage_score_params,
                        account_output.foundry_counter() + 1,
                        TokenScheme::Simple(SimpleTokenScheme::new(
                            params.circulating_supply,
                            0,
                            params.maximum_supply,
                        )?),
                    )
                    .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(AccountAddress::from(
                        account_id,
                    )));

                    if let Some(foundry_metadata) = params.foundry_metadata {
                        foundry_builder = foundry_builder.add_immutable_feature(foundry_metadata);
                    }

                    foundry_builder.finish_output()?
                }, // Native Tokens will be added automatically in the remainder output in try_select_inputs()
            ];

            self.prepare_transaction(outputs, options)
                .await
                .map(|transaction| PreparedCreateNativeTokenTransaction { token_id, transaction })
        } else {
            unreachable!("We checked if it's an account output before")
        }
    }
}
