// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Custom plugin call

use core::str::FromStr;

use reqwest::Method;

use crate::client::{Client, Result};

impl Client {
    /// Extension method which provides request methods for plugins.
    pub async fn call_plugin_route<T>(
        &self,
        base_plugin_path: &str,
        method: &str,
        endpoint: &str,
        query_params: Vec<String>,
        request_object: Option<String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug + serde::Serialize + Send,
    {
        let mut method = method.to_string();
        method.make_ascii_uppercase();

        let req_method = reqwest::Method::from_str(&method);

        let path = format!("{}{}{}", base_plugin_path, endpoint, query_params.join("&"));
        let timeout = self.get_timeout().await;

        match req_method {
            Ok(Method::GET) => self.get_request(&path, None, timeout, false, false).await,
            Ok(Method::POST) => {
                self.post_request_json(&path, timeout, request_object.into(), true)
                    .await
            }
            _ => Err(crate::client::Error::Node(
                crate::client::node_api::error::Error::NotSupported(method.to_string()),
            )),
        }
    }
}
