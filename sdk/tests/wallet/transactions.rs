// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::{account::TransactionOptions, AddressAndNftId, AddressWithAmount, NftOptions, Result};

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
        .send_amount(
            vec![AddressWithAmount::new(
                account_1.addresses().await?[0].address().to_bech32(),
                amount,
            )],
            None,
        )
        .await?;

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin.available, amount);

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
        .send_amount(
            vec![
                AddressWithAmount::new(
                    account_1.addresses().await?[0].address().to_bech32(),
                    amount,
                );
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
    assert_eq!(balance.base_coin.available, 127 * amount);

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
        .send_amount(
            vec![AddressWithAmount::new(account_1.addresses().await?[0].address().to_bech32(), amount); 10],
            None,
        )
        .await?;

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin.available, 10 * amount);

    // Send back with custom provided input
    let custom_input = &account_1.unspent_outputs(None).await?[5];
    let tx = account_1
        .send_amount(
            vec![AddressWithAmount::new(
                account_0.addresses().await?[0].address().to_bech32(),
                amount,
            )],
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

    let nft_options = vec![NftOptions {
        address: Some(accounts[0].addresses().await?[0].address().to_bech32()),
        sender: None,
        metadata: Some(b"some nft metadata".to_vec()),
        tag: None,
        issuer: None,
        immutable_metadata: Some(b"some immutable nft metadata".to_vec()),
    }];

    let transaction = accounts[0].mint_nfts(nft_options, None).await.unwrap();
    accounts[0]
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let nft_id = *accounts[0].sync(None).await?.nfts.first().unwrap();

    // Send to account 1
    let transaction = accounts[0]
        .send_nft(
            vec![AddressAndNftId {
                address: accounts[1].addresses().await?[0].address().to_bech32(),
                nft_id,
            }],
            None,
        )
        .await
        .unwrap();
    accounts[0]
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let balance = accounts[1].sync(None).await?;
    assert_eq!(balance.nfts.len(), 1);
    assert_eq!(*balance.nfts.first().unwrap(), nft_id);

    tear_down(storage_path)
}
