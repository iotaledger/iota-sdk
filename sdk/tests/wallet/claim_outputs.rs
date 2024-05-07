// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Duration;
use iota_sdk::{
    client::{
        api::{options::TransactionOptions, transaction_builder::TransactionBuilderError},
        ClientError,
    },
    types::block::output::{
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        BasicOutputBuilder, NativeToken, NftId, NftOutputBuilder, UnlockCondition,
    },
    wallet::{CreateNativeTokenParams, OutputsToClaim, SendNativeTokenParams, SendParams, WalletError},
    U256,
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn claim_2_basic_micro_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_2_basic_micro_outputs_0";
    let storage_path_1 = "test-storage/claim_2_basic_micro_outputs_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    let micro_amount = 1;
    let tx = wallet_1
        .send_with_params(
            [
                SendParams::new(micro_amount, wallet_0.address().await)?,
                SendParams::new(micro_amount, wallet_0.address().await)?,
            ],
            TransactionOptions {
                allow_micro_amount: true,
                ..Default::default()
            },
        )
        .await?;

    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with account 0
    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);
    let base_coin_amount_before_claiming = balance.base_coin().available();

    let tx = wallet_0
        .claim_outputs(
            wallet_0.claimable_outputs(OutputsToClaim::MicroTransactions).await?,
            None,
        )
        .await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(
        balance.base_coin().available(),
        base_coin_amount_before_claiming + 2 * micro_amount
    );

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn claim_1_of_2_basic_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_1_of_2_basic_outputs_0";
    let storage_path_1 = "test-storage/claim_1_of_2_basic_outputs_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    let amount = 10;
    let tx = wallet_1
        .send_with_params(
            [
                SendParams::new(amount, wallet_0.address().await)?,
                SendParams::new(0, wallet_0.address().await)?,
            ],
            TransactionOptions {
                allow_micro_amount: true,
                ..Default::default()
            },
        )
        .await?;

    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with account 0
    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);
    let base_coin_amount_before_claiming = balance.base_coin().available();

    let tx = wallet_0
        .claim_outputs(wallet_0.claimable_outputs(OutputsToClaim::Amount).await?, None)
        .await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(
        balance.base_coin().available(),
        base_coin_amount_before_claiming + amount
    );

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn claim_2_basic_outputs_no_available_in_claim_account() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_2_basic_outputs_no_available_in_claim_account_0";
    let storage_path_1 = "test-storage/claim_2_basic_outputs_no_available_in_claim_account_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    // Send all available from wallet 1 away
    let balance = wallet_1.sync(None).await?;
    let tx = wallet_1
        .send_with_params(
            [SendParams::new(
                balance.base_coin().available(),
                wallet_0.address().await,
            )?],
            None,
        )
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let storage_score_params = wallet_0.client().get_storage_score_parameters().await?;
    let slots_in_one_day = wallet_0
        .client()
        .get_protocol_parameters()
        .await?
        .slots_in_duration(Duration::from_secs(86400));

    let expiration_slot = wallet_0.client().get_slot_index().await? + slots_in_one_day;

    let output = BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
        .add_unlock_condition(AddressUnlockCondition::new(wallet_1.address().await))
        .add_unlock_condition(ExpirationUnlockCondition::new(
            wallet_0.address().await,
            expiration_slot,
        )?)
        .finish_output()?;
    let amount = output.amount();

    let outputs = vec![output; 2];

    let tx = wallet_0.send_outputs(outputs, None).await?;

    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with wallet 1
    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);
    let base_coin_amount_before_claiming = balance.base_coin().available();

    let tx = wallet_1
        .claim_outputs(wallet_1.claimable_outputs(OutputsToClaim::All).await?, None)
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(
        balance.base_coin().available(),
        base_coin_amount_before_claiming + 2 * amount
    );

    tear_down(storage_path_0)
}

#[ignore]
#[tokio::test]
async fn claim_2_native_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_2_native_tokens_0";
    let storage_path_1 = "test-storage/claim_2_native_tokens_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    let native_token_amount = U256::from(100);

    let tx = wallet_1.create_account_output(None, None).await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;
    wallet_1.sync(None).await?;

    let create_tx_0 = wallet_1
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
    wallet_1
        .wait_for_transaction_acceptance(&create_tx_0.transaction.transaction_id, None, None)
        .await?;
    wallet_1.sync(None).await?;

    let create_tx_1 = wallet_1
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
    wallet_1
        .wait_for_transaction_acceptance(&create_tx_1.transaction.transaction_id, None, None)
        .await?;
    wallet_1.sync(None).await?;

    let tx = wallet_1
        .send_native_tokens(
            [
                SendNativeTokenParams::new(wallet_0.address().await, (create_tx_0.token_id, native_token_amount))?,
                SendNativeTokenParams::new(wallet_0.address().await, (create_tx_1.token_id, native_token_amount))?,
            ],
            None,
        )
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with account 0
    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);

    let tx = wallet_0
        .claim_outputs(wallet_0.claimable_outputs(OutputsToClaim::NativeTokens).await?, None)
        .await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.native_tokens().len(), 2);
    let native_token_0 = balance.native_tokens().get(&create_tx_0.token_id).unwrap();
    assert_eq!(native_token_0.total(), native_token_amount);
    let native_token_1 = balance.native_tokens().get(&create_tx_1.token_id).unwrap();
    assert_eq!(native_token_1.total(), native_token_amount);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn claim_2_native_tokens_no_available_balance_in_claim_account() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_2_native_tokens_no_available_balance_in_claim_account_0";
    let storage_path_1 = "test-storage/claim_2_native_tokens_no_available_balance_in_claim_account_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    // Send all available from wallet 1 away
    let balance = wallet_1.sync(None).await?;
    let tx = wallet_1
        .send_with_params(
            [SendParams::new(
                balance.base_coin().available(),
                wallet_0.address().await,
            )?],
            None,
        )
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let native_token_amount = U256::from(100);

    let tx = wallet_0.create_account_output(None, None).await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;
    wallet_0.sync(None).await?;

    let create_tx_0 = wallet_0
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
    wallet_0
        .wait_for_transaction_acceptance(&create_tx_0.transaction.transaction_id, None, None)
        .await?;
    wallet_0.sync(None).await?;

    let create_tx_1 = wallet_0
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
    wallet_0
        .wait_for_transaction_acceptance(&create_tx_1.transaction.transaction_id, None, None)
        .await?;
    wallet_0.sync(None).await?;

    let storage_score_params = wallet_0.client().get_storage_score_parameters().await?;

    let tx = wallet_0
        .send_outputs(
            [
                BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
                    .add_unlock_condition(AddressUnlockCondition::new(wallet_1.address().await))
                    .add_unlock_condition(ExpirationUnlockCondition::new(
                        wallet_0.address().await,
                        wallet_0.client().get_slot_index().await? + 5000,
                    )?)
                    .with_native_token(NativeToken::new(create_tx_0.token_id, native_token_amount)?)
                    .finish_output()?,
                BasicOutputBuilder::new_with_minimum_amount(storage_score_params)
                    .add_unlock_condition(AddressUnlockCondition::new(wallet_1.address().await))
                    .add_unlock_condition(ExpirationUnlockCondition::new(
                        wallet_0.address().await,
                        wallet_0.client().get_slot_index().await? + 5000,
                    )?)
                    .with_native_token(NativeToken::new(create_tx_1.token_id, native_token_amount)?)
                    .finish_output()?,
            ],
            None,
        )
        .await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with account 1
    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);

    let tx = wallet_1
        .claim_outputs(wallet_1.claimable_outputs(OutputsToClaim::NativeTokens).await?, None)
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.native_tokens().len(), 2);
    let native_token_0 = balance.native_tokens().get(&create_tx_0.token_id).unwrap();
    assert_eq!(native_token_0.total(), native_token_amount);
    let native_token_1 = balance.native_tokens().get(&create_tx_1.token_id).unwrap();
    assert_eq!(native_token_1.total(), native_token_amount);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn claim_2_nft_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_2_nft_outputs_0";
    let storage_path_1 = "test-storage/claim_2_nft_outputs_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    let outputs = [
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(wallet_0.address().await)),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    wallet_1.address().await,
                    wallet_1.client().get_slot_index().await? + 5000,
                )?),
            ])
            .finish_output()?,
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(wallet_0.address().await)),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    wallet_1.address().await,
                    wallet_1.client().get_slot_index().await? + 5000,
                )?),
            ])
            .finish_output()?,
    ];

    let tx = wallet_1.send_outputs(outputs, None).await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with account 0
    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);

    let tx = wallet_0
        .claim_outputs(wallet_0.claimable_outputs(OutputsToClaim::Nfts).await?, None)
        .await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_0.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.nfts().len(), 2);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn claim_2_nft_outputs_no_available_in_claim_account() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_2_nft_outputs_no_available_in_claim_account_0";
    let storage_path_1 = "test-storage/claim_2_nft_outputs_no_available_in_claim_account_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    // Send all available from wallet 1 away
    let balance = wallet_1.sync(None).await?;
    let tx = wallet_1
        .send_with_params(
            [SendParams::new(
                balance.base_coin().available(),
                wallet_0.address().await,
            )?],
            None,
        )
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let outputs = [
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(wallet_1.address().await)),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    wallet_0.address().await,
                    wallet_0.client().get_slot_index().await? + 5000,
                )?),
            ])
            .finish_output()?,
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .with_unlock_conditions([
                UnlockCondition::Address(AddressUnlockCondition::new(wallet_1.address().await)),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    wallet_0.address().await,
                    wallet_0.client().get_slot_index().await? + 5000,
                )?),
            ])
            .finish_output()?,
    ];

    let tx = wallet_0.send_outputs(outputs, None).await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Claim with wallet 1
    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 2);

    let tx = wallet_1
        .claim_outputs(wallet_1.claimable_outputs(OutputsToClaim::Nfts).await?, None)
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.nfts().len(), 2);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn claim_basic_micro_output_error() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/claim_basic_micro_output_error_0";
    let storage_path_1 = "test-storage/claim_basic_micro_output_error_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    // Send all available from wallet 1 away
    let balance = wallet_1.sync(None).await?;
    let tx = wallet_1
        .send_with_params(
            [SendParams::new(
                balance.base_coin().available(),
                wallet_0.address().await,
            )?],
            None,
        )
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let micro_amount = 1;
    let tx = wallet_0
        .send_with_params(
            [SendParams::new(micro_amount, wallet_1.address().await)?],
            TransactionOptions {
                allow_micro_amount: true,
                ..Default::default()
            },
        )
        .await?;

    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // Try claim with account 1 will fail since it has no funds to cover the storage deposit
    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.potentially_locked_outputs().len(), 1);

    let result = wallet_1
        .claim_outputs(
            wallet_1.claimable_outputs(OutputsToClaim::MicroTransactions).await?,
            None,
        )
        .await;
    assert!(matches!(
        result,
        Err(WalletError::Client(ClientError::TransactionBuilder(
            TransactionBuilderError::InsufficientAmount { .. }
        )))
    ));

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}
