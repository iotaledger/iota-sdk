// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::output::{AddressUnlockCondition, DelegationId, DelegationOutputBuilder, MinimumOutputAmount},
    wallet::{types::TransactionWithMetadata, Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Delay a delegation's claiming. The `reclaim_excess` flag indicates whether excess value over the minimum storage
    /// requirements will be moved to a basic output that is unlockable by the same address which controls the
    /// delegation. Otherwise it will be added to a new delegation. In this manner, one can delegate for one epoch
    /// at a time and never lose out on any rewards.
    pub async fn delay_delegation_claiming(
        &self,
        delegation_id: DelegationId,
        reclaim_excess: bool,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let prepared_transaction = self
            .prepare_delay_delegation_claiming(delegation_id, reclaim_excess)
            .await?;

        self.sign_and_submit_transaction(prepared_transaction, None).await
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
    ) -> Result<PreparedTransactionData, WalletError> {
        let delegation_output = self
            .ledger()
            .await
            .unspent_delegation_output(&delegation_id)
            .ok_or(WalletError::MissingDelegation(delegation_id))?
            .output
            .clone();
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let builder =
            DelegationOutputBuilder::from(delegation_output.as_delegation()).with_delegation_id(delegation_id);

        // If we're reclaiming the excess, just lower the amount to the min and allow ISA to create a remainder.
        // Otherwise, we'll try to split the delegation.
        let mut outputs = Vec::new();
        if reclaim_excess {
            outputs.push(
                builder
                    .with_minimum_amount(protocol_parameters.storage_score_parameters())
                    .finish_output()?,
            );
        } else {
            let min_delegation_amount =
                delegation_output.minimum_amount(protocol_parameters.storage_score_parameters());

            // In order to split the output, the amount must be at least twice the minimum for a delegation output
            if delegation_output.amount() >= 2 * min_delegation_amount {
                outputs.push(
                    builder
                        .with_minimum_amount(protocol_parameters.storage_score_parameters())
                        .finish_output()?,
                );

                outputs.push(
                    DelegationOutputBuilder::new_with_amount(
                        delegation_output.amount() - min_delegation_amount,
                        DelegationId::null(),
                        *delegation_output.as_delegation().validator_address(),
                    )
                    .add_unlock_condition(AddressUnlockCondition::new(
                        delegation_output.as_delegation().address().clone(),
                    ))
                    .finish_output()?,
                );
            } else {
                outputs.push(builder.finish_output()?);
            }
        };

        self.prepare_transaction(outputs, None).await
    }
}
