// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    types::block::{
        address::{Address, NftAddress},
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NftId, Output, OutputId},
    },
    wallet::{
        account::{
            operations::helpers::time::can_output_be_unlocked_now, Account,
        },
        Error,
    },
};

impl Account {
    /// Function that returns the inputs and outputs for burning an nft.
    pub(super) async fn get_inputs_outputs_for_burn_nft(
        &self,
        nft_id: NftId,
    ) -> crate::wallet::Result<(Vec<OutputId>, Vec<Output>)> {
        log::debug!("[TRANSACTION] burn_nft");

        let current_time = self.client().get_time_checked().await?;

        let mut owned_outputs = Vec::new();

        for output_data in self.unspent_outputs(None).await? {
            if can_output_be_unlocked_now(
                // Don't provide any addresses here, since we're only interested in outputs that can be unlocked by
                // the nft address
                &[],
                &[Address::Nft(NftAddress::new(nft_id))],
                &output_data,
                current_time,
                None,
            )? {
                owned_outputs.push(output_data);
            }
        }

        if !owned_outputs.is_empty() {
            return Err(Error::BurningOrMeltingFailed(format!(
                "nft still owns outputs: {:?}",
                owned_outputs.iter().map(|o| o.output_id).collect::<Vec<OutputId>>()
            )));
        }

        let (output_id, basic_output) = self.output_id_and_basic_output_for_nft(nft_id).await?;
        let custom_inputs = vec![output_id];
        let outputs = vec![basic_output];

        Ok((custom_inputs, outputs))
    }

    // Get the current output id for the nft and build a basic output with the amount, native tokens and
    // governor address from the nft output.
    async fn output_id_and_basic_output_for_nft(&self, nft_id: NftId) -> crate::wallet::Result<(OutputId, Output)> {
        let account_details = self.details().await;
        let token_supply = self.client().get_token_supply().await?;
        let current_time = self.client().get_time_checked().await?;

        let (output_id, nft_output) = account_details
            .unspent_outputs()
            .iter()
            .find_map(|(&output_id, output_data)| match &output_data.output {
                Output::Nft(nft_output) => {
                    if nft_output.nft_id_non_null(&output_id) == nft_id {
                        Some((output_id, nft_output))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .ok_or(Error::NftNotFoundInUnspentOutputs)?;

        let unlock_address = nft_output
            .unlock_conditions()
            .locked_address(nft_output.address(), current_time);

        let basic_output = Output::Basic(
            BasicOutputBuilder::new_with_amount(nft_output.amount())
                .add_unlock_condition(AddressUnlockCondition::new(*unlock_address))
                .with_native_tokens(nft_output.native_tokens().clone())
                .finish(token_supply)?,
        );

        Ok((output_id, basic_output))
    }
}
