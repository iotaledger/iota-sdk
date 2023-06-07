// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::api::PreparedTransactionData,
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
            },
            BasicOutputBuilder, NativeToken, TokenId,
        },
        ConvertTo,
    },
    wallet::{
        account::{
            constants::DEFAULT_EXPIRATION_TIME,
            operations::transaction::{
                high_level::minimum_storage_deposit::minimum_storage_deposit_basic_native_tokens, Transaction,
            },
            Account, TransactionOptions,
        },
        Error, Result,
    },
};

/// Params for `send_native_tokens()`
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct SendNativeTokensParams {
    /// Bech32 encoded address
    #[getset(get = "pub")]
    address: Bech32Address,
    /// Native tokens
    #[getset(get = "pub")]
    native_tokens: Vec<(TokenId, U256)>,
    /// Bech32 encoded address return address, to which the storage deposit will be returned. Default will use the
    /// first address of the account
    #[getset(get = "pub")]
    return_address: Option<Bech32Address>,
    /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
    /// receiver before. Default is 1 day
    #[getset(get = "pub")]
    expiration: Option<u32>,
}

impl SendNativeTokensParams {
    /// Creates a new instance of [`SendNativeTokensParams`]
    pub fn new(
        address: impl ConvertTo<Bech32Address>,
        native_tokens: impl IntoIterator<Item = (TokenId, U256)>,
    ) -> Result<Self> {
        Ok(Self {
            address: address.convert()?,
            native_tokens: native_tokens.into_iter().collect(),
            return_address: None,
            expiration: None,
        })
    }

    /// Set the return address and try convert to [`Bech32Address`]
    pub fn try_with_return_address(mut self, return_address: impl ConvertTo<Bech32Address>) -> Result<Self> {
        self.return_address = Some(return_address.convert()?);
        Ok(self)
    }

    /// Set the return address
    pub fn with_return_address(mut self, return_address: impl Into<Option<Bech32Address>>) -> Self {
        self.return_address = return_address.into();
        self
    }

    /// Set the expiration in seconds
    pub fn with_expiration(mut self, expiration_secs: Option<u32>) -> Self {
        self.expiration = expiration_secs;
        self
    }
}

impl Account {
    /// Function to send native tokens in basic outputs with a [StorageDepositReturnUnlockCondition] and
    /// [ExpirationUnlockCondition], so the storage deposit gets back to the sender and also that the sender gets access
    /// to the output again after a defined time (default 1 day),
    /// Calls [Account.send()](crate::account::Account.send) internally, the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let outputs = [SendNativeTokensParams {
    ///     address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
    ///     native_tokens: vec![(
    ///         TokenId::from_str("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")?,
    ///         U256::from(50),
    ///     )],
    ///     ..Default::default()
    /// }];
    ///
    /// let tx = account.send_native_tokens(outputs, None).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_native_tokens<I: IntoIterator<Item = SendNativeTokensParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction>
    where
        I::IntoIter: Send,
    {
        let prepared_transaction = self.prepare_send_native_tokens(params, options).await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [Account.send_native_tokens()](crate::account::Account.send_native_tokens)
    pub async fn prepare_send_native_tokens<I: IntoIterator<Item = SendNativeTokensParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData>
    where
        I::IntoIter: Send,
    {
        log::debug!("[TRANSACTION] prepare_send_native_tokens");
        let rent_structure = self.client().get_rent_structure().await?;
        let token_supply = self.client().get_token_supply().await?;

        let account_addresses = self.addresses().await?;
        let default_return_address = account_addresses.first().ok_or(Error::FailedToGetRemainder)?;

        let local_time = self.client().get_time_checked().await?;

        let mut outputs = Vec::new();
        for SendNativeTokensParams {
            address,
            native_tokens,
            return_address,
            expiration,
        } in params
        {
            self.client().bech32_hrp_matches(address.hrp()).await?;
            let return_address = return_address
                .map(|addr| {
                    if address.hrp() != addr.hrp() {
                        Err(crate::client::Error::Bech32HrpMismatch {
                            provided: addr.hrp().to_string(),
                            expected: address.hrp().to_string(),
                        })?;
                    }
                    Ok::<_, Error>(addr)
                })
                .transpose()?
                .unwrap_or(default_return_address.address);

            // get minimum required amount for such an output, so we don't lock more than required
            // We have to check it for every output individually, because different address types and amount of
            // different native tokens require a different storage deposit
            let storage_deposit_amount = minimum_storage_deposit_basic_native_tokens(
                &rent_structure,
                address.inner(),
                return_address.inner(),
                Some(native_tokens.clone()),
                token_supply,
            )?;

            let expiration_time = expiration.map_or(local_time + DEFAULT_EXPIRATION_TIME, |expiration_time| {
                local_time + expiration_time
            });

            outputs.push(
                BasicOutputBuilder::new_with_amount(storage_deposit_amount)
                    .with_native_tokens(
                        native_tokens
                            .into_iter()
                            .map(|(id, amount)| {
                                NativeToken::new(id, amount)
                                    .map_err(|e| crate::wallet::Error::Client(Box::new(e.into())))
                            })
                            .collect::<Result<Vec<NativeToken>>>()?,
                    )
                    .add_unlock_condition(AddressUnlockCondition::new(address))
                    .add_unlock_condition(
                        // We send the full storage_deposit_amount back to the sender, so only the native tokens are
                        // sent
                        StorageDepositReturnUnlockCondition::new(return_address, storage_deposit_amount, token_supply)?,
                    )
                    .add_unlock_condition(ExpirationUnlockCondition::new(return_address, expiration_time)?)
                    .finish_output(token_supply)?,
            )
        }

        self.prepare_transaction(outputs, options).await
    }
}
