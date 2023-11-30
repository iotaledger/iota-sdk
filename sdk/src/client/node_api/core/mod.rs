// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

use packable::PackableExt;

use crate::{
    client::{node_api::error::Error as NodeApiError, Client, Error, Result},
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
    pub async fn get_outputs(&self, output_ids: impl IntoIterator<Item = &OutputId> + Send) -> Result<Vec<Output>> {
        futures::future::try_join_all(output_ids.into_iter().map(|id| self.get_output(id))).await
    }

    /// Requests outputs by their output ID in parallel, ignoring outputs not found.
    /// Useful to get data about spent outputs, that might not be pruned yet.
    pub async fn get_outputs_ignore_not_found(
        &self,
        output_ids: impl IntoIterator<Item = &OutputId> + Send,
    ) -> Result<Vec<Output>> {
        futures::future::join_all(output_ids.into_iter().map(|id| self.get_output(id)))
            .await
            .into_iter()
            .filter(|res| !matches!(res, Err(Error::Node(NodeApiError::NotFound(_)))))
            .collect()
    }

    /// Requests metadata for outputs by their output ID in parallel.
    pub async fn get_outputs_metadata(
        &self,
        output_ids: impl IntoIterator<Item = &OutputId> + Send,
    ) -> Result<Vec<OutputMetadata>> {
        futures::future::try_join_all(output_ids.into_iter().map(|id| self.get_output_metadata(id))).await
    }

    /// Requests metadata for outputs by their output ID in parallel, ignoring outputs not found.
    pub async fn get_outputs_metadata_ignore_not_found(
        &self,
        output_ids: impl IntoIterator<Item = &OutputId> + Send,
    ) -> Result<Vec<OutputMetadata>> {
        futures::future::join_all(output_ids.into_iter().map(|id| self.get_output_metadata(id)))
            .await
            .into_iter()
            .filter(|res| !matches!(res, Err(Error::Node(NodeApiError::NotFound(_)))))
            .collect()
    }

    /// Requests outputs and their metadata by their output ID in parallel.
    pub async fn get_outputs_with_metadata(
        &self,
        output_ids: impl IntoIterator<Item = &OutputId> + Send,
    ) -> Result<Vec<OutputWithMetadata>> {
        futures::future::try_join_all(output_ids.into_iter().map(|id| self.get_output_with_metadata(id))).await
    }

    /// Requests outputs and their metadata by their output ID in parallel, ignoring outputs not found.
    /// Useful to get data about spent outputs, that might not be pruned yet.
    pub async fn get_outputs_with_metadata_ignore_not_found(
        &self,
        output_ids: impl IntoIterator<Item = &OutputId> + Send,
    ) -> Result<Vec<OutputWithMetadata>> {
        futures::future::join_all(output_ids.into_iter().map(|id| self.get_output_with_metadata(id)))
            .await
            .into_iter()
            .filter(|res| !matches!(res, Err(Error::Node(NodeApiError::NotFound(_)))))
            .collect()
    }
}
