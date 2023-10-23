// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node indexer API.

pub mod query_parameters;
pub mod routes;

use self::query_parameters::QueryParameterHelper;
use crate::{
    client::{ClientInner, Result},
    types::api::plugins::indexer::OutputIdsResponse,
};

impl ClientInner {
    /// Get all output ids for a provided URL route and query parameters.
    /// If a `OutputsQueryParameter::Cursor(_)` is provided, only a single page will be queried.
    pub async fn get_output_ids(
        &self,
        route: &str,
        mut query_parameters: impl QueryParameterHelper,
        need_quorum: bool,
        prefer_permanode: bool,
    ) -> Result<OutputIdsResponse> {
        let mut merged_output_ids_response = OutputIdsResponse {
            ledger_index: 0,
            cursor: None,
            items: Vec::new(),
        };

        let query_string = query_parameters.to_query_string();
        // Return early with only a single page if a `QueryParameter::Cursor(_)` is provided.
        let return_early = query_string
            .map(|s| s.contains("cursor=&") || s.ends_with("cursor="))
            .unwrap_or(false);

        while let Some(cursor) = {
            let output_ids_response = self
                .get_request::<OutputIdsResponse>(
                    route,
                    query_parameters.to_query_string().as_deref(),
                    need_quorum,
                    prefer_permanode,
                )
                .await?;

            if return_early {
                return Ok(output_ids_response);
            }

            merged_output_ids_response.ledger_index = output_ids_response.ledger_index;
            merged_output_ids_response.cursor = output_ids_response.cursor;
            merged_output_ids_response.items.extend(output_ids_response.items);

            &merged_output_ids_response.cursor
        } {
            query_parameters.replace_cursor(cursor.to_string());
        }

        Ok(merged_output_ids_response)
    }
}
