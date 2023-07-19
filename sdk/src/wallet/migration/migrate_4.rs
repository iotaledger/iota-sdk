// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub struct Migrate;

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 4;
    const SDK_VERSION: &'static str = "1.0.0-rc.0";
    const DATE: time::Date = time::macros::date!(2023 - 07 - 19);
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
            migrate_wallet(&mut wallet)?;

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
        Ok(())
    }
}

fn migrate_wallet(wallet: &mut serde_json::Value) -> Result<()> {
    migrate_client_options(&mut wallet["clientOptions"])?;
    if let Some(storage_opts) = wallet.get_mut("storageOptions") {
        let storage_opts = storage_opts
            .as_object_mut()
            .ok_or(Error::Storage("malformatted storage options".to_owned()))?;
        check_omitted_opt("encryptionKey", storage_opts)
    }
    Ok(())
}

fn migrate_client_options(client_options: &mut serde_json::Value) -> Result<()> {
    let client_options = client_options
        .as_object_mut()
        .ok_or(Error::Storage("malformatted client options".to_owned()))?;
    check_omitted_opt("powWorkerCount", client_options);
    check_omitted_opt("latestMilestoneTimestamp", client_options);
    check_omitted_opt("primaryNode", client_options);
    check_omitted_opt("primaryPowNode", client_options);
    check_omitted_list("nodes", client_options);
    check_omitted_list("permanodes", client_options);
    if let Some(nodes) = client_options.get_mut("nodes") {
        migrate_nodes(nodes)?;
    }
    if let Some(nodes) = client_options.get_mut("permanodes") {
        migrate_nodes(nodes)?;
    }
    Ok(())
}

fn migrate_nodes(nodes: &mut serde_json::Value) -> Result<()> {
    for node in nodes
        .as_array_mut()
        .ok_or(Error::Storage("malformatted nodes".to_owned()))?
    {
        let node = node
            .as_object_mut()
            .ok_or(Error::Storage("malformatted node".to_owned()))?;
        check_omitted_opt("auth", node);
    }
    Ok(())
}

fn migrate_account(account: &mut serde_json::Value) -> Result<()> {
    for output_data in account["outputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
        .values_mut()
    {
        migrate_output(&mut output_data["output"])?;
    }

    for output_data in account["unspentOutputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
        .values_mut()
    {
        migrate_output(&mut output_data["output"])?;
    }

    for transaction in account["transactions"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted transactions".to_owned()))?
        .values_mut()
    {
        migrate_transaction(transaction)?;
    }

    for transaction in account["incomingTransactions"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted incoming transactions".to_owned()))?
        .values_mut()
    {
        migrate_transaction(transaction)?;
    }

    if let Some(foundries) = account.get_mut("nativeTokenFoundries") {
        for foundry_output in foundries
            .as_object_mut()
            .ok_or(Error::Storage("malformatted foundry outputs".to_owned()))?
            .values_mut()
        {
            migrate_output(foundry_output)?;
        }
    }

    Ok(())
}

fn migrate_output(output: &mut serde_json::Value) -> Result<()> {
    let output = output
        .as_object_mut()
        .ok_or(Error::Storage("malformatted output".to_owned()))?;
    check_omitted_str("stateMetadata", output, "0x");
    check_omitted_list("features", output);
    check_omitted_list("immutableFeatures", output);
    if let Some(features) = output.get_mut("features") {
        for feature in features
            .as_array_mut()
            .ok_or(Error::Storage("malformatted features".to_owned()))?
        {
            migrate_feature(feature)?;
        }
    }
    if let Some(features) = output.get_mut("immutableFeatures") {
        for feature in features
            .as_array_mut()
            .ok_or(Error::Storage("malformatted immutable features".to_owned()))?
        {
            migrate_feature(feature)?;
        }
    }
    Ok(())
}

fn migrate_transaction(transaction: &mut serde_json::Value) -> Result<()> {
    let transaction = transaction
        .as_object_mut()
        .ok_or(Error::Storage("malformatted transaction".to_owned()))?;
    check_omitted_str("note", transaction, "");
    check_omitted_str("blockId", transaction, "");

    for output in transaction["payload"]["essence"]["outputs"]
        .as_array_mut()
        .ok_or(Error::Storage("malformatted transaction outputs".to_owned()))?
    {
        migrate_output(output)?;
    }

    for input in transaction["inputs"]
        .as_array_mut()
        .ok_or(Error::Storage("malformatted transaction inputs".to_owned()))?
    {
        migrate_output(&mut input["output"])?;
    }

    if let Some(payload) = transaction["payload"]["essence"].get_mut("payload") {
        migrate_payload(payload)?;
    }

    Ok(())
}

fn migrate_payload(payload: &mut serde_json::Value) -> Result<()> {
    let payload = payload
        .as_object_mut()
        .ok_or(Error::Storage("malformatted payload".to_owned()))?;
    check_omitted_str("data", payload, "0x");
    check_omitted_str("tag", payload, "0x");
    Ok(())
}

fn migrate_feature(feature: &mut serde_json::Value) -> Result<()> {
    let feature = feature
        .as_object_mut()
        .ok_or(Error::Storage("malformatted feature".to_owned()))?;
    check_omitted_str("data", feature, "0x");
    check_omitted_str("tag", feature, "0x");
    Ok(())
}

fn check_omitted_str(field: &str, value: &mut serde_json::Map<String, serde_json::Value>, empty_val: &str) {
    if let Some(f) = value.get(field) {
        if f.is_null() || matches!(f.as_str(), Some(v) if v == empty_val) {
            value.remove(field);
        }
    }
}

fn check_omitted_list(field: &str, value: &mut serde_json::Map<String, serde_json::Value>) {
    if let Some(f) = value.get(field) {
        if f.is_null() || matches!(f.as_array().map(Vec::as_slice), Some(&[])) {
            value.remove(field);
        }
    }
}

fn check_omitted_opt(field: &str, value: &mut serde_json::Map<String, serde_json::Value>) {
    if matches!(value.get(field), Some(f) if f.is_null()) {
        value.remove(field);
    }
}
