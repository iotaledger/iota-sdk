// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

use crate::{
    client::{node_api::error::Error as NodeApiError, Client, Error, Result},
    types::{
        api::core::OutputWithProof,
        block::output::{OutputId, OutputMetadata, OutputWithMetadataFull},
    },
};

impl Client {
    /// Requests outputs by their output ID in parallel.
    pub async fn get_outputs(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithProof>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output_with_proof(id))).await
    }

    /// Requests outputs by their output ID in parallel, ignoring outputs not found.
    /// Useful to get data about spent outputs, that might not be pruned yet.
    pub async fn get_outputs_ignore_not_found(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithProof>> {
        futures::future::join_all(output_ids.iter().map(|id| self.get_output_with_proof(id)))
            .await
            .into_iter()
            .filter(|res| !matches!(res, Err(Error::Node(NodeApiError::NotFound(_)))))
            .collect()
    }

    /// Requests metadata for outputs by their output ID in parallel.
    pub async fn get_outputs_metadata(&self, output_ids: &[OutputId]) -> Result<Vec<OutputMetadata>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output_metadata(id))).await
    }

    /// Requests metadata for outputs by their output ID in parallel, ignoring outputs not found.
    pub async fn get_outputs_metadata_ignore_not_found(&self, output_ids: &[OutputId]) -> Result<Vec<OutputMetadata>> {
        futures::future::join_all(output_ids.iter().map(|id| self.get_output_metadata(id)))
            .await
            .into_iter()
            .filter(|res| !matches!(res, Err(Error::Node(NodeApiError::NotFound(_)))))
            .collect()
    }

    /// Requests outputs and their metadata by their output ID in parallel.
    pub async fn get_outputs_with_metadata(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithMetadataFull>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output_with_metadata_full(id))).await
    }

    /// Requests outputs and their metadata by their output ID in parallel, ignoring outputs not found.
    /// Useful to get data about spent outputs, that might not be pruned yet.
    pub async fn get_outputs_with_metadata_ignore_not_found(
        &self,
        output_ids: &[OutputId],
    ) -> Result<Vec<OutputWithMetadataFull>> {
        futures::future::join_all(output_ids.iter().map(|id| self.get_output_with_metadata_full(id)))
            .await
            .into_iter()
            .filter(|res| !matches!(res, Err(Error::Node(NodeApiError::NotFound(_)))))
            .collect()
    }
}
