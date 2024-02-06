// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::{
        api::{input_selection::Selected, transaction::validate_transaction_length, PreparedTransactionData},
        secret::SecretManage,
    },
    types::block::{
        input::{Input, UtxoInput},
        mana::ManaAllotment,
        output::AccountId,
        payload::signed_transaction::Transaction,
    },
    wallet::{operations::transaction::TransactionOptions, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Builds the transaction from the selected inputs and outputs.
    pub(crate) async fn build_transaction(
        &self,
        mut selected_transaction_data: Selected,
        issuer_id: impl Into<Option<AccountId>> + Send,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] build_transaction");

        let build_transaction_start_time = Instant::now();
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let mut inputs: Vec<Input> = Vec::new();

        for input in &selected_transaction_data.inputs {
            inputs.push(Input::Utxo(UtxoInput::from(*input.output_id())));
        }

        let first_account_id = self.data.read().await.first_account_id();
        let issuer_id = issuer_id
            .into()
            .or(first_account_id)
            .ok_or(crate::wallet::Error::AccountNotFound)?;
        let rmc = self
            .client()
            .get_account_congestion(&issuer_id, None)
            .await?
            .reference_mana_cost;
        let allotment_mana = selected_transaction_data.block_work_score as u64 * rmc;

        // Add the required allotment to the issuing allotment if it exists, or create a new one
        if let Some(allotment) = selected_transaction_data
            .mana_allotments
            .iter_mut()
            .find(|allotment| allotment.account_id() == &issuer_id)
        {
            allotment.mana += allotment_mana;
        } else {
            selected_transaction_data
                .mana_allotments
                .push(ManaAllotment::new(issuer_id, allotment_mana)?);
        }

        // Build transaction

        let mut builder = Transaction::builder(protocol_parameters.network_id())
            .with_inputs(inputs)
            .with_outputs(selected_transaction_data.outputs)
            .with_mana_allotments(selected_transaction_data.mana_allotments)
            .with_context_inputs(selected_transaction_data.context_inputs);

        if let Some(options) = options.into() {
            builder = builder.with_payload(options.tagged_data_payload);

            if let Some(capabilities) = options.capabilities {
                builder = builder.add_capabilities(capabilities.capabilities_iter());
            }
        }

        let transaction = builder.finish_with_params(&protocol_parameters)?;

        validate_transaction_length(&transaction)?;

        let prepared_transaction_data = PreparedTransactionData {
            transaction,
            inputs_data: selected_transaction_data.inputs,
            remainders: selected_transaction_data.remainders,
            mana_rewards: selected_transaction_data.mana_rewards.into_iter().collect(),
        };

        log::debug!(
            "[TRANSACTION] finished build_transaction in {:.2?}",
            build_transaction_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
