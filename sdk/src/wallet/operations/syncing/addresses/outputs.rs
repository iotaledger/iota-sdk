// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::secret::SecretManage,
    types::block::output::OutputId,
    wallet::{types::OutputData, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Get outputs from output ids
    pub(crate) async fn get_outputs_from_address_output_ids(
        &self,
        output_ids: Vec<OutputId>,
    ) -> crate::wallet::Result<Vec<OutputData>> {
        log::debug!("[SYNC] start get_outputs_from_address_output_ids");
        let address_outputs_start_time = Instant::now();

        let output_responses = self.get_outputs(output_ids.clone()).await?;

        let outputs_data = self.output_response_to_output_data(output_responses).await?;

        log::debug!(
            "[SYNC] finished get_outputs_from_address_output_ids in {:.2?}",
            address_outputs_start_time.elapsed()
        );
        Ok(outputs_data)
    }
}
