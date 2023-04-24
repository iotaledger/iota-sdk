// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Type alias of `Result` in Node errors
pub type Result<T> = std::result::Result<T, Error>;

/// Stronghold errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Node returned a result that is not 200
    #[error("{0}")]
    BadResponse(String),
    ///
    #[error("couldn't get a result from any node")]
    NoResult,
    ///
    #[error("No output for {0}")]
    NoOutput(&'static str),
    ///
    #[error("No node available for remote Pow")]
    UnavailablePow,
    /// reqwest error
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error from RestAPI calls with unexpected status code response
    #[error("response error with status code {code}: {text}, URL: {url}")]
    ResponseError {
        /// The status code.
        code: u16,
        /// The text from the response.
        text: String,
        /// The url of the API.
        url: String,
    },
}
