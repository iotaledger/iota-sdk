// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
            },
            BasicOutputBuilder, MinimumStorageDepositBasicOutput, NativeToken, TokenId,
        },
        slot::SlotIndex,
        ConvertTo,
    },
    wallet::{
        account::{
            constants::DEFAULT_EXPIRATION_SLOTS, operations::transaction::Transaction, Account, TransactionOptions,
        },
        Error, Result,
    },
};

/// Params for `send_native_token()`
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct SendNativeTokenParams {
    /// Bech32 encoded address
    #[getset(get = "pub")]
    address: Bech32Address,
    /// Native token
    #[getset(get = "pub")]
    native_token: (TokenId, U256),
    /// Bech32 encoded address return address, to which the storage deposit will be returned. Default will use the
    /// first address of the account
    #[getset(get = "pub")]
    return_address: Option<Bech32Address>,
    /// Expiration in slot indices, after which the output will be available for the sender again, if not spent by the
    /// receiver before. Default is [`DEFAULT_EXPIRATION_SLOTS`] slots.
    #[getset(get = "pub")]
    expiration: Option<SlotIndex>,
}

impl SendNativeTokenParams {
    /// Creates a new instance of [`SendNativeTokenParams`]
    pub fn new(address: impl ConvertTo<Bech32Address>, native_token: (TokenId, U256)) -> Result<Self> {
        Ok(Self {
            address: address.convert()?,
            native_token,
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
    pub fn with_expiration(mut self, expiration: Option<SlotIndex>) -> Self {
        self.expiration = expiration;
        self
    }
}

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Sends a native token in basic outputs with a [`StorageDepositReturnUnlockCondition`] and an
    /// [`ExpirationUnlockCondition`], so that the storage deposit is returned to the sender and the sender gets access
    /// to the output again after a predefined time (default 1 day).
    /// Calls [Account::send_outputs()](crate::wallet::Account::send_outputs) internally. The options may define the
    /// remainder value strategy or custom inputs. Note that the address needs to be bech32-encoded.
    /// ```ignore
    /// let params = [SendNativeTokenParams {
    ///     address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
    ///     native_token: (
    ///         TokenId::from_str("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")?,
    ///         U256::from(50),
    ///     ),
    ///     ..Default::default()
    /// }];
    ///
    /// let tx = account.send_native_token(params, None).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_native_token<I: IntoIterator<Item = SendNativeTokenParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction>
    where
        I::IntoIter: Send,
    {
        let options = options.into();
        let prepared_transaction = self.prepare_send_native_token(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for
    /// [Account::send_native_token()](crate::wallet::Account::send_native_token).
    pub async fn prepare_send_native_token<I: IntoIterator<Item = SendNativeTokenParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData>
    where
        I::IntoIter: Send,
    {
        log::debug!("[TRANSACTION] prepare_send_native_token");
        let rent_structure = self.client().get_rent_structure().await?;
        let token_supply = self.client().get_token_supply().await?;

        let account_addresses = self.addresses().await;
        let default_return_address = account_addresses
            .into_iter()
            .next()
            .ok_or(Error::FailedToGetRemainder)?;

        let slot_index = self.client().get_slot_index().await?;

        let mut outputs = Vec::new();
        for SendNativeTokenParams {
            address,
            native_token,
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
                .unwrap_or_else(|| default_return_address.address.clone());

            let native_token = NativeToken::new(native_token.0, native_token.1)?;

            // get minimum required amount for such an output, so we don't lock more than required
            // We have to check it for every output individually, because different address types and amount of
            // different native token require a different storage deposit
            let storage_deposit_amount = MinimumStorageDepositBasicOutput::new(rent_structure, token_supply)
                .with_native_token(native_token.clone())
                .with_storage_deposit_return()?
                .with_expiration()?
                .finish()?;

            let expiration_slot_index = expiration
                .map_or(slot_index + DEFAULT_EXPIRATION_SLOTS, |expiration_slot_index| {
                    slot_index + expiration_slot_index
                });

            outputs.push(
                BasicOutputBuilder::new_with_amount(storage_deposit_amount)
                    .with_native_token(native_token)
                    .add_unlock_condition(AddressUnlockCondition::new(address))
                    .add_unlock_condition(
                        // We send the full storage_deposit_amount back to the sender, so only the native token is
                        // sent
                        StorageDepositReturnUnlockCondition::new(
                            return_address.clone(),
                            storage_deposit_amount,
                            token_supply,
                        )?,
                    )
                    .add_unlock_condition(ExpirationUnlockCondition::new(return_address, expiration_slot_index)?)
                    .finish_output(token_supply)?,
            )
        }

        self.prepare_transaction(outputs, options).await
    }
}
