// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe},
};

use backtrace::Backtrace;
use futures::{Future, FutureExt};

use crate::{response::Response, Result};

fn panic_to_response_message(panic: Box<dyn Any>) -> Response {
    let msg = panic.downcast_ref::<String>().map_or_else(
        || {
            panic.downcast_ref::<&str>().map_or_else(
                || "Internal error".to_string(),
                |message| format!("Internal error: {message}"),
            )
        },
        |message| format!("Internal error: {message}"),
    );

    let current_backtrace = Backtrace::new();
    Response::Panic(format!("{msg}\n\n{current_backtrace:?}"))
}

#[cfg(not(target_family = "wasm"))]
pub(crate) async fn convert_async_panics<F>(f: impl FnOnce() -> F + Send) -> Result<Response>
where
    F: Future<Output = Result<Response>> + Send,
{
    AssertUnwindSafe(f())
        .catch_unwind()
        .await
        .unwrap_or_else(|panic| Ok(panic_to_response_message(panic)))
}

#[cfg(target_family = "wasm")]
#[allow(clippy::future_not_send)]
pub(crate) async fn convert_async_panics<F>(f: impl FnOnce() -> F) -> Result<Response>
where
    F: Future<Output = Result<Response>>,
{
    AssertUnwindSafe(f())
        .catch_unwind()
        .await
        .unwrap_or_else(|panic| Ok(panic_to_response_message(panic)))
}

pub(crate) fn convert_panics<F: FnOnce() -> Result<Response>>(f: F) -> Result<Response> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(panic) => Ok(panic_to_response_message(panic)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{panic::convert_async_panics, Response};

    #[tokio::test]
    async fn panic_to_response() {
        match convert_async_panics(|| async { panic!("rekt") }).await.unwrap() {
            Response::Panic(msg) => {
                assert!(msg.contains("rekt"));
            }
            response_type => panic!("Unexpected response type: {response_type:?}"),
        };
    }
}
