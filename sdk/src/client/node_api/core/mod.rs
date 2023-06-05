// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

#[cfg(not(target_family = "wasm"))]
use crate::client::constants::MAX_PARALLEL_API_REQUESTS;
use crate::{
    client::{Client, Result},
    types::block::output::{dto::OutputMetadataDto, OutputId, OutputWithMetadata},
};

impl Client {
    /// Request outputs by their output ID in parallel
    pub async fn get_outputs(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithMetadata>> {
        #[cfg(target_family = "wasm")]
        let outputs = futures::future::try_join_all(output_ids.iter().map(|id| self.get_output(id))).await?;

        #[cfg(not(target_family = "wasm"))]
        let outputs =
            futures::future::try_join_all(output_ids.chunks(MAX_PARALLEL_API_REQUESTS).map(|output_ids_chunk| {
                let client = self.clone();
                let output_ids_chunk = output_ids_chunk.to_vec();
                async move {
                    tokio::spawn(async move {
                        futures::future::try_join_all(output_ids_chunk.iter().map(|id| client.get_output(id))).await
                    })
                    .await?
                }
            }))
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(outputs)
    }

    /// Request outputs by their output ID in parallel, ignoring failed requests
    /// Useful to get data about spent outputs, that might not be pruned yet
    pub async fn get_outputs_ignore_errors(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithMetadata>> {
        #[cfg(target_family = "wasm")]
        let outputs = futures::future::join_all(output_ids.iter().map(|id| self.get_output(id)))
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        #[cfg(not(target_family = "wasm"))]
        let outputs =
            futures::future::try_join_all(output_ids.chunks(MAX_PARALLEL_API_REQUESTS).map(|output_ids_chunk| {
                let client = self.clone();
                let output_ids_chunk = output_ids_chunk.to_vec();
                tokio::spawn(async move {
                    futures::future::join_all(output_ids_chunk.iter().map(|id| client.get_output(id)))
                        .await
                        .into_iter()
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>()
                })
            }))
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(outputs)
    }

    /// Requests metadata for outputs by their output ID in parallel, ignoring failed requests
    pub async fn get_outputs_metadata_ignore_errors(&self, output_ids: &[OutputId]) -> Result<Vec<OutputMetadataDto>> {
        #[cfg(target_family = "wasm")]
        let metadata = futures::future::join_all(output_ids.iter().map(|id| self.get_output_metadata(id)))
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        #[cfg(not(target_family = "wasm"))]
        let metadata =
            futures::future::try_join_all(output_ids.chunks(MAX_PARALLEL_API_REQUESTS).map(|output_ids_chunk| {
                let client = self.clone();
                let output_ids_chunk = output_ids_chunk.to_vec();
                tokio::spawn(async move {
                    futures::future::join_all(output_ids_chunk.iter().map(|id| client.get_output_metadata(id)))
                        .await
                        .into_iter()
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>()
                })
            }))
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(metadata)
    }
}
