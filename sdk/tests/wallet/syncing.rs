// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::output::{
        unlock_condition::{
            AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
            StateControllerAddressUnlockCondition, StorageDepositReturnUnlockCondition,
        },
        AccountId, AccountOutputBuilder, BasicOutputBuilder, NftId, NftOutputBuilder, UnlockCondition,
    },
    wallet::{account::SyncOptions, Result},
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[tokio::test]
#[cfg(feature = "rocksdb")]
async fn updated_default_sync_options() -> Result<()> {
    let storage_path = "test-storage/updated_default_sync_options";
    setup(storage_path)?;

    let default_sync = SyncOptions::default();

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = wallet.create_account().finish().await?;

    assert_eq!(default_sync, account.default_sync_options().await);

    let custom_options = SyncOptions {
        address_start_index: 10,
        ..Default::default()
    };
    account.set_default_sync_options(custom_options.clone()).await?;
    assert_eq!(custom_options, account.default_sync_options().await);

    drop(account);
    drop(wallet);

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = wallet.get_account(0).await?;

    assert_eq!(custom_options, account.default_sync_options().await);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn sync_only_most_basic_outputs() -> Result<()> {
    let storage_path = "test-storage/sync_only_most_basic_outputs";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    let account_1_address = account_1.first_address_bech32().await;

    let token_supply = account_0.client().get_token_supply().await?;
    // Only one basic output without further unlock conditions
    let outputs = [
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_unlock_conditions([AddressUnlockCondition::new(account_1_address.clone())])
            .finish_output(token_supply)?,
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(account_1_address.clone())),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    account_1_address.clone(),
                    // Already expired
                    account_0.client().get_slot_index().await? - 5000,
                )?),
            ])
            .finish_output(token_supply)?,
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(account_1_address.clone())),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    account_1_address.clone(),
                    // Not expired
                    account_0.client().get_slot_index().await? + 5000,
                )?),
            ])
            .finish_output(token_supply)?,
        BasicOutputBuilder::new_with_amount(1_000_000)
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(account_1_address.clone())),
                UnlockCondition::StorageDepositReturn(StorageDepositReturnUnlockCondition::new(
                    account_1_address.clone(),
                    1_000_000,
                    token_supply,
                )?),
            ])
            .finish_output(token_supply)?,
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .with_unlock_conditions([AddressUnlockCondition::new(account_1_address.clone())])
            .finish_output(token_supply)?,
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(account_1_address.clone())),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    account_1_address.clone(),
                    account_0.client().get_slot_index().await? + 5000,
                )?),
            ])
            .finish_output(token_supply)?,
        AccountOutputBuilder::new_with_amount(1_000_000, AccountId::null())
            .with_unlock_conditions([
                UnlockCondition::StateControllerAddress(StateControllerAddressUnlockCondition::new(
                    account_1_address.clone(),
                )),
                UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(account_1_address.clone())),
            ])
            .finish_output(token_supply)?,
    ];

    let tx = account_0.send_outputs(outputs, None).await?;
    account_0
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    // Sync with sync_only_most_basic_outputs: true, only the first output should be synced
    let balance = account_1
        .sync(Some(SyncOptions {
            sync_only_most_basic_outputs: true,
            ..Default::default()
        }))
        .await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.nfts().len(), 0);
    assert_eq!(balance.accounts().len(), 0);
    let unspent_outputs = account_1.unspent_outputs(None).await?;
    assert_eq!(unspent_outputs.len(), 1);
    unspent_outputs.into_iter().for_each(|output_data| {
        assert!(output_data.output.is_basic());
        assert_eq!(output_data.output.unlock_conditions().unwrap().len(), 1);
        assert_eq!(
            output_data
                .output
                .unlock_conditions()
                .unwrap()
                .address()
                .unwrap()
                .address(),
            account_1_address.as_ref()
        );
    });

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn sync_incoming_transactions() -> Result<()> {
    let storage_path = "test-storage/sync_incoming_transactions";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    let account_1_address = account_1.first_address_bech32().await;

    let token_supply = account_0.client().get_token_supply().await?;

    let outputs = [
        BasicOutputBuilder::new_with_amount(750_000)
            .with_unlock_conditions([AddressUnlockCondition::new(account_1_address.clone())])
            .finish_output(token_supply)?,
        BasicOutputBuilder::new_with_amount(250_000)
            .with_unlock_conditions([AddressUnlockCondition::new(account_1_address)])
            .finish_output(token_supply)?,
    ];

    let tx = account_0.send_outputs(outputs, None).await?;
    account_0
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    account_1
        .sync(Some(SyncOptions {
            sync_incoming_transactions: true,
            ..Default::default()
        }))
        .await?;
    let incoming_transactions = account_1.incoming_transactions().await;
    assert_eq!(incoming_transactions.len(), 1);
    let incoming_tx = account_1.get_incoming_transaction(&tx.transaction_id).await.unwrap();
    assert_eq!(incoming_tx.inputs.len(), 1);
    let essence = incoming_tx.payload.essence();

    // 2 created outputs plus remainder
    assert_eq!(essence.outputs().len(), 3);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
#[cfg(feature = "storage")]
async fn background_syncing() -> Result<()> {
    let storage_path = "test-storage/background_syncing";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    wallet.start_background_syncing(None, None).await?;

    let account = wallet.create_account().finish().await?;

    iota_sdk::client::request_funds_from_faucet(
        crate::wallet::common::FAUCET_URL,
        &account.first_address_bech32().await,
    )
    .await?;

    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let balance = account.balance().await?;
        if balance.base_coin().available() > 0 {
            break;
        }
    }

    // Balance should be != 0 without calling account.sync()
    let balance = account.balance().await?;
    if balance.base_coin().available() == 0 {
        panic!("Faucet no longer wants to hand over coins or background syncing failed");
    }

    wallet.stop_background_syncing().await?;

    tear_down(storage_path)
}
