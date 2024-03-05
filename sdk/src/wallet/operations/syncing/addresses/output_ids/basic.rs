// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{node_api::indexer::query_parameters::BasicOutputQueryParameters, secret::SecretManage},
    types::block::{address::Bech32Address, output::OutputId},
    utils::ConvertTo,
    wallet::{Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Returns output ids of basic outputs that have only the address unlock condition
    pub(crate) async fn get_basic_output_ids_with_address_unlock_condition_only(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> Result<Vec<OutputId>, WalletError> {
        let bech32_address = bech32_address.convert()?;

        Ok(self
            .client()
            .basic_output_ids(BasicOutputQueryParameters::only_address_unlock_condition(
                bech32_address,
            ))
            .await?
            .items)
    }

    /// Returns output ids of basic outputs that have the address in the `AddressUnlockCondition` or
    /// `ExpirationUnlockCondition`
    pub(crate) async fn get_basic_output_ids_with_any_unlock_condition(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> Result<Vec<OutputId>, WalletError> {
        let bech32_address = bech32_address.convert()?;

        Ok(self
            .client()
            .basic_output_ids(BasicOutputQueryParameters::new().unlockable_by_address(bech32_address.clone()))
            .await?
            .items)
    }
}
