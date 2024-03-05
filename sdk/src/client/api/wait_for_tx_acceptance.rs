// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use crate::{
    client::{Client, ClientError},
    types::{api::core::TransactionState, block::payload::signed_transaction::TransactionId},
};

pub(crate) const DEFAULT_WAIT_FOR_TX_ACCEPTANCE_INTERVAL: Duration = Duration::from_millis(500);
pub(crate) const DEFAULT_WAIT_FOR_TX_ACCEPTANCE_MAX_ATTEMPTS: u64 = 80;

impl Client {
    /// Checks the transaction state for a provided transaction id until it's accepted. Interval in milliseconds.
    pub async fn wait_for_transaction_acceptance(
        &self,
        transaction_id: &TransactionId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> Result<(), ClientError> {
        log::debug!("[wait_for_transaction_acceptance]");

        let duration = interval
            .map(std::time::Duration::from_millis)
            .unwrap_or(DEFAULT_WAIT_FOR_TX_ACCEPTANCE_INTERVAL);

        for _ in 0..max_attempts.unwrap_or(DEFAULT_WAIT_FOR_TX_ACCEPTANCE_MAX_ATTEMPTS) {
            let transaction_metadata = self.get_transaction_metadata(transaction_id).await?;

            match transaction_metadata.transaction_state {
                TransactionState::Accepted | TransactionState::Confirmed | TransactionState::Finalized => {
                    return Ok(());
                }
                TransactionState::Failed => return Err(ClientError::TransactionAcceptance(transaction_id.to_string())),
                TransactionState::Pending => {} // Just need to wait longer
            };

            #[cfg(target_family = "wasm")]
            gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
            #[cfg(not(target_family = "wasm"))]
            tokio::time::sleep(duration).await;
        }

        Err(ClientError::TransactionAcceptance(transaction_id.to_string()).into())
    }
}
