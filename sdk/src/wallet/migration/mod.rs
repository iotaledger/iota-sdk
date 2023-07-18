// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod migrate_0;
mod migrate_1;
mod migrate_2;
mod migrate_3;

use std::collections::HashMap;

use anymap::Map;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    client::storage::StorageAdapter,
    wallet::{Error, Result},
};

pub(crate) const MIGRATION_VERSION_KEY: &str = "migration-version";

#[cfg(feature = "stronghold")]
struct LatestBackupMigration(MigrationVersion);

static MIGRATIONS: Lazy<Map<dyn anymap::any::Any + Send + Sync>> = Lazy::new(|| {
    let mut migrations = Map::new();
    #[cfg(feature = "storage")]
    {
        use super::storage::Storage;
        const STORAGE_MIGRATIONS: [(Option<usize>, &'static dyn DynMigration<Storage>); 4] = [
            // In order to add a new storage migration, add an entry at the bottom of this list
            // and change the list length above.
            // The entry should be in the form of a key-value pair, from previous migration to next.
            // i.e. (Some(migrate_<N>::Migrate::ID), &migrate_<N+1>::Migrate)
            (None, &migrate_0::Migrate),
            (Some(migrate_0::Migrate::ID), &migrate_1::Migrate),
            (Some(migrate_1::Migrate::ID), &migrate_2::Migrate),
            (Some(migrate_2::Migrate::ID), &migrate_3::Migrate),
        ];
        migrations.insert(std::collections::HashMap::from(STORAGE_MIGRATIONS));
    }
    #[cfg(feature = "stronghold")]
    {
        use crate::client::stronghold::StrongholdAdapter;
        const BACKUP_MIGRATIONS: [(Option<usize>, &'static dyn DynMigration<StrongholdAdapter>); 4] = [
            // In order to add a new backup migration, and add an entry at the bottom of this list
            // and change the list length above.
            // The entry should be in the form of a key-value pair, from previous migration to next.
            // i.e. (Some(migrate_<N>::Migrate::ID), &migrate_<N+1>::Migrate)
            (None, &migrate_0::Migrate),
            (Some(migrate_0::Migrate::ID), &migrate_1::Migrate),
            (Some(migrate_1::Migrate::ID), &migrate_2::Migrate),
            (Some(migrate_2::Migrate::ID), &migrate_3::Migrate),
        ];
        migrations.insert(LatestBackupMigration(BACKUP_MIGRATIONS.last().unwrap().1.version()));
        migrations.insert(std::collections::HashMap::from(BACKUP_MIGRATIONS));
    }
    migrations
});

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

pub(crate) trait MigrationData {
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
}

#[async_trait]
pub(crate) trait Migration<S: StorageAdapter>: MigrationData {
    async fn migrate(storage: &S) -> Result<()>;
}

#[async_trait]
trait DynMigration<S: StorageAdapter>: Send + Sync {
    fn version(&self) -> MigrationVersion;

    async fn migrate(&self, storage: &S) -> Result<()>;
}

#[async_trait]
impl<S: StorageAdapter, T: Migration<S> + Send + Sync> DynMigration<S> for T
where
    crate::wallet::Error: From<S::Error>,
    S::Error: From<serde_json::Error>,
{
    fn version(&self) -> MigrationVersion {
        T::version()
    }

    async fn migrate(&self, storage: &S) -> Result<()> {
        let version = self.version();
        log::info!("Migrating to version {}", version);
        T::migrate(storage).await?;
        storage.set(MIGRATION_VERSION_KEY, &version).await?;
        Ok(())
    }
}

pub async fn migrate<S: 'static + StorageAdapter>(storage: &S) -> Result<()>
where
    crate::wallet::Error: From<S::Error>,
    S::Error: From<serde_json::Error>,
{
    let last_migration = storage.get::<MigrationVersion>(MIGRATION_VERSION_KEY).await?;
    for migration in migrations(last_migration)? {
        migration.migrate(storage).await?;
    }
    Ok(())
}

fn migrations<S: 'static + StorageAdapter>(
    mut last_migration: Option<MigrationVersion>,
) -> Result<Vec<&'static dyn DynMigration<S>>> {
    let migrations = MIGRATIONS
        .get::<HashMap<Option<usize>, &'static dyn DynMigration<S>>>()
        .ok_or_else(|| {
            Error::Migration(format!(
                "invalid migration storage kind: {}",
                std::any::type_name::<S>()
            ))
        })?;
    let mut res = Vec::new();
    while let Some(next) = migrations.get(&last_migration.as_ref().map(|m| m.id)) {
        last_migration = Some(next.version());
        res.push(*next);
    }
    Ok(res)
}

#[cfg(feature = "stronghold")]
pub fn latest_backup_migration_version() -> MigrationVersion {
    MIGRATIONS.get::<LatestBackupMigration>().unwrap().0.clone()
}

trait Convert {
    type New: Serialize + DeserializeOwned;
    type Old: DeserializeOwned;

    fn check(value: &mut serde_json::Value) -> crate::wallet::Result<()> {
        if Self::New::deserialize(&*value).is_err() {
            *value = serde_json::to_value(Self::convert(Self::Old::deserialize(&*value)?)?)?;
        }
        Ok(())
    }

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New>;
}

fn rename_keys(json: &mut serde_json::Value) {
    match json {
        serde_json::Value::Array(a) => a.iter_mut().for_each(rename_keys),
        serde_json::Value::Object(o) => {
            let mut replace = serde_json::Map::with_capacity(o.len());
            o.retain(|k, v| {
                rename_keys(v);
                replace.insert(
                    heck::ToLowerCamelCase::to_lower_camel_case(k.as_str()),
                    std::mem::replace(v, serde_json::Value::Null),
                );
                true
            });
            *o = replace;
        }
        _ => (),
    }
}
