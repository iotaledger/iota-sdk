// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::GetAddressesOptions;
use crate::{
    client::{node_api::indexer::query_parameters::QueryParameter, secret::SecretManager, Client, Result},
    types::block::{
        address::Bech32Address,
        input::{UtxoInput, INPUT_COUNT_MAX},
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder},
    },
};

impl Client {
    /// Function to consolidate all funds and native tokens from a range of addresses to the address with the lowest
    /// index in that range. Returns the address to which the funds got consolidated, if any were available
    pub async fn consolidate_funds(
        &self,
        secret_manager: &SecretManager,
        options: GetAddressesOptions,
    ) -> Result<Bech32Address> {
        let token_supply = self.get_token_supply().await?;
        let mut last_transfer_index = options.range.start;
        // use the start index as offset
        let offset = last_transfer_index;

        let addresses = secret_manager.generate_ed25519_addresses(options).await?;

        let consolidation_address = addresses[0];

        'consolidation: loop {
            let mut block_ids = Vec::new();
            // Iterate over addresses reversed so the funds end up on the first address in the range
            for (index, address) in addresses.iter().enumerate().rev() {
                let index = index as u32;
                // add the offset so the index matches the address index also for higher start indexes
                let index = index + offset;

                // Get output ids of outputs that can be controlled by this address without further unlock constraints
                let output_ids_response = self
                    .basic_output_ids([
                        QueryParameter::Address(*address),
                        QueryParameter::HasExpiration(false),
                        QueryParameter::HasTimelock(false),
                        QueryParameter::HasStorageDepositReturn(false),
                    ])
                    .await?;

                let basic_outputs_responses = self.get_outputs(&output_ids_response.items).await?;

                if !basic_outputs_responses.is_empty() {
                    // If we reach the same index again
                    if last_transfer_index == index {
                        if basic_outputs_responses.len() < 2 {
                            break 'consolidation;
                        }
                    } else {
                        last_transfer_index = index;
                    }
                }

                let outputs_chunks = basic_outputs_responses.chunks(INPUT_COUNT_MAX.into());

                self.bech32_hrp_matches(consolidation_address.hrp()).await?;

                for chunk in outputs_chunks {
                    let mut block_builder = self.block().with_secret_manager(secret_manager);
                    let mut total_amount = 0;
                    let mut total_native_tokens = NativeTokensBuilder::new();

                    for output_with_meta in chunk {
                        block_builder = block_builder
                            .with_input(UtxoInput::from(output_with_meta.metadata().output_id().to_owned()))?;

                        if let Some(native_tokens) = output_with_meta.output().native_tokens() {
                            total_native_tokens.add_native_tokens(native_tokens.clone())?;
                        }
                        total_amount += output_with_meta.output().amount();
                    }

                    let consolidation_output = BasicOutputBuilder::new_with_amount(total_amount)
                        .add_unlock_condition(AddressUnlockCondition::new(consolidation_address))
                        .with_native_tokens(total_native_tokens.finish()?)
                        .finish_output(token_supply)?;

                    let block = block_builder
                        .with_input_range(index..index + 1)
                        .with_outputs([consolidation_output])?
                        .with_initial_address_index(0)
                        .finish()
                        .await?;
                    block_ids.push(block.id());
                }
            }

            if block_ids.is_empty() {
                break 'consolidation;
            }
            // Wait for txs to get confirmed so we don't create conflicting txs
            for block_id in block_ids {
                let _ = self.retry_until_included(&block_id, None, None).await?;
            }
        }
        Ok(consolidation_address)
    }
}
