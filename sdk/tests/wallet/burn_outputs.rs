// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::transaction_builder::Burn,
    types::block::output::{
        feature::MetadataFeature,
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        NativeToken, NftId, NftOutputBuilder, OutputId, UnlockCondition,
    },
    wallet::{CreateNativeTokenParams, MintNftParams, Wallet},
    U256,
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn mint_and_burn_nft() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/mint_and_burn_outputs";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;

    let nft_options = [MintNftParams::new()
        .with_address(wallet.address().await)
        .with_metadata(MetadataFeature::new([("data".to_owned(), b"some nft metadata".to_vec())]).unwrap())
        .with_immutable_metadata(
            MetadataFeature::new([("data".to_owned(), b"some immutable nft metadata".to_vec())]).unwrap(),
        )];

    let transaction = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    let balance = wallet.sync(None).await.unwrap();

    let output_id = OutputId::new(transaction.transaction_id, 0u16);
    let nft_id = NftId::from(&output_id);

    let search = balance.nfts().iter().find(|&balance_nft_id| *balance_nft_id == nft_id);
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_some());

    let transaction = wallet.burn(nft_id, None).await.unwrap();
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    let search = balance.nfts().iter().find(|&balance_nft_id| *balance_nft_id == nft_id);
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_none());

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn mint_and_burn_expired_nft() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/mint_and_burn_expired_nft";
    setup(storage_path)?;

    let wallet_0 = make_wallet(storage_path, None, None).await?;
    let wallet_1 = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet_0).await?;

    let amount = 1_000_000;
    let outputs = [NftOutputBuilder::new_with_amount(amount, NftId::null())
        .with_unlock_conditions([
            UnlockCondition::Address(AddressUnlockCondition::new(wallet_0.address().await)),
            // immediately expired to account_1
            UnlockCondition::Expiration(ExpirationUnlockCondition::new(wallet_1.address().await, 1)?),
        ])
        .finish_output()?];

    let transaction = wallet_0.send_outputs(outputs, None).await?;
    wallet_0
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    let output_id = OutputId::new(transaction.transaction_id, 0u16);
    let nft_id = NftId::from(&output_id);

    wallet_1.sync(None).await?;
    let transaction = wallet_1.burn(nft_id, None).await?;
    wallet_1
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet_1.sync(None).await?;
    // After burning the amount is available on account_1
    assert_eq!(balance.base_coin().available(), amount);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn create_and_melt_native_token() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/create_and_melt_native_token";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;

    // First create an account output, this needs to be done only once, because an account can have many foundry outputs
    let transaction = wallet.create_account_output(None, None).await?;

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
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
        .wait_for_transaction_acceptance(&create_transaction.transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();

    let search = balance
        .native_tokens()
        .get(&create_transaction.token_id)
        .filter(|token| token.available() == circulating_supply);
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());
    assert!(search.is_some());

    // Melt some of the circulating supply
    let melt_amount = U256::from(40i32);
    let transaction = wallet
        .melt_native_token(create_transaction.token_id, melt_amount, None)
        .await
        .unwrap();

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());

    let search = balance
        .native_tokens()
        .get(&create_transaction.token_id)
        .filter(|token| token.available() == circulating_supply - melt_amount);
    assert!(search.is_some());

    // Then melt the rest of the supply
    let melt_amount = circulating_supply - melt_amount;
    let transaction = wallet
        .melt_native_token(create_transaction.token_id, melt_amount, None)
        .await
        .unwrap();

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await.unwrap();
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());

    let search = balance.native_tokens().get(&create_transaction.token_id);
    assert!(search.is_none());

    // Call to run tests in sequence
    destroy_foundry(&wallet).await?;
    destroy_account(&wallet).await?;

    tear_down(storage_path)
}

async fn destroy_foundry(wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    let balance = wallet.sync(None).await?;
    println!("wallet balance -> {}", serde_json::to_string(&balance).unwrap());

    // Let's burn the first foundry we can find, although we may not find the required account output so maybe not a
    // good idea
    let foundry_id = *balance.foundries().first().unwrap();

    let transaction = wallet.burn(foundry_id, None).await.unwrap();
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
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

async fn destroy_account(wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    let balance = wallet.sync(None).await.unwrap();
    println!("account balance -> {}", serde_json::to_string(&balance).unwrap());

    // Let's destroy the first account we can find
    let account_id = *balance.accounts().first().unwrap();
    println!("account_id -> {account_id}");
    let transaction = wallet.burn(account_id, None).await.unwrap();
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
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
async fn create_and_burn_native_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/create_and_burn_native_tokens";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    request_funds(&wallet).await?;

    let native_token_amount = U256::from(100);

    let tx = wallet.create_account_output(None, None).await?;
    wallet
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
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
        .wait_for_transaction_acceptance(&create_tx.transaction.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let tx = wallet
        .burn(NativeToken::new(create_tx.token_id, native_token_amount)?, None)
        .await?;
    wallet
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;

    assert!(balance.native_tokens().is_empty());

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn mint_and_burn_nft_with_account() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/mint_and_burn_nft_with_account";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;

    let tx = wallet.create_account_output(None, None).await?;
    wallet
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;
    wallet.sync(None).await?;

    let nft_options = [MintNftParams::new()
        .with_metadata(MetadataFeature::new([("data".to_owned(), b"some nft metadata".to_vec())]).unwrap())
        .with_immutable_metadata(
            MetadataFeature::new([("data".to_owned(), b"some immutable nft metadata".to_vec())]).unwrap(),
        )];
    let nft_tx = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .wait_for_transaction_acceptance(&nft_tx.transaction_id, None, None)
        .await?;
    let output_id = OutputId::new(nft_tx.transaction_id, 0u16);
    let nft_id = NftId::from(&output_id);

    let balance = wallet.sync(None).await?;
    let account_id = balance.accounts().first().unwrap();

    let burn_tx = wallet
        .burn(Burn::new().add_nft(nft_id).add_account(*account_id), None)
        .await?;
    wallet
        .wait_for_transaction_acceptance(&burn_tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;

    assert!(balance.accounts().is_empty());
    assert!(balance.nfts().is_empty());

    tear_down(storage_path)
}
