// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod build_transaction;
pub(crate) mod high_level;
mod input_selection;
mod options;
pub(crate) mod prepare_output;
mod prepare_transaction;
mod sign_transaction;
pub(crate) mod submit_transaction;

pub use self::options::{RemainderValueStrategy, TransactionOptions};
use crate::{
    client::{
        api::{verify_semantic, PreparedTransactionData, SignedTransactionData},
        secret::{types::InputSigningData, SecretManage},
        Error,
    },
    types::{
        api::core::OutputWithMetadataResponse,
        block::{
            output::{dto::OutputDto, Output},
            payload::signed_transaction::SignedTransactionPayload,
        },
    },
    wallet::{
        types::{InclusionState, TransactionWithMetadata},
        Wallet,
    },
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
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
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let outputs = outputs.into();
        // here to check before syncing, how to prevent duplicated verification (also in prepare_transaction())?
        // Checking it also here is good to return earlier if something is invalid
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        // Check if the outputs have enough amount to cover the storage deposit
        for output in &outputs {
            output.verify_storage_deposit(protocol_parameters.rent_structure(), protocol_parameters.token_supply())?;
        }

        self.finish_transaction(outputs, options).await
    }

    /// Separated function from send, so syncing isn't called recursively with the consolidation function, which sends
    /// transactions
    pub async fn finish_transaction(
        &self,
        outputs: impl Into<Vec<Output>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        log::debug!("[TRANSACTION] finish_transaction");
        let options = options.into();

        let prepared_transaction_data = self.prepare_transaction(outputs, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction_data, options)
            .await
    }

    /// Signs a transaction, submit it to a node and store it in the wallet
    pub async fn sign_and_submit_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        log::debug!("[TRANSACTION] sign_and_submit_transaction");

        let signed_transaction_data = match self.sign_transaction(&prepared_transaction_data).await {
            Ok(res) => res,
            Err(err) => {
                // unlock outputs so they are available for a new transaction
                self.unlock_inputs(&prepared_transaction_data.inputs_data).await?;
                return Err(err);
            }
        };

        self.submit_and_store_transaction(signed_transaction_data, options)
            .await
    }

    /// Validates the transaction, submit it to a node and store it in the wallet
    pub async fn submit_and_store_transaction(
        &self,
        signed_transaction_data: SignedTransactionData,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        log::debug!(
            "[TRANSACTION] submit_and_store_transaction {}",
            signed_transaction_data.payload.transaction().id()
        );
        let options = options.into();

        // Validate transaction before sending and storing it
        let conflict = verify_semantic(
            &signed_transaction_data.inputs_data,
            &signed_transaction_data.payload,
            self.client().get_protocol_parameters().await?,
        )?;

        if let Some(conflict) = conflict {
            log::debug!(
                "[TRANSACTION] conflict: {conflict:?} for {:?}",
                signed_transaction_data.payload
            );
            // unlock outputs so they are available for a new transaction
            self.unlock_inputs(&signed_transaction_data.inputs_data).await?;
            return Err(Error::TransactionSemantic(conflict).into());
        }

        // Ignore errors from sending, we will try to send it again during [`sync_pending_transactions`]
        let block_id = match self
            .submit_transaction_payload(signed_transaction_data.payload.clone())
            .await
        {
            Ok(block_id) => Some(block_id),
            Err(err) => {
                log::error!("Failed to submit_transaction_payload {}", err);
                None
            }
        };

        let transaction_id = signed_transaction_data.payload.transaction().id();

        // store transaction payload to account (with db feature also store the account to the db)
        let network_id = self.client().get_network_id().await?;

        let inputs = signed_transaction_data
            .inputs_data
            .into_iter()
            .map(|input| OutputWithMetadataResponse {
                metadata: input.output_metadata,
                output: OutputDto::from(&input.output),
            })
            .collect();

        let transaction = TransactionWithMetadata {
            transaction_id,
            payload: signed_transaction_data.payload,
            block_id,
            network_id,
            timestamp: crate::client::unix_timestamp_now().as_millis(),
            inclusion_state: InclusionState::Pending,
            incoming: false,
            note: options.and_then(|o| o.note),
            inputs,
        };

        let mut wallet_data = self.data_mut().await;

        wallet_data.transactions.insert(transaction_id, transaction.clone());
        wallet_data.pending_transactions.insert(transaction_id);
        #[cfg(feature = "storage")]
        {
            // TODO: maybe better to use the wallt address as identifier now?
            log::debug!("[TRANSACTION] storing wallet");
            self.save(Some(&wallet_data)).await?;
        }

        Ok(transaction)
    }

    // unlock outputs
    async fn unlock_inputs(&self, inputs: &[InputSigningData]) -> crate::wallet::Result<()> {
        let mut wallet_data = self.data_mut().await;
        for input_signing_data in inputs {
            let output_id = input_signing_data.output_id();
            wallet_data.locked_outputs.remove(output_id);
            log::debug!(
                "[TRANSACTION] Unlocked output {} because of transaction error",
                output_id
            );
        }
        Ok(())
    }
}
