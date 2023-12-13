// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::Response;
use napi::Error;

#[non_exhaustive]
pub struct NodejsError(pub(crate) iota_sdk_bindings_core::Error);

impl NodejsError {
    pub fn new(err: impl Into<iota_sdk_bindings_core::Error>) -> Self {
        Self(err.into())
    }
}

impl From<iota_sdk_bindings_core::Error> for NodejsError {
    fn from(error: iota_sdk_bindings_core::Error) -> Self {
        Self(error)
    }
}

impl From<serde_json::error::Error> for NodejsError {
    fn from(error: serde_json::error::Error) -> Self {
        Self(error.into())
    }
}

// To the specific bindings glue error.
impl From<NodejsError> for Error {
    fn from(error: NodejsError) -> Self {
        crate::build_js_error(Response::Error(error.0))
    }
}
