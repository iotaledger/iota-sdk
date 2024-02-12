// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManage, Error as ClientError},
    types::{
        api::core::TransactionState,
        block::{payload::signed_transaction::TransactionId, BlockId},
    },
    wallet::{types::InclusionState, Error, Wallet},
};

// Time in milliseconds
const DEFAULT_AWAIT_TX_ACCEPTANCE_INTERVAL: u64 = 500;
const DEFAULT_AWAIT_TX_ACCEPTANCE_MAX_AMOUNT: u64 = 80;

impl<S: 'static + SecretManage> Wallet<S>
where
    Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Checks the transaction state for a provided transaction id until it's accepted. Interval in milliseconds.
    /// Returns the block id that contains this transaction.
    pub async fn wait_for_transaction_acceptance(
        &self,
        transaction_id: &TransactionId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[wait_for_transaction_acceptance]");

        let transaction = self
            .data()
            .await
            .transactions
            .get(transaction_id)
            .cloned()
            .ok_or_else(|| Error::TransactionNotFound(*transaction_id))?;

        if matches!(
            transaction.inclusion_state,
            InclusionState::Accepted | InclusionState::Confirmed | InclusionState::Finalized
        ) {
            return Ok(transaction
                .block_id
                // Safe to unwrap, we always set the block id together with any of these inclusion states.
                .expect("missing block id in TransactionWithMetadata"));
        }

        if matches!(
            transaction.inclusion_state,
            InclusionState::Conflicting | InclusionState::UnknownPruned
        ) {
            return Err(ClientError::TransactionAcceptance(format!(
                "transaction id: {} inclusion state: {:?}",
                transaction_id, transaction.inclusion_state
            ))
            .into());
        }

        let block_id = transaction
            .block_id
            .ok_or_else(|| ClientError::TransactionAcceptance(transaction_id.to_string()))?;

        let duration = std::time::Duration::from_millis(interval.unwrap_or(DEFAULT_AWAIT_TX_ACCEPTANCE_INTERVAL));
        for _ in 0..max_attempts.unwrap_or(DEFAULT_AWAIT_TX_ACCEPTANCE_MAX_AMOUNT) {
            #[cfg(target_family = "wasm")]
            gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;

            #[cfg(not(target_family = "wasm"))]
            tokio::time::sleep(duration).await;

            let block_metadata = self.client().get_block_metadata(&block_id).await?;
            if let Some(transaction_state) = block_metadata.transaction_metadata.map(|m| m.transaction_state) {
                match transaction_state {
                    TransactionState::Accepted | TransactionState::Confirmed | TransactionState::Finalized => {
                        return Ok(block_id);
                    }
                    TransactionState::Failed => {
                        // Check if the transaction got reissued in another block and confirmed there
                        let included_block = self.client().get_included_block(transaction_id).await.map_err(|e| {
                            if matches!(e, ClientError::Node(crate::client::node_api::error::Error::NotFound(_))) {
                                // If no block was found with this transaction id, then it couldn't get accepted
                                ClientError::TransactionAcceptance(transaction_id.to_string())
                            } else {
                                e
                            }
                        })?;
                        let protocol_parameters = self.client().get_protocol_parameters().await?;
                        return Ok(included_block.id(&protocol_parameters));
                    }
                    TransactionState::Pending => {} // Just need to wait longer
                };
            }
        }
        Err(ClientError::TransactionAcceptance(transaction_id.to_string()).into())
    }
}
