// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use futures::{StreamExt, TryStreamExt};

use crate::{
    client::{
        api::input_selection::Error as InputSelectionError,
        constants::FIVE_MINUTES_IN_SECONDS,
        error::{Error, Result},
        node_api::indexer::query_parameters::QueryParameter,
        secret::SecretManage,
        Client,
    },
    types::block::{
        address::Bech32Address,
        core::Block,
        input::{Input, UtxoInput, INPUT_COUNT_MAX},
        output::OutputWithMetadata,
        payload::{
            transaction::{TransactionEssence, TransactionId},
            Payload,
        },
        BlockId,
    },
    utils::unix_timestamp_now,
};

impl Client {
    /// Get the inputs of a transaction for the given transaction id.
    pub async fn inputs_from_transaction_id(&self, transaction_id: &TransactionId) -> Result<Vec<OutputWithMetadata>> {
        let block = self.get_included_block(transaction_id).await?;

        let inputs = match block.payload() {
            Some(Payload::Transaction(t)) => match t.essence() {
                TransactionEssence::Regular(e) => e.inputs(),
            },
            _ => {
                unreachable!()
            }
        };

        let input_ids = inputs
            .iter()
            .map(|i| match i {
                Input::Utxo(input) => *input.output_id(),
            })
            .collect::<Vec<_>>();

        self.get_outputs_with_metadata(&input_ids).await
    }

    /// Find all blocks by provided block IDs.
    pub async fn find_blocks(&self, block_ids: &[BlockId]) -> Result<Vec<Block>> {
        // Use a `HashSet` to prevent duplicate block_ids.
        let block_ids = block_ids.iter().copied().collect::<HashSet<_>>();
        futures::future::try_join_all(block_ids.iter().map(|block_id| self.get_block(block_id))).await
    }

    /// Function to find inputs from addresses for a provided amount (useful for offline signing), ignoring outputs with
    /// additional unlock conditions
    pub async fn find_inputs(&self, addresses: Vec<Bech32Address>, amount: u64) -> Result<Vec<UtxoInput>> {
        // Get outputs from node and select inputs
        let available_outputs = futures::stream::iter(addresses)
            .then(|address| {
                self.basic_output_ids([
                    QueryParameter::Address(address),
                    QueryParameter::HasExpiration(false),
                    QueryParameter::HasTimelock(false),
                    QueryParameter::HasStorageDepositReturn(false),
                ])
            })
            .and_then(|res| async {
                let items = res.items;
                self.get_outputs_with_metadata(&items).await
            })
            .try_collect::<Vec<_>>()
            .await?;

        let mut basic_outputs = available_outputs
            .into_iter()
            .flatten()
            .map(|output_with_meta| {
                Ok((
                    UtxoInput::new(
                        output_with_meta.metadata().transaction_id().to_owned(),
                        output_with_meta.metadata().output_index(),
                    )?,
                    output_with_meta.output().amount(),
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        basic_outputs.sort_by(|l, r| r.1.cmp(&l.1));

        let mut total_already_spent = 0;
        let mut selected_inputs = Vec::new();
        for (_offset, output_wrapper) in basic_outputs
            .into_iter()
            // Max inputs is 128
            .take(INPUT_COUNT_MAX.into())
            .enumerate()
        {
            // Break if we have enough funds and don't create dust for the remainder
            if total_already_spent == amount || total_already_spent >= amount {
                break;
            }
            selected_inputs.push(output_wrapper.0);
            total_already_spent += output_wrapper.1;
        }

        if total_already_spent < amount {
            return Err(InputSelectionError::InsufficientAmount {
                found: total_already_spent,
                required: amount,
            })?;
        }

        Ok(selected_inputs)
    }

    /// Returns the local time checked with the timestamp of the latest milestone, if the difference is larger than 5
    /// minutes an error is returned to prevent locking outputs by accident for a wrong time.
    pub async fn get_time_checked(&self) -> Result<u32> {
        let current_time = unix_timestamp_now().as_secs() as u32;

        let network_info = self.get_network_info().await?;

        if let Some(latest_ms_timestamp) = network_info.latest_milestone_timestamp {
            // Check the local time is in the range of +-5 minutes of the node to prevent locking funds by accident
            if !(latest_ms_timestamp - FIVE_MINUTES_IN_SECONDS..latest_ms_timestamp + FIVE_MINUTES_IN_SECONDS)
                .contains(&current_time)
            {
                return Err(Error::TimeNotSynced {
                    current_time,
                    milestone_timestamp: latest_ms_timestamp,
                });
            }
        }

        Ok(current_time)
    }
}
