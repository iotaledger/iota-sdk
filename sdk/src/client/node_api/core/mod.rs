// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node core API

pub mod routes;

use crate::{
    client::{Client, Result},
    types::block::output::{OutputId, OutputMetadata, OutputWithMetadata},
};

impl Client {
    /// Request outputs by their output ID in parallel
    pub async fn get_outputs(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithMetadata>> {
        futures::future::try_join_all(output_ids.iter().map(|id| self.get_output(id))).await
    }

    /// Request outputs by their output ID in parallel, ignoring failed requests
    /// Useful to get data about spent outputs, that might not be pruned yet
    pub async fn get_outputs_ignore_errors(&self, output_ids: &[OutputId]) -> Result<Vec<OutputWithMetadata>> {
        Ok(
            futures::future::join_all(output_ids.iter().map(|id| self.get_output(id)))
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        )
    }

    /// Requests metadata for outputs by their output ID in parallel, ignoring failed requests
    pub async fn get_outputs_metadata_ignore_errors(&self, output_ids: &[OutputId]) -> Result<Vec<OutputMetadata>> {
        Ok(
            futures::future::join_all(output_ids.iter().map(|id| self.get_output_metadata(id)))
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        )
    }
}
