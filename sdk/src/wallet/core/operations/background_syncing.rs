// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{sync::atomic::Ordering, time::Duration};

use tokio::time::sleep;

use crate::{
    client::secret::SecretManage,
    wallet::{operations::syncing::SyncOptions, Wallet},
};

/// The default interval for background syncing
pub(crate) const DEFAULT_BACKGROUNDSYNCING_INTERVAL: Duration = Duration::from_secs(7);

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Start the background syncing process for all accounts, default interval is 7 seconds
    pub async fn start_background_syncing(
        &self,
        options: Option<SyncOptions>,
        interval: Option<Duration>,
    ) -> crate::wallet::Result<()> {
        log::debug!("[start_background_syncing]");
        // stop existing process if running
        if self.background_syncing_status.load(Ordering::Relaxed) == 1 {
            self.background_syncing_status.store(2, Ordering::Relaxed);
        };
        while self.background_syncing_status.load(Ordering::Relaxed) == 2 {
            log::debug!("[background_syncing]: waiting for the old process to stop");
            sleep(Duration::from_secs(1)).await;
        }

        self.background_syncing_status.store(1, Ordering::Relaxed);
        let wallet = self.clone();
        let _background_syncing = std::thread::spawn(move || {
            #[cfg(not(target_family = "wasm"))]
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            #[cfg(target_family = "wasm")]
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(async {
                'outer: loop {
                    log::debug!("[background_syncing]: syncing wallet");

                    match wallet.sync(options.clone()).await {
                        Ok(_) => {}
                        Err(err) => log::debug!("[background_syncing] error: {}", err),
                    };

                    // split interval syncing to seconds so stopping the process doesn't have to wait long
                    let seconds = interval.unwrap_or(DEFAULT_BACKGROUNDSYNCING_INTERVAL).as_secs();
                    for _ in 0..seconds {
                        if wallet.background_syncing_status.load(Ordering::Relaxed) == 2 {
                            log::debug!("[background_syncing]: stopping");
                            break 'outer;
                        }
                        sleep(Duration::from_secs(1)).await;
                    }
                }
                wallet.background_syncing_status.store(0, Ordering::Relaxed);
                log::debug!("[background_syncing]: stopped");
            });
        });
        Ok(())
    }

    /// Stop the background syncing of the accounts
    pub async fn stop_background_syncing(&self) -> crate::wallet::Result<()> {
        log::debug!("[stop_background_syncing]");
        // immediately return if not running
        if self.background_syncing_status.load(Ordering::Relaxed) == 0 {
            return Ok(());
        }
        // send stop request
        self.background_syncing_status.store(2, Ordering::Relaxed);
        // wait until it stopped
        while self.background_syncing_status.load(Ordering::Relaxed) != 0 {
            #[cfg(target_family = "wasm")]
            gloo_timers::future::TimeoutFuture::new(10).await;
            #[cfg(not(target_family = "wasm"))]
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        Ok(())
    }
}
