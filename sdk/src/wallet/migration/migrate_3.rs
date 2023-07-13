// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub struct Migrate;

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 3;
    const SDK_VERSION: &'static str = "0.4.0";
    const DATE: time::Date = time::macros::date!(2023 - 07 - 13);
}

#[async_trait]
#[cfg(feature = "storage")]
impl Migration<crate::wallet::storage::Storage> for Migrate {
    async fn migrate(storage: &crate::wallet::storage::Storage) -> Result<()> {
        use crate::wallet::storage::constants::WALLET_INDEXATION_KEY;

        if let Some(mut wallet) = storage.get::<serde_json::Value>(WALLET_INDEXATION_KEY).await? {
            let params = wallet["client_options"]["protocolParameters"].as_object_mut().unwrap();
            if let Some(version) = params.remove("protocol_version") {
                params.insert("version".to_owned(), version);
            }
            rename_keys(&mut wallet);
            ConvertNetworkName::check(&mut wallet["clientOptions"]["protocolParameters"]["networkName"])?;
            ConvertTokenSupply::check(&mut wallet["clientOptions"]["protocolParameters"]["tokenSupply"])?;

            storage.set(WALLET_INDEXATION_KEY, &wallet).await?;
        }
        Ok(())
    }
}

#[async_trait]
#[cfg(feature = "stronghold")]
impl Migration<crate::client::stronghold::StrongholdAdapter> for Migrate {
    async fn migrate(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
        use crate::{
            client::storage::StorageAdapter,
            wallet::core::operations::stronghold_backup::stronghold_snapshot::CLIENT_OPTIONS_KEY,
        };

        if let Some(mut client_options) = storage.get::<serde_json::Value>(CLIENT_OPTIONS_KEY).await? {
            let params = client_options["protocolParameters"].as_object_mut().unwrap();
            if let Some(version) = params.remove("protocol_version") {
                params.insert("version".to_owned(), version);
            }
            rename_keys(&mut client_options);
            ConvertNetworkName::check(&mut client_options["protocolParameters"]["networkName"])?;
            ConvertTokenSupply::check(&mut client_options["protocolParameters"]["tokenSupply"])?;

            storage.set(CLIENT_OPTIONS_KEY, &client_options).await?;
        }
        Ok(())
    }
}

struct ConvertNetworkName;
impl Convert for ConvertNetworkName {
    type New = String;
    type Old = super::migrate_1::types::StringPrefix<u8>;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(old.inner)
    }
}

struct ConvertTokenSupply;
impl Convert for ConvertTokenSupply {
    type New = String;
    type Old = u64;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(old.to_string())
    }
}
