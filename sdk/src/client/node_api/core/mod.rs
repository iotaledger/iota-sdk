// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

use packable::PackableExt;

use crate::{
    client::{Client, Result},
    types::block::output::{Output, OutputId, OutputMetadata, OutputWithMetadata},
};

impl Client {
    // Finds output and its metadata by output ID.
    /// GET /api/core/v3/outputs/{outputId}
    /// + GET /api/core/v3/outputs/{outputId}/metadata
    pub async fn get_output_with_metadata(&self, output_id: &OutputId) -> Result<OutputWithMetadata> {
        let output = Output::unpack_verified(
            self.get_output_raw(output_id).await?,
            &self.get_protocol_parameters().await?,
        )?;
        let metadata = self.get_output_metadata(output_id).await?;

        Ok(OutputWithMetadata::new(output, metadata))
    }

    /// Requests outputs by their output ID in parallel.
    pub async fn get_outputs(&self, output_ids: &[OutputId]) -> Result<Vec<Output>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output(id))).await
    }

    /// Requests outputs by their output ID in parallel, ignoring failed requests.
    /// Useful to get data about spent outputs, that might not be pruned yet.
    pub async fn get_outputs_ignore_errors(&self, output_ids: &[OutputId]) -> Result<Vec<Output>> {
        Ok(
            futures::future::join_all(output_ids.iter().map(|id| self.get_output(id)))
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        )
    }

    /// Requests metadata for outputs by their output ID in parallel.
    pub async fn get_outputs_metadata(&self, output_ids: &[OutputId]) -> Result<Vec<OutputMetadata>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output_metadata(id))).await
    }

    /// Requests metadata for outputs by their output ID in parallel, ignoring failed requests.
    pub async fn get_outputs_metadata_ignore_errors(&self, output_ids: &[OutputId]) -> Result<Vec<OutputMetadata>> {
        Ok(
            futures::future::join_all(output_ids.iter().map(|id| self.get_output_metadata(id)))
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        )
    }

    /// Requests outputs and their metadata by their output ID in parallel.
    pub async fn get_outputs_with_metadata(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithMetadata>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output_with_metadata(id))).await
    }

    /// Requests outputs and their metadata by their output ID in parallel, ignoring failed requests.
    /// Useful to get data about spent outputs, that might not be pruned yet.
    pub async fn get_outputs_with_metadata_ignore_errors(
        &self,
        output_ids: &[OutputId],
    ) -> Result<Vec<OutputWithMetadata>> {
        Ok(
            futures::future::join_all(output_ids.iter().map(|id| self.get_output_with_metadata(id)))
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        )
    }
}
