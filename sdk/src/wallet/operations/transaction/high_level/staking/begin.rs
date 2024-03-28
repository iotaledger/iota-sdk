// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{
        api::{
            options::TransactionOptions,
            transaction_builder::transition::{AccountChange, Transitions},
            PreparedTransactionData,
        },
        secret::SecretManage,
        ClientError,
    },
    types::block::output::AccountId,
    utils::serde::string,
    wallet::{types::TransactionWithMetadata, Wallet, WalletError},
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

        let change = AccountChange::BeginStaking {
            staked_amount: params.staked_amount,
            fixed_cost: params.fixed_cost,
            staking_period: params.staking_period,
        };

        let mut options = options.into();
        if let Some(options) = options.as_mut() {
            if let Some(transitions) = options.transitions.take() {
                options.transitions = Some(transitions.add_account(params.account_id, change));
            }
        } else {
            options.replace(TransactionOptions {
                transitions: Some(Transitions::new().add_account(params.account_id, change)),
                ..Default::default()
            });
        }

        let transaction = self.prepare_send_outputs(None, options).await?;

        Ok(transaction)
    }
}
