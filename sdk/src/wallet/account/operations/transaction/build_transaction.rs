// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::{
        api::{
            input_selection::Selected, transaction::validate_regular_transaction_essence_length,
            PreparedTransactionData,
        },
        secret::types::InputSigningData,
    },
    types::block::{
        input::{Input, UtxoInput},
        output::{InputsCommitment, Output},
        payload::transaction::{RegularTransactionEssence, TransactionEssence},
    },
    wallet::account::{operations::transaction::TransactionOptions, Account},
};

impl Account {
    /// Builds the transaction essence from the selected in and outputs.
    pub(crate) async fn build_transaction_essence(
        &self,
        selected_transaction_data: Selected,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] build_transaction");

        let build_transaction_essence_start_time = Instant::now();
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let mut inputs_for_essence: Vec<Input> = Vec::new();
        let mut inputs_for_signing: Vec<InputSigningData> = Vec::new();

        for utxo in &selected_transaction_data.inputs {
            let input = Input::Utxo(UtxoInput::from(*utxo.output_id()));
            inputs_for_essence.push(input.clone());
            inputs_for_signing.push(utxo.clone());
        }

        // Build transaction essence

        let input_outputs = inputs_for_signing
            .iter()
            .map(|i| i.output.clone())
            .collect::<Vec<Output>>();
        let inputs_commitment = InputsCommitment::new(input_outputs.iter());
        // TODO: Add an appropriate mana allotment here for this account
        let mut essence_builder =
            RegularTransactionEssence::builder(protocol_parameters.network_id(), inputs_commitment)
                .with_inputs(inputs_for_essence)
                .with_outputs(selected_transaction_data.outputs);

        // Optional add a tagged payload
        if let Some(options) = options.into() {
            essence_builder = essence_builder.with_payload(options.tagged_data_payload);
        }

        let essence = essence_builder.finish_with_params(protocol_parameters)?;

        validate_regular_transaction_essence_length(&essence)?;

        let essence = TransactionEssence::Regular(essence);

        let prepared_transaction_data = PreparedTransactionData {
            essence,
            inputs_data: inputs_for_signing,
            remainder: selected_transaction_data.remainder,
        };

        log::debug!(
            "[TRANSACTION] finished build_transaction in {:.2?}",
            build_transaction_essence_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
