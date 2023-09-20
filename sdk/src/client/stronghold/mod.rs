// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! [Stronghold] integration for the iota-sdk.
//!
//! Stronghold can be used as a multi-purpose secret service providing:
//!
//! - Smart-card-like secret vault
//! - Generic key-value, encrypted database
//!
//! [`StrongholdAdapter`] respectively implements [`StorageAdapter`] and [`SecretManage`] for the above purposes
//! using Stronghold. Type aliases `StrongholdStorageAdapter` and `StrongholdSecretManager` are also provided if one
//! wants to have a more consistent naming when using any of the feature sets.
//!
//! Use [`builder()`] to construct a [`StrongholdAdapter`] with customized parameters; see documentation of methods of
//! [`StrongholdAdapterBuilder`] for details. All fields are optional, but:
//!
//! - Without a password, all cryptographic operations (including database operations, as they encrypt / decrypt data)
//!   would fail.
//! - Without a password clearing timeout, the derived key would be stored in the memory for as long as possible, and
//!   could be used as an attack vector.
//! - Without a snapshot path configured, all operations would be _transient_ (i.e. all data would be lost when
//!   [`StrongholdAdapter`] is dropped, or the cached key has been cleared).
//!
//! They can also be set later on [`StrongholdAdapter`] using [`set_password()`], [`set_timeout()`], etc.
//!
//! With [`set_timeout()`], an automatic task can be spawned in the background to purge the key from memory using
//! [zeroize] after the `timeout` duration. It's used to reduce the attack vector. When the key is cleared from the
//! memory, Stronghold will be unloaded from the memory too. If no `snapshot_path` has been set at this point, then
//! secrets stored in Stronghold will be dropped and lost.
//!
//! Nevertheless, Stronghold is memory-based, so it's not required to use a snapshot file on the disk. Without a
//! snapshot path set, [`StrongholdAdapter`] will run purely in memory. If a snapshot path is set, then
//! [`StrongholdAdapter`] would lazily load the file on _the first call_ that performs some actions on Stronghold.
//! Subsequent actions are still performed in memory. If the snapshot file doesn't exist, these function calls will all
//! fail. To proactively load or store the Stronghold state from or to a Stronghold snapshot on disk, use
//! [`read_stronghold_snapshot()`] or [`write_stronghold_snapshot()`]. The latter can be used to create a snapshot file
//! after creating a [`StrongholdAdapter`] with a non-existent snapshot path.
//!
//! [Stronghold]: iota_stronghold
//! [`StorageAdapter`]: crate::client::storage::StorageAdapter
//! [`SecretManage`]: crate::client::secret::SecretManage
//! [`builder()`]: self::StrongholdAdapter::builder()
//! [`set_password()`]: self::StrongholdAdapter::set_password()
//! [`set_timeout()`]: self::StrongholdAdapter::set_timeout()
//! [`read_stronghold_snapshot()`]: self::StrongholdAdapter::read_stronghold_snapshot()
//! [`write_stronghold_snapshot()`]: self::StrongholdAdapter::write_stronghold_snapshot()

mod common;
mod error;
mod migration;
mod secret;
mod storage;

use alloc::sync::Weak;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use iota_stronghold::{KeyProvider, SnapshotPath, Stronghold};
use log::{debug, error, warn};
use tokio::{
    sync::{Mutex, MutexGuard},
    task::JoinHandle,
};
use zeroize::Zeroizing;

pub(crate) use self::common::PRIVATE_DATA_CLIENT_PATH;
pub use self::error::Error;
use super::{storage::StorageAdapter, utils::Password};

/// A wrapper on [Stronghold].
///
/// See the [module-level documentation](self) for more details.
#[derive(Debug)]
pub struct StrongholdAdapter {
    /// A stronghold instance.
    stronghold: Arc<Mutex<Stronghold>>,

    /// A key to open the Stronghold vault.
    ///
    /// Note that in [`StrongholdAdapterBuilder`] there isn't a `key()` setter, because we don't want a user to
    /// directly set this field. Instead, [`password()`] is provided to hash a user-input password string and
    /// derive a key from it.
    ///
    /// [`password()`]: self::StrongholdAdapterBuilder::password()
    key_provider: Arc<Mutex<Option<KeyProvider>>>,

    /// An interval of time, after which `key` will be cleared from the memory.
    ///
    /// This is an extra security measure to further prevent attacks. If a timeout is set, then upon a `key` is set, a
    /// timer will be spawned in the background to clear ([zeroize]) the key after `timeout`.
    ///
    /// If a [`StrongholdAdapter`] is destroyed (dropped), then the timer will stop too.
    timeout: Option<Duration>,

    /// A handle to the timeout task.
    timeout_task: Arc<Mutex<Option<TaskHandle>>>,

    /// The path to a Stronghold snapshot file.
    pub(crate) snapshot_path: PathBuf,
}

fn check_or_create_snapshot(
    stronghold: &Stronghold,
    key_provider: &KeyProvider,
    snapshot_path: &SnapshotPath,
) -> Result<(), Error> {
    let result = stronghold.load_client_from_snapshot(PRIVATE_DATA_CLIENT_PATH, key_provider, snapshot_path);

    match result {
        Err(iota_stronghold::ClientError::SnapshotFileMissing(_)) => {
            stronghold.create_client(PRIVATE_DATA_CLIENT_PATH)?;
            stronghold.commit_with_keyprovider(snapshot_path, key_provider)?;
        }
        Err(iota_stronghold::ClientError::ClientAlreadyLoaded(_)) => {
            stronghold.get_client(PRIVATE_DATA_CLIENT_PATH)?;
        }
        Err(iota_stronghold::ClientError::Inner(ref err_msg)) => {
            // Matching the error string is not ideal but stronghold doesn't wrap the error types at the moment.
            if err_msg.contains("XCHACHA20-POLY1305") || err_msg.contains("BadFileKey") {
                return Err(Error::InvalidPassword);
            } else if err_msg.contains("unsupported version") {
                if err_msg.contains("expected [3, 0], found [2, 0]") {
                    return Err(Error::UnsupportedSnapshotVersion { found: 2, expected: 3 });
                } else {
                    panic!("unsupported version mismatch");
                }
            }
        }
        _ => {}
    }

    Ok(())
}

#[derive(Default, Debug)]
pub struct StrongholdAdapterBuilder {
    stronghold: Option<Stronghold>,
    key_provider: Option<KeyProvider>,
    timeout: Option<Duration>,
}

/// Extra / custom builder method implementations.
impl StrongholdAdapterBuilder {
    pub fn stronghold(mut self, stronghold: impl Into<Option<Stronghold>>) -> Self {
        self.stronghold = stronghold.into();
        self
    }

    pub fn key_provider(mut self, key_provider: impl Into<Option<KeyProvider>>) -> Self {
        self.key_provider = key_provider.into();
        self
    }

    pub fn timeout(mut self, timeout: impl Into<Option<Duration>>) -> Self {
        self.timeout = timeout.into();
        self
    }

    /// Use an user-input password string to derive a key to use Stronghold.
    pub fn password(mut self, password: impl Into<Password>) -> Self {
        self.key_provider
            .replace(self::common::key_provider_from_password(password.into()));
        self
    }

    /// Builds a [`StrongholdAdapter`] from the configuration.
    ///
    /// If both `key` (via [`password()`]) and `timeout` (via [`timeout()`]) are set, then an asynchronous task would be
    /// spawned in Tokio to purge ([zeroize]) `key` after `timeout`. There is a small delay (usually a few milliseconds)
    /// from the return of this function to this task actually being spawned and set in the returned
    /// [`StrongholdAdapter`].
    ///
    /// **This function must be called inside a Tokio runtime context (usually in an `async fn` invoked by a Tokio
    /// runtime, either directly or indirectly)**, as it uses [tokio::spawn()], which requires a Tokio context.
    /// Otherwise, the function would panic. If this is not desired, one needs to avoid calling [`password()`] and
    /// [`timeout()`] during the building process.
    ///
    /// [`password()`]: Self::password()
    /// [`timeout()`]: Self::timeout()
    pub fn build<P: AsRef<Path>>(self, snapshot_path: P) -> Result<StrongholdAdapter, Error> {
        if snapshot_path.as_ref().is_dir() {
            // TODO: Add Error in 2.0 as its breaking.
            // Issue #1197
            panic!("Path is not a file: {:?}", snapshot_path.as_ref().to_path_buf());
        }

        // In any case, Stronghold - as a necessary component - needs to be present at this point.
        let stronghold = self.stronghold.unwrap_or_default();

        #[cfg(test)]
        iota_stronghold::engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

        if let Some(key_provider) = &self.key_provider {
            check_or_create_snapshot(&stronghold, key_provider, &SnapshotPath::from_path(&snapshot_path))?;
        }

        let has_key_provider = self.key_provider.is_some();
        let key_provider = Arc::new(Mutex::new(self.key_provider));
        let stronghold = Arc::new(Mutex::new(stronghold));
        let timeout_task = Arc::new(Mutex::new(None));

        // If both `key` and `timeout` are set, then we spawn the task and keep its join handle.
        if let (true, Some(timeout)) = (has_key_provider, self.timeout) {
            let weak = Arc::downgrade(&timeout_task);
            *timeout_task.try_lock().unwrap() = Some(tokio::spawn(task_key_clear(
                weak,
                stronghold.clone(),
                key_provider.clone(),
                timeout,
            )));
        }

        // Create the adapter as per configuration and return it.
        Ok(StrongholdAdapter {
            stronghold,
            key_provider,
            timeout: self.timeout,
            timeout_task,
            snapshot_path: snapshot_path.as_ref().to_path_buf(),
        })
    }
}

impl StrongholdAdapter {
    /// Create a builder to construct a [StrongholdAdapter].
    pub fn builder() -> StrongholdAdapterBuilder {
        StrongholdAdapterBuilder::default()
    }

    /// Test if the key hasn't been cleared.
    pub async fn is_key_available(&self) -> bool {
        self.key_provider.lock().await.is_some()
    }

    /// Use an user-input password string to derive a key to use Stronghold.
    ///
    /// This function will also spawn an asynchronous task in Tokio to automatically purge the derived key from
    /// `password` after `timeout` (if set).
    /// It will also try to load a snapshot to check if the provided password is correct, if not it's cleared and an
    /// error will be returned.
    pub async fn set_password(&self, password: impl Into<Password> + Send) -> Result<(), Error> {
        let password = password.into();

        let mut key_provider_guard = self.key_provider.lock().await;

        let key_provider = self::common::key_provider_from_password(password);

        if let Some(old_key_provider) = &*key_provider_guard {
            if old_key_provider.try_unlock()? != key_provider.try_unlock()? {
                return Err(Error::InvalidPassword);
            }
        }

        let snapshot_path = SnapshotPath::from_path(&self.snapshot_path);
        let stronghold = self.stronghold.lock().await;

        check_or_create_snapshot(&stronghold, &key_provider, &snapshot_path)?;

        *key_provider_guard = Some(key_provider);
        drop(key_provider_guard);

        // If a timeout is set, spawn a task to clear the key after the timeout.
        if let Some(timeout) = self.timeout {
            // If there has been a spawned task, stop it and re-spawn one.
            if let Some(timeout_task) = self.timeout_task.lock().await.take() {
                timeout_task.abort();
            }

            let key_provider = self.key_provider.clone();

            *self.timeout_task.lock().await = Some(tokio::spawn(task_key_clear(
                Arc::downgrade(&self.timeout_task),
                self.stronghold.clone(),
                key_provider,
                timeout,
            )));
        }

        Ok(())
    }

    /// Change the password of the currently loaded Stronghold.
    ///
    /// If a snapshot path has been set, then it'll be rewritten with the newly set password.
    ///
    /// The secrets (e.g. mnemonic) stored in the Stronghold vault will be preserved, but the data saved via the
    /// [`StorageAdapter`] interface won't - they'll stay encrypted with the old password. To re-encrypt these
    /// data, provide a list of keys in `keys_to_re_encrypt`, as we have no way to list and iterate over every
    /// key-value in the Stronghold store - we'll attempt on the ones provided instead. Set it to `None` to skip
    /// re-encryption.
    pub async fn change_password(&self, new_password: impl Into<Password> + Send) -> Result<(), Error> {
        let new_password = new_password.into();

        // Stop the key clearing task to prevent the key from being abruptly cleared (largely).
        if let Some(timeout_task) = self.timeout_task.lock().await.take() {
            timeout_task.abort();
        }

        // In case something goes wrong we can recover from the snapshot.
        self.write_stronghold_snapshot(None).await?;

        // If there are keys to re-encrypt, we iterate over the requested keys and attempt to re-encrypt the
        // corresponding values.
        //
        // Note that [`StorageAdapter`] methods will do encryption / decryption automatically, so we collect values
        // to the memory first (decrypted with the old key), then change `self.key`, then store them back (encrypted
        // with the new key).
        let mut values = Vec::new();
        let keys_to_re_encrypt = self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .keys()?
            .into_iter()
            .map(|k| unsafe { String::from_utf8_unchecked(k) })
            .collect::<Vec<_>>();

        for key in keys_to_re_encrypt {
            let value = match self.get_bytes(&key).await {
                Err(err) => {
                    error!("an error occurred during the re-encryption of Stronghold Store: {err}");

                    // Recover: restart the key clearing task
                    if let Some(timeout) = self.timeout {
                        let key_provider = self.key_provider.clone();

                        *self.timeout_task.lock().await = Some(tokio::spawn(task_key_clear(
                            Arc::downgrade(&self.timeout_task),
                            self.stronghold.clone(),
                            key_provider,
                            timeout,
                        )));
                    }

                    return Err(err);
                }
                Ok(None) => continue,
                Ok(Some(value)) => Zeroizing::new(value),
            };

            values.push((key, value));
        }

        // Now we put the new key in, enabling encryption with the new key. Also, take the old key out to prevent
        // disasters.
        let old_key_provider = {
            let mut lock = self.key_provider.lock().await;
            let old_key_provider = lock.take();
            *lock = Some(self::common::key_provider_from_password(new_password));

            old_key_provider
        };

        for (key, value) in values {
            if let Err(err) = self.set_bytes(&key, &value).await {
                error!("an error occurred during the re-encryption of Stronghold store: {err}");

                // Recover: put the old key back
                *self.key_provider.lock().await = old_key_provider;

                // Recover: forcefully reload Stronghold
                self.read_stronghold_snapshot().await?;

                // Recover: restart key clearing task
                if let Some(timeout) = self.timeout {
                    let key_provider = self.key_provider.clone();

                    *self.timeout_task.lock().await = Some(tokio::spawn(task_key_clear(
                        Arc::downgrade(&self.timeout_task),
                        self.stronghold.clone(),
                        key_provider,
                        timeout,
                    )));
                }

                return Err(err);
            }
        }

        // Rewrite the snapshot to finish the password changing process.
        self.write_stronghold_snapshot(None).await?;

        // Restart the key clearing task.
        if let Some(timeout) = self.timeout {
            let key_provider = self.key_provider.clone();

            *self.timeout_task.lock().await = Some(tokio::spawn(task_key_clear(
                Arc::downgrade(&self.timeout_task),
                self.stronghold.clone(),
                key_provider,
                timeout,
            )));
        }

        Ok(())
    }

    /// Immediately clear ([zeroize]) the stored key.
    ///
    /// If a key clearing thread has been spawned, then it'll be stopped too.
    pub async fn clear_key(&self) {
        // Stop a spawned task and setting it to None first.
        if let Some(timeout_task) = self.timeout_task.lock().await.take() {
            timeout_task.abort();
        }

        // Unloading the snapshot requires the key
        if self.is_key_available().await {
            // Unload Stronghold first, but we can't do much about the errors.
            if let Err(err) = self.unload_stronghold_snapshot().await {
                warn!("failed to unload Stronghold while clearing the key: {err}");
            }
        }

        // Purge the key, setting it to None then.
        self.key_provider.lock().await.take();
        debug!("cleared stronghold key");
    }

    /// Get timeout for the key clearing task.
    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Set timeout for the key clearing task.
    ///
    /// If there has been a key clearing task running, then it will be terminated before a new one is spawned. If
    /// `new_timeout` is `None`, or the key has been purged, then no new task will be spawned (the current running task
    /// will be terminated).
    ///
    /// The key won't be cleared.
    pub async fn set_timeout(&mut self, new_timeout: Option<Duration>) {
        // In any case we terminate the current task (if there is) first.
        if let Some(timeout_task) = self.timeout_task.lock().await.take() {
            timeout_task.abort();
        }

        // Keep the new timeout.
        self.timeout = new_timeout;

        // If a new timeout is set and the key is still in the memory, spawn a new task; otherwise we do nothing.
        if let (Some(_), Some(timeout)) = (self.key_provider.lock().await.as_ref(), self.timeout) {
            let key_provider = self.key_provider.clone();

            *self.timeout_task.lock().await = Some(tokio::spawn(task_key_clear(
                Arc::downgrade(&self.timeout_task),
                self.stronghold.clone(),
                key_provider,
                timeout,
            )));
        }
    }

    /// Restart the key clearing task.
    ///
    /// This is equivalent to calling `set_timeout()` with the currently set `timeout`.
    pub async fn restart_key_clearing_task(&mut self) {
        self.set_timeout(self.get_timeout()).await;
    }

    /// Load Stronghold from a snapshot at `snapshot_path`, if it hasn't been loaded yet.
    pub async fn read_stronghold_snapshot(&self) -> Result<(), Error> {
        // The key needs to be supplied first.
        let locked_key_provider = self.key_provider.lock().await;
        let key_provider = if let Some(key_provider) = &*locked_key_provider {
            key_provider
        } else {
            return Err(Error::KeyCleared);
        };

        self.stronghold.lock().await.load_client_from_snapshot(
            PRIVATE_DATA_CLIENT_PATH,
            key_provider,
            &SnapshotPath::from_path(&self.snapshot_path),
        )?;

        Ok(())
    }

    /// Persist Stronghold to a snapshot at a provided `snapshot_path` or at the Stronghold's own `snapshot_path` if
    /// None.
    ///
    /// It doesn't unload the snapshot; see also [`unload_stronghold_snapshot()`].
    ///
    /// [`unload_stronghold_snapshot()`]: Self::unload_stronghold_snapshot()
    pub async fn write_stronghold_snapshot(&self, snapshot_path: Option<&Path>) -> Result<(), Error> {
        if matches!(snapshot_path, Some(p) if p.is_dir()) {
            // TODO: Add Error in 2.0 as its breaking.
            // Issue #1197
            panic!("Path is not a file: {:?}", p);
        }

        // The key needs to be supplied first.
        let locked_key_provider = self.key_provider.lock().await;
        let key_provider = if let Some(key_provider) = &*locked_key_provider {
            key_provider
        } else {
            return Err(Error::KeyCleared);
        };

        self.stronghold.lock().await.commit_with_keyprovider(
            &SnapshotPath::from_path(snapshot_path.unwrap_or(&self.snapshot_path)),
            key_provider,
        )?;

        Ok(())
    }

    /// Unload Stronghold from memory.
    ///
    /// It writes Stronghold snapshot to disk. All secrets will be purged from the
    /// memory, so if secrets aren't written to disk (for example, no snapshot path has been provided, i.e. running
    /// Stronghold purely in memory) then secrets stored in Stronghold will be lost.
    ///
    /// To further prevent Stronghold methods to be invoked without valid key, this method will be invoked every time
    /// the cached key is cleared from the memory. In other words, if a `timeout` is set and a `snapshot_path` is not
    /// set for a [`StrongholdAdapter`], then after `timeout` Stronghold will be purged. See the [module-level
    /// documentation](self) for more details.
    pub async fn unload_stronghold_snapshot(&self) -> Result<(), Error> {
        // Flush Stronghold.
        self.write_stronghold_snapshot(None).await?;

        self.stronghold.lock().await.clear()?;

        Ok(())
    }

    /// Acquire the stronghold lock.
    pub async fn inner(&self) -> MutexGuard<'_, Stronghold> {
        self.stronghold.lock().await
    }
}

type TaskHandle = JoinHandle<()>;

/// The asynchronous key clearing task purging `key` after `timeout` spent in Tokio.
async fn task_key_clear(
    task: Weak<Mutex<Option<TaskHandle>>>,
    stronghold: Arc<Mutex<Stronghold>>,
    key_provider: Arc<Mutex<Option<KeyProvider>>>,
    timeout: Duration,
) {
    tokio::time::sleep(timeout).await;

    // If the weak pointer cannot upgrade, that means the secret manager has been dropped,
    // so we can just exit.
    if let Some(task) = task.upgrade() {
        // Take the join handle, but hold the lock until we're done
        let mut lock = task.lock().await;
        lock.take();

        debug!("StrongholdAdapter is purging the key");
        key_provider.lock().await.take();

        if let Err(e) = stronghold.lock().await.clear() {
            log::error!("Failed to clear stronghold keys: {e}");
        }

        drop(lock);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn test_clear_key() {
        let timeout = Duration::from_millis(100);

        let stronghold_path = "test_clear_key.stronghold";
        let mut adapter = StrongholdAdapter::builder()
            .password("drowssap".to_owned())
            .timeout(timeout)
            .build(stronghold_path)
            .unwrap();

        // There is a small delay between `build()` and the key clearing task being actually spawned and kept.
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Setting a password would spawn a task to automatically clear the key.
        assert!(adapter.key_provider.lock().await.is_some());
        assert_eq!(adapter.get_timeout(), Some(timeout));
        assert!(adapter.timeout_task.lock().await.is_some());

        // After the timeout, the key should be purged.
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(adapter.key_provider.lock().await.is_none());
        assert_eq!(adapter.get_timeout(), Some(timeout));
        assert!(adapter.timeout_task.lock().await.is_none());

        // Set the key again, but this time we manually purge the key.
        let timeout = None;
        adapter.set_timeout(timeout).await;

        assert!(adapter.set_password("password".to_owned()).await.is_err());

        adapter.clear_key().await;
        assert!(adapter.key_provider.lock().await.is_none());
        assert_eq!(adapter.get_timeout(), timeout);
        assert!(adapter.timeout_task.lock().await.is_none());

        // Even if we attempt to restart the task, it won't.
        adapter.restart_key_clearing_task().await;
        assert!(adapter.key_provider.lock().await.is_none());
        assert_eq!(adapter.get_timeout(), timeout);
        assert!(adapter.timeout_task.lock().await.is_none());

        fs::remove_file(stronghold_path).unwrap();
    }

    #[tokio::test]
    async fn stronghold_password_already_set() {
        let stronghold_path = "stronghold_password_already_set.stronghold";
        let adapter = StrongholdAdapter::builder()
            .password("drowssap".to_owned())
            .build(stronghold_path)
            .unwrap();

        adapter.clear_key().await;
        // After the key got cleared it should work again to set it
        assert!(adapter.set_password("drowssap".to_owned()).await.is_ok());
        // When the password already exists, it should still work
        assert!(adapter.set_password("drowssap".to_owned()).await.is_ok());
        // When the password already exists, but a wrong one is provided, it should return an error
        assert!(adapter.set_password("other_password".to_owned()).await.is_err());

        fs::remove_file(stronghold_path).unwrap();
    }
}
