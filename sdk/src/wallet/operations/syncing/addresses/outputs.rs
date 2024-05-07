// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::secret::SecretManage,
    wallet::{
        constants::PARALLEL_REQUESTS_AMOUNT,
        task,
        types::address::{AddressWithUnspentOutputIds, AddressWithUnspentOutputs},
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Get unspent outputs from addresses
    pub(crate) async fn get_outputs_from_address_output_ids(
        &self,
        addresses_with_unspent_output_ids: &[AddressWithUnspentOutputIds],
    ) -> Result<Vec<AddressWithUnspentOutputs>, WalletError> {
        log::debug!("[SYNC] start get_outputs_from_address_output_ids");
        let address_outputs_start_time = Instant::now();

        let network_id = self.client().get_network_id().await?;

        let mut addresses_with_outputs = Vec::new();

        // We split the addresses into chunks so we don't get timeouts if we have thousands
        for addresses_chunk in addresses_with_unspent_output_ids
            .chunks(PARALLEL_REQUESTS_AMOUNT)
            .map(|x: &[AddressWithUnspentOutputIds]| x.to_vec())
        {
            let mut tasks = Vec::new();
            for address_with_unspent_output_ids in addresses_chunk {
                let wallet = self.clone();
                tasks.push(async move {
                    task::spawn(async move {
                        let unspent_outputs_with_metadata = wallet
                            .get_outputs_request_unknown(address_with_unspent_output_ids.unspent_output_ids())
                            .await?;
                        let unspent_outputs = wallet
                            .output_response_to_output_data(unspent_outputs_with_metadata, network_id)
                            .await?;

                        Ok(AddressWithUnspentOutputs {
                            address_with_unspent_output_ids,
                            unspent_outputs,
                        })
                    })
                    .await
                });
            }
            let results: Vec<Result<_, WalletError>> = futures::future::try_join_all(tasks).await?;
            let result = results.into_iter().collect::<Result<Vec<_>, _>>()?;
            addresses_with_outputs.extend(result.into_iter());
        }
        log::debug!(
            "[SYNC] finished get_outputs_from_address_output_ids in {:.2?}",
            address_outputs_start_time.elapsed()
        );
        Ok(addresses_with_outputs)
    }
}
