// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::api::input_selection::Burn,
    types::block::output::{Output, OutputId},
    wallet::{
        account::{types::Transaction, TransactionOptions},
        Account,
    },
};

mod burn_native_token;
mod burn_nft;
pub(crate) mod decrease_native_token_supply;
mod destroy_alias;
mod destroy_foundry;

impl Account {
    /// A generic `burn()` function that can be used to burn native tokens, nfts, foundries and aliases

    /// When burn **native tokens**. This doesn't require the foundry output which minted them, but will not increase
    /// the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
    /// recommended to use melting, if the foundry output is available.
    pub async fn burn(
        &self,
        burn: impl Into<Burn> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let burn: Burn = burn.into();
        let mut options: TransactionOptions = options.into().unwrap_or_default();
        let mut all_outputs: Vec<Output> = Default::default();
        let mut all_inputs: Vec<OutputId> = Default::default();

        for (token_id, burn_amount) in burn.native_tokens() {
            let (custom_inputs, outputs) = self
                .get_inputs_outputs_for_burn_native_tokens(*token_id, *burn_amount)
                .await?;
            all_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for nft_id in burn.nfts() {
            let (custom_inputs, outputs) = self.get_inputs_outputs_for_burn_nft(*nft_id).await?;
            all_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for foundry_id in burn.foundries() {
            let (custom_inputs, outputs) = self.get_inputs_outputs_for_destroy_foundry(*foundry_id).await?;
            all_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for alias_id in burn.aliases() {
            let (custom_inputs, outputs) = self.get_inputs_outputs_for_destroy_alias(*alias_id).await?;
            all_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        options.burn = Some(burn);
        options.custom_inputs = Some(all_inputs);

        self.send(all_outputs, Some(options)).await
    }
}
