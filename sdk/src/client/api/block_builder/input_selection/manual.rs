// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Manual input selection for transactions

use std::collections::HashSet;

use crypto::keys::bip44::Bip44;

use crate::{
    client::{
        api::{
            address::search_address,
            block_builder::input_selection::{Burn, InputSelection, Selected},
            input_selection::is_alias_transition,
            ClientBlockBuilder,
        },
        secret::types::InputSigningData,
        Result,
    },
    types::block::{address::Address, protocol::ProtocolParameters},
};

impl<'a> ClientBlockBuilder<'a> {
    /// If custom inputs are provided we check if they are unspent, get the balance and search the Ed25519 addresses for
    /// them with the provided input_range so we can later sign them.
    /// Forwards to [try_select_inputs()] with all inputs in `mandatory_inputs`, so they will all be included in the
    /// transaction, even if not required for the provided outputs.
    pub(crate) async fn get_custom_inputs(
        &self,
        protocol_parameters: &ProtocolParameters,
        burn: Option<Burn>,
    ) -> Result<Selected> {
        log::debug!("[get_custom_inputs]");

        let mut inputs_data = Vec::new();
        let current_time = self.client.get_time_checked().await?;

        if let Some(inputs) = &self.inputs {
            for input in inputs {
                let output_with_meta = self.client.get_output(input.output_id()).await?;

                if !output_with_meta.metadata().is_spent() {
                    let alias_transition = is_alias_transition(
                        output_with_meta.output(),
                        *input.output_id(),
                        &self.outputs,
                        burn.as_ref(),
                    );
                    let (unlock_address, _) = output_with_meta.output().required_and_unlocked_address(
                        current_time,
                        input.output_id(),
                        alias_transition,
                    )?;

                    let bech32_hrp = self.client.get_bech32_hrp().await?;
                    let address_index_internal = match self.secret_manager {
                        Some(secret_manager) => {
                            match unlock_address {
                                Address::Ed25519(_) => Some(
                                    search_address(
                                        secret_manager,
                                        bech32_hrp,
                                        self.coin_type,
                                        self.account_index,
                                        self.input_range.clone(),
                                        &unlock_address,
                                    )
                                    .await?,
                                ),
                                // Alias and NFT addresses can't be generated from a private key.
                                _ => None,
                            }
                        }
                        // Assuming default for offline signing.
                        None => Some((0, false)),
                    };

                    inputs_data.push(InputSigningData {
                        output: output_with_meta.output,
                        output_metadata: output_with_meta.metadata,
                        chain: address_index_internal.map(|(address_index, internal)| {
                            Bip44::new()
                                .with_coin_type(self.coin_type)
                                .with_account(self.account_index)
                                .with_change(internal as _)
                                .with_address_index(address_index)
                        }),
                    });
                }
            }
        }

        let required_inputs = inputs_data
            .iter()
            .map(|input| *input.output_id())
            .collect::<HashSet<_>>();

        // Assume that we own the addresses for inputs that are provided
        let mut available_input_addresses = Vec::new();
        for input in &inputs_data {
            let alias_transition = is_alias_transition(&input.output, *input.output_id(), &self.outputs, burn.as_ref());
            let (required_unlock_address, unlocked_alias_or_nft_address) =
                input
                    .output
                    .required_and_unlocked_address(current_time, input.output_id(), alias_transition)?;
            available_input_addresses.push(required_unlock_address);
            if let Some(unlocked_alias_or_nft_address) = unlocked_alias_or_nft_address {
                available_input_addresses.push(unlocked_alias_or_nft_address);
            }
        }

        inputs_data.sort_unstable_by_key(|input| *input.output_id());
        inputs_data.dedup_by_key(|input| *input.output_id());

        let mut input_selection = InputSelection::new(
            inputs_data,
            self.outputs.clone(),
            available_input_addresses,
            protocol_parameters.clone(),
        )
        .required_inputs(required_inputs)
        .timestamp(current_time);

        if let Some(address) = self.custom_remainder_address {
            input_selection = input_selection.remainder_address(address);
        }

        if let Some(burn) = burn {
            input_selection = input_selection.burn(burn);
        }

        Ok(input_selection.select()?)
    }
}
