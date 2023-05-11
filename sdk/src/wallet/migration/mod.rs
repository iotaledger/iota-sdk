// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::wallet::Result;

mod migrate_0;

pub type LatestMigration = migrate_0::Migrate;

/// The list of migrations, in order.
const MIGRATIONS: &[&'static dyn DynMigration] = &[
    // In order to add a new migration, change the `LatestMigration` type above and add an entry at the bottom of this
    // list.
    &migrate_0::Migrate,
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationVersion {
    pub id: usize,
    pub sdk_version: String,
    pub date: time::Date,
}

impl std::fmt::Display for MigrationVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - {}", self.id, self.sdk_version, self.date)
    }
}

#[async_trait]
pub(crate) trait Migration {
    const ID: usize;
    const SDK_VERSION: &'static str;
    const DATE: time::Date;

    fn version() -> MigrationVersion {
        MigrationVersion {
            id: Self::ID,
            sdk_version: Self::SDK_VERSION.to_string(),
            date: Self::DATE,
        }
    }

    #[cfg(feature = "storage")]
    async fn migrate_storage(storage: &super::storage::manager::StorageManager) -> Result<()>;

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()>;
}

#[async_trait]
trait DynMigration: Send + Sync {
    fn version(&self) -> MigrationVersion;

    #[cfg(feature = "storage")]
    async fn migrate_storage(&self, storage: &super::storage::manager::StorageManager) -> Result<()>;

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(&self, storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()>;
}

#[async_trait]
impl<T: Migration + Send + Sync> DynMigration for T {
    fn version(&self) -> MigrationVersion {
        T::version()
    }

    #[cfg(feature = "storage")]
    async fn migrate_storage(&self, storage: &super::storage::manager::StorageManager) -> Result<()> {
        use crate::wallet::storage::constants::MIGRATION_VERSION_KEY;

        let version = self.version();
        log::info!("Migrating to version {}", version);
        T::migrate_storage(storage).await?;
        storage.set(MIGRATION_VERSION_KEY, version).await?;
        Ok(())
    }

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(&self, storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
        use crate::{
            client::storage::StorageProvider,
            wallet::wallet::operations::stronghold_backup::stronghold_snapshot::BACKUP_MIGRATION_VERSION_KEY,
        };

        let version = self.version();
        log::info!("Migrating backups to version {}", version);
        T::migrate_backup(storage).await?;
        storage
            .insert(
                BACKUP_MIGRATION_VERSION_KEY.as_bytes(),
                serde_json::to_string(&version)?.as_bytes(),
            )
            .await?;
        Ok(())
    }
}

#[cfg(feature = "storage")]
pub async fn migrate_storage(storage: &super::storage::manager::StorageManager) -> Result<()> {
    let last_migration = storage.migration.as_ref();
    if last_migration.map(|m| m.id >= MIGRATIONS.len()).unwrap_or_default() {
        return Ok(());
    }
    let next_migration = last_migration.map(|m| m.id + 1).unwrap_or_default();
    for &migration in &MIGRATIONS[next_migration..] {
        migration.migrate_storage(storage).await?;
    }
    Ok(())
}

#[cfg(feature = "stronghold")]
pub async fn migrate_backup(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
    use crate::{
        client::storage::StorageProvider,
        wallet::wallet::operations::stronghold_backup::stronghold_snapshot::BACKUP_MIGRATION_VERSION_KEY,
    };

    let last_migration = storage
        .get(BACKUP_MIGRATION_VERSION_KEY.as_bytes())
        .await?
        .map(|bytes| serde_json::from_slice::<MigrationVersion>(&bytes))
        .transpose()?;
    if last_migration
        .as_ref()
        .map(|m| m.id >= MIGRATIONS.len())
        .unwrap_or_default()
    {
        return Ok(());
    }
    let next_migration = last_migration.map(|m| m.id + 1).unwrap_or_default();
    for &migration in &MIGRATIONS[next_migration..] {
        migration.migrate_backup(storage).await?;
    }
    Ok(())
}

pub fn latest_migration_version() -> MigrationVersion {
    <LatestMigration as Migration>::version()
}
