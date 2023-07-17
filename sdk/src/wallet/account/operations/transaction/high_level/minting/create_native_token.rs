// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto},
        secret::SecretManage,
    },
    types::block::{
        address::AliasAddress,
        output::{
            feature::MetadataFeature, unlock_condition::ImmutableAliasAddressUnlockCondition, AliasId,
            AliasOutputBuilder, FoundryId, FoundryOutputBuilder, Output, SimpleTokenScheme, TokenId, TokenScheme,
        },
    },
    wallet::account::{
        types::{Transaction, TransactionDto},
        Account, TransactionOptions,
    },
};

/// Address and foundry data for `create_native_token()`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNativeTokenParams {
    /// The alias id which should be used to create the foundry.
    pub alias_id: Option<AliasId>,
    /// Circulating supply
    pub circulating_supply: U256,
    /// Maximum supply
    pub maximum_supply: U256,
    /// Foundry metadata
    #[serde(default, with = "crate::utils::serde::option_prefix_hex_vec")]
    pub foundry_metadata: Option<Vec<u8>>,
}

/// The result of a transaction to create a native token
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNativeTokenTransaction {
    pub token_id: TokenId,
    pub transaction: Transaction,
}

/// Dto for NativeTokenTransaction
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNativeTokenTransactionDto {
    pub token_id: TokenId,
    pub transaction: TransactionDto,
}

impl From<&CreateNativeTokenTransaction> for CreateNativeTokenTransactionDto {
    fn from(value: &CreateNativeTokenTransaction) -> Self {
        Self {
            token_id: value.token_id,
            transaction: TransactionDto::from(&value.transaction),
        }
    }
}

/// The result of preparing a transaction to create a native token
#[derive(Debug)]
pub struct PreparedCreateNativeTokenTransaction {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionData,
}

/// Dto for PreparedNativeTokenTransaction
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedCreateNativeTokenTransactionDto {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionDataDto,
}

impl From<&PreparedCreateNativeTokenTransaction> for PreparedCreateNativeTokenTransactionDto {
    fn from(value: &PreparedCreateNativeTokenTransaction) -> Self {
        Self {
            token_id: value.token_id,
            transaction: PreparedTransactionDataDto::from(&value.transaction),
        }
    }
}

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Creates a new foundry output with minted native tokens.
    ///
    /// Calls [Account::send_outputs()](crate::wallet::Account::send_outputs) internally, the options may define the
    /// remainder value strategy or custom inputs. Note that addresses need to be bech32-encoded.
    /// ```ignore
    /// let params = CreateNativeTokenParams {
    ///     alias_id: None,
    ///     circulating_supply: U256::from(100),
    ///     maximum_supply: U256::from(100),
    ///     foundry_metadata: None
    /// };
    ///
    /// let tx = account.create_native_token(params, None,).await?;
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

    /// Prepares the transaction for
    /// [Account::create_native_token()](crate::wallet::Account::create_native_token).
    pub async fn prepare_create_native_token(
        &self,
        params: CreateNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedCreateNativeTokenTransaction> {
        log::debug!("[TRANSACTION] create_native_token");
        let rent_structure = self.client().get_rent_structure().await?;
        let token_supply = self.client().get_token_supply().await?;

        let (alias_id, alias_output) = self
            .get_alias_output(params.alias_id)
            .await
            .ok_or_else(|| crate::wallet::Error::MintingFailed("Missing alias output".to_string()))?;

        if let Output::Alias(alias_output) = &alias_output.output {
            // Create the new alias output with the same feature blocks, just updated state_index and foundry_counter
            let new_alias_output_builder = AliasOutputBuilder::from(alias_output)
                .with_alias_id(alias_id)
                .with_state_index(alias_output.state_index() + 1)
                .with_foundry_counter(alias_output.foundry_counter() + 1);

            // create foundry output with minted native tokens
            let foundry_id = FoundryId::build(
                &AliasAddress::new(alias_id),
                alias_output.foundry_counter() + 1,
                SimpleTokenScheme::KIND,
            );
            let token_id = TokenId::from(foundry_id);

            let outputs = [
                new_alias_output_builder.finish_output(token_supply)?,
                {
                    let mut foundry_builder = FoundryOutputBuilder::new_with_minimum_storage_deposit(
                        rent_structure,
                        alias_output.foundry_counter() + 1,
                        TokenScheme::Simple(SimpleTokenScheme::new(
                            params.circulating_supply,
                            0,
                            params.maximum_supply,
                        )?),
                    )
                    .add_unlock_condition(ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)));

                    if let Some(foundry_metadata) = params.foundry_metadata {
                        foundry_builder = foundry_builder.add_immutable_feature(MetadataFeature::new(foundry_metadata)?)
                    }

                    foundry_builder.finish_output(token_supply)?
                }, // Native Tokens will be added automatically in the remainder output in try_select_inputs()
            ];

            self.prepare_transaction(outputs, options)
                .await
                .map(|transaction| PreparedCreateNativeTokenTransaction { token_id, transaction })
        } else {
            unreachable!("We checked if it's an alias output before")
        }
    }
}
