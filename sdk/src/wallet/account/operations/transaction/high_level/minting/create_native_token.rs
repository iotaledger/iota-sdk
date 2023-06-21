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
    #[serde(with = "crate::utils::serde::option_prefix_hex_vec")]
    pub foundry_metadata: Option<Vec<u8>>,
}

/// The result of a minting native token transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTokenTransaction {
    pub token_id: TokenId,
    pub transaction: Transaction,
}

/// Dto for MintTokenTransaction
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTokenTransactionDto {
    pub token_id: TokenId,
    pub transaction: TransactionDto,
}

impl From<&NativeTokenTransaction> for NativeTokenTransactionDto {
    fn from(value: &NativeTokenTransaction) -> Self {
        Self {
            token_id: value.token_id,
            transaction: TransactionDto::from(&value.transaction),
        }
    }
}

/// The result of preparing a minting native token transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedNativeTokenTransaction {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionData,
}

/// Dto for CreateNativeTokenTransaction
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedNativeTokenTransactionDto {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionDataDto,
}

impl From<&PreparedNativeTokenTransaction> for PreparedNativeTokenTransactionDto {
    fn from(value: &PreparedNativeTokenTransaction) -> Self {
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
    /// Function to create a new foundry output with minted native tokens.
    /// Calls [Account.send()](crate::account::Account.send) internally, the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let params = CreateNativeTokenParams {
    ///     alias_id: None,
    ///     circulating_supply: U256::from(100),
    ///     maximum_supply: U256::from(100),
    ///     foundry_metadata: None
    /// };
    ///
    /// let tx = account.mint_native_token(params, None,).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn create_native_token(
        &self,
        params: CreateNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<NativeTokenTransaction> {
        let options = options.into();
        let prepared = self.prepare_create_native_token(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared.transaction, options)
            .await
            .map(|transaction| NativeTokenTransaction {
                token_id: prepared.token_id,
                transaction,
            })
    }

    /// Function to prepare the transaction for
    /// [Account.create_native_token()](crate::account::Account.create_native_token)
    pub async fn prepare_create_native_token(
        &self,
        params: CreateNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedNativeTokenTransaction> {
        log::debug!("[TRANSACTION] mint_native_token");
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
                            U256::from(0u8),
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
                .map(|transaction| PreparedNativeTokenTransaction { token_id, transaction })
        } else {
            unreachable!("We checked if it's an alias output before")
        }
    }
}
