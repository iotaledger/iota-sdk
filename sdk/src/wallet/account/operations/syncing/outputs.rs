// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::slip10::Chain;
use instant::Instant;

use crate::{
    client::Client,
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            input::Input,
            output::{
                dto::{OutputDto, OutputMetadataDto},
                OutputId, OutputMetadata, OutputWithMetadata,
            },
            payload::{
                transaction::{TransactionEssence, TransactionId},
                Payload, TransactionPayload,
            },
        },
    },
    wallet::{
        account::{build_transaction_from_payload_and_inputs, types::OutputData, Account, AddressWithUnspentOutputs},
        task,
    },
};

impl Account {
    /// Convert OutputWithMetadataResponse to OutputData with the network_id added
    pub(crate) async fn output_response_to_output_data(
        &self,
        outputs_with_meta: Vec<OutputWithMetadata>,
        associated_address: &AddressWithUnspentOutputs,
    ) -> crate::wallet::Result<Vec<OutputData>> {
        log::debug!("[SYNC] convert output_responses");
        // store outputs with network_id
        let network_id = self.client.get_network_id().await?;
        let mut outputs = Vec::new();
        let account_details = self.read().await;

        for output_with_meta in outputs_with_meta {
            // check if we know the transaction that created this output and if we created it (if we store incoming
            // transactions separated, then this check wouldn't be required)
            let remainder = account_details
                .transactions
                .get(output_with_meta.metadata().transaction_id())
                .map_or(false, |tx| !tx.incoming);

            // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
            let chain = Chain::from_u32_hardened(vec![
                44,
                account_details.coin_type,
                account_details.index,
                associated_address.internal as u32,
                associated_address.key_index,
            ]);

            outputs.push(OutputData {
                output_id: output_with_meta.metadata().output_id().to_owned(),
                metadata: OutputMetadataDto::from(&output_with_meta.metadata().clone()),
                output: output_with_meta.output().clone(),
                is_spent: output_with_meta.metadata().is_spent(),
                address: associated_address.address.inner,
                network_id,
                remainder,
                chain: Some(chain),
            });
        }

        Ok(outputs)
    }

    /// Gets outputs by their id, already known outputs are not requested again, but loaded from the account set as
    /// unspent, because we wouldn't get them from the node if they were spent
    pub(crate) async fn get_outputs(
        &self,
        output_ids: Vec<OutputId>,
    ) -> crate::wallet::Result<Vec<OutputWithMetadata>> {
        log::debug!("[SYNC] start get_outputs");
        let get_outputs_start_time = Instant::now();
        let mut outputs = Vec::new();
        let mut unknown_outputs = Vec::new();
        let mut unspent_outputs = Vec::new();
        let mut account_details = self.write().await;

        for output_id in output_ids {
            match account_details.outputs.get_mut(&output_id) {
                // set unspent
                Some(output_data) => {
                    output_data.is_spent = false;
                    unspent_outputs.push((output_id, output_data.clone()));
                    outputs.push(OutputWithMetadata::new(
                        output_data.output.clone(),
                        OutputMetadata::try_from(&output_data.metadata)?,
                    ));
                }
                None => unknown_outputs.push(output_id),
            }
        }
        // known output is unspent, so insert it to the unspent outputs again, because if it was an
        // alias/nft/foundry output it could have been removed when syncing without them
        for (output_id, output_data) in unspent_outputs {
            account_details.unspent_outputs.insert(output_id, output_data);
        }

        drop(account_details);

        if !unknown_outputs.is_empty() {
            outputs.extend(self.client.get_outputs(unknown_outputs).await?);
        }

        log::debug!(
            "[SYNC] finished get_outputs in {:.2?}",
            get_outputs_start_time.elapsed()
        );

        Ok(outputs)
    }

    // Try to get transactions and inputs for received outputs
    // Because the transactions and outputs are pruned, we might can not get them anymore, in that case errors are not
    // returned
    pub(crate) async fn request_incoming_transaction_data(
        &self,
        transaction_ids: Vec<TransactionId>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] request_incoming_transaction_data");

        // Limit parallel requests to 100, to avoid timeouts
        for transaction_ids_chunk in transaction_ids.chunks(100).map(|x: &[TransactionId]| x.to_vec()) {
            let mut tasks = Vec::new();
            let account_details = self.read().await;

            for transaction_id in transaction_ids_chunk {
                // Don't request known or inaccessible transactions again
                if account_details.transactions.contains_key(&transaction_id)
                    || account_details.incoming_transactions.contains_key(&transaction_id)
                    || account_details
                        .inaccessible_incoming_transactions
                        .contains(&transaction_id)
                {
                    continue;
                }

                let client = self.client.clone();
                tasks.push(async move {
                    task::spawn(async move {
                        match client.get_included_block(&transaction_id).await {
                            Ok(block) => {
                                if let Some(Payload::Transaction(transaction_payload)) = block.payload() {
                                    let inputs_with_meta =
                                        get_inputs_for_transaction_payload(&client, transaction_payload).await?;
                                    let inputs_response: Vec<OutputWithMetadataResponse> = inputs_with_meta
                                        .into_iter()
                                        .map(|o| OutputWithMetadataResponse {
                                            output: OutputDto::from(o.output()),
                                            metadata: OutputMetadataDto::from(o.metadata()),
                                        })
                                        .collect();

                                    let transaction = build_transaction_from_payload_and_inputs(
                                        transaction_id,
                                        *transaction_payload.clone(),
                                        inputs_response,
                                    )?;

                                    Ok((transaction_id, Some(transaction)))
                                } else {
                                    Ok((transaction_id, None))
                                }
                            }
                            Err(crate::client::Error::NotFound(_)) => Ok((transaction_id, None)),
                            Err(e) => Err(crate::wallet::Error::Client(e.into())),
                        }
                    })
                    .await
                });
            }

            drop(account_details);

            let results = futures::future::try_join_all(tasks).await?;
            // Update account with new transactions
            let mut account_details = self.write().await;
            for res in results {
                match res? {
                    (transaction_id, Some(transaction)) => {
                        account_details
                            .incoming_transactions
                            .insert(transaction_id, transaction);
                    }
                    (transaction_id, None) => {
                        log::debug!("[SYNC] adding {transaction_id} to inaccessible_incoming_transactions");
                        // Save transactions that weren't found by the node to avoid requesting them endlessly.
                        // Will be cleared when new client options are provided.
                        account_details
                            .inaccessible_incoming_transactions
                            .insert(transaction_id);
                    }
                }
            }
        }

        Ok(())
    }
}

// Try to fetch the inputs of the transaction
pub(crate) async fn get_inputs_for_transaction_payload(
    client: &Client,
    transaction_payload: &TransactionPayload,
) -> crate::wallet::Result<Vec<OutputWithMetadata>> {
    let TransactionEssence::Regular(essence) = transaction_payload.essence();
    let mut output_ids = Vec::new();

    for input in essence.inputs() {
        if let Input::Utxo(input) = input {
            output_ids.push(*input.output_id());
        }
    }

    client.try_get_outputs(output_ids).await.map_err(|e| e.into())
}
