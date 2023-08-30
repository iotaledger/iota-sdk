// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    collections::{BinaryHeap, VecDeque},
    sync::Arc,
};
use core::pin::Pin;

use futures::{future::BoxFuture, Future, FutureExt};
use pin_project::pin_project;
use thiserror::Error;
use tokio::{
    sync::{mpsc::error::TryRecvError, oneshot::error::RecvError, Mutex},
    task::{JoinError, JoinHandle},
};

#[derive(Debug, Error)]
pub(crate) enum WorkerError {
    #[error("worker pool is empty")]
    EmptyPool,
    #[error("error sending worker task to processor")]
    Send,
    #[error("error receiving worker output: {0}")]
    Receive(#[from] RecvError),
    #[error("error exiting worker: {0}")]
    Join(#[from] JoinError),
}

#[derive(Debug)]
pub(crate) struct WorkerPool(Mutex<VecDeque<Arc<Worker>>>);

impl WorkerPool {
    pub(crate) fn new(count: usize) -> Self {
        let mut workers = VecDeque::with_capacity(count);
        for _ in 0..count {
            workers.push_back(Arc::new(Worker::spawn()));
        }
        Self(Mutex::new(workers))
    }

    pub(crate) async fn process_task<F: 'static + Future + Send>(
        &self,
        priority: TaskPriority,
        future: F,
    ) -> Result<F::Output, WorkerError>
    where
        F::Output: Send,
    {
        let mut pool = self.0.lock().await;
        let worker = pool.front().ok_or(WorkerError::EmptyPool)?.clone();
        // Move the worker to the back
        pool.rotate_left(1);
        drop(pool);
        let output = worker.process_task(priority, future).await?;
        Ok(output)
    }

    pub(crate) async fn resize(&self, new_size: usize) -> Result<(), WorkerError> {
        if new_size == 0 {
            return Err(WorkerError::EmptyPool);
        }
        let mut pool = self.0.lock().await;
        let curr_size = pool.len();
        match new_size.cmp(&curr_size) {
            core::cmp::Ordering::Less => {
                while pool.len() > new_size {
                    if let Some(worker) = pool.pop_front() {
                        worker.exit()?;
                    }
                }
            }
            core::cmp::Ordering::Greater => {
                while pool.len() < new_size {
                    pool.push_front(Arc::new(Worker::spawn()));
                }
            }
            core::cmp::Ordering::Equal => (),
        }
        Ok(())
    }

    pub(crate) async fn size(&self) -> usize {
        self.0.lock().await.len()
    }
}

pub(crate) enum WorkerEvent {
    Task(WorkerTask),
    Exit,
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum TaskPriority {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[derive(Debug)]
pub(crate) struct Worker {
    join_handle: JoinHandle<()>,
    sender: tokio::sync::mpsc::UnboundedSender<WorkerEvent>,
}

impl Worker {
    fn spawn() -> Self {
        let (sender, mut recv) = tokio::sync::mpsc::unbounded_channel();
        let join_handle = tokio::spawn(async move {
            let mut queue = BinaryHeap::new();
            let mut exiting = false;
            // Wait to be awakened by the channel
            while let Some(task) = recv.recv().await {
                match task {
                    WorkerEvent::Task(task) => queue.push(task),
                    WorkerEvent::Exit => exiting = true,
                }
                loop {
                    if !exiting {
                        // Get up to 10 messages at a time
                        for _ in 0..10 {
                            match recv.try_recv() {
                                Ok(task) => match task {
                                    WorkerEvent::Task(task) => queue.push(task),
                                    WorkerEvent::Exit => exiting = true,
                                },
                                Err(e) => match e {
                                    TryRecvError::Empty => break,
                                    TryRecvError::Disconnected => return,
                                },
                            }
                        }
                    }
                    if let Some(next) = queue.pop() {
                        next.await;
                    } else {
                        break;
                    }
                }
                if exiting {
                    return;
                }
            }
        });
        Self { join_handle, sender }
    }

    async fn process_task<F: 'static + Future + Send>(
        &self,
        priority: TaskPriority,
        future: F,
    ) -> Result<F::Output, WorkerError>
    where
        F::Output: Send,
    {
        let (task, recv) = WorkerTask::new(priority, future);
        self.sender
            .send(WorkerEvent::Task(task))
            .map_err(|_| WorkerError::Send)?;
        Ok(recv.await?)
    }

    fn exit(&self) -> Result<(), WorkerError> {
        self.sender.send(WorkerEvent::Exit).map_err(|_| WorkerError::Send)?;
        Ok(())
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}

#[pin_project]
pub(crate) struct WorkerTask {
    id: u128,
    priority: TaskPriority,
    #[pin]
    future: BoxFuture<'static, ()>,
}

impl WorkerTask {
    fn new<F: 'static + Future + Send>(
        priority: TaskPriority,
        future: F,
    ) -> (Self, tokio::sync::oneshot::Receiver<F::Output>)
    where
        F::Output: Send,
    {
        let uuid = rand::random();
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let future = async {
            sender.send(future.await).ok();
        }
        .boxed();
        (
            Self {
                id: uuid,
                priority,
                future,
            },
            receiver,
        )
    }
}

impl Future for WorkerTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        let this = self.project();
        this.future.poll(cx)
    }
}

impl core::fmt::Debug for WorkerTask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WorkerTask").field("priority", &self.priority).finish()
    }
}

impl Ord for WorkerTask {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for WorkerTask {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for WorkerTask {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for WorkerTask {}
