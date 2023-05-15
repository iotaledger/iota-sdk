// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod migrate_0;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::wallet::Result;

pub type LatestMigration = migrate_0::Migrate;

pub(crate) const MIGRATION_VERSION_KEY: &str = "migration-version";

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
    async fn migrate_storage(storage: &super::storage::Storage) -> Result<()>;

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()>;
}

#[async_trait]
trait DynMigration: Send + Sync {
    fn version(&self) -> MigrationVersion;

    #[cfg(feature = "storage")]
    async fn migrate_storage(&self, storage: &super::storage::Storage) -> Result<()>;

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(&self, storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()>;
}

#[async_trait]
impl<T: Migration + Send + Sync> DynMigration for T {
    fn version(&self) -> MigrationVersion {
        T::version()
    }

    #[cfg(feature = "storage")]
    async fn migrate_storage(&self, storage: &super::storage::Storage) -> Result<()> {
        let version = self.version();
        log::info!("Migrating to version {}", version);
        T::migrate_storage(storage).await?;
        storage.set(MIGRATION_VERSION_KEY, version).await?;
        Ok(())
    }

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(&self, storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
        use crate::client::storage::StorageProvider;

        let version = self.version();
        log::info!("Migrating backup to version {}", version);
        T::migrate_backup(storage).await?;
        storage
            .insert(
                MIGRATION_VERSION_KEY.as_bytes(),
                serde_json::to_string(&version)?.as_bytes(),
            )
            .await?;
        Ok(())
    }
}

#[cfg(feature = "storage")]
pub async fn migrate_storage(storage: &super::storage::Storage) -> Result<()> {
    let last_migration = storage.get::<MigrationVersion>(MIGRATION_VERSION_KEY).await?;
    for migration in migrations(last_migration)? {
        migration.migrate_storage(storage).await?;
    }
    Ok(())
}

#[cfg(feature = "stronghold")]
pub async fn migrate_backup(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
    use crate::client::storage::StorageProvider;

    let last_migration = storage
        .get(MIGRATION_VERSION_KEY.as_bytes())
        .await?
        .map(|bytes| serde_json::from_slice::<MigrationVersion>(&bytes))
        .transpose()?;
    for migration in migrations(last_migration)? {
        migration.migrate_backup(storage).await?;
    }
    Ok(())
}

fn migrations(last_migration: Option<MigrationVersion>) -> Result<impl Iterator<Item = &'static dyn DynMigration>> {
    Ok(match last_migration {
        Some(last_migration) => {
            if last_migration.id > LatestMigration::ID {
                return Err(crate::wallet::Error::Migration(format!(
                    "invalid migration version: {last_migration}, current sdk version: {}",
                    env!("CARGO_PKG_VERSION")
                )));
            }
            MIGRATIONS[last_migration.id + 1..].iter().copied()
        }
        None => MIGRATIONS.iter().copied(),
    })
}

#[allow(unused)]
pub fn latest_migration_version() -> MigrationVersion {
    <LatestMigration as Migration>::version()
}
