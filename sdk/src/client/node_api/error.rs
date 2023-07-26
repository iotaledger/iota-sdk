// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Type alias of `Result` in Node errors
pub type Result<T> = std::result::Result<T, Error>;

/// Node errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The requested data was not found. (404)
    #[error("The requested data {0} was not found.")]
    NotFound(String),
    /// Reqwest error
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error from RestAPI calls with unexpected status code response
    #[error("Response error with status code {code}: {text}, URL: {url}")]
    ResponseError {
        /// The status code.
        code: u16,
        /// The text from the response.
        text: String,
        /// The url of the API.
        url: String,
    },
    /// We made a call to the node but the protocol was unsupported
    #[error("Call to {0} is not supported on this node")]
    NotSupported(String),
}
