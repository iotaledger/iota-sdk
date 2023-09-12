// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::Error as ClientError,
    types::{
        api::core::response::{BlockState, TransactionState},
        block::{
            payload::{transaction::TransactionId, Payload},
            BlockId,
        },
    },
    wallet::{
        account::{types::InclusionState, Account},
        Error,
    },
};

const DEFAULT_REISSUE_UNTIL_INCLUDED_INTERVAL: u64 = 1;
const DEFAULT_REISSUE_UNTIL_INCLUDED_MAX_AMOUNT: u64 = 40;

impl Account {
    /// Reissues a transaction sent from the account for a provided transaction id until it's
    /// included (referenced by a milestone). Returns the included block id.
    pub async fn reissue_transaction_until_included(
        &self,
        transaction_id: &TransactionId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[reissue_transaction_until_included]");

        let transaction = self.details().await.transactions.get(transaction_id).cloned();

        if let Some(transaction) = transaction {
            if transaction.inclusion_state == InclusionState::Confirmed {
                return transaction.block_id.ok_or(Error::MissingParameter("block id"));
            }

            if transaction.inclusion_state == InclusionState::Conflicting
                || transaction.inclusion_state == InclusionState::UnknownPruned
            {
                return Err(ClientError::TangleInclusion(format!(
                    "transaction id: {} inclusion state: {:?}",
                    transaction_id, transaction.inclusion_state
                ))
                .into());
            }

            let first_block_id = match transaction.block_id {
                Some(block_id) => block_id,
                None => self
                    .client()
                    .finish_basic_block_builder(
                        todo!("issuer id"),
                        todo!("block signature"),
                        todo!("issuing time"),
                        None,
                        Some(Payload::Transaction(Box::new(transaction.payload.clone()))),
                    )
                    .await?
                    .id(),
            };

            // Attachments of the Block to check inclusion state
            // TODO: remove when todos in `finish_basic_block_builder()` are removed
            #[allow(unused_mut)]
            let mut block_ids = vec![first_block_id];
            for _ in 0..max_attempts.unwrap_or(DEFAULT_REISSUE_UNTIL_INCLUDED_MAX_AMOUNT) {
                let duration =
                    std::time::Duration::from_secs(interval.unwrap_or(DEFAULT_REISSUE_UNTIL_INCLUDED_INTERVAL));

                #[cfg(target_family = "wasm")]
                gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;

                #[cfg(not(target_family = "wasm"))]
                tokio::time::sleep(duration).await;

                // Check inclusion state for each attachment
                let block_ids_len = block_ids.len();
                let mut failed = false;
                for (index, block_id) in block_ids.clone().iter().enumerate() {
                    let block_metadata = self.client().get_block_metadata(block_id).await?;
                    if let Some(transaction_state) = block_metadata.tx_state {
                        match transaction_state {
                            // TODO: find out what to do with TransactionState::Confirmed
                            TransactionState::Finalized => return Ok(*block_id),
                            // only set it as failed here and don't return, because another reissued block could
                            // have the included transaction
                            // TODO: check if the comment above is still correct with IOTA 2.0
                            TransactionState::Failed => failed = true,
                            // TODO: what to do when confirmed?
                            _ => {}
                        };
                    }
                    // Only reissue latest attachment of the block
                    let should_reissue = block_metadata
                        .block_state
                        .map_or(false, |block_state| block_state == BlockState::Rejected);
                    if index == block_ids_len - 1 && should_reissue {
                        let reissued_block = self
                            .client()
                            .finish_basic_block_builder(
                                todo!("issuer id"),
                                todo!("block signature"),
                                todo!("issuing time"),
                                None,
                                Some(Payload::Transaction(Box::new(transaction.payload.clone()))),
                            )
                            .await?;
                        block_ids.push(reissued_block.id());
                    }
                }
                // After we checked all our reissued blocks, check if the transaction got reissued in another block
                // and confirmed
                // TODO: can this still be the case? Is the TransactionState per transaction or per attachment in a
                // block?
                if failed {
                    let included_block = self.client().get_included_block(transaction_id).await.map_err(|e| {
                        if matches!(e, ClientError::Node(crate::client::node_api::error::Error::NotFound(_))) {
                            // If no block was found with this transaction id, then it can't get included
                            ClientError::TangleInclusion(first_block_id.to_string())
                        } else {
                            e
                        }
                    })?;
                    return Ok(included_block.id());
                }
            }
            Err(ClientError::TangleInclusion(first_block_id.to_string()).into())
        } else {
            Err(Error::TransactionNotFound(*transaction_id))
        }
    }
}
