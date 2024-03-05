// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub(crate) struct Migrate;

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 0;
    const SDK_VERSION: &'static str = "1.X.X";
    const DATE: time::Date = time::macros::date!(2023 - 06 - 14);
}

#[async_trait]
#[cfg(feature = "storage")]
impl Migration<crate::wallet::storage::Storage> for Migrate {
    async fn migrate(_storage: &crate::wallet::storage::Storage) -> Result<(), WalletError> {
        Ok(())
    }
}

#[async_trait]
#[cfg(feature = "stronghold")]
impl Migration<crate::client::stronghold::StrongholdAdapter> for Migrate {
    async fn migrate(_storage: &crate::client::stronghold::StrongholdAdapter) -> Result<(), WalletError> {
        Ok(())
    }
}
