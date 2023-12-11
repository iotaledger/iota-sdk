// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::{
        api::{input_selection::Selected, transaction::validate_transaction_length, PreparedTransactionData},
        secret::{types::InputSigningData, SecretManage},
    },
    types::block::{
        input::{Input, UtxoInput},
        payload::signed_transaction::Transaction,
    },
    wallet::{operations::transaction::TransactionOptions, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Builds the transaction from the selected in and outputs.
    pub(crate) async fn build_transaction(
        &self,
        selected_transaction_data: Selected,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] build_transaction");

        let build_transaction_start_time = Instant::now();
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let mut inputs: Vec<Input> = Vec::new();
        let mut inputs_for_signing: Vec<InputSigningData> = Vec::new();

        for utxo in &selected_transaction_data.inputs {
            let input = Input::Utxo(UtxoInput::from(*utxo.output_id()));
            inputs.push(input.clone());
            inputs_for_signing.push(utxo.clone());
        }

        // Build transaction

        // TODO: Add an appropriate mana allotment here for this account
        let mut builder = Transaction::builder(protocol_parameters.network_id())
            .with_inputs(inputs)
            .with_outputs(selected_transaction_data.outputs);

        if let Some(options) = options.into() {
            // Optional add a tagged payload
            builder = builder.with_payload(options.tagged_data_payload);

            if let Some(context_inputs) = options.context_inputs {
                builder = builder.with_context_inputs(context_inputs);
            }

            if let Some(capabilities) = options.capabilities {
                builder = builder.add_capabilities(capabilities.capabilities_iter());
            }

            if let Some(mana_allotments) = options.mana_allotments {
                builder = builder.with_mana_allotments(mana_allotments);
            }
        }

        let transaction = builder.finish_with_params(&protocol_parameters)?;

        validate_transaction_length(&transaction)?;

        let prepared_transaction_data = PreparedTransactionData {
            transaction,
            inputs_data: inputs_for_signing,
            remainder: selected_transaction_data.remainder,
        };

        log::debug!(
            "[TRANSACTION] finished build_transaction in {:.2?}",
            build_transaction_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
