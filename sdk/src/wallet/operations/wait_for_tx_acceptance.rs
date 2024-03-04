// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManage, Error as ClientError},
    types::block::payload::signed_transaction::TransactionId,
    wallet::{types::InclusionState, Error, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Checks the transaction state for a provided transaction id until it's accepted. Interval in milliseconds.
    pub async fn wait_for_transaction_acceptance(
        &self,
        transaction_id: &TransactionId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[wait_for_transaction_acceptance]");

        let transaction = self
            .ledger()
            .await
            .transactions
            .get(transaction_id)
            .cloned()
            .ok_or_else(|| Error::TransactionNotFound(*transaction_id))?;

        if matches!(
            transaction.inclusion_state,
            InclusionState::Accepted | InclusionState::Confirmed | InclusionState::Finalized
        ) {
            return Ok(());
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

        self.client()
            .wait_for_transaction_acceptance(transaction_id, interval, max_attempts)
            .await?;

        Ok(())
    }
}
