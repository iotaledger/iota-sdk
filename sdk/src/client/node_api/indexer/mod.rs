// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! [REST API for IOTA UTXO indexers](https://editor.swagger.io/?url=https://raw.githubusercontent.com/iotaledger/tips/tip48/tips/TIP-0048/openapi3-indexer.yaml)

pub mod query_parameters;
pub mod routes;

use self::query_parameters::QueryParameter;
use crate::{
    client::{ClientError, ClientInner},
    types::api::plugins::indexer::OutputIdsResponse,
};

impl ClientInner {
    /// Get all output ids for a provided URL route and query parameters.
    /// If an empty cursor is provided, only a single page will be queried.
    pub async fn get_output_ids(
        &self,
        route: &str,
        mut query_parameters: impl QueryParameter,
        need_quorum: bool,
    ) -> Result<OutputIdsResponse, ClientError> {
        let mut merged_output_ids_response = OutputIdsResponse {
            committed_slot: 0,
            page_size: 1000,
            cursor: None,
            items: Vec::new(),
        };

        let query_string = query_parameters.to_query_string();
        // Return early with only a single page if an empty string is provided as cursor.
        let return_early = query_string
            .map(|s| s.contains("cursor=&") || s.ends_with("cursor="))
            .unwrap_or(false);

        while let Some(cursor) = {
            let output_ids_response = self
                .get_request::<OutputIdsResponse>(route, query_parameters.to_query_string().as_deref(), need_quorum)
                .await?;

            if return_early {
                return Ok(output_ids_response);
            }

            merged_output_ids_response.committed_slot = output_ids_response.committed_slot;
            merged_output_ids_response.page_size = output_ids_response.page_size;
            merged_output_ids_response.cursor = output_ids_response.cursor;
            merged_output_ids_response.items.extend(output_ids_response.items);

            &merged_output_ids_response.cursor
        } {
            query_parameters.replace_cursor(cursor.to_string());
        }

        Ok(merged_output_ids_response)
    }
}
