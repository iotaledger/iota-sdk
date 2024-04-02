// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use crate::{
    client::{node_api::indexer::query_parameters::OutputQueryParameters, Client, ClientError},
    types::{
        api::core::TransactionState,
        block::{address::ToBech32Ext, output::OutputId, payload::signed_transaction::TransactionId},
    },
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
            match self.get_transaction_metadata(transaction_id).await {
                Ok(transaction_metadata) => {
                    match transaction_metadata.transaction_state {
                        TransactionState::Accepted | TransactionState::Committed | TransactionState::Finalized => {
                            let slot_index = self.get_slot_index().await?;
                            let protocol_parameters = self.get_protocol_parameters().await?;
                            if let Ok(output) = self.get_output(&OutputId::new(*transaction_id, 0)).await {
                                if let Some(required_address) = output
                                    .output
                                    .required_address(slot_index, protocol_parameters.committable_age_range())?
                                {
                                    // Even though the output was created already, the indexer might take some time
                                    // until it returns the output id for the address, that's why we wait for this here.
                                    for _ in 0..20 {
                                        if let Ok(output_ids) = self
                                            .output_ids(OutputQueryParameters::new().unlockable_by_address(
                                                required_address.clone().to_bech32(protocol_parameters.bech32_hrp),
                                            ))
                                            .await
                                        {
                                            if output_ids.contains(&OutputId::new(*transaction_id, 0)) {
                                                return Ok(());
                                            }
                                        }
                                        let duration = std::time::Duration::from_millis(50);
                                        #[cfg(target_family = "wasm")]
                                        gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
                                        #[cfg(not(target_family = "wasm"))]
                                        tokio::time::sleep(duration).await;
                                    }
                                }
                            }
                            // Just wait a second if the output was not returned or the required_address is None, so
                            // that the output should then be available from the indexer.
                            let duration = std::time::Duration::from_secs(1);
                            #[cfg(target_family = "wasm")]
                            gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
                            #[cfg(not(target_family = "wasm"))]
                            tokio::time::sleep(duration).await;
                            return Ok(());
                        }
                        TransactionState::Failed => {
                            return Err(ClientError::TransactionAcceptance(transaction_id.to_string()));
                        }
                        TransactionState::Pending => {} // Just need to wait longer
                    };
                }
                Err(ClientError::Node(crate::client::node_api::error::Error::NotFound(_))) => {}
                Err(e) => return Err(e),
            };

            #[cfg(target_family = "wasm")]
            gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
            #[cfg(not(target_family = "wasm"))]
            tokio::time::sleep(duration).await;
        }

        Err(ClientError::TransactionAcceptance(transaction_id.to_string()))
    }
}
