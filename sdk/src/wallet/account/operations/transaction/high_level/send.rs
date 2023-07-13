// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
            },
            BasicOutputBuilder, MinimumStorageDepositBasicOutput,
        },
        ConvertTo,
    },
    wallet::{
        account::{
            constants::DEFAULT_EXPIRATION_TIME, operations::transaction::Transaction, Account, TransactionOptions,
        },
        Error,
    },
};

/// Parameters for `send()`
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct SendParams {
    /// Amount
    #[serde(with = "crate::utils::serde::string")]
    #[getset(get = "pub")]
    amount: u64,
    /// Bech32 encoded address
    #[getset(get = "pub")]
    address: Bech32Address,
    /// Bech32 encoded return address, to which the storage deposit will be returned if one is necessary
    /// given the provided amount. If a storage deposit is needed and a return address is not provided, it will
    /// default to the first address of the account.
    #[getset(get = "pub")]
    return_address: Option<Bech32Address>,
    /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
    /// receiver already. The expiration will only be used if one is necessary given the provided amount. If an
    /// expiration is needed but not provided, it will default to one day.
    #[getset(get = "pub")]
    expiration: Option<u32>,
}

impl SendParams {
    pub fn new(amount: u64, address: impl ConvertTo<Bech32Address>) -> Result<Self, crate::wallet::Error> {
        Ok(Self {
            amount,
            address: address.convert()?,
            return_address: None,
            expiration: None,
        })
    }

    pub fn try_with_return_address(
        mut self,
        address: impl ConvertTo<Bech32Address>,
    ) -> Result<Self, crate::wallet::Error> {
        self.return_address = Some(address.convert()?);
        Ok(self)
    }

    pub fn with_return_address(mut self, address: impl Into<Option<Bech32Address>>) -> Self {
        self.return_address = address.into();
        self
    }

    pub fn with_expiration(mut self, expiration: impl Into<Option<u32>>) -> Self {
        self.expiration = expiration.into();
        self
    }
}

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Sends a certain amount of base coins to a single address.
    ///
    /// Calls [Account::send_with_params()](crate::wallet::Account::send_with_params) internally.
    /// The options may define the remainder value strategy or custom inputs.
    /// The provided Addresses provided with [`SendParams`] need to be bech32-encoded.
    pub async fn send(
        &self,
        amount: u64,
        address: impl ConvertTo<Bech32Address>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let params = [SendParams::new(amount, address)?];
        self.send_with_params(params, options).await
    }

    /// Sends a certain amount of base coins with full customizability of the transaction.
    ///
    /// Calls [Account::send_outputs()](crate::wallet::Account::send_outputs) internally.
    /// The options may define the remainder value strategy or custom inputs.
    /// Addresses provided with [`SendParams`] need to be bech32-encoded.
    /// ```ignore
    /// let params = [SendParams::new(
    ///     "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    ///     1_000_000)?
    /// ];
    ///
    /// let tx = account.send(params, None ).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_with_params<I: IntoIterator<Item = SendParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction>
    where
        I::IntoIter: Send,
    {
        let options = options.into();
        let prepared_transaction = self.prepare_send(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for
    /// [Account::send()](crate::wallet::Account::send).
    pub async fn prepare_send<I: IntoIterator<Item = SendParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData>
    where
        I::IntoIter: Send,
    {
        log::debug!("[TRANSACTION] prepare_send");
        let options = options.into();
        let rent_structure = self.client().get_rent_structure().await?;
        let token_supply = self.client().get_token_supply().await?;

        let account_addresses = self.addresses().await?;
        let default_return_address = account_addresses.first().ok_or(Error::FailedToGetRemainder)?;

        let local_time = self.client().get_time_checked().await?;

        let mut outputs = Vec::new();
        for SendParams {
            address,
            amount,
            return_address,
            expiration,
        } in params
        {
            self.client().bech32_hrp_matches(address.hrp()).await?;
            let return_address = return_address
                .map(|return_address| {
                    if return_address.hrp() != address.hrp() {
                        Err(crate::client::Error::Bech32HrpMismatch {
                            provided: return_address.hrp().to_string(),
                            expected: address.hrp().to_string(),
                        })?;
                    }
                    Ok::<_, Error>(return_address)
                })
                .transpose()?
                .unwrap_or(default_return_address.address);

            // Get the minimum required amount for an output assuming it does not need a storage deposit.
            let output = BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .finish_output(token_supply)?;

            if amount >= output.amount() {
                outputs.push(
                    BasicOutputBuilder::from(output.as_basic())
                        .with_amount(amount)
                        .finish_output(token_supply)?,
                )
            } else {
                let expiration_time = expiration.map_or(local_time + DEFAULT_EXPIRATION_TIME, |expiration_time| {
                    local_time + expiration_time
                });

                // Since it does need a storage deposit, calculate how much that should be
                let storage_deposit_amount = MinimumStorageDepositBasicOutput::new(rent_structure, token_supply)
                    .with_storage_deposit_return()?
                    .with_expiration()?
                    .finish()?;

                if !options.as_ref().map(|o| o.allow_micro_amount).unwrap_or_default() {
                    return Err(Error::InsufficientFunds {
                        available: amount,
                        required: amount + storage_deposit_amount,
                    });
                }

                outputs.push(
                    // Add address_and_amount.amount+storage_deposit_amount, so receiver can get
                    // address_and_amount.amount
                    BasicOutputBuilder::from(output.as_basic())
                        .with_amount(amount + storage_deposit_amount)
                        .add_unlock_condition(
                            // We send the storage_deposit_amount back to the sender, so only the additional amount is
                            // sent
                            StorageDepositReturnUnlockCondition::new(
                                return_address,
                                storage_deposit_amount,
                                token_supply,
                            )?,
                        )
                        .add_unlock_condition(ExpirationUnlockCondition::new(return_address, expiration_time)?)
                        .finish_output(token_supply)?,
                )
            }
        }

        self.prepare_transaction(outputs, options).await
    }
}
