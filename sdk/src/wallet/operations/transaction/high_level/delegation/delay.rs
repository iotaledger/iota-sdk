// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        context_input::{CommitmentContextInput, ContextInput},
        output::{AddressUnlockCondition, BasicOutput, DelegationId, DelegationOutputBuilder, MinimumOutputAmount},
    },
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Delay a delegation's claiming. The `reclaim_excess` flag indicates whether excess value over the minimum storage
    /// requirements will be moved to a basic output that is unlockable by the same address which controls the
    /// delegation. Otherwise it will be added to a new delegation. In this manner, one can delegate for one epoch
    /// at a time and never lose out on any rewards.
    pub async fn delay_delegation_claiming(
        &self,
        delegation_id: DelegationId,
        reclaim_excess: bool,
    ) -> crate::wallet::Result<TransactionWithMetadata> {
        let prepared_transaction = self
            .prepare_delay_delegation_claiming(delegation_id, reclaim_excess)
            .await?;

        self.sign_and_submit_transaction(prepared_transaction, None, None).await
    }

    /// Prepare to delay a delegation's claiming. The `reclaim_excess` flag indicates whether excess value
    /// over the minimum storage requirements will be moved to a basic output that is unlockable by the same address
    /// which controls the delegation.
    /// Otherwise it will be added to a new delegation. In this manner, one can delegate for one epoch at a time and
    /// never lose out on any rewards.
    pub async fn prepare_delay_delegation_claiming(
        &self,
        delegation_id: DelegationId,
        reclaim_excess: bool,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        let delegation_output = self.get_delegation_output(delegation_id).await.ok_or_else(|| {
            crate::wallet::Error::DelegationTransitionFailed(format!(
                "no delegation output found with id {delegation_id}"
            ))
        })?;
        let protocol_parameters = self.client().get_protocol_parameters().await?;
        // Add a commitment context input with the latest slot commitment
        let slot_commitment_id = self.client().get_info().await?.node_info.status.latest_commitment_id;
        let context_input = ContextInput::from(CommitmentContextInput::new(slot_commitment_id));

        let min_delegation_amount = delegation_output
            .output
            .minimum_amount(protocol_parameters.storage_score_parameters());

        // TODO: Should we return an error if `reclaim_excess == true` and `can_reclaim == false`?
        let can_reclaim = if reclaim_excess {
            let min_basic_amount = BasicOutput::minimum_amount(
                delegation_output.output.as_delegation().address(),
                protocol_parameters.storage_score_parameters(),
            );
            delegation_output.output.amount() >= min_delegation_amount + min_basic_amount
        } else {
            false
        };

        let mut builder = DelegationOutputBuilder::from(delegation_output.output.as_delegation())
            .with_delegation_id(delegation_id)
            .with_end_epoch(protocol_parameters.expected_end_epoch(slot_commitment_id));

        // In order to split the output, the amount must be at least twice the minimum for a delegation output
        let can_split = delegation_output.output.amount() >= 2 * min_delegation_amount;

        if can_split || can_reclaim {
            builder = builder.with_minimum_amount(protocol_parameters.storage_score_parameters());
        }

        let output = builder.finish_output()?;

        let mut outputs = Vec::new();

        // If we can split and we aren't reclaiming, we will create a new delegation with those funds
        if can_split && !reclaim_excess {
            outputs.push(
                DelegationOutputBuilder::new_with_amount(
                    delegation_output.output.amount() - output.amount(),
                    DelegationId::null(),
                    *output.as_delegation().validator_address(),
                )
                .with_start_epoch(protocol_parameters.expected_start_epoch(slot_commitment_id))
                .add_unlock_condition(AddressUnlockCondition::new(output.as_delegation().address().clone()))
                .finish_output()?,
            );
        }

        outputs.push(output);

        self.prepare_transaction(
            outputs,
            TransactionOptions {
                custom_inputs: Some(vec![delegation_output.output_id]),
                context_inputs: Some(vec![context_input]),
                ..Default::default()
            },
        )
        .await
    }
}
