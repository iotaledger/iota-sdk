// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{node_api::indexer::query_parameters::NftOutputQueryParameters, secret::SecretManage},
    types::block::{address::Bech32Address, output::OutputId, ConvertTo},
    wallet::Account,
};

impl<S: 'static + SecretManage> Account<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Returns output ids of nft outputs that have the address in any unlock condition
    pub(crate) async fn get_nft_output_ids_with_any_unlock_condition(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> crate::wallet::Result<Vec<OutputId>> {
        let bech32_address = bech32_address.convert()?;
        Ok(self
            .client()
            .nft_output_ids(NftOutputQueryParameters::new().storage_deposit_return_address(bech32_address))
            .await?
            .items)
    }
}
