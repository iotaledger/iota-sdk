// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::input_selection::Burn,
    types::block::output::{
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        NativeToken, NftId, NftOutputBuilder, OutputId, UnlockCondition,
    },
    wallet::{CreateNativeTokenParams, MintNftParams, Result, Wallet},
    U256,
};

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn mint_and_burn_nft() -> Result<()> {
    let storage_path = "test-storage/mint_and_burn_outputs";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;

    let nft_options = [MintNftParams::new()
        .with_address(wallet.address_as_bech32().await)
        .with_metadata(b"some nft metadata".to_vec())
        .with_immutable_metadata(b"some immutable nft metadata".to_vec())];

    let transaction = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let balance = wallet.sync(None).await.unwrap();

    let output_id = OutputId::new(transaction.transaction_id, 0u16).unwrap();
    let nft_id = NftId::from(&output_id);

    let search = balance.nfts().iter().find(|&balance_nft_id| *balance_nft_id == nft_id);
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_some());

    let transaction = wallet.burn(nft_id, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    let search = balance.nfts().iter().find(|&balance_nft_id| *balance_nft_id == nft_id);
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_none());

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn mint_and_burn_expired_nft() -> Result<()> {
    let storage_path = "test-storage/mint_and_burn_expired_nft";
    setup(storage_path)?;

    let wallet_0 = make_wallet(storage_path, None, None, None).await?;
    let wallet_1 = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet_0).await?;

    let token_supply = wallet_0.client().get_token_supply().await?;

    let amount = 1_000_000;
    let outputs = [NftOutputBuilder::new_with_amount(amount, NftId::null())
        .with_unlock_conditions([
            UnlockCondition::Address(AddressUnlockCondition::new(wallet_0.address().await)),
            // immediately expired to account_1
            UnlockCondition::Expiration(ExpirationUnlockCondition::new(wallet_1.address().await, 1)?),
        ])
        .finish_output(token_supply)?];

    let transaction = wallet_0.send_outputs(outputs, None).await?;
    wallet_0
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let output_id = OutputId::new(transaction.transaction_id, 0u16)?;
    let nft_id = NftId::from(&output_id);

    wallet_1.sync(None).await?;
    let transaction = wallet_1.burn(nft_id, None).await?;
    wallet_1
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet_1.sync(None).await?;
    // After burning the amount is available on account_1
    assert_eq!(balance.base_coin().available(), amount);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn create_and_melt_native_token() -> Result<()> {
    let storage_path = "test-storage/create_and_melt_native_token";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;

    // First create an account output, this needs to be done only once, because an account can have many foundry outputs
    let transaction = wallet.create_account_output(None, None).await?;

    // Wait for transaction to get included
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let circulating_supply = U256::from(60i32);
    let params = CreateNativeTokenParams {
        account_id: None,
        circulating_supply,
        maximum_supply: U256::from(100i32),
        foundry_metadata: None,
    };

    let create_transaction = wallet.create_native_token(params, None).await.unwrap();

    wallet
        .reissue_transaction_until_included(&create_transaction.transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();

    let search = balance
        .native_tokens()
        .iter()
        .find(|token| token.token_id() == &create_transaction.token_id && token.available() == circulating_supply);
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_some());

    // Melt some of the circulating supply
    let melt_amount = U256::from(40i32);
    let transaction = wallet
        .melt_native_token(create_transaction.token_id, melt_amount, None)
        .await
        .unwrap();

    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());

    let search = balance.native_tokens().iter().find(|token| {
        (token.token_id() == &create_transaction.token_id) && (token.available() == circulating_supply - melt_amount)
    });
    assert!(search.is_some());

    // Then melt the rest of the supply
    let melt_amount = circulating_supply - melt_amount;
    let transaction = wallet
        .melt_native_token(create_transaction.token_id, melt_amount, None)
        .await
        .unwrap();

    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());

    let search = balance
        .native_tokens()
        .iter()
        .find(|token| token.token_id() == &create_transaction.token_id);
    assert!(search.is_none());

    // Call to run tests in sequence
    destroy_foundry(&wallet).await?;
    destroy_account(&wallet).await?;

    tear_down(storage_path)
}

async fn destroy_foundry(wallet: &Wallet) -> Result<()> {
    let balance = wallet.sync(None).await?;
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());

    // Let's burn the first foundry we can find, although we may not find the required account output so maybe not a
    // good idea
    let foundry_id = *balance.foundries().first().unwrap();

    let transaction = wallet.burn(foundry_id, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    let search = balance
        .foundries()
        .iter()
        .find(|&balance_foundry_id| *balance_foundry_id == foundry_id);
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_none());

    Ok(())
}

async fn destroy_account(wallet: &Wallet) -> Result<()> {
    let balance = wallet.sync(None).await.unwrap();
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());

    // Let's destroy the first account we can find
    let account_id = *balance.accounts().first().unwrap();
    println!("account_id -> {account_id}");
    let transaction = wallet.burn(account_id, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    let search = balance
        .accounts()
        .iter()
        .find(|&balance_account_id| *balance_account_id == account_id);
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_none());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn create_and_burn_native_tokens() -> Result<()> {
    let storage_path = "test-storage/create_and_burn_native_tokens";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;

    request_funds(&wallet).await?;

    let native_token_amount = U256::from(100);

    let tx = wallet.create_account_output(None, None).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let create_tx = wallet
        .create_native_token(
            CreateNativeTokenParams {
                account_id: None,
                circulating_supply: native_token_amount,
                maximum_supply: native_token_amount,
                foundry_metadata: None,
            },
            None,
        )
        .await?;
    wallet
        .reissue_transaction_until_included(&create_tx.transaction.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let tx = wallet
        .burn(NativeToken::new(create_tx.token_id, native_token_amount)?, None)
        .await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;

    assert!(balance.native_tokens().is_empty());

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn mint_and_burn_nft_with_account() -> Result<()> {
    let storage_path = "test-storage/mint_and_burn_nft_with_account";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;

    let tx = wallet.create_account_output(None, None).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let nft_options = [MintNftParams::new()
        .with_metadata(b"some nft metadata".to_vec())
        .with_immutable_metadata(b"some immutable nft metadata".to_vec())];
    let nft_tx = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&nft_tx.transaction_id, None, None)
        .await?;
    let output_id = OutputId::new(nft_tx.transaction_id, 0u16).unwrap();
    let nft_id = NftId::from(&output_id);

    let balance = wallet.sync(None).await?;
    let account_id = balance.accounts().first().unwrap();

    let burn_tx = wallet
        .burn(Burn::new().add_nft(nft_id).add_account(*account_id), None)
        .await?;
    wallet
        .reissue_transaction_until_included(&burn_tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;

    assert!(balance.accounts().is_empty());
    assert!(balance.nfts().is_empty());

    tear_down(storage_path)
}
