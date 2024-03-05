// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::secret::SecretManage,
    wallet::{
        constants::PARALLEL_REQUESTS_AMOUNT,
        task,
        types::{address::AddressWithUnspentOutputs, OutputData},
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Get outputs from addresses
    pub(crate) async fn get_outputs_from_address_output_ids(
        &self,
        addresses_with_unspent_outputs: Vec<AddressWithUnspentOutputs>,
    ) -> Result<Vec<(AddressWithUnspentOutputs, Vec<OutputData>)>, WalletError> {
        log::debug!("[SYNC] start get_outputs_from_address_output_ids");
        let address_outputs_start_time = Instant::now();

        let mut addresses_with_outputs = Vec::new();

        // We split the addresses into chunks so we don't get timeouts if we have thousands
        for addresses_chunk in &mut addresses_with_unspent_outputs
            .chunks(PARALLEL_REQUESTS_AMOUNT)
            .map(|x: &[AddressWithUnspentOutputs]| x.to_vec())
        {
            let mut tasks = Vec::new();
            for address_with_unspent_outputs in addresses_chunk {
                let wallet = self.clone();
                tasks.push(async move {
                    task::spawn(async move {
                        let unspent_outputs_with_metadata = wallet
                            .get_outputs(address_with_unspent_outputs.output_ids.clone())
                            .await?;
                        let unspent_outputs_data = wallet
                            .output_response_to_output_data(unspent_outputs_with_metadata)
                            .await?;
                        Ok((address_with_unspent_outputs, unspent_outputs_data))
                    })
                    .await
                });
            }
            let results: Vec<Result<_, WalletError>> = futures::future::try_join_all(tasks).await?;
            for res in results {
                addresses_with_outputs.push(res?);
            }
        }
        log::debug!(
            "[SYNC] finished get_outputs_from_address_output_ids in {:.2?}",
            address_outputs_start_time.elapsed()
        );
        Ok(addresses_with_outputs)
    }
}
