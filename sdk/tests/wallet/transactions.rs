// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::{account::TransactionOptions, MintNftParams, Result, SendNftParams, SendParams};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn send_amount() -> Result<()> {
    let storage_path = "test-storage/send_amount";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    let amount = 1_000_000;
    let tx = account_0
        .send_with_params(
            [SendParams::new(amount, *account_1.addresses().await?[0].address())?],
            None,
        )
        .await?;

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin().available(), amount);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn send_amount_127_outputs() -> Result<()> {
    let storage_path = "test-storage/send_amount_127_outputs";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    let amount = 1_000_000;
    let tx = account_0
        .send_with_params(
            vec![
                SendParams::new(
                    amount,
                    *account_1.addresses().await?[0].address(),
                )?;
                // Only 127, because we need one remainder
                127
            ],
            None,
        )
        .await?;

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin().available(), 127 * amount);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn send_amount_custom_input() -> Result<()> {
    let storage_path = "test-storage/send_amount_custom_input";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    // Send 10 outputs to account_1
    let amount = 1_000_000;
    let tx = account_0
        .send_with_params(
            vec![SendParams::new(amount, *account_1.addresses().await?[0].address())?; 10],
            None,
        )
        .await?;

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin().available(), 10 * amount);

    // Send back with custom provided input
    let custom_input = &account_1.unspent_outputs(None).await?[5];
    let tx = account_1
        .send_with_params(
            [SendParams::new(amount, *account_0.addresses().await?[0].address())?],
            Some(TransactionOptions {
                custom_inputs: Some(vec![custom_input.output_id]),
                ..Default::default()
            }),
        )
        .await?;

    assert_eq!(tx.inputs.len(), 1);
    assert_eq!(tx.inputs.first().unwrap().metadata.output_id()?, custom_input.output_id);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn send_nft() -> Result<()> {
    let storage_path = "test-storage/send_nft";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let accounts = &create_accounts_with_funds(&wallet, 2).await?;

    let nft_options = [MintNftParams::new()
        .with_address(*accounts[0].addresses().await?[0].address())
        .with_metadata(b"some nft metadata".to_vec())
        .with_immutable_metadata(b"some immutable nft metadata".to_vec())];

    let transaction = accounts[0].mint_nfts(nft_options, None).await.unwrap();
    accounts[0]
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let nft_id = *accounts[0].sync(None).await?.nfts().first().unwrap();

    // Send to account 1
    let transaction = accounts[0]
        .send_nft(
            [SendNftParams::new(
                *accounts[1].addresses().await?[0].address(),
                nft_id,
            )?],
            None,
        )
        .await
        .unwrap();
    accounts[0]
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let balance = accounts[1].sync(None).await?;
    assert_eq!(balance.nfts().len(), 1);
    assert_eq!(*balance.nfts().first().unwrap(), nft_id);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn send_with_note() -> Result<()> {
    let storage_path = "test-storage/send_with_note";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    let amount = 1_000_000;
    let tx = account_0
        .send_with_params(
            [SendParams::new(amount, *account_1.addresses().await?[0].address())?],
            Some(TransactionOptions {
                note: Some(String::from("send_with_note")),
                ..Default::default()
            }),
        )
        .await?;

    assert_eq!(tx.note, Some(String::from("send_with_note")));

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn conflicting_transaction() -> Result<()> {
    let storage_path_0 = "test-storage/conflicting_transaction_0";
    let storage_path_1 = "test-storage/conflicting_transaction_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let mnemonic = iota_sdk::client::utils::generate_mnemonic()?;
    // Create two wallets with the same mnemonic
    let wallet_0 = make_wallet(storage_path_0, Some(mnemonic.clone()), None).await?;
    let wallet_0_account = &create_accounts_with_funds(&wallet_0, 1).await?[0];
    let wallet_1 = make_wallet(storage_path_1, Some(mnemonic), None).await?;
    let wallet_1_account = wallet_1.create_account().finish().await?;

    // Balance should be equal
    assert_eq!(wallet_0_account.sync(None).await?, wallet_1_account.sync(None).await?);

    // Send transaction with each account and without syncing again
    let tx = wallet_0_account
        .send_with_params(
            [SendParams::new(
                1_000_000,
                *wallet_0_account.addresses().await?[0].address(),
            )?],
            None,
        )
        .await?;
    wallet_0_account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    // Second transaction will be conflicting
    let tx = wallet_1_account
        .send_with_params(
            [SendParams::new(
                // Something in the transaction must be different than in the first one, otherwise it will be the same
                // one
                2_000_000,
                *wallet_0_account.addresses().await?[0].address(),
            )?],
            None,
        )
        .await?;
    // Should return an error since the tx is conflicting
    match wallet_1_account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await
        .unwrap_err()
    {
        iota_sdk::wallet::Error::Client(client_error) => {
            let iota_sdk::client::Error::TangleInclusion(_) = *client_error else {
                panic!("Expected TangleInclusion error");
            };
        }
        _ => panic!("Expected TangleInclusion error"),
    }

    // After syncing the balance is still equal
    assert_eq!(wallet_0_account.sync(None).await?, wallet_1_account.sync(None).await?);

    let conflicting_tx = wallet_1_account.get_transaction(&tx.transaction_id).await.unwrap();
    assert_eq!(
        conflicting_tx.inclusion_state,
        iota_sdk::wallet::account::types::InclusionState::Conflicting
    );
    // The conflicting tx is also removed from the pending txs
    assert!(wallet_1_account.pending_transactions().await.is_empty());

    tear_down(storage_path_0).ok();
    tear_down(storage_path_1)
}
