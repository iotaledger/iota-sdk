// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        context_input::{CommitmentContextInput, ContextInput},
        output::{DelegationId, DelegationOutputBuilder},
    },
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    pub async fn delay_delegation_claiming(
        &self,
        delegation_id: DelegationId,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let prepared_transaction = self.prepare_delay_delegation_claiming(delegation_id).await?;

        self.sign_and_submit_transaction(prepared_transaction, None, None).await
    }

    pub async fn prepare_delay_delegation_claiming(
        &self,
        delegation_id: DelegationId,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        if let Some(delegation_output) = self.get_delegation_output(delegation_id).await {
            let protocol_parameters = self.client().get_protocol_parameters().await?;
            // Add a commitment context input with the latest slot commitment
            let slot_commitment_id = self.client().get_info().await?.node_info.status.latest_commitment_id;
            let context_input = ContextInput::from(CommitmentContextInput::new(slot_commitment_id));

            let future_bounded_slot_index = slot_commitment_id
                .slot_index()
                .future_bounded_slot(protocol_parameters.min_committable_age);
            let future_bounded_epoch_index =
                future_bounded_slot_index.to_epoch_index(protocol_parameters.slots_per_epoch_exponent);

            let registration_slot = (future_bounded_epoch_index + 1).registration_slot(
                protocol_parameters.slots_per_epoch_exponent,
                protocol_parameters.epoch_nearing_threshold,
            );

            let output = DelegationOutputBuilder::from(delegation_output.output.as_delegation())
                .with_delegation_id(delegation_id)
                .with_end_epoch(if future_bounded_slot_index < registration_slot {
                    future_bounded_epoch_index
                } else {
                    future_bounded_epoch_index + 1
                })
                .finish_output()?;

            self.prepare_transaction(
                [output],
                TransactionOptions {
                    custom_inputs: Some(vec![delegation_output.output_id]),
                    context_inputs: Some(vec![context_input]),
                    ..Default::default()
                },
            )
            .await
        } else {
            Err(crate::wallet::Error::DelegationTransitionFailed(format!(
                "no delegation output found with id {delegation_id}"
            )))
        }
    }
}
