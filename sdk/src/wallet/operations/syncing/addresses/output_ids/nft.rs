// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{node_api::indexer::query_parameters::NftOutputQueryParameters, secret::SecretManage},
    types::block::{address::Bech32Address, output::OutputId},
    utils::ConvertTo,
    wallet::{Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Returns output ids of NFT outputs that have the address in the `AddressUnlockCondition` or
    /// `ExpirationUnlockCondition`
    pub(crate) async fn get_nft_output_ids_with_any_unlock_condition(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> Result<Vec<OutputId>, WalletError> {
        let bech32_address = bech32_address.convert()?;

        Ok(self
            .client()
            .nft_output_ids(NftOutputQueryParameters::new().unlockable_by_address(bech32_address))
            .await?
            .items)
    }
}
