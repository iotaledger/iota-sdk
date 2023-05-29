// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    client::api::PreparedTransactionData,
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, NftId, NftOutputBuilder, Output},
    },
    wallet::account::{operations::transaction::Transaction, Account, TransactionOptions},
};

/// Params for `send_nft()`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendNftParams {
    /// Bech32 encoded address
    pub(crate) address: Bech32Address,
    /// Nft id
    pub(crate) nft_id: NftId,
}

impl SendNftParams {
    /// Creates a  new instance of [`SendNftParams`]
    pub fn new(
        address: impl TryInto<Bech32Address, Error = impl Into<crate::wallet::Error>>,
        nft_id: impl TryInto<NftId, Error = impl Into<crate::wallet::Error>>,
    ) -> Result<Self, crate::wallet::Error> {
        Ok(Self {
            address: address.try_into().map_err(Into::into)?,
            nft_id: nft_id.try_into().map_err(Into::into)?,
        })
    }

    pub fn address(&self) -> Bech32Address {
        self.address
    }

    pub fn nft_id(&self) -> NftId {
        self.nft_id
    }

    pub fn set_address(
        &mut self,
        address: impl TryInto<Bech32Address, Error = impl Into<crate::wallet::Error>>,
    ) -> Result<(), crate::wallet::Error> {
        self.address = address.try_into().map_err(Into::into)?;
        Ok(())
    }

    pub fn set_nft_id(
        &mut self,
        nft_id: impl TryInto<NftId, Error = impl Into<crate::wallet::Error>>,
    ) -> Result<(), crate::wallet::Error> {
        self.nft_id = nft_id.try_into().map_err(Into::into)?;
        Ok(())
    }
}

impl Account {
    /// Function to send native tokens in basic outputs with a
    /// [`StorageDepositReturnUnlockCondition`](crate::types::block::output::unlock_condition::StorageDepositReturnUnlockCondition) and
    /// [`ExpirationUnlockCondition`](crate::types::block::output::unlock_condition::ExpirationUnlockCondition), so the
    /// storage deposit gets back to the sender and also that the sender gets access to the output again after a
    /// defined time (default 1 day), Calls [Account.send()](crate::wallet::account::Account.send)
    /// internally, the options can define the RemainderValueStrategy. Custom inputs will be replaced with the
    /// required nft inputs. Address needs to be Bech32 encoded
    /// ```ignore
    /// let outputs = vec![SendNftParams::new(
    ///     "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    ///     "0xe645042a8a082957cb4bec4927936699ee8e56048834b090379da64213ce231b",
    /// )?];
    ///
    /// let transaction = account.send_nft(outputs, None).await?;
    ///
    /// println!(
    ///     "Transaction sent: {}/transaction/{}",
    ///     std::env::var("EXPLORER_URL").unwrap(),
    ///     transaction.transaction_id
    /// );
    /// ```
    pub async fn send_nft(
        &self,
        params: Vec<SendNftParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction> {
        let prepared_transaction = self.prepare_send_nft(params, options).await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [Account.send_nft()](crate::account::Account.send_nft)
    pub async fn prepare_send_nft(
        &self,
        params: Vec<SendNftParams>,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData> {
        log::debug!("[TRANSACTION] prepare_send_nft");

        let unspent_outputs = self.unspent_outputs(None).await?;
        let token_supply = self.client().get_token_supply().await?;

        let mut outputs = Vec::new();

        for SendNftParams { address, nft_id } in params {
            self.client().bech32_hrp_matches(address.hrp()).await?;

            // Find nft output from the inputs
            if let Some(nft_output_data) = unspent_outputs.iter().find(|o| {
                if let Output::Nft(nft_output) = &o.output {
                    nft_id == nft_output.nft_id_non_null(&o.output_id)
                } else {
                    false
                }
            }) {
                if let Output::Nft(nft_output) = &nft_output_data.output {
                    // Set the nft id and new address unlock condition
                    let nft_builder = NftOutputBuilder::from(nft_output)
                        .with_nft_id(nft_id)
                        .with_unlock_conditions(vec![AddressUnlockCondition::new(address)]);
                    outputs.push(nft_builder.finish_output(token_supply)?);
                }
            } else {
                return Err(crate::wallet::Error::NftNotFoundInUnspentOutputs);
            };
        }

        self.prepare_transaction(outputs, options).await
    }
}
