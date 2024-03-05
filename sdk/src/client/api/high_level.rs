// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use futures::{StreamExt, TryStreamExt};

use crate::{
    client::{
        api::input_selection::Error as InputSelectionError, constants::FIVE_MINUTES_IN_NANOSECONDS, error::ClientError,
        node_api::indexer::query_parameters::BasicOutputQueryParameters, unix_timestamp_now, Client,
    },
    types::{
        api::core::OutputWithMetadataResponse,
        block::{
            address::Bech32Address,
            core::{BasicBlockBody, Block, BlockBody},
            input::{Input, UtxoInput, INPUT_COUNT_MAX},
            payload::{signed_transaction::TransactionId, Payload},
            slot::SlotIndex,
            BlockId,
        },
    },
};

impl Client {
    /// Get the inputs of a transaction for the given transaction id.
    pub async fn inputs_from_transaction_id(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<Vec<OutputWithMetadataResponse>, ClientError> {
        let block = self.get_included_block(transaction_id).await?;

        if let BlockBody::Basic(basic_block_body) = block.body() {
            let inputs = if let Some(Payload::SignedTransaction(t)) = basic_block_body.payload() {
                t.transaction().inputs()
            } else {
                return Err(ClientError::MissingTransactionPayload);
            };

            let input_ids = inputs
                .iter()
                .map(|i| match i {
                    Input::Utxo(input) => *input.output_id(),
                })
                .collect::<Vec<_>>();

            self.get_outputs_with_metadata(&input_ids).await
        } else {
            Err(ClientError::UnexpectedBlockBodyKind {
                expected: BasicBlockBody::KIND,
                actual: block.body().kind(),
            })
        }
    }

    /// Find all blocks by provided block IDs.
    pub async fn find_blocks(&self, block_ids: &[BlockId]) -> Result<Vec<Block>, ClientError> {
        // Use a `HashSet` to prevent duplicate block_ids.
        let block_ids = block_ids.iter().copied().collect::<HashSet<_>>();
        futures::future::try_join_all(block_ids.iter().map(|block_id| self.get_block(block_id))).await
    }

    /// Function to find inputs from addresses for a provided amount (useful for offline signing), ignoring outputs with
    /// additional unlock conditions
    pub async fn find_inputs(&self, addresses: Vec<Bech32Address>, amount: u64) -> Result<Vec<UtxoInput>, ClientError> {
        // Get outputs from node and select inputs
        let available_outputs = futures::stream::iter(addresses)
            .then(|address| self.basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(address)))
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
                    UtxoInput::from(*output_with_meta.metadata().output_id()),
                    output_with_meta.output().amount(),
                ))
            })
            .collect::<Result<Vec<_>, ClientError>>()?;
        basic_outputs.sort_by(|l, r| r.1.cmp(&l.1));

        let mut total_already_spent = 0;
        let mut selected_inputs = Vec::new();
        for output_wrapper in basic_outputs
            .into_iter()
            // Max inputs is 128
            .take(INPUT_COUNT_MAX.into())
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

    // Returns the slot index corresponding to the current timestamp.
    pub async fn get_slot_index(&self) -> Result<SlotIndex, ClientError> {
        let unix_timestamp = unix_timestamp_now();
        let current_time_nanos = unix_timestamp.as_nanos() as u64;

        let network_info = self.get_network_info().await?;

        if let Some(tangle_time) = network_info.tangle_time {
            // Check the local time is in the range of +-5 minutes of the node to prevent locking funds by accident
            if !(tangle_time - FIVE_MINUTES_IN_NANOSECONDS..tangle_time + FIVE_MINUTES_IN_NANOSECONDS)
                .contains(&current_time_nanos)
            {
                return Err(ClientError::TimeNotSynced {
                    current_time: current_time_nanos,
                    tangle_time,
                });
            }
        }

        Ok(network_info.protocol_parameters.slot_index(unix_timestamp.as_secs()))
    }
}
