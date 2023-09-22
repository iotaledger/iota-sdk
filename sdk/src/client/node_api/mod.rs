// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! node API modules

pub mod core;
pub mod error;
pub mod indexer;
#[cfg(feature = "mqtt")]
#[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
pub mod mqtt;
#[cfg(feature = "participation")]
#[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
pub mod participation;
pub mod plugin;

pub(crate) fn query_string(query: impl IntoIterator<Item = Option<(&'static str, String)>>) -> Option<String> {
    let query = query
        .into_iter()
        .filter_map(|q| q.map(|q| format!("{}={}", q.0, q.1)))
        .collect::<Vec<_>>();

    if query.is_empty() { None } else { Some(query.join("&")) }
}
