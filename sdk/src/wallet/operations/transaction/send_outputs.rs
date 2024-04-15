// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;
use packable::bounded::TryIntoBoundedU16Error;

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::{input::INPUT_COUNT_MAX, output::Output, payload::PayloadError},
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Sends a transaction by specifying its outputs.
    ///
    /// Note that, if sending a block fails, the method will return `None` for the block id, but the wallet
    /// will reissue the transaction during syncing.
    /// ```ignore
    /// let outputs = [
    ///    BasicOutputBuilder::new_with_amount(1_000_000)?
    ///    .add_unlock_condition(AddressUnlockCondition::new(
    ///        Address::try_from_bech32("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")?,
    ///    ))
    ///    .finish_output(account.client.get_token_supply().await?;)?,
    /// ];
    /// let tx = account
    ///     .send_outputs(
    ///         outputs,
    ///         Some(TransactionOptions {
    ///             remainder_value_strategy: RemainderValueStrategy::ReuseAddress,
    ///             ..Default::default()
    ///         }),
    ///     )
    ///     .await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_outputs(
        &self,
        outputs: impl Into<Vec<Output>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let outputs = outputs.into();
        let options = options.into();
        // here to check before syncing, how to prevent duplicated verification (also in prepare_send_outputs())?
        // Checking it also here is good to return earlier if something is invalid
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        // Check if the outputs have enough amount to cover the storage deposit
        for output in &outputs {
            output.verify_storage_deposit(protocol_parameters.storage_score_parameters())?;
        }

        let prepared_transaction_data = self.prepare_send_outputs(outputs, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction_data, options)
            .await
    }

    /// Get inputs and build the transaction
    pub async fn prepare_send_outputs(
        &self,
        outputs: impl IntoIterator<Item = Output> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] prepare_send_outputs");
        let options = options.into().unwrap_or_default();
        log::debug!("prepare_send_outputs {options:#?}");
        let outputs = outputs.into_iter().collect::<Vec<_>>();
        let prepare_send_outputs_start_time = Instant::now();
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        // Check if the outputs have enough amount to cover the storage deposit
        for output in &outputs {
            output.verify_storage_deposit(storage_score_params)?;
        }

        if options.required_inputs.len() as u16 > INPUT_COUNT_MAX {
            return Err(PayloadError::InputCount(TryIntoBoundedU16Error::Truncated(
                options.required_inputs.len(),
            )))?;
        }

        let prepared_transaction_data = self.build_transaction(outputs, options).await?;

        log::debug!(
            "[TRANSACTION] finished prepare_send_outputs in {:.2?}",
            prepare_send_outputs_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
