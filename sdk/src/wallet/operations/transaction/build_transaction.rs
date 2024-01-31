// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use instant::Instant;

use super::options::BlockOptions;
use crate::{
    client::{
        api::{input_selection::Selected, transaction::validate_transaction_length, PreparedTransactionData},
        secret::SecretManage,
    },
    types::block::{
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput, ContextInput},
        input::{Input, UtxoInput},
        output::Output,
        payload::signed_transaction::Transaction,
    },
    wallet::Wallet,
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Builds the transaction from the selected in and outputs.
    pub(crate) async fn build_transaction(
        &self,
        selected_transaction_data: Selected,
        options: impl Into<Option<BlockOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] build_transaction");

        let build_transaction_start_time = Instant::now();
        let protocol_parameters = self.client().get_protocol_parameters().await?;
        let options: Option<BlockOptions> = options.into();

        let mut inputs: Vec<Input> = Vec::new();
        let mut context_inputs = HashSet::new();

        for input in &selected_transaction_data.inputs {
            // Transitioning an issuer account requires a BlockIssuanceCreditContextInput.
            if let Output::Account(account) = &input.output {
                if account.features().block_issuer().is_some() {
                    context_inputs.insert(ContextInput::from(BlockIssuanceCreditContextInput::from(
                        account.account_id_non_null(input.output_id()),
                    )));
                }
            }

            inputs.push(Input::Utxo(UtxoInput::from(*input.output_id())));
        }

        // BlockIssuanceCreditContextInput requires a CommitmentContextInput.
        if context_inputs
            .iter()
            .any(|c| c.kind() == BlockIssuanceCreditContextInput::KIND)
            && !context_inputs.iter().any(|c| c.kind() == CommitmentContextInput::KIND)
        {
            let id = match options.as_ref().and_then(|o| o.latest_slot_commitment_id) {
                Some(id) => id,
                None => self.client().get_issuance().await?.latest_commitment.id(),
            };
            context_inputs.insert(CommitmentContextInput::new(id).into());
        }

        // Build transaction

        // TODO: Add an appropriate mana allotment here for this account
        let mut builder = Transaction::builder(protocol_parameters.network_id())
            .with_inputs(inputs)
            .with_outputs(selected_transaction_data.outputs);

        if let Some(options) = options.and_then(|o| o.transaction_options) {
            // Optional add a tagged payload
            builder = builder.with_payload(options.tagged_data_payload);

            if let Some(context_inputs_opt) = options.context_inputs {
                context_inputs.extend(context_inputs_opt);
            }

            if let Some(capabilities) = options.capabilities {
                builder = builder.add_capabilities(capabilities.capabilities_iter());
            }

            if let Some(mana_allotments) = options.mana_allotments {
                builder = builder.with_mana_allotments(mana_allotments);
            }
        }

        let transaction = builder
            .with_context_inputs(context_inputs)
            .finish_with_params(&protocol_parameters)?;

        validate_transaction_length(&transaction)?;

        let prepared_transaction_data = PreparedTransactionData {
            transaction,
            inputs_data: selected_transaction_data.inputs,
            remainders: selected_transaction_data.remainders,
        };

        log::debug!(
            "[TRANSACTION] finished build_transaction in {:.2?}",
            build_transaction_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
