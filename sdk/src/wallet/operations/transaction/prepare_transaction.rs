// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use instant::Instant;
use packable::bounded::TryIntoBoundedU16Error;

use super::options::BlockOptions;
use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{input::INPUT_COUNT_RANGE, output::Output},
    wallet::{
        operations::transaction::{RemainderValueStrategy, TransactionOptions},
        Wallet,
    },
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
        options: impl Into<Option<BlockOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_transaction");
        let block_options = options.into();
        let options = block_options
            .as_ref()
            .map(|o| o.transaction_options.as_ref())
            .unwrap_or_default();

        let outputs = outputs.into();
        let prepare_transaction_start_time = Instant::now();
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        // Check if the outputs have enough amount to cover the storage deposit
        for output in &outputs {
            output.verify_storage_deposit(storage_score_params)?;
        }

        if let Some(custom_inputs) = options.as_ref().and_then(|options| options.custom_inputs.as_ref()) {
            // validate inputs amount
            if !INPUT_COUNT_RANGE.contains(&(custom_inputs.len() as u16)) {
                return Err(crate::types::block::Error::InvalidInputCount(
                    TryIntoBoundedU16Error::Truncated(custom_inputs.len()),
                ))?;
            }
        }

        if let Some(mandatory_inputs) = options.as_ref().and_then(|options| options.mandatory_inputs.as_ref()) {
            // validate inputs amount
            if !INPUT_COUNT_RANGE.contains(&(mandatory_inputs.len() as u16)) {
                return Err(crate::types::block::Error::InvalidInputCount(
                    TryIntoBoundedU16Error::Truncated(mandatory_inputs.len()),
                ))?;
            }
        }

        let remainder_address = options
            .as_ref()
            .and_then(|options| match &options.remainder_value_strategy {
                RemainderValueStrategy::ReuseAddress => None,
                RemainderValueStrategy::CustomAddress(address) => Some(address.clone()),
            });

        let selected_transaction_data = self
            .select_inputs(
                outputs,
                options
                    .as_ref()
                    .and_then(|options| options.custom_inputs.as_ref())
                    .map(|inputs| HashSet::from_iter(inputs.clone())),
                options
                    .as_ref()
                    .and_then(|options| options.mandatory_inputs.as_ref())
                    .map(|inputs| HashSet::from_iter(inputs.clone())),
                remainder_address,
                options.as_ref().and_then(|options| options.burn.as_ref()),
                options.as_ref().and_then(|options| options.mana_allotments.clone()),
            )
            .await?;

        let prepared_transaction_data = match self
            .build_transaction(selected_transaction_data.clone(), block_options)
            .await
        {
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
