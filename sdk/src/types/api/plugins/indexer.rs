// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Node indexer responses.

use alloc::{string::String, vec::Vec};
use core::ops::Deref;

use crate::types::block::output::OutputId;

/// Response of GET /api/indexer/v2/*
/// Returns the output_ids for the provided query parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OutputIdsResponse {
    /// The committed slot at which these outputs where available at.
    pub committed_slot: u32,
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too.
    pub page_size: u32,
    /// Cursor confirmationMS+outputId.pageSize
    pub cursor: Option<String>,
    /// The output ids
    pub items: Vec<OutputId>,
}

impl Deref for OutputIdsResponse {
    type Target = Vec<OutputId>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
