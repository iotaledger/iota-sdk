// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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

/// Address, amount and native tokens for `send_native_tokens()`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressNativeTokens {
    /// Bech32 encoded address
    pub address: Bech32Address,
    /// Native tokens
    pub native_tokens: Vec<(TokenId, U256)>,
    /// Bech32 encoded address return address, to which the storage deposit will be returned. Default will use the
    /// first address of the account
    pub return_address: Option<Bech32Address>,
    /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
    /// receiver before. Default is 1 day
    pub expiration: Option<u32>,
}

impl Account {
    /// Function to send native tokens in basic outputs with a [StorageDepositReturnUnlockCondition] and
    /// [ExpirationUnlockCondition], so the storage deposit gets back to the sender and also that the sender gets access
    /// to the output again after a defined time (default 1 day),
    /// Calls [Account.send()](crate::account::Account.send) internally, the options can define the
    /// RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let outputs = vec![AddressNativeTokens {
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
    pub async fn send_native_tokens(
        &self,
        addresses_and_native_tokens: Vec<AddressNativeTokens>,
        options: Option<TransactionOptions>,
    ) -> crate::wallet::Result<Transaction> {
        let prepared_transaction = self
            .prepare_send_native_tokens(addresses_and_native_tokens, options)
            .await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [Account.send_native_tokens()](crate::account::Account.send_native_tokens)
    async fn prepare_send_native_tokens(
        &self,
        addresses_and_native_tokens: Vec<AddressNativeTokens>,
        options: Option<TransactionOptions>,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_send_native_tokens");
        let rent_structure = self.client.get_rent_structure().await?;
        let token_supply = self.client.get_token_supply().await?;

        let account_addresses = self.addresses().await?;
        let return_address = account_addresses.first().ok_or(Error::FailedToGetRemainder)?;

        let local_time = self.client.get_time_checked().await?;

        let mut outputs = Vec::new();
        for address_with_amount in addresses_and_native_tokens {
            self.client
                .bech32_hrp_matches(address_with_amount.address.hrp())
                .await?;
            // get minimum required amount for such an output, so we don't lock more than required
            // We have to check it for every output individually, because different address types and amount of
            // different native tokens require a different storage deposit
            let storage_deposit_amount = minimum_storage_deposit_basic_native_tokens(
                &rent_structure,
                address_with_amount.address.inner(),
                return_address.address.inner(),
                Some(address_with_amount.native_tokens.clone()),
                token_supply,
            )?;

            let expiration_time = address_with_amount
                .expiration
                .map_or(local_time + DEFAULT_EXPIRATION_TIME, |expiration_time| {
                    local_time + expiration_time
                });

            outputs.push(
                BasicOutputBuilder::new_with_amount(storage_deposit_amount)
                    .with_native_tokens(
                        address_with_amount
                            .native_tokens
                            .into_iter()
                            .map(|(id, amount)| {
                                NativeToken::new(id, amount)
                                    .map_err(|e| crate::wallet::Error::Client(Box::new(e.into())))
                            })
                            .collect::<Result<Vec<NativeToken>>>()?,
                    )
                    .add_unlock_condition(AddressUnlockCondition::new(&address_with_amount.address))
                    .add_unlock_condition(
                        // We send the full storage_deposit_amount back to the sender, so only the native tokens are
                        // sent
                        StorageDepositReturnUnlockCondition::new(
                            &return_address.address,
                            storage_deposit_amount,
                            token_supply,
                        )?,
                    )
                    .add_unlock_condition(ExpirationUnlockCondition::new(
                        &return_address.address,
                        expiration_time,
                    )?)
                    .finish_output(token_supply)?,
            )
        }

        self.prepare_transaction(outputs, options).await
    }
}
