// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    client::api::PreparedTransactionData,
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, NftId, NftOutputBuilder, Output},
        ConvertTo,
    },
    wallet::account::{operations::transaction::Transaction, Account, TransactionOptions},
};

/// Params for `send_nft()`
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct SendNftParams {
    /// Bech32 encoded address
    #[getset(get = "pub")]
    address: Bech32Address,
    /// Nft id
    #[getset(get = "pub")]
    nft_id: NftId,
}

impl SendNftParams {
    /// Creates a new instance of [`SendNftParams`]
    pub fn new(
        address: impl ConvertTo<Bech32Address>,
        nft_id: impl ConvertTo<NftId>,
    ) -> Result<Self, crate::wallet::Error> {
        Ok(Self {
            address: address.convert()?,
            nft_id: nft_id.convert()?,
        })
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
    /// let outputs = [SendNftParams::new(
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
    pub async fn send_nft<I: IntoIterator<Item = SendNftParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<Transaction>
    where
        I::IntoIter: Send,
    {
        let prepared_transaction = self.prepare_send_nft(params, options).await?;
        self.sign_and_submit_transaction(prepared_transaction).await
    }

    /// Function to prepare the transaction for
    /// [Account.send_nft()](crate::account::Account.send_nft)
    pub async fn prepare_send_nft<I: IntoIterator<Item = SendNftParams> + Send>(
        &self,
        params: I,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedTransactionData>
    where
        I::IntoIter: Send,
    {
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
                        .with_unlock_conditions([AddressUnlockCondition::new(address)]);
                    outputs.push(nft_builder.finish_output(token_supply)?);
                }
            } else {
                return Err(crate::wallet::Error::NftNotFoundInUnspentOutputs);
            };
        }

        self.prepare_transaction(outputs, options).await
    }
}
