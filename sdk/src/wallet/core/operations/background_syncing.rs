// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use tokio::time::interval;

use crate::{
    client::secret::SecretManage,
    wallet::{operations::syncing::SyncOptions, Wallet, task},
};

/// The default interval for background syncing
pub(crate) const DEFAULT_BACKGROUNDSYNCING_INTERVAL: Duration = Duration::from_secs(7);

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum BackgroundSyncStatus {
    NotRunning,
    Running,
    Stopping
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Start the background syncing process for the wallet, default interval is 7 seconds
    pub async fn start_background_syncing(
        &self,
        options: Option<SyncOptions>,
        requested_interval: Option<Duration>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[start_background_syncing]");

        let (tx_background_sync, mut rx_background_sync) = self.background_syncing_status.clone();
       
        // stop existing process if running
        if *rx_background_sync.borrow() == BackgroundSyncStatus::Running {
            tx_background_sync.send(BackgroundSyncStatus::Stopping).ok();
        }

        log::debug!("[background_syncing]: waiting for the old process to stop");
        rx_background_sync.wait_for(|status| *status != BackgroundSyncStatus::Stopping).await.ok();

        tx_background_sync.send(BackgroundSyncStatus::Running).ok();

        let wallet = self.clone();
     
        task::spawn(async move {
            'outer: loop {
                log::debug!("[background_syncing]: syncing wallet");

                if let Err(err) = wallet.sync(options.clone()).await {
                    log::debug!("[background_syncing] error: {}", err)
                }

                // split interval syncing to seconds so stopping the process doesn't have to wait long
                let seconds = requested_interval.unwrap_or(DEFAULT_BACKGROUNDSYNCING_INTERVAL).as_secs();
                let mut interval = interval(Duration::from_secs(1));
                for _ in 0..seconds {
                    if *rx_background_sync.borrow() == BackgroundSyncStatus::Stopping  {
                        log::debug!("[background_syncing]: stopping");
                        break 'outer;
                    }
                    interval.tick().await;
                }
            }
            tx_background_sync.send(BackgroundSyncStatus::NotRunning).ok();
            log::debug!("[background_syncing]: stopped");
        });
        Ok(())
    }

    /// Request to stop the background syncing of the wallet
    pub fn request_stop_background_syncing(&self) {
        log::debug!("[request_stop_background_syncing]");
        self.background_syncing_status.0.send(BackgroundSyncStatus::Stopping).ok();
    }

    /// Stop the background syncing of the wallet
    pub async fn stop_background_syncing(&self) -> crate::wallet::Result<()> {
        log::debug!("[stop_background_syncing]");

        let mut rx_background_sync = self.background_syncing_status.1.clone();

        // immediately return if not running
        if *rx_background_sync.borrow() == BackgroundSyncStatus::NotRunning {
            return Ok(());
        }
        
        // send stop request
        self.request_stop_background_syncing();

        // wait until it stopped
        rx_background_sync.wait_for(|status| *status == BackgroundSyncStatus::NotRunning).await.ok();

        Ok(())
    }
}
