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
        context_input::{BlockIssuanceCreditContextInput, CommitmentContextInput, ContextInput, RewardContextInput},
        input::{Input, UtxoInput},
        output::{DelegationOutputBuilder, Output},
        payload::signed_transaction::Transaction,
    },
    wallet::{operations::transaction::TransactionOptions, Wallet},
};

impl<T> Wallet<T> {
    /// Builds the transaction from the selected in and outputs.
    pub(crate) async fn build_transaction(
        &self,
        mut selected_transaction_data: Selected,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] build_transaction");

        let build_transaction_start_time = Instant::now();
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let mut inputs: Vec<Input> = Vec::new();
        let mut context_inputs = HashSet::new();
        // TODO: Use for semantic validation https://github.com/iotaledger/iota-sdk/pull/1906
        let mut mana_rewards = 0;

        let issuance = self.client().get_issuance().await?;
        let latest_slot_commitment_id = issuance.latest_commitment.id();

        let mut needs_commitment_context = false;

        for (idx, input) in selected_transaction_data.inputs.iter().enumerate() {
            // Transitioning an issuer account requires a BlockIssuanceCreditContextInput.
            if let Output::Account(account) = &input.output {
                if account.features().block_issuer().is_some() {
                    context_inputs.insert(ContextInput::from(BlockIssuanceCreditContextInput::from(
                        account.account_id_non_null(input.output_id()),
                    )));
                }
            }

            // Inputs with timelock or expiration unlock condition require a CommitmentContextInput
            if input
                .output
                .unlock_conditions()
                .map_or(false, |u| u.iter().any(|u| u.is_timelock() || u.is_expiration()))
            {
                needs_commitment_context = true;
            }

            inputs.push(Input::Utxo(UtxoInput::from(*input.output_id())));

            if let Some(reward) = selected_transaction_data.mana_rewards.get(input.output_id()) {
                mana_rewards += *reward;
                context_inputs.insert(ContextInput::from(RewardContextInput::new(idx as _)?));
                needs_commitment_context = true;
            }
        }

        // TODO https://github.com/iotaledger/iota-sdk/issues/1937
        for output in selected_transaction_data
            .outputs
            .iter_mut()
            .filter(|o| o.is_delegation())
        {
            // Created delegations have their start epoch set, and delayed delegations have their end set
            if output.as_delegation().delegation_id().is_null() {
                *output = DelegationOutputBuilder::from(output.as_delegation())
                    .with_start_epoch(protocol_parameters.delegation_start_epoch(latest_slot_commitment_id))
                    .finish_output()?;
            } else {
                *output = DelegationOutputBuilder::from(output.as_delegation())
                    .with_end_epoch(protocol_parameters.delegation_end_epoch(latest_slot_commitment_id))
                    .finish_output()?;
            }
            needs_commitment_context = true;
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
        {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            needs_commitment_context = true;
        }

        if needs_commitment_context && !context_inputs.iter().any(|c| c.kind() == CommitmentContextInput::KIND) {
            // TODO https://github.com/iotaledger/iota-sdk/issues/1740
            context_inputs.insert(CommitmentContextInput::new(latest_slot_commitment_id).into());
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
