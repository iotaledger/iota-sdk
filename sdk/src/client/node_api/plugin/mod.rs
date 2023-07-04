// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Custom plugin call

use core::str::FromStr;

use reqwest::Method;

use crate::client::{ClientInner, Result};

impl ClientInner {
    /// Extension method which provides request methods for plugins.
    pub async fn call_plugin_route<T>(
        &self,
        base_plugin_path: &str,
        method: &str,
        method_path: &str,
        query_params: Vec<String>,
        request_object: Option<String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug + serde::Serialize,
    {
        let mut method = method.to_string();
        method.make_ascii_uppercase();

        let req_method = reqwest::Method::from_str(&method);

        let node_manager = self.node_manager.read().await;
        let path = format!("{}{}{}", base_plugin_path, method_path, query_params.join("&"));
        let timeout = self.get_timeout().await;

        match req_method {
            Ok(Method::GET) => node_manager.get_request(&path, None, timeout, false, false).await,
            Ok(Method::POST) => {
                node_manager
                    .post_request_json(&path, timeout, request_object.into(), true)
                    .await
            }
            _ => Err(crate::client::Error::Node(
                crate::client::node_api::error::Error::NotSupported(method.to_string()),
            )),
        }
    }
}
