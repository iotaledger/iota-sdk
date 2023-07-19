// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use instant::Instant;

use crate::{
    client::{secret::SecretManage, Client},
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            input::Input,
            output::{OutputId, OutputWithMetadata},
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

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Convert OutputWithMetadataResponse to OutputData with the network_id added
    pub(crate) async fn output_response_to_output_data(
        &self,
        outputs_with_meta: Vec<OutputWithMetadata>,
        associated_address: &AddressWithUnspentOutputs,
    ) -> crate::wallet::Result<Vec<OutputData>> {
        log::debug!("[SYNC] convert output_responses");
        // store outputs with network_id
        let network_id = self.client().get_network_id().await?;
        let account_details = self.details().await;

        Ok(outputs_with_meta
            .into_iter()
            .map(|output_with_meta| {
                // check if we know the transaction that created this output and if we created it (if we store incoming
                // transactions separated, then this check wouldn't be required)
                let remainder = account_details
                    .transactions
                    .get(output_with_meta.metadata().transaction_id())
                    .map_or(false, |tx| !tx.incoming);

                // BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
                let chain = Bip44::new(account_details.coin_type)
                    .with_account(account_details.index)
                    .with_change(associated_address.internal as _)
                    .with_address_index(associated_address.key_index);

                OutputData {
                    output_id: output_with_meta.metadata().output_id().to_owned(),
                    metadata: *output_with_meta.metadata(),
                    output: output_with_meta.output().clone(),
                    is_spent: output_with_meta.metadata().is_spent(),
                    address: associated_address.address.inner,
                    network_id,
                    remainder,
                    chain: Some(chain),
                }
            })
            .collect())
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
        let mut account_details = self.details_mut().await;

        for output_id in output_ids {
            match account_details.outputs.get_mut(&output_id) {
                // set unspent
                Some(output_data) => {
                    output_data.is_spent = false;
                    unspent_outputs.push((output_id, output_data.clone()));
                    outputs.push(OutputWithMetadata::new(
                        output_data.output.clone(),
                        output_data.metadata,
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
            outputs.extend(self.client().get_outputs(&unknown_outputs).await?);
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
    ) -> crate::wallet::Result<()> {
        log::debug!("[SYNC] request_incoming_transaction_data");

        let account_details = self.details().await;
        transaction_ids.retain(|transaction_id| {
            !(account_details.transactions.contains_key(transaction_id)
                || account_details.incoming_transactions.contains_key(transaction_id)
                || account_details
                    .inaccessible_incoming_transactions
                    .contains(transaction_id))
        });
        drop(account_details);

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
                                    if let Some(Payload::Transaction(transaction_payload)) = block.payload() {
                                        let inputs_with_meta =
                                            get_inputs_for_transaction_payload(&client, transaction_payload).await?;
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
                                }
                                Err(crate::client::Error::Node(crate::client::node_api::error::Error::NotFound(_))) => {
                                    Ok((transaction_id, None))
                                }
                                Err(e) => Err(crate::wallet::Error::Client(e.into())),
                            }
                        }))
                        .await
                    })
                    .await?
                }
            }))
            .await?;

        // Update account with new transactions
        let mut account_details = self.details_mut().await;
        for (transaction_id, txn) in results.into_iter().flatten() {
            if let Some(transaction) = txn {
                account_details
                    .incoming_transactions
                    .insert(transaction_id, transaction);
            } else {
                log::debug!("[SYNC] adding {transaction_id} to inaccessible_incoming_transactions");
                // Save transactions that weren't found by the node to avoid requesting them endlessly.
                // Will be cleared when new client options are provided.
                account_details
                    .inaccessible_incoming_transactions
                    .insert(transaction_id);
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

    let output_ids = essence
        .inputs()
        .iter()
        .filter_map(|input| {
            if let Input::Utxo(input) = input {
                Some(*input.output_id())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    client
        .get_outputs_ignore_errors(&output_ids)
        .await
        .map_err(|e| e.into())
}
