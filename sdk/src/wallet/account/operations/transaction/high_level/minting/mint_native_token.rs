// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::api::{PreparedTransactionData, PreparedTransactionDataDto},
    types::block::{
        address::AliasAddress,
        output::{
            feature::MetadataFeature, unlock_condition::ImmutableAliasAddressUnlockCondition, AliasId,
            AliasOutputBuilder, FoundryId, FoundryOutputBuilder, Output, SimpleTokenScheme, TokenId, TokenScheme,
        },
        Error,
    },
    wallet::account::{
        types::{Transaction, TransactionDto},
        Account, TransactionOptions,
    },
};

/// Address and foundry data for `mint_native_token()`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintNativeTokenParams {
    /// The alias id which should be used to create the foundry.
    pub alias_id: Option<AliasId>,
    /// Circulating supply
    pub circulating_supply: U256,
    /// Maximum supply
    pub maximum_supply: U256,
    /// Foundry metadata
    pub foundry_metadata: Option<Vec<u8>>,
}

/// Dto for MintNativeTokenParams
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintNativeTokenParamsDto {
    /// The alias id which should be used to create the foundry.
    pub alias_id: Option<AliasId>,
    /// Circulating supply
    pub circulating_supply: U256,
    /// Maximum supply
    pub maximum_supply: U256,
    /// Foundry metadata, hex encoded bytes
    pub foundry_metadata: Option<String>,
}

impl TryFrom<MintNativeTokenParamsDto> for MintNativeTokenParams {
    type Error = crate::wallet::Error;

    fn try_from(value: MintNativeTokenParamsDto) -> crate::wallet::Result<Self> {
        Ok(Self {
            alias_id: value.alias_id,
            circulating_supply: value.circulating_supply,
            maximum_supply: value.maximum_supply,
            foundry_metadata: value
                .foundry_metadata
                .map(|metadata| prefix_hex::decode(metadata).map_err(|_| Error::InvalidField("foundry_metadata")))
                .transpose()?,
        })
    }
}

/// The result of a minting native token transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MintTokenTransaction {
    pub token_id: TokenId,
    pub transaction: Transaction,
}

/// Dto for MintTokenTransaction
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MintTokenTransactionDto {
    pub token_id: TokenId,
    pub transaction: TransactionDto,
}

impl From<&MintTokenTransaction> for MintTokenTransactionDto {
    fn from(value: &MintTokenTransaction) -> Self {
        Self {
            token_id: value.token_id,
            transaction: TransactionDto::from(&value.transaction),
        }
    }
}

/// The result of preparing a minting native token transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedMintTokenTransaction {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionData,
}

/// Dto for MintTokenTransaction
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedMintTokenTransactionDto {
    pub token_id: TokenId,
    pub transaction: PreparedTransactionDataDto,
}

impl From<&PreparedMintTokenTransaction> for PreparedMintTokenTransactionDto {
    fn from(value: &PreparedMintTokenTransaction) -> Self {
        Self {
            token_id: value.token_id,
            transaction: PreparedTransactionDataDto::from(&value.transaction),
        }
    }
}

impl Account {
    /// Function to create a new foundry output with minted native tokens.
    /// Calls [Account.send()](crate::account::Account.send) internally, the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let params = MintNativeTokenParams {
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
    pub async fn mint_native_token(
        &self,
        params: MintNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<MintTokenTransaction> {
        let prepared = self.prepare_mint_native_token(params, options).await?;
        self.sign_and_submit_transaction(prepared.transaction)
            .await
            .map(|transaction| MintTokenTransaction {
                token_id: prepared.token_id,
                transaction,
            })
    }

    /// Function to prepare the transaction for
    /// [Account.mint_native_token()](crate::account::Account.mint_native_token)
    pub async fn prepare_mint_native_token(
        &self,
        params: MintNativeTokenParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedMintTokenTransaction> {
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
                .map(|transaction| PreparedMintTokenTransaction { token_id, transaction })
        } else {
            unreachable!("We checked if it's an alias output before")
        }
    }
}
