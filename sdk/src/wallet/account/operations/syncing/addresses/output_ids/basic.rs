// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{node_api::indexer::query_parameters::QueryParameter, secret::SecretManage},
    types::block::{address::Bech32Address, output::OutputId, ConvertTo},
    wallet::Account,
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Returns output ids of basic outputs that have only the address unlock condition
    pub(crate) async fn get_basic_output_ids_with_address_unlock_condition_only(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> crate::client::Result<Vec<OutputId>> {
        let bech32_address = bech32_address.convert()?;
        // Only request basic outputs with `AddressUnlockCondition` only
        Ok(self
            .client()
            .basic_output_ids([
                QueryParameter::Address(bech32_address),
                QueryParameter::HasExpiration(false),
                QueryParameter::HasTimelock(false),
                QueryParameter::HasStorageDepositReturn(false),
            ])
            .await?
            .items)
    }

    /// Returns output ids of basic outputs that have the address in the `AddressUnlockCondition`,
    /// `ExpirationUnlockCondition` or `StorageDepositReturnUnlockCondition`
    pub(crate) async fn get_basic_output_ids_with_any_unlock_condition(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        let bech32_address = bech32_address.convert()?;

        Ok(self
            .client()
            .basic_output_ids([QueryParameter::UnlockableByAddress(bech32_address)])
            .await?
            .items)
    }
}
