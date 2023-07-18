// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::output::{AliasOutputBuilder, FoundryOutputBuilder, Output, SimpleTokenScheme, TokenId, TokenScheme},
    wallet::{
        account::{types::Transaction, Account, TransactionOptions},
        Error,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Mints additional native tokens.
    ///
    /// The max supply must not be reached yet. The foundry needs to be
    /// controlled by this account. Address needs to be Bech32 encoded. This will not change the max supply.
    /// ```ignore
    /// let tx = account.mint_native_token(
    ///             TokenId::from_str("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")?,
    ///             100,
    ///             None
    ///         ).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn mint_native_token(
        &self,
        token_id: TokenId,
        mint_amount: impl Into<U256> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let options = options.into();
        let prepared = self
            .prepare_mint_native_token(token_id, mint_amount, options.clone())
            .await?;
        let transaction = self.sign_and_submit_transaction(prepared, options).await?;

        Ok(transaction)
    }

    /// Prepares the transaction for
    /// [Account::mint_native_token()](crate::wallet::Account::mint_native_token).
    pub async fn prepare_mint_native_token(
        &self,
        token_id: TokenId,
        mint_amount: impl Into<U256> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] mint_native_token");

        let mint_amount = mint_amount.into();
        let account_details = self.details().await;
        let token_supply = self.client().get_token_supply().await?;
        let existing_foundry_output = account_details.unspent_outputs().values().find(|output_data| {
            if let Output::Foundry(output) = &output_data.output {
                TokenId::new(*output.id()) == token_id
            } else {
                false
            }
        });

        let existing_foundry_output = existing_foundry_output
            .ok_or_else(|| Error::MintingFailed(format!("foundry output {token_id} is not available")))?
            .clone();

        let existing_alias_output = if let Output::Foundry(foundry_output) = &existing_foundry_output.output {
            let TokenScheme::Simple(token_scheme) = foundry_output.token_scheme();
            // Check if we can mint the provided amount without exceeding the maximum_supply
            if token_scheme.maximum_supply() - token_scheme.circulating_supply() < mint_amount {
                return Err(Error::MintingFailed(format!(
                    "minting additional {mint_amount} tokens would exceed the maximum supply: {}",
                    token_scheme.maximum_supply()
                )));
            }

            // Get the alias output that controls the foundry output
            let existing_alias_output = account_details.unspent_outputs().values().find(|output_data| {
                if let Output::Alias(output) = &output_data.output {
                    output.alias_id_non_null(&output_data.output_id) == **foundry_output.alias_address()
                } else {
                    false
                }
            });
            existing_alias_output
                .ok_or_else(|| Error::MintingFailed("alias output is not available".to_string()))?
                .clone()
        } else {
            return Err(Error::MintingFailed("alias output is not available".to_string()));
        };

        drop(account_details);

        let alias_output = if let Output::Alias(alias_output) = existing_alias_output.output {
            alias_output
        } else {
            unreachable!("We checked if it's an alias output before")
        };
        let foundry_output = if let Output::Foundry(foundry_output) = existing_foundry_output.output {
            foundry_output
        } else {
            unreachable!("We checked if it's an foundry output before")
        };

        // Create the next alias output with the same data, just updated state_index
        let new_alias_output_builder =
            AliasOutputBuilder::from(&alias_output).with_state_index(alias_output.state_index() + 1);

        // Create next foundry output with minted native tokens

        let TokenScheme::Simple(token_scheme) = foundry_output.token_scheme();

        let updated_token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(
            token_scheme.minted_tokens() + mint_amount,
            token_scheme.melted_tokens(),
            token_scheme.maximum_supply(),
        )?);

        let new_foundry_output_builder =
            FoundryOutputBuilder::from(&foundry_output).with_token_scheme(updated_token_scheme);

        let outputs = [
            new_alias_output_builder.finish_output(token_supply)?,
            new_foundry_output_builder.finish_output(token_supply)?,
            // Native Tokens will be added automatically in the remainder output in try_select_inputs()
        ];

        self.prepare_transaction(outputs, options).await
    }
}
