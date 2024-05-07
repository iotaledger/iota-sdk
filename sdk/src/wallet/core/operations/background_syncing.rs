// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use tokio::time::timeout;

use crate::{
    client::{secret::SecretManage, ClientError},
    wallet::{operations::syncing::SyncOptions, task, Wallet, WalletError},
};

/// The default interval for background syncing
pub(crate) const DEFAULT_BACKGROUNDSYNCING_INTERVAL: Duration = Duration::from_secs(7);

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum BackgroundSyncStatus {
    Stopped,
    Running,
    Stopping,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Start the background syncing process for the wallet, default interval is 7 seconds
    pub async fn start_background_syncing(
        &self,
        options: impl Into<Option<SyncOptions>> + Send,
        interval: Option<Duration>,
    ) -> Result<(), WalletError> {
        log::debug!("[start_background_syncing]");

        let options = options.into();
        let (tx_background_sync, mut rx_background_sync) = self.background_syncing_status.clone();

        // stop existing process if running
        if *rx_background_sync.borrow() == BackgroundSyncStatus::Running {
            tx_background_sync.send(BackgroundSyncStatus::Stopping).ok();
        }

        log::debug!("[background_syncing]: waiting for the old process to stop");
        rx_background_sync
            .wait_for(|status| *status != BackgroundSyncStatus::Stopping)
            .await
            .ok();

        tx_background_sync.send(BackgroundSyncStatus::Running).ok();

        let wallet = self.clone();
        let interval_seconds = interval.unwrap_or(DEFAULT_BACKGROUNDSYNCING_INTERVAL);

        task::spawn(async move {
            loop {
                log::debug!("[background_syncing]: syncing wallet");

                if let Err(err) = wallet.sync(options.clone()).await {
                    log::debug!("[background_syncing] error: {}", err)
                }

                let res = timeout(interval_seconds, async {
                    rx_background_sync
                        .wait_for(|status| *status == BackgroundSyncStatus::Stopping)
                        .await
                        .is_ok()
                })
                .await;

                // If true it means rx_background_sync changed to BackgroundSyncStatus::Stopping
                if Ok(true) == res {
                    log::debug!("[background_syncing]: stopping");
                    break;
                }
            }
            tx_background_sync.send(BackgroundSyncStatus::Stopped).ok();
            log::debug!("[background_syncing]: stopped");
        });
        Ok(())
    }

    /// Request to stop the background syncing of the wallet
    pub fn request_stop_background_syncing(&self) {
        log::debug!("[request_stop_background_syncing]");
        self.background_syncing_status
            .0
            .send(BackgroundSyncStatus::Stopping)
            .ok();
    }

    /// Stop the background syncing of the wallet
    pub async fn stop_background_syncing(&self) -> Result<(), WalletError> {
        log::debug!("[stop_background_syncing]");

        let mut rx_background_sync = self.background_syncing_status.1.clone();

        // immediately return if is stopped
        if *rx_background_sync.borrow() == BackgroundSyncStatus::Stopped {
            return Ok(());
        }

        // send stop request
        self.request_stop_background_syncing();

        // wait until it has stopped
        rx_background_sync
            .wait_for(|status| *status == BackgroundSyncStatus::Stopped)
            .await
            .ok();

        Ok(())
    }
}
