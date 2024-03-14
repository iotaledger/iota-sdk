// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::{secret::SecretManage, Client, ClientError},
    types::{
        api::core::OutputWithMetadataResponse,
        block::{
            core::{BasicBlockBody, BlockBody},
            input::Input,
            output::OutputId,
            payload::{signed_transaction::TransactionId, Payload, SignedTransactionPayload},
        },
    },
    wallet::{build_transaction_from_payload_and_inputs, task, types::OutputData, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Convert OutputWithMetadataResponse to OutputData with the network_id added
    pub(crate) async fn output_response_to_output_data(
        &self,
        outputs_with_meta: Vec<OutputWithMetadataResponse>,
    ) -> Result<Vec<OutputData>, WalletError> {
        log::debug!("[SYNC] convert output_responses");
        // store outputs with network_id
        let network_id = self.client().get_network_id().await?;
        let wallet_ledger = self.ledger().await;

        Ok(outputs_with_meta
            .into_iter()
            .map(
                |OutputWithMetadataResponse {
                     output,
                     output_id_proof,
                     metadata,
                 }| {
                    // check if we know the transaction that created this output and if we created it (if we store
                    // incoming transactions separated, then this check wouldn't be required)
                    let remainder = wallet_ledger
                        .transactions
                        .get(metadata.output_id().transaction_id())
                        .map_or(false, |tx| !tx.incoming);

                    OutputData {
                        output_id: metadata.output_id().to_owned(),
                        metadata,
                        output,
                        output_id_proof,
                        network_id,
                        remainder,
                    }
                },
            )
            .collect())
    }

    /// Gets outputs by their id, already known outputs are not requested again, but loaded from the account set as
    /// unspent, because we wouldn't get them from the node if they were spent
    pub(crate) async fn get_outputs(
        &self,
        output_ids: Vec<OutputId>,
    ) -> Result<Vec<OutputWithMetadataResponse>, WalletError> {
        log::debug!("[SYNC] start get_outputs");
        let get_outputs_start_time = Instant::now();
        let mut outputs = Vec::new();
        let mut unknown_outputs = Vec::new();
        let mut unspent_outputs = Vec::new();
        let mut wallet_ledger = self.ledger_mut().await;

        for output_id in output_ids {
            match wallet_ledger.outputs.get_mut(&output_id) {
                // set unspent if not already
                Some(output_data) => {
                    if output_data.is_spent() {
                        log::warn!("Removing spent output metadata for {output_id}, because it's still unspent");
                        output_data.metadata.spent = None;
                    }
                    unspent_outputs.push((output_id, output_data.clone()));
                    outputs.push(OutputWithMetadataResponse::new(
                        output_data.output.clone(),
                        output_data.output_id_proof.clone(),
                        output_data.metadata,
                    ));
                }
                None => unknown_outputs.push(output_id),
            }
        }
        // known output is unspent, so insert it to the unspent outputs again, because if it was an
        // account/nft/foundry output it could have been removed when syncing without them
        for (output_id, output_data) in unspent_outputs {
            wallet_ledger.unspent_outputs.insert(output_id, output_data);
        }

        drop(wallet_ledger);

        if !unknown_outputs.is_empty() {
            outputs.extend(self.client().get_outputs_with_metadata(&unknown_outputs).await?);
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
        mut transaction_ids: Vec<TransactionId>,
    ) -> Result<(), WalletError> {
        log::debug!("[SYNC] request_incoming_transaction_data");

        let wallet_ledger = self.ledger().await;
        transaction_ids.retain(|transaction_id| {
            !(wallet_ledger.transactions.contains_key(transaction_id)
                || wallet_ledger.incoming_transactions.contains_key(transaction_id)
                || wallet_ledger
                    .inaccessible_incoming_transactions
                    .contains(transaction_id))
        });
        drop(wallet_ledger);

        // Limit parallel requests to 100, to avoid timeouts
        let results =
            futures::future::try_join_all(transaction_ids.chunks(100).map(|x| x.to_vec()).map(|transaction_ids| {
                let client = self.client().clone();
                async move {
                    task::spawn(async move {
                        futures::future::try_join_all(transaction_ids.iter().map(|transaction_id| async {
                            let transaction_id = *transaction_id;
                            match client.get_included_block(&transaction_id).await {
                                Ok(block) => {
                                    if let BlockBody::Basic(basic_block_body) = block.body() {
                                        if let Some(Payload::SignedTransaction(transaction_payload)) =
                                            basic_block_body.payload()
                                        {
                                            let inputs_with_meta =
                                                get_inputs_for_transaction_payload(&client, transaction_payload)
                                                    .await?;
                                            let inputs_response: Vec<OutputWithMetadataResponse> = inputs_with_meta
                                                .into_iter()
                                                .map(OutputWithMetadataResponse::from)
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
                                    } else {
                                        Err(ClientError::UnexpectedBlockBodyKind {
                                            expected: BasicBlockBody::KIND,
                                            actual: block.body().kind(),
                                        }
                                        .into())
                                    }
                                }
                                Err(ClientError::Node(crate::client::node_api::error::Error::NotFound(_))) => {
                                    Ok((transaction_id, None))
                                }
                                Err(e) => Err(WalletError::Client(e)),
                            }
                        }))
                        .await
                    })
                    .await?
                }
            }))
            .await?;

        // Update account with new transactions
        let mut wallet_ledger = self.ledger_mut().await;
        for (transaction_id, txn) in results.into_iter().flatten() {
            if let Some(transaction) = txn {
                wallet_ledger.incoming_transactions.insert(transaction_id, transaction);
            } else {
                log::debug!("[SYNC] adding {transaction_id} to inaccessible_incoming_transactions");
                // Save transactions that weren't found by the node to avoid requesting them endlessly.
                // Will be cleared when new client options are provided.
                wallet_ledger.inaccessible_incoming_transactions.insert(transaction_id);
            }
        }

        Ok(())
    }
}

// Try to fetch the inputs of the transaction
pub(crate) async fn get_inputs_for_transaction_payload(
    client: &Client,
    transaction_payload: &SignedTransactionPayload,
) -> Result<Vec<OutputWithMetadataResponse>, WalletError> {
    let output_ids = transaction_payload
        .transaction()
        .inputs()
        .iter()
        .map(|input| {
            let Input::Utxo(input) = input;
            *input.output_id()
        })
        .collect::<Vec<_>>();

    client
        .get_outputs_with_metadata_ignore_not_found(&output_ids)
        .await
        .map_err(|e| e.into())
}
