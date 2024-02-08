// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;
use packable::bounded::TryIntoBoundedU16Error;

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{input::INPUT_COUNT_MAX, output::Output},
    wallet::{operations::transaction::TransactionOptions, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Get inputs and build the transaction
    pub async fn prepare_transaction(
        &self,
        outputs: impl Into<Vec<Output>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_transaction");
        let options = options.into().unwrap_or_default();
        let outputs = outputs.into();
        let prepare_transaction_start_time = Instant::now();
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        // Check if the outputs have enough amount to cover the storage deposit
        for output in &outputs {
            output.verify_storage_deposit(storage_score_params)?;
        }

        if options.mandatory_inputs.len() as u16 > INPUT_COUNT_MAX {
            return Err(crate::types::block::Error::InvalidInputCount(
                TryIntoBoundedU16Error::Truncated(options.mandatory_inputs.len()),
            ))?;
        }

        let selected_transaction_data = self.select_inputs(outputs, options.clone()).await?;

        let prepared_transaction_data = match self.build_transaction(selected_transaction_data.clone(), options).await {
            Ok(res) => res,
            Err(err) => {
                // unlock outputs so they are available for a new transaction
                self.unlock_inputs(&selected_transaction_data.inputs).await?;
                return Err(err);
            }
        };

        log::debug!(
            "[TRANSACTION] finished prepare_transaction in {:.2?}",
            prepare_transaction_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
