// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage, ClientError},
    types::block::{
        address::{Bech32Address, ToBech32Ext},
        output::{
            unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
            BasicOutputBuilder, NativeToken, TokenId,
        },
        slot::SlotIndex,
    },
    utils::ConvertTo,
    wallet::{
        constants::DEFAULT_EXPIRATION_SLOTS,
        operations::transaction::{TransactionOptions, TransactionWithMetadata},
        Wallet, WalletError,
    },
};

/// Params for `send_native_tokens()`
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
    /// address of the wallet.
    #[getset(get = "pub")]
    return_address: Option<Bech32Address>,
    /// Expiration in slot indices, after which the output will be available for the sender again, if not spent by the
    /// receiver before. Default is DEFAULT_EXPIRATION_SLOTS slots.
    #[getset(get = "pub")]
    expiration: Option<SlotIndex>,
}

impl SendNativeTokenParams {
    /// Creates a new instance of [`SendNativeTokenParams`]
    pub fn new(address: impl ConvertTo<Bech32Address>, native_token: (TokenId, U256)) -> Result<Self, WalletError> {
        Ok(Self {
            address: address.convert()?,
            native_token,
            return_address: None,
            expiration: None,
        })
    }

    /// Set the return address and try convert to [`Bech32Address`]
    pub fn try_with_return_address(
        mut self,
        return_address: impl ConvertTo<Bech32Address>,
    ) -> Result<Self, WalletError> {
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

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Sends native tokens in basic outputs with a
    /// [`StorageDepositReturnUnlockCondition`](crate::types::block::output::unlock_condition::StorageDepositReturnUnlockCondition)
    /// and an [`ExpirationUnlockCondition`], so that the storage deposit is returned to the sender and the sender
    /// gets access to the output again after a predefined time (default 1 day).
    /// Calls [Wallet::send_outputs()] internally. The options may define the remainder value strategy or custom inputs.
    /// Note that the address needs to be bech32-encoded.
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
    /// let tx = account.send_native_tokens(params, None).await?;
    /// println!("Transaction created: {}", tx.transaction_id);
    /// if let Some(block_id) = tx.block_id {
    ///     println!("Block sent: {}", block_id);
    /// }
    /// ```
    pub async fn send_native_tokens<I: IntoIterator<Item = SendNativeTokenParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<TransactionWithMetadata, WalletError>
    where
        I::IntoIter: Send,
    {
        let options = options.into();
        let prepared_transaction = self.prepare_send_native_tokens(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared_transaction, options).await
    }

    /// Prepares the transaction for [Wallet::send_native_tokens()].
    pub async fn prepare_send_native_tokens<I: IntoIterator<Item = SendNativeTokenParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> Result<PreparedTransactionData, WalletError>
    where
        I::IntoIter: Send,
    {
        log::debug!("[TRANSACTION] prepare_send_native_tokens");
        let storage_score_params = self.client().get_storage_score_parameters().await?;

        let wallet_address = self.address().await;
        let default_return_address = wallet_address.to_bech32(self.client().get_bech32_hrp().await?);

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
                        Err(ClientError::Bech32HrpMismatch {
                            provided: addr.hrp().to_string(),
                            expected: address.hrp().to_string(),
                        })?;
                    }
                    Ok::<_, WalletError>(addr)
                })
                .transpose()?
                .unwrap_or_else(|| default_return_address.clone());

            let native_token = NativeToken::new(native_token.0, native_token.1)?;

            let expiration_slot_index = expiration
                .map_or(slot_index + DEFAULT_EXPIRATION_SLOTS, |expiration_slot_index| {
                    slot_index + expiration_slot_index
                });

            outputs.push(
                BasicOutputBuilder::new_with_amount(0)
                    .with_native_token(native_token)
                    .add_unlock_condition(AddressUnlockCondition::new(address))
                    .add_unlock_condition(ExpirationUnlockCondition::new(
                        return_address.clone(),
                        expiration_slot_index,
                    )?)
                    .with_sufficient_storage_deposit(return_address, storage_score_params)?
                    .finish_output()?,
            )
        }

        self.prepare_transaction(outputs, options).await
    }
}
