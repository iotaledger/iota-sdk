// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::wallet::Error;

pub struct Migrate;

fn migrate_native_token(output: &mut serde_json::Value) {
    let native_tokens = output["native_tokens"]["inner"].as_array_mut().unwrap();

    for native_token in native_tokens.iter_mut() {
        if let Some(id) = native_token.get("token_id") {
            *native_token = serde_json::json!({ "amount": native_token["amount"], "id": id});
        }
    }
}

fn migrate_account(account: &mut serde_json::Value) -> Result<()> {
    for output_data in account["outputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
        .values_mut()
    {
        if let Some(chain) = output_data.get_mut("chain").and_then(|c| c.as_array_mut()) {
            for segment in chain {
                ConvertSegment::check(segment)?;
            }
        }

        migrate_native_token(&mut output_data["output"]["data"]);
    }

    for output_data in account["unspentOutputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
        .values_mut()
    {
        if let Some(chain) = output_data.get_mut("chain").and_then(|c| c.as_array_mut()) {
            for segment in chain {
                ConvertSegment::check(segment)?;
            }
        }

        migrate_native_token(&mut output_data["output"]["data"]);
    }

    for (_key, transaction) in account["transactions"].as_object_mut().unwrap() {
        let outputs = transaction["payload"]["essence"]["data"]["outputs"]["inner"]
            .as_array_mut()
            .unwrap();
        for output in outputs {
            migrate_native_token(&mut output["data"]);
        }
    }

    for (_key, transaction) in account["incomingTransactions"].as_object_mut().unwrap() {
        let outputs = transaction["payload"]["essence"]["data"]["outputs"]["inner"]
            .as_array_mut()
            .unwrap();
        for output in outputs {
            migrate_native_token(&mut output["data"]);
        }
    }

    if let Some(native_token_foundries) = account.get_mut("nativeTokenFoundries") {
        for (_key, foundry) in native_token_foundries.as_object_mut().unwrap() {
            migrate_native_token(foundry);
        }
    }

    Ok(())
}

fn migrate_client_options(client_options: &mut serde_json::Value) -> Result<()> {
    let protocol_parameters = &mut client_options["protocolParameters"];

    ConvertHrp::check(&mut protocol_parameters["bech32_hrp"])?;

    rename_keys(&mut protocol_parameters["rent_structure"]);

    Ok(())
}

fn migrate_storage_options(storage_options: &mut serde_json::Value) -> Result<()> {
    if !storage_options.is_null() {
        *storage_options = serde_json::json!({
            "path": storage_options["storage_path"],
            "encryptionKey": storage_options["storage_encryption_key"],
            "kind": storage_options["manager_store"]
        });
    }

    Ok(())
}

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 1;
    const SDK_VERSION: &'static str = "0.5.0";
    const DATE: time::Date = time::macros::date!(2023 - 07 - 12);
}

#[async_trait]
#[cfg(feature = "storage")]
impl Migration<crate::wallet::storage::Storage> for Migrate {
    async fn migrate(storage: &crate::wallet::storage::Storage) -> Result<()> {
        use crate::wallet::storage::constants::{
            ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY, WALLET_INDEXATION_KEY,
        };

        if let Some(account_indexes) = storage.get::<Vec<u32>>(ACCOUNTS_INDEXATION_KEY).await? {
            for account_index in account_indexes {
                if let Some(mut account) = storage
                    .get::<serde_json::Value>(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"))
                    .await?
                {
                    migrate_account(&mut account)?;

                    storage
                        .set(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"), &account)
                        .await?;
                }
            }
        }

        if let Some(mut wallet) = storage.get::<serde_json::Value>(WALLET_INDEXATION_KEY).await? {
            migrate_client_options(&mut wallet["client_options"])?;
            migrate_storage_options(&mut wallet["storage_options"])?;

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
            wallet::core::operations::stronghold_backup::stronghold_snapshot::{ACCOUNTS_KEY, CLIENT_OPTIONS_KEY},
        };

        if let Some(mut accounts) = storage.get::<Vec<serde_json::Value>>(ACCOUNTS_KEY).await? {
            for account in &mut accounts {
                migrate_account(account)?;
            }
            storage.set(ACCOUNTS_KEY, &accounts).await?;
        }
        if let Some(mut client_options) = storage.get::<serde_json::Value>(CLIENT_OPTIONS_KEY).await? {
            migrate_client_options(&mut client_options)?;

            storage.set(CLIENT_OPTIONS_KEY, &client_options).await?;
        }
        storage.delete("backup_schema_version").await.ok();
        Ok(())
    }
}

mod types {
    use core::{marker::PhantomData, str::FromStr};

    use serde::{Deserialize, Serialize};

    use crate::types::block::Error;

    macro_rules! string_serde_impl {
        ($type:ty) => {
            impl serde::Serialize for $type {
                fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                    use alloc::string::ToString;

                    s.serialize_str(&self.to_string())
                }
            }

            impl<'de> serde::Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> Result<$type, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct StringVisitor;

                    impl<'de> serde::de::Visitor<'de> for StringVisitor {
                        type Value = $type;

                        fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            formatter.write_str("a string representing the value")
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            let value = core::str::FromStr::from_str(v).map_err(serde::de::Error::custom)?;
                            Ok(value)
                        }
                    }

                    deserializer.deserialize_str(StringVisitor)
                }
            }
        };
    }

    #[derive(Deserialize)]
    #[allow(non_camel_case_types)]
    pub struct Crypto_0_18_0_Segment {
        pub bs: [u8; 4],
        pub hardened: bool,
    }

    pub struct Hrp {
        inner: [u8; 83],
        len: u8,
    }

    impl Hrp {
        /// Convert a string to an Hrp without checking validity.
        pub const fn from_str_unchecked(hrp: &str) -> Self {
            let len = hrp.len();
            let mut bytes = [0; 83];
            let hrp = hrp.as_bytes();
            let mut i = 0;
            while i < len {
                bytes[i] = hrp[i];
                i += 1;
            }
            Self {
                inner: bytes,
                len: len as _,
            }
        }
    }

    impl FromStr for Hrp {
        type Err = Error;

        fn from_str(hrp: &str) -> Result<Self, Self::Err> {
            let len = hrp.len();
            if hrp.is_ascii() && len <= 83 {
                let mut bytes = [0; 83];
                bytes[..len].copy_from_slice(hrp.as_bytes());
                Ok(Self {
                    inner: bytes,
                    len: len as _,
                })
            } else {
                Err(Error::InvalidBech32Hrp(hrp.to_string()))
            }
        }
    }

    impl core::fmt::Display for Hrp {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let hrp_str = self.inner[..self.len as usize]
                .iter()
                .map(|b| *b as char)
                .collect::<String>();
            f.write_str(&hrp_str)
        }
    }

    string_serde_impl!(Hrp);

    #[derive(Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct StringPrefix<B> {
        pub inner: String,
        bounded: PhantomData<B>,
    }
}

struct ConvertSegment;
impl Convert for ConvertSegment {
    type New = u32;
    type Old = types::Crypto_0_18_0_Segment;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(u32::from_be_bytes(old.bs))
    }
}

struct ConvertHrp;
impl Convert for ConvertHrp {
    type New = types::Hrp;
    type Old = types::StringPrefix<u8>;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New::from_str_unchecked(&old.inner))
    }
}
