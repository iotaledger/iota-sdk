// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::output::{
        AliasId, AliasOutputBuilder, FoundryId, FoundryOutputBuilder, Output, SimpleTokenScheme, TokenId, TokenScheme,
    },
    wallet::{
        account::{operations::transaction::Transaction, types::OutputData, Account, TransactionOptions},
        Error,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Melts native tokens.
    ///
    /// This happens with the foundry output which minted them, by increasing it's
    /// `melted_tokens` field. This should be preferred over burning, because after burning, the foundry can never be
    /// destroyed anymore.
    pub async fn melt_native_token(
        &self,
        token_id: TokenId,
        melt_amount: impl Into<U256> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let options = options.into();
        let prepared_transaction = self
            .prepare_melt_native_token(token_id, melt_amount, options.clone())
            .await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for
    /// [Account::melt_native_token()](crate::wallet::Account::melt_native_token).
    pub async fn prepare_melt_native_token(
        &self,
        token_id: TokenId,
        melt_amount: impl Into<U256> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_melt_native_token");

        let foundry_id = FoundryId::from(token_id);
        let alias_id = *foundry_id.alias_address().alias_id();
        let token_supply = self.client().get_token_supply().await?;

        let (existing_alias_output_data, existing_foundry_output) = self
            .find_alias_and_foundry_output_data(alias_id, foundry_id)
            .await
            .map(|(alias_data, foundry_data)| match foundry_data.output {
                Output::Foundry(foundry_output) => (alias_data, foundry_output),
                _ => unreachable!("We already checked it's a foundry output"),
            })?;

        if let Output::Alias(alias_output) = &existing_alias_output_data.output {
            // Create the new alias output with updated amount and state_index
            let alias_output = AliasOutputBuilder::from(alias_output)
                .with_alias_id(alias_id)
                .with_state_index(alias_output.state_index() + 1)
                .finish_output(token_supply)?;

            let TokenScheme::Simple(token_scheme) = existing_foundry_output.token_scheme();
            let outputs = [
                alias_output,
                FoundryOutputBuilder::from(&existing_foundry_output)
                    .with_token_scheme(TokenScheme::Simple(SimpleTokenScheme::new(
                        token_scheme.minted_tokens(),
                        token_scheme.melted_tokens() + melt_amount,
                        token_scheme.maximum_supply(),
                    )?))
                    .finish_output(token_supply)?,
            ];
            // Input selection will detect that we're melting native tokens and add the required inputs if available
            self.prepare_transaction(outputs, options).await
        } else {
            unreachable!("We checked if it's an alias output before")
        }
    }

    /// Find and return unspent `OutputData` for given `alias_id` and `foundry_id`
    async fn find_alias_and_foundry_output_data(
        &self,
        alias_id: AliasId,
        foundry_id: FoundryId,
    ) -> crate::wallet::Result<(OutputData, OutputData)> {
        let mut existing_alias_output_data = None;
        let mut existing_foundry_output = None;

        for (output_id, output_data) in self.details().await.unspent_outputs().iter() {
            match &output_data.output {
                Output::Alias(output) => {
                    if output.alias_id_non_null(output_id) == alias_id {
                        existing_alias_output_data = Some(output_data.clone());
                    }
                }
                Output::Foundry(output) => {
                    if output.id() == foundry_id {
                        existing_foundry_output = Some(output_data.clone());
                    }
                }
                // Not interested in these outputs here
                Output::Treasury(_) | Output::Basic(_) | Output::Nft(_) => {}
            }

            if existing_alias_output_data.is_some() && existing_foundry_output.is_some() {
                break;
            }
        }

        let existing_alias_output_data = existing_alias_output_data
            .ok_or_else(|| Error::BurningOrMeltingFailed("required alias output for foundry not found".to_string()))?;

        let existing_foundry_output_data = existing_foundry_output
            .ok_or_else(|| Error::BurningOrMeltingFailed("required foundry output not found".to_string()))?;

        Ok((existing_alias_output_data, existing_foundry_output_data))
    }
}
