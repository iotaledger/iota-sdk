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

pub(crate) mod burn_native_token;
pub(crate) mod burn_nft;
pub(crate) mod decrease_native_token_supply;
pub(crate) mod destroy_alias;
pub(crate) mod destroy_foundry;

impl Account {
    // new method "burn" which will take as an argument structure Burn and will be a unified combination of methods:
    // burn_native_token and burn_nft, destroy_foundry and destroy_alias
    pub async fn burn(&self, burn: Burn, options: Option<TransactionOptions>) -> crate::wallet::Result<Transaction> {
        let mut all_outputs: Vec<Output> = Default::default();
        let mut all_custom_inputs: Vec<OutputId> = Default::default();
        let mut options = options.unwrap_or_default();

        for (token_id, burn_amount) in burn.native_tokens() {
            let (custom_inputs, outputs) = self
                .get_inputs_outputs_for_burn_native_tokens(*token_id, *burn_amount)
                .await?;
            all_custom_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for (token_id, burn_amount) in burn.native_tokens() {
            let (custom_inputs, outputs) = self
                .get_inputs_outputs_for_burn_native_tokens(*token_id, *burn_amount)
                .await?;
            all_custom_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for nft_id in burn.nfts() {
            let (custom_inputs, outputs) = self.get_inputs_outputs_for_burn_nft(*nft_id).await?;
            all_custom_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for foundry_id in burn.foundries() {
            let (custom_inputs, outputs) = self.get_inputs_outputs_for_destroy_foundry(*foundry_id).await?;
            all_custom_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        for alias_id in burn.aliases() {
            let (custom_inputs, outputs) = self.get_inputs_outputs_for_destroy_alias(*alias_id).await?;
            all_custom_inputs.extend(custom_inputs);
            all_outputs.extend(outputs);
        }

        options.burn = Some(burn);
        options.custom_inputs = Some(all_custom_inputs);

        self.send(all_outputs, Some(options)).await
    }
}
