// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    wallet::{CreateNativeTokenParams, Result, SyncOptions},
    U256,
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn create_and_mint_native_token() -> Result<()> {
    let storage_path = "test-storage/create_and_mint_native_token";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    request_funds(&wallet).await?;

    let tx = wallet.create_account_output(None, None).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let create_tx = wallet
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
    wallet
        .reissue_transaction_until_included(&create_tx.transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;
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

    let tx = wallet.mint_native_token(create_tx.token_id, 50, None).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;
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

    request_funds(&wallet).await?;

    let tx = wallet.create_account_output(None, None).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let foundry_metadata = [1, 3, 3, 7];

    let create_tx = wallet
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
    wallet
        .reissue_transaction_until_included(&create_tx.transaction.transaction_id, None, None)
        .await?;
    // Sync native_token_foundries to get the metadata
    let balance = wallet
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
            .data()
            .get(&packable::prefix::BoxedSlicePrefix::try_from(b"foundry".to_vec().into_boxed_slice()).unwrap())
            .unwrap()
            .to_vec(),
        foundry_metadata.to_vec()
    );

    tear_down(storage_path)
}
