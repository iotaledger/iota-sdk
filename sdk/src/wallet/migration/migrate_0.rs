// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::prefix::StringPrefix;

use super::*;
use crate::{
    types::block::address::Hrp,
    wallet::{storage::constants::WALLET_INDEXATION_KEY, Error, WalletBuilder},
};

pub struct Migrate;

#[async_trait]
impl Migration for Migrate {
    const ID: usize = 0;
    const WALLET_VERSION: &'static str = "0.3.0";
    const DATE: time::Date = time::macros::date!(2023 - 05 - 09);

    async fn migrate(storage: &StorageManager) -> Result<()> {
        if let Some(mut wallet) = storage.get::<serde_json::Value>(WALLET_INDEXATION_KEY).await? {
            let hrp_value = wallet
                .get_mut("clientOptions")
                .ok_or(Error::Storage("missing client options".to_owned()))?
                .get_mut("protocolParameters")
                .ok_or(Error::Storage("missing protocol params".to_owned()))?
                .get_mut("bech32Hrp")
                .ok_or(Error::Storage("missing bech32 hrp".to_owned()))?;
            let old_hrp = serde_json::from_value::<StringPrefix<u8>>(hrp_value.clone())?;
            let new_hrp = Hrp::from_str_unchecked(old_hrp.as_str());
            *hrp_value = serde_json::to_value(new_hrp)?;
            let wallet_builder = serde_json::from_value::<WalletBuilder>(hrp_value.clone())?;

            storage.save_wallet_data(&wallet_builder).await?;
        }
        Ok(())
    }
}
