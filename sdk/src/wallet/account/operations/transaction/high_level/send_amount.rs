// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::api::PreparedTransactionData,
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
            },
            BasicOutputBuilder,
        },
    },
    wallet::{
        account::{
            constants::DEFAULT_EXPIRATION_TIME,
            operations::transaction::{
                high_level::minimum_storage_deposit::minimum_storage_deposit_basic_native_tokens, Transaction,
            },
            Account, TransactionOptions,
        },
        Error,
    },
};

/// address with amount for `send_amount()`
#[derive(Debug, Clone)]
pub struct AddressWithAmount {
    /// Bech32 encoded address
    address: Bech32Address,
    /// Amount
    amount: u64,
    /// Bech32 encoded return address, to which the storage deposit will be returned if one is necessary
    /// given the provided amount. If a storage deposit is needed and a return address is not provided, it will
    /// default to the first address of the account.
    return_address: Option<Bech32Address>,
    /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
    /// receiver already. The expiration will only be used if one is necessary given the provided amount. If an
    /// expiration is needed but not provided, it will default to one day.
    expiration: Option<u32>,
}

impl AddressWithAmount {
    pub fn new(address: Bech32Address, amount: u64) -> Self {
        Self {
            address,
            amount,
            return_address: None,
            expiration: None,
        }
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

impl Account {
    /// Function to create basic outputs with which we then will call
    /// [Account.send()](crate::account::Account.send), the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let outputs = vec![AddressWithAmount{
    ///     address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
    ///     amount: 1_000_000,
    /// }];
    ///
    /// let tx = account.send_amount(outputs, None ).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_amount(
        &self,
        addresses_with_amount: Vec<AddressWithAmount>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let prepared_transaction = self.prepare_send_amount(addresses_with_amount, options).await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [Account.send_amount()](crate::account::Account.send_amount)
    pub async fn prepare_send_amount(
        &self,
        addresses_with_amount: Vec<AddressWithAmount>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_send_amount");
        let options = options.into();
        let rent_structure = self.client.get_rent_structure().await?;
        let token_supply = self.client.get_token_supply().await?;

        let account_addresses = self.addresses().await?;
        let default_return_address = account_addresses.first().ok_or(Error::FailedToGetRemainder)?;

        let local_time = self.client.get_time_checked().await?;

        let mut outputs = Vec::new();
        for AddressWithAmount {
            address,
            amount,
            return_address,
            expiration,
        } in addresses_with_amount
        {
            self.client.bech32_hrp_matches(address.hrp()).await?;
            let return_address = return_address
                .map(|return_address| {
                    if return_address.hrp() != address.hrp() {
                        Err(crate::client::Error::InvalidBech32Hrp {
                            provided: return_address.hrp().to_string(),
                            expected: address.hrp().to_string(),
                        })?;
                    }
                    Ok::<_, Error>(return_address)
                })
                .transpose()?
                .unwrap_or_else(|| default_return_address.address.clone());

            // Get the minimum required amount for an output assuming it does not need a storage deposit.
            let output = BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
                .add_unlock_condition(AddressUnlockCondition::new(&address))
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
                let storage_deposit_amount = minimum_storage_deposit_basic_native_tokens(
                    &rent_structure,
                    address.inner(),
                    return_address.inner(),
                    None,
                    token_supply,
                )?;

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
                                &return_address,
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
