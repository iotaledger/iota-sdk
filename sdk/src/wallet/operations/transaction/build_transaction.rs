// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use instant::Instant;

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
    wallet::{operations::transaction::TransactionOptions, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Builds the transaction from the selected inputs and outputs.
    pub(crate) async fn build_transaction(
        &self,
        selected_transaction_data: Selected,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] build_transaction");

        let build_transaction_start_time = Instant::now();
        let protocol_parameters = self.client().get_protocol_parameters().await?;

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

        // Build transaction

        // TODO: Add an appropriate mana allotment here for this account
        let mut builder = Transaction::builder(protocol_parameters.network_id())
            .with_inputs(inputs)
            .with_outputs(selected_transaction_data.outputs);

        if let Some(options) = options.into() {
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

        // BlockIssuanceCreditContextInput requires a CommitmentContextInput.
        if context_inputs
            .iter()
            .any(|c| c.kind() == BlockIssuanceCreditContextInput::KIND)
            && !context_inputs.iter().any(|c| c.kind() == CommitmentContextInput::KIND)
        {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            let issuance = self.client().get_issuance().await?;
            context_inputs.insert(CommitmentContextInput::new(issuance.latest_commitment.id()).into());
        }

        let transaction = builder
            .with_context_inputs(context_inputs)
            .finish_with_params(&protocol_parameters)?;

        validate_transaction_length(&transaction)?;

        let mut mana_rewards = 0;
        let transaction_id = transaction.id();
        for input in &selected_transaction_data.inputs {
            if match &input.output {
                // Validator Rewards
                Output::Account(account_input) => account_input.can_claim_rewards(
                    transaction
                        .outputs()
                        .iter()
                        .filter_map(|o| o.as_account_opt())
                        .find(|account_output| {
                            account_output.account_id() == &account_input.account_id_non_null(input.output_id())
                        }),
                ),
                // Delegator Rewards
                Output::Delegation(delegation_input) => delegation_input.can_claim_rewards(
                    transaction
                        .outputs()
                        .iter()
                        .filter_map(|o| o.as_delegation_opt())
                        .find(|delegation_output| {
                            delegation_output.delegation_id()
                                == &delegation_input.delegation_id_non_null(input.output_id())
                        }),
                ),
                _ => false,
            } {
                mana_rewards += self
                    .client()
                    .get_output_mana_rewards(input.output_id(), transaction_id.slot_index())
                    .await?
                    .rewards;
            }
        }

        let prepared_transaction_data = PreparedTransactionData {
            transaction,
            inputs_data: selected_transaction_data.inputs,
            remainder: selected_transaction_data.remainder,
            mana_rewards: Some(mana_rewards),
        };

        log::debug!(
            "[TRANSACTION] finished build_transaction in {:.2?}",
            build_transaction_start_time.elapsed()
        );
        Ok(prepared_transaction_data)
    }
}
