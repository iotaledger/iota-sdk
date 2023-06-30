// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    wallet::{account::SyncOptions, CreateNativeTokenParams, Result},
    U256,
};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn create_and_mint_native_token() -> Result<()> {
    let storage_path = "test-storage/create_and_mint_native_token";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account = &create_accounts_with_funds(&wallet, 1).await?[0];

    let tx = account.create_alias_output(None, None).await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    account.sync(None).await?;

    let create_tx = account
        .create_native_token(
            CreateNativeTokenParams {
                account_id: None,
                circulating_supply: U256::from(50),
                maximum_supply: U256::from(100),
                foundry_metadata: None,
            },
            None,
        )
        .await?;
    account
        .retry_transaction_until_included(&create_tx.transaction.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;
    assert_eq!(balance.native_tokens().len(), 1);
    assert_eq!(
        balance
            .native_tokens()
            .iter()
            .find(|t| t.token_id() == &create_tx.token_id)
            .unwrap()
            .available(),
        U256::from(50)
    );

    let tx = account
        .mint_native_token(create_tx.token_id, U256::from(50), None)
        .await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;
    assert_eq!(balance.native_tokens().len(), 1);
    assert_eq!(
        balance
            .native_tokens()
            .iter()
            .find(|t| t.token_id() == &create_tx.token_id)
            .unwrap()
            .available(),
        U256::from(100)
    );

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn native_token_foundry_metadata() -> Result<()> {
    let storage_path = "test-storage/native_token_foundry_metadata";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account = &create_accounts_with_funds(&wallet, 1).await?[0];

    let tx = account.create_alias_output(None, None).await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    account.sync(None).await?;

    let foundry_metadata = [1, 3, 3, 7];

    let create_tx = account
        .create_native_token(
            CreateNativeTokenParams {
                account_id: None,
                circulating_supply: U256::from(50),
                maximum_supply: U256::from(100),
                foundry_metadata: Some(foundry_metadata.to_vec()),
            },
            None,
        )
        .await?;
    account
        .retry_transaction_until_included(&create_tx.transaction.transaction_id, None, None)
        .await?;
    // Sync native_token_foundries to get the metadata
    let balance = account
        .sync(Some(SyncOptions {
            sync_native_token_foundries: true,
            ..Default::default()
        }))
        .await?;
    assert_eq!(balance.native_tokens().len(), 1);
    // Metadata should exist and be the same
    assert_eq!(
        balance
            .native_tokens()
            .iter()
            .find(|t| t.token_id() == &create_tx.token_id)
            .unwrap()
            .metadata()
            .as_ref()
            .unwrap()
            .data(),
        &foundry_metadata
    );

    tear_down(storage_path)
}
