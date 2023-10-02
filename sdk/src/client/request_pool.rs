// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::sync::Arc;

use async_trait::async_trait;
use futures::Future;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};

#[derive(Debug, Clone)]
pub(crate) struct RequestPool {
    inner: Arc<RwLock<RequestPoolInner>>,
}

#[derive(Debug)]
pub(crate) struct RequestPoolInner {
    sender: UnboundedSender<()>,
    recv: UnboundedReceiver<()>,
    size: usize,
}

#[derive(Debug)]
pub(crate) struct Requester {
    sender: UnboundedSender<()>,
}

impl RequestPool {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(RequestPoolInner::new(size))),
        }
    }

    pub(crate) async fn borrow(&self) -> Requester {
        // Get permission to request
        let mut lock = self.write().await;
        lock.recv.recv().await;
        let sender = lock.sender.clone();
        drop(lock);
        Requester { sender }
    }

    pub(crate) async fn size(&self) -> usize {
        self.read().await.size
    }

    pub(crate) async fn resize(&self, new_size: usize) {
        *self.write().await = RequestPoolInner::new(new_size);
    }
}

impl core::ops::Deref for RequestPool {
    type Target = RwLock<RequestPoolInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl RequestPoolInner {
    fn new(size: usize) -> Self {
        let (sender, recv) = tokio::sync::mpsc::unbounded_channel();
        // Prepare the channel with the requesters
        for _ in 0..size {
            sender.send(()).ok();
        }
        Self { sender, recv, size }
    }
}

impl Drop for Requester {
    fn drop(&mut self) {
        // This can only fail if the receiver is closed, in which case we don't care.
        self.sender.send(()).ok();
    }
}

#[async_trait]
pub(crate) trait RateLimitExt: Future {
    async fn rate_limit(self, request_pool: &RequestPool) -> Self::Output
    where
        Self: Sized,
    {
        let requester = request_pool.borrow().await;
        let output = self.await;
        drop(requester);
        output
    }
}
impl<F: Future> RateLimitExt for F {}
