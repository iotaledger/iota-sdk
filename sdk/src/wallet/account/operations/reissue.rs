// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManage, Error as ClientError},
    types::{
        api::core::response::LedgerInclusionState,
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

impl<S: 'static + SecretManage> Account<S>
where
    Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Reissues a transaction sent from the account for a provided transaction id until it's
    /// included (referenced by a milestone). Returns the included block id.
    pub async fn reissue_transaction_until_included(
        &self,
        transaction_id: &TransactionId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> crate::wallet::Result<BlockId> {
        log::debug!("[reissue_transaction_until_included]");

        let protocol_params = self.client().get_protocol_parameters().await?;

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

            let block_id = match transaction.block_id {
                Some(block_id) => block_id,
                None => self
                    .client()
                    .finish_basic_block_builder(
                        todo!("issuer id"),
                        todo!("issuing time"),
                        None,
                        Some(Payload::Transaction(Box::new(transaction.payload.clone()))),
                        self.wallet.coin_type(),
                        &*self.get_secret_manager().read().await,
                    )
                    .await?
                    .id(&protocol_params),
            };

            // Attachments of the Block to check inclusion state
            // TODO: remove when todos in `finish_basic_block_builder()` are removed
            #[allow(unused_mut)]
            let mut block_ids = vec![block_id];
            for _ in 0..max_attempts.unwrap_or(DEFAULT_REISSUE_UNTIL_INCLUDED_MAX_AMOUNT) {
                let duration =
                    std::time::Duration::from_secs(interval.unwrap_or(DEFAULT_REISSUE_UNTIL_INCLUDED_INTERVAL));

                #[cfg(target_family = "wasm")]
                gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;

                #[cfg(not(target_family = "wasm"))]
                tokio::time::sleep(duration).await;

                // Check inclusion state for each attachment
                let block_ids_len = block_ids.len();
                let mut conflicting = false;
                for (index, block_id_) in block_ids.clone().iter().enumerate() {
                    let block_metadata = self.client().get_block_metadata(block_id_).await?;
                    if let Some(inclusion_state) = block_metadata.ledger_inclusion_state {
                        match inclusion_state {
                            LedgerInclusionState::Included | LedgerInclusionState::NoTransaction => {
                                return Ok(*block_id_);
                            }
                            // only set it as conflicting here and don't return, because another reissued block could
                            // have the included transaction
                            LedgerInclusionState::Conflicting => conflicting = true,
                        };
                    }
                    // Only reissue latest attachment of the block
                    if index == block_ids_len - 1 && block_metadata.should_reattach.unwrap_or(false) {
                        let reissued_block = self
                            .client()
                            .finish_basic_block_builder(
                                todo!("issuer id"),
                                todo!("issuing time"),
                                None,
                                Some(Payload::Transaction(Box::new(transaction.payload.clone()))),
                                self.wallet.coin_type(),
                                &*self.get_secret_manager().read().await,
                            )
                            .await?;
                        block_ids.push(reissued_block.id(&protocol_params));
                    }
                }
                // After we checked all our reissued blocks, check if the transaction got reissued in another block
                // and confirmed
                if conflicting {
                    let included_block = self.client().get_included_block(transaction_id).await.map_err(|e| {
                        if matches!(e, ClientError::Node(crate::client::node_api::error::Error::NotFound(_))) {
                            // If no block was found with this transaction id, then it can't get included
                            ClientError::TangleInclusion(block_id.to_string())
                        } else {
                            e
                        }
                    })?;
                    return Ok(included_block.id(&protocol_params));
                }
            }
            Err(ClientError::TangleInclusion(block_id.to_string()).into())
        } else {
            Err(Error::TransactionNotFound(*transaction_id))
        }
    }
}
