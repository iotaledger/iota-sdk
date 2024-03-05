// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::{
        output::{feature::StakingFeature, AccountId, AccountOutputBuilder},
        slot::EpochIndex,
    },
    utils::serde::string,
    wallet::{types::TransactionWithMetadata, TransactionOptions, Wallet, WalletError},
};

/// Parameters for beginning a staking period.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginStakingParams {
    /// The account id which will begin staking.
    pub account_id: AccountId,
    /// The amount of tokens to stake.
    #[serde(with = "string")]
    pub staked_amount: u64,
    /// The fixed cost of the validator, which it receives as part of its Mana rewards.
    #[serde(with = "string")]
    pub fixed_cost: u64,
    /// The staking period (in epochs). Will default to the staking unbonding period.
    pub staking_period: Option<u32>,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    pub async fn begin_staking(
        &self,
        params: BeginStakingParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let options = options.into();
        let prepared = self.prepare_begin_staking(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared, options).await
    }

    /// Prepares the transaction for [Wallet::begin_staking()].
    pub async fn prepare_begin_staking(
        &self,
        params: BeginStakingParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError> {
        log::debug!("[TRANSACTION] prepare_begin_staking");

        let account_id = params.account_id;
        let account_output_data = self
            .ledger()
            .await
            .unspent_account_output(&account_id)
            .cloned()
            .ok_or_else(|| WalletError::AccountNotFound)?;

        if account_output_data
            .output
            .features()
            .map_or(false, |f| f.staking().is_some())
        {
            return Err(WalletError::StakingFailed(format!(
                "account id {account_id} already has a staking feature"
            )));
        }

        let protocol_parameters = self.client().get_protocol_parameters().await?;

        if let Some(staking_period) = params.staking_period {
            if staking_period < protocol_parameters.staking_unbonding_period() {
                return Err(WalletError::StakingFailed(format!(
                    "staking period {staking_period} is less than the minimum {}",
                    protocol_parameters.staking_unbonding_period()
                )));
            }
        }

        let slot_commitment_id = self.client().get_issuance().await?.latest_commitment.id();
        let start_epoch = protocol_parameters.epoch_index_of(protocol_parameters.past_bounded_slot(slot_commitment_id));

        let output = AccountOutputBuilder::from(account_output_data.output.as_account())
            .with_account_id(account_id)
            .add_feature(StakingFeature::new(
                params.staked_amount,
                params.fixed_cost,
                start_epoch,
                params
                    .staking_period
                    .map(|period| start_epoch + period)
                    .unwrap_or(EpochIndex(u32::MAX)),
            ))
            .finish_output()?;

        let transaction = self.prepare_transaction([output], options).await?;

        Ok(transaction)
    }
}
