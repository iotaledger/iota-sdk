// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::{
    client::{secret::SecretManage, ClientError},
    types::{
        api::core::{BlockState, TransactionState},
        block::{input::Input, output::OutputId, BlockId},
    },
    wallet::{
        core::WalletLedger,
        types::{InclusionState, TransactionWithMetadata},
        Wallet, WalletError,
    },
};

// ignore outputs and transactions from other networks
// check if outputs are unspent
// also revalidate that the locked outputs needs to be there, maybe there was a conflict or the transaction got
// confirmed, then they should get removed

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Sync transactions. Returns the transaction with updated metadata and spent
    /// output ids that don't need to be locked anymore
    /// Return true if a transaction got confirmed for which we don't have an output already, based on this outputs will
    /// be synced again
    pub(crate) async fn sync_pending_transactions(&self) -> Result<bool, WalletError> {
        log::debug!("[SYNC] sync pending transactions");
        let network_id = self.client().get_network_id().await?;
        let wallet_ledger = self.ledger().await;

        // only set to true if a transaction got confirmed for which we don't have an output
        // (transaction_output.is_none())
        let mut confirmed_unknown_output = false;

        if wallet_ledger.pending_transactions.is_empty() {
            return Ok(confirmed_unknown_output);
        }

        let mut updated_transactions = Vec::new();
        let mut spent_output_ids = Vec::new();
        // Inputs from conflicting transactions that are unspent, but should be removed from the locked outputs so they
        // are available again
        let mut output_ids_to_unlock = Vec::new();

        let pending_transactions = wallet_ledger
            .pending_transactions
            .iter()
            .copied()
            .map(|id| {
                (
                    id,
                    wallet_ledger
                        .transactions
                        .get(&id)
                        // panic during development to easier detect if something is wrong, should be handled different
                        // later
                        .expect("transaction id stored, but transaction is missing")
                        .clone(),
                )
            })
            .collect::<HashMap<_, _>>();

        drop(wallet_ledger);

        for (transaction_id, transaction) in pending_transactions {
            log::debug!("[SYNC] sync pending transaction {transaction_id}");

            // only check transaction from the network we're connected to
            if transaction.network_id != network_id {
                continue;
            }

            // check if we have an output (remainder, if not sending to an own address) that got created by this
            // transaction, if that's the case, then the transaction got confirmed
            let transaction_output = self
                .ledger()
                .await
                .outputs
                .keys()
                .find(|o| o.transaction_id() == &transaction_id)
                .copied();

            if let Some(transaction_output) = transaction_output {
                {
                    let wallet_ledger = self.ledger().await;
                    // Safe to unwrap, we just got the output
                    let confirmed_output_data = wallet_ledger.outputs.get(&transaction_output).expect("output exists");
                    log::debug!(
                        "[SYNC] confirmed transaction {transaction_id} in block {}",
                        confirmed_output_data.metadata.block_id()
                    );
                    updated_transaction_and_outputs(
                        transaction,
                        Some(*confirmed_output_data.metadata.block_id()),
                        InclusionState::Confirmed,
                        &mut updated_transactions,
                        &mut spent_output_ids,
                    );
                }
                continue;
            }

            // Check if the inputs of the transaction are still unspent
            let mut input_got_spent = false;
            for input in transaction.payload.transaction().inputs() {
                let Input::Utxo(input) = input;
                if let Some(input) = self.ledger().await.outputs.get(input.output_id()) {
                    if input.metadata.is_spent() {
                        input_got_spent = true;
                    }
                }
            }

            match self.client().get_transaction_metadata(&transaction_id).await {
                Ok(metadata) => {
                    match metadata.transaction_state {
                        // TODO: Separate TransactionState::Finalized, TransactionState::Accepted? https://github.com/iotaledger/iota-sdk/issues/1814
                        TransactionState::Accepted | TransactionState::Committed | TransactionState::Finalized => {
                            log::debug!(
                                "[SYNC] confirmed transaction {transaction_id} in slot {}",
                                metadata.earliest_attachment_slot
                            );
                            confirmed_unknown_output = true;
                            updated_transaction_and_outputs(
                                transaction,
                                // Some(metadata.block_id),
                                None,
                                InclusionState::Confirmed,
                                &mut updated_transactions,
                                &mut spent_output_ids,
                            );
                        }
                        TransactionState::Failed => {
                            // try to get the included block, because maybe only this attachment is
                            // conflicting because it got confirmed in another block
                            if let Ok(included_block) = self
                                .client()
                                .get_included_block(&transaction.payload.transaction().id())
                                .await
                            {
                                confirmed_unknown_output = true;
                                updated_transaction_and_outputs(
                                    transaction,
                                    Some(self.client().block_id(&included_block).await?),
                                    // block metadata was Conflicting, but it's confirmed in another attachment
                                    InclusionState::Confirmed,
                                    &mut updated_transactions,
                                    &mut spent_output_ids,
                                );
                            } else {
                                log::debug!("[SYNC] conflicting transaction {transaction_id}");
                                updated_transaction_and_outputs(
                                    transaction,
                                    None,
                                    InclusionState::Conflicting,
                                    &mut updated_transactions,
                                    &mut spent_output_ids,
                                );
                            }
                        }
                        // Do nothing, just need to wait a bit more
                        TransactionState::Pending => {}
                    }
                    // else if input_got_spent {
                    //     process_transaction_with_unknown_state(
                    //         &*self.ledger().await,
                    //         transaction,
                    //         &mut updated_transactions,
                    //         &mut output_ids_to_unlock,
                    //     )?;
                    // }
                }
                Err(ClientError::Node(crate::client::node_api::error::Error::NotFound(_))) => {
                    if input_got_spent {
                        process_transaction_with_unknown_state(
                            &*self.ledger().await,
                            transaction,
                            &mut updated_transactions,
                            &mut output_ids_to_unlock,
                        )?;
                    }
                }
                Err(e) => return Err(e.into()),
            }

            //  else if input_got_spent {
            //     process_transaction_with_unknown_state(
            //         &*self.ledger().await,
            //         transaction,
            //         &mut updated_transactions,
            //         &mut output_ids_to_unlock,
            //     )?;
            // }
            // else {
            //     // Reissue if there was no block id yet, because then we also didn't burn any mana
            //     log::debug!("[SYNC] reissue transaction {}", transaction.transaction_id);
            //     let reissued_block = self
            //         .submit_signed_transaction(transaction.payload.clone(), None)
            //         .await?;
            //     transaction.block_id.replace(reissued_block);
            //     updated_transactions.push(transaction);
            // }
        }

        // updates account with balances, output ids, outputs
        self.update_with_transactions(updated_transactions, spent_output_ids, output_ids_to_unlock)
            .await?;

        Ok(confirmed_unknown_output)
    }
}

// Set the outputs as spent so they will not be used as input again
fn updated_transaction_and_outputs(
    mut transaction: TransactionWithMetadata,
    block_id: Option<BlockId>,
    inclusion_state: InclusionState,
    updated_transactions: &mut Vec<TransactionWithMetadata>,
    spent_output_ids: &mut Vec<OutputId>,
) {
    transaction.block_id = block_id;
    transaction.inclusion_state = inclusion_state;
    // get spent inputs
    for input in transaction.payload.transaction().inputs() {
        let Input::Utxo(input) = input;
        spent_output_ids.push(*input.output_id());
    }
    updated_transactions.push(transaction);
}

// When a transaction got pruned, the inputs and outputs are also not available, then this could mean that it was
// confirmed and the created outputs got also already spent and pruned or the inputs got spent in another transaction
fn process_transaction_with_unknown_state(
    wallet_ledger: &WalletLedger,
    mut transaction: TransactionWithMetadata,
    updated_transactions: &mut Vec<TransactionWithMetadata>,
    output_ids_to_unlock: &mut Vec<OutputId>,
) -> Result<(), WalletError> {
    let mut all_inputs_spent = true;
    for input in transaction.payload.transaction().inputs() {
        let Input::Utxo(input) = input;
        if let Some(output_data) = wallet_ledger.outputs.get(input.output_id()) {
            if !output_data.is_spent() {
                // unspent output needs to be made available again
                output_ids_to_unlock.push(*input.output_id());
                all_inputs_spent = false;
            }
        } else {
            all_inputs_spent = false;
        }
    }
    // If only a part of the inputs got spent, then it couldn't happen with this transaction, so it's conflicting
    if all_inputs_spent {
        transaction.inclusion_state = InclusionState::UnknownPruned;
    } else {
        log::debug!("[SYNC] conflicting transaction {}", transaction.transaction_id);
        transaction.inclusion_state = InclusionState::Conflicting;
    }
    updated_transactions.push(transaction);
    Ok(())
}
