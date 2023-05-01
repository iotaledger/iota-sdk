// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Tests for recovering accounts from mnemonic without a backup

use std::time::Duration;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        Client,
    },
    wallet::Result,
};

use crate::wallet::common::{make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn account_recovery_empty() -> Result<()> {
    let storage_path = "test-storage/account_recovery_empty";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let accounts = wallet.recover_accounts(0, 2, 2, None).await?;

    // accounts should be empty if no account was created before and no account was found with balance
    assert_eq!(0, accounts.len());
    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn account_recovery_existing_accounts() -> Result<()> {
    let storage_path = "test-storage/account_recovery_existing_accounts";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    // create two accounts
    wallet.create_account().finish().await?;
    wallet.create_account().finish().await?;

    let accounts = wallet.recover_accounts(0, 2, 2, None).await?;

    // accounts should still be ordered
    for (index, account) in accounts.iter().enumerate() {
        assert_eq!(&(index as u32), account.read().await.index());
    }
    // accounts should be 2 because we created 2 accounts before and no new account was found with balance
    assert_eq!(2, accounts.len());
    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn account_recovery_with_balance_and_empty_addresses() -> Result<()> {
    let storage_path = "test-storage/account_recovery_with_balance_and_empty_addresses";
    setup(storage_path)?;

    let mnemonic = Client::generate_mnemonic()?;
    let client = Client::builder()
        .with_node(crate::wallet::common::NODE_LOCAL)?
        .finish()?;

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&mnemonic)?);

    let address = &client
        .get_addresses(&secret_manager)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_bech32_hrp(client.get_bech32_hrp().await?)
        .with_account_index(2)
        .with_range(2..3)
        .finish()
        .await?[0];

    // Add funds to the address with account index 2 and address key_index 2, so recover works
    iota_sdk::client::request_funds_from_faucet(crate::wallet::common::FAUCET_URL, address).await?;

    // Wait for faucet transaction
    tokio::time::sleep(Duration::new(10, 0)).await;

    let wallet = make_wallet(storage_path, Some(&mnemonic), None).await?;

    let accounts = wallet.recover_accounts(0, 3, 2, None).await?;

    // accounts should still be ordered
    for (index, account) in accounts.iter().enumerate() {
        assert_eq!(&(index as u32), account.read().await.index());
    }
    // accounts should be 3 because account with index 2 has balance
    assert_eq!(3, accounts.len());

    let account_with_balance = accounts[2].read().await;
    // should have 3 addresses
    assert_eq!(3, account_with_balance.public_addresses().len());
    tear_down(storage_path)
}
