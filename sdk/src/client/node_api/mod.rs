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

pub(crate) fn query_tuples_to_query_string(
    tuples: impl IntoIterator<Item = Option<(&'static str, String)>>,
) -> Option<String> {
    let query = tuples
        .into_iter()
        .filter_map(|tuple| tuple.map(|(key, value)| format!("{key}={value}")))
        .collect::<Vec<_>>();

    (!query.is_empty()).then_some(query.join("&"))
}
