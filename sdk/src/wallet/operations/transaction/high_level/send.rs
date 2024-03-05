// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::{
        address::{Bech32Address, ToBech32Ext},
        output::{
            unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
            BasicOutputBuilder, MinimumOutputAmount,
        },
        slot::SlotIndex,
    },
    utils::{serde::string, ConvertTo},
    wallet::{
        constants::DEFAULT_EXPIRATION_SLOTS,
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Wallet, WalletError,
    },
};

/// Parameters for `send()`
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct SendParams {
    /// Amount
    #[serde(with = "string")]
    #[getset(get = "pub")]
    amount: u64,
    /// Bech32 encoded address
    #[getset(get = "pub")]
    address: Bech32Address,
    /// Bech32 encoded return address, to which the storage deposit will be returned if one is necessary
    /// given the provided amount. If a storage deposit is needed and a return address is not provided, it will
    /// default to the address of the wallet.
    #[getset(get = "pub")]
    return_address: Option<Bech32Address>,
    /// Expiration in slot indices, after which the output will be available for the sender again, if not spent by the
    /// receiver already. The expiration will only be used if one is necessary given the provided amount. If an
    /// expiration is needed but not provided, it will default to one day.
    #[getset(get = "pub")]
    expiration: Option<SlotIndex>,
}

impl SendParams {
    pub fn new(amount: u64, address: impl ConvertTo<Bech32Address>) -> Result<Self, WalletError> {
        Ok(Self {
            amount,
            address: address.convert()?,
            return_address: None,
            expiration: None,
        })
    }

    pub fn try_with_return_address(mut self, address: impl ConvertTo<Bech32Address>) -> Result<Self, WalletError> {
        self.return_address = Some(address.convert()?);
        Ok(self)
    }

    pub fn with_return_address(mut self, address: impl Into<Option<Bech32Address>>) -> Self {
        self.return_address = address.into();
        self
    }

    pub fn with_expiration(mut self, expiration: impl Into<Option<SlotIndex>>) -> Self {
        self.expiration = expiration.into();
        self
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Sends a certain amount of base coins to a single address.
    ///
    /// Calls [Wallet::send_with_params()] internally.
    /// The options may define the remainder value strategy or custom inputs.
    /// The provided Addresses provided with [`SendParams`] need to be bech32-encoded.
    pub async fn send(
        &self,
        amount: u64,
        address: impl ConvertTo<Bech32Address>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError> {
        let params = [SendParams::new(amount, address)?];
        self.send_with_params(params, options).await
    }

    /// Sends a certain amount of base coins with full customizability of the transaction.
    ///
    /// Calls [Wallet::send_outputs()] internally.
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
    ) -> Result<TransactionWithMetadata, WalletError>
    where
        I::IntoIter: Send,
    {
        let options = options.into();
        let prepared_transaction = self.prepare_send(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for [Wallet::send()].
    pub async fn prepare_send<I: IntoIterator<Item = SendParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError>
    where
        I::IntoIter: Send,
    {
        log::debug!("[TRANSACTION] prepare_send");
        let options = options.into();
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let wallet_address = self.address().await;

        let default_return_address = wallet_address.to_bech32(self.client().get_bech32_hrp().await?);

        let slot_index = self.client().get_slot_index().await?;

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
                        Err(ClientError::Bech32HrpMismatch {
                            provided: return_address.hrp().to_string(),
                            expected: address.hrp().to_string(),
                        })?;
                    }
                    Ok::<_, WalletError>(return_address)
                })
                .transpose()?
                .unwrap_or_else(|| default_return_address.clone());

            // Get the minimum required amount for an output assuming it does not need a storage deposit.
            let output = BasicOutputBuilder::new_with_amount(amount)
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .finish()?;

            if amount >= output.minimum_amount(storage_score_params) {
                outputs.push(output.into())
            } else {
                let expiration_slot_index = expiration
                    .map_or(slot_index + DEFAULT_EXPIRATION_SLOTS, |expiration_slot_index| {
                        slot_index + expiration_slot_index
                    });

                // Since it does need a storage deposit, calculate how much that should be
                let output = BasicOutputBuilder::from(&output)
                    .add_unlock_condition(ExpirationUnlockCondition::new(
                        return_address.clone(),
                        expiration_slot_index,
                    )?)
                    .with_sufficient_storage_deposit(return_address, storage_score_params)?
                    .finish_output()?;

                if !options.as_ref().map(|o| o.allow_micro_amount).unwrap_or_default() {
                    return Err(WalletError::InsufficientFunds {
                        available: amount,
                        required: output.amount(),
                    });
                }

                outputs.push(output)
            }
        }

        self.prepare_send_outputs(outputs, options).await
    }
}
