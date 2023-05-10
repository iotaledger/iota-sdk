// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::storage::manager::StorageManager;
use crate::wallet::{storage::constants::MIGRATION_VERSION_KEY, Result};

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
    pub app_version: String,
    pub date: time::Date,
}

impl std::fmt::Display for MigrationVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - {}", self.id, self.app_version, self.date)
    }
}

#[async_trait]
pub(crate) trait Migration {
    const ID: usize;
    const WALLET_VERSION: &'static str;
    const DATE: time::Date;

    fn version() -> MigrationVersion {
        MigrationVersion {
            id: Self::ID,
            app_version: Self::WALLET_VERSION.to_string(),
            date: Self::DATE,
        }
    }

    async fn migrate(storage: &StorageManager) -> Result<()>;
}

#[async_trait]
trait DynMigration: Send + Sync {
    fn version(&self) -> MigrationVersion;

    async fn migrate(&self, storage: &StorageManager) -> Result<()>;
}

#[async_trait]
impl<T: Migration + Send + Sync> DynMigration for T {
    fn version(&self) -> MigrationVersion {
        T::version()
    }

    async fn migrate(&self, storage: &StorageManager) -> Result<()> {
        let version = self.version();
        log::info!("Migrating to version {}", version);
        T::migrate(storage).await?;
        storage.set(MIGRATION_VERSION_KEY, version).await?;
        Ok(())
    }
}

pub async fn migrate(storage: &StorageManager) -> Result<()> {
    let last_migration = storage.migration.as_ref();
    if last_migration.map(|m| m.id >= MIGRATIONS.len()).unwrap_or_default() {
        return Ok(());
    }
    let next_migration = last_migration.map(|m| m.id + 1).unwrap_or_default();
    for &migration in &MIGRATIONS[next_migration..] {
        migration.migrate(storage).await?;
    }
    Ok(())
}

pub fn latest_migration_version() -> MigrationVersion {
    <LatestMigration as Migration>::version()
}
