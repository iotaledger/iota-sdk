// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::output::{FoundryOutputBuilder, Output, SimpleTokenScheme, TokenId, TokenScheme},
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
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
    ) -> Result<TransactionWithMetadata, WalletError> {
        let options = options.into();
        let prepared = self
            .prepare_mint_native_token(token_id, mint_amount, options.clone())
            .await?;
        let transaction = self.sign_and_submit_transaction(prepared, options).await?;

        Ok(transaction)
    }

    /// Prepares the transaction for [Wallet::mint_native_token()].
    pub async fn prepare_mint_native_token(
        &self,
        token_id: TokenId,
        mint_amount: impl Into<U256> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] mint_native_token");

        let mint_amount = mint_amount.into();
        let wallet_ledger = self.ledger().await;
        let existing_foundry_output = wallet_ledger.unspent_outputs.values().find(|output_data| {
            if let Output::Foundry(output) = &output_data.output {
                TokenId::new(*output.id()) == token_id
            } else {
                false
            }
        });

        let existing_foundry_output = existing_foundry_output
            .ok_or_else(|| WalletError::MintingFailed(format!("foundry output {token_id} is not available")))?
            .clone();

        let existing_account_output = if let Output::Foundry(foundry_output) = &existing_foundry_output.output {
            let TokenScheme::Simple(token_scheme) = foundry_output.token_scheme();
            // Check if we can mint the provided amount without exceeding the maximum_supply
            if token_scheme.maximum_supply() - token_scheme.circulating_supply() < mint_amount {
                return Err(WalletError::MintingFailed(format!(
                    "minting additional {mint_amount} tokens would exceed the maximum supply: {}",
                    token_scheme.maximum_supply()
                )));
            }

            // Get the account output that controls the foundry output
            let existing_account_output = wallet_ledger.unspent_outputs.values().find(|output_data| {
                if let Output::Account(output) = &output_data.output {
                    output.account_id_non_null(&output_data.output_id) == **foundry_output.account_address()
                } else {
                    false
                }
            });
            existing_account_output
                .ok_or_else(|| WalletError::MintingFailed("account output is not available".to_string()))?
                .clone()
        } else {
            return Err(WalletError::MintingFailed(
                "account output is not available".to_string(),
            ));
        };

        drop(wallet_ledger);

        let foundry_output = existing_foundry_output.output.as_foundry();

        let mut options = options.into();
        if let Some(options) = options.as_mut() {
            options.required_inputs.insert(existing_account_output.output_id);
        } else {
            options.replace(TransactionOptions {
                required_inputs: [existing_account_output.output_id].into(),
                ..Default::default()
            });
        }

        // Create next foundry output with minted native tokens

        let TokenScheme::Simple(token_scheme) = foundry_output.token_scheme();

        let updated_token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(
            token_scheme.minted_tokens() + mint_amount,
            token_scheme.melted_tokens(),
            token_scheme.maximum_supply(),
        )?);

        let new_foundry_output_builder =
            FoundryOutputBuilder::from(foundry_output).with_token_scheme(updated_token_scheme);

        let outputs = [new_foundry_output_builder.finish_output()?];

        self.prepare_send_outputs(outputs, options).await
    }
}
