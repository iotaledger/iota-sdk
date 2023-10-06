// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use instant::Instant;
use packable::bounded::TryIntoBoundedU16Error;

#[cfg(feature = "events")]
use crate::wallet::events::types::{AddressData, TransactionProgressEvent, WalletEvent};
use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        input::INPUT_COUNT_RANGE,
        output::{Output, OUTPUT_COUNT_RANGE},
    },
    wallet::account::{
        operations::transaction::{RemainderValueStrategy, TransactionOptions},
        Account,
    },
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Get inputs and build the transaction essence
    pub async fn prepare_transaction(
        &self,
        outputs: impl Into<Vec<Output>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_transaction");
        let options = options.into();
        let outputs = outputs.into();
        let prepare_transaction_start_time = Instant::now();
        let rent_struct = self.client().get_rent_parameters().await?.into();
        let token_supply = self.client().get_token_supply().await?;

        // Check if the outputs have enough amount to cover the storage deposit
        for output in &outputs {
            output.verify_storage_deposit(rent_struct, token_supply)?;
        }

        let is_burn_present = options.as_ref().map(|options| options.burn.is_some()).unwrap_or(false);

        // Validate the number of outputs. The validation shouldn't be performed if [`Burn`] is present.
        // The outputs will be generated by the input selection algorithm (ISA).
        if !OUTPUT_COUNT_RANGE.contains(&(outputs.len() as u16)) && !is_burn_present {
            return Err(crate::types::block::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(outputs.len()),
            ))?;
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

        let remainder_address = match &options {
            Some(options) => {
                match &options.remainder_value_strategy {
                    RemainderValueStrategy::ReuseAddress => {
                        // select_inputs will select an address from the inputs if it's none
                        None
                    }
                    RemainderValueStrategy::ChangeAddress => {
                        let remainder_address = self.generate_remainder_address().await?;
                        #[cfg(feature = "events")]
                        {
                            let account_index = self.details().await.index;
                            self.emit(
                                account_index,
                                WalletEvent::TransactionProgress(
                                    TransactionProgressEvent::GeneratingRemainderDepositAddress(AddressData {
                                        address: remainder_address.address,
                                    }),
                                ),
                            )
                            .await;
                        }
                        Some(remainder_address.address().inner)
                    }
                    RemainderValueStrategy::CustomAddress(address) => Some(*address),
                }
            }
            None => None,
        };

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
            )
            .await?;

        let prepared_transaction_data = match self
            .build_transaction_essence(selected_transaction_data.clone(), options)
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
