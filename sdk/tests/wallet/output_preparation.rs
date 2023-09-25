// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    types::block::{
        address::{Address, Bech32Address, ToBech32Ext},
        output::{MinimumStorageDepositBasicOutput, NativeToken, NftId, Output, Rent, TokenId},
        slot::SlotIndex,
    },
    wallet::{
        account::{Assets, Features, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
        MintNftParams, Result,
    },
};

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn output_preparation() -> Result<()> {
    let storage_path = "test-storage/output_preparation";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;

    let recipient_address_bech32 = String::from("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu");
    // Roundtrip to get the correct bech32 HRP
    let recipient_address =
        Address::try_from_bech32(&recipient_address_bech32)?.to_bech32(wallet.client().get_bech32_hrp().await?);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 46800);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 46300);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 500000);
    // only address condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);

    let native_token = NativeToken::new(
        TokenId::from_str("0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000")?,
        10,
    )?;
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: Some(Assets {
                    native_tokens: Some(vec![native_token]),
                    nft_id: None,
                }),
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 500000);
    // only address condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);
    assert_eq!(output.native_tokens().unwrap().first(), Some(&native_token));

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 300000,
                assets: None,
                features: Some(Features {
                    metadata: Some(prefix_hex::encode(b"Hello world")),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 300000);
    // only address condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    // only send 1 with metadata feature
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 1,
                assets: None,
                features: Some(Features {
                    metadata: Some(prefix_hex::encode(b"Hello world")),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 49000);
    let unlock_conditions = output.unlock_conditions().unwrap();
    // address + sdr
    assert_eq!(unlock_conditions.len(), 2);
    let storage_deposit_return = unlock_conditions.storage_deposit_return().unwrap();
    // output amount -1
    assert_eq!(storage_deposit_return.amount(), 48999);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 12000,
                assets: None,
                features: Some(Features {
                    metadata: Some(prefix_hex::encode(b"Hello world")),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 54600);
    // address and storage deposit unlock condition, because of the metadata feature block, 12000 is not enough for the
    // required storage deposit
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 1,
                assets: None,
                features: Some(Features {
                    metadata: Some(prefix_hex::encode(b"Hello world")),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 49000);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 48999);

    // address and storage deposit unlock condition, because of the metadata feature block, 213000 is not enough for the
    // required storage deposit
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    // Error if this NftId is not in the account
    let error = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(NftId::from_str(
                        "0xa068e00a79922eaef241592a7440f131ea7f8ad9e22e580ef139415f273eff30",
                    )?),
                }),
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await
        .unwrap_err();
    match error {
        iota_sdk::wallet::Error::NftNotFoundInUnspentOutputs => {}
        _ => panic!("should return NftNotFoundInUnspentOutputs error"),
    }

    if let Ok(output) = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(NftId::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )?),
                }),
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await
    {
        assert_eq!(output.kind(), iota_sdk::types::block::output::NftOutput::KIND);
        assert_eq!(output.amount(), 500000);
        // only address condition
        assert_eq!(output.unlock_conditions().unwrap().len(), 1);
    }

    let issuer_and_sender_address_bech32 =
        Bech32Address::try_from_str("rms1qq724zgvdujt3jdcd3xzsuqq7wl9pwq3dvsa5zvx49rj9tme8cat6qptyfm")?;
    // Roundtrip to get the correct bech32 HRP
    let issuer_and_sender_address = issuer_and_sender_address_bech32
        .inner()
        .to_bech32(wallet.client().get_bech32_hrp().await?);
    let expected_address = issuer_and_sender_address.inner();

    // sender address present when building basic output
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: Some(Assets {
                    native_tokens: Some(vec![native_token]),
                    nft_id: None,
                }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: None,
                    sender: Some(issuer_and_sender_address),
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;

    assert_eq!(output.kind(), iota_sdk::types::block::output::BasicOutput::KIND);
    assert_eq!(output.amount(), 500000);
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);
    let features = output.features().unwrap();
    assert_eq!(features.len(), 1);
    assert_eq!(features.sender().unwrap().address(), expected_address);

    // error when adding issuer when building basic output
    let error = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: None,
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: Some(issuer_and_sender_address),
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await
        .unwrap_err();
    match error {
        iota_sdk::wallet::Error::MissingParameter(_) => {}
        _ => panic!("should return MissingParameter error"),
    }

    // issuer and sender address present when building nft output
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500000,
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(NftId::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )?),
                }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: Some(issuer_and_sender_address),
                    sender: Some(issuer_and_sender_address),
                }),
                unlocks: Some(Unlocks {
                    expiration_slot_index: Some(SlotIndex::from(1)),
                    timelock_slot_index: Some(SlotIndex::from(1)),
                }),
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.kind(), iota_sdk::types::block::output::NftOutput::KIND);
    assert_eq!(output.amount(), 500000);
    let features = output.features().unwrap();
    // sender feature
    assert_eq!(features.len(), 1);
    let immutable_features = output.immutable_features().unwrap();
    // issuer feature
    assert_eq!(immutable_features.len(), 1);
    let issuer_feature = immutable_features.issuer().unwrap();
    assert_eq!(issuer_feature.address(), issuer_and_sender_address.inner());
    let sender_feature = features.sender().unwrap();
    assert_eq!(sender_feature.address(), issuer_and_sender_address.inner());
    // Unlocks
    let conditions = output.unlock_conditions().unwrap();
    assert!(conditions.is_time_locked(0));
    assert!(conditions.is_expired(2));

    // nft with expiration
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 500,
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(NftId::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )?),
                }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: None,
                    sender: None,
                }),
                unlocks: Some(Unlocks {
                    expiration_slot_index: Some(SlotIndex::from(1)),
                    timelock_slot_index: None,
                }),
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.kind(), iota_sdk::types::block::output::NftOutput::KIND);
    assert_eq!(output.amount(), 53900);
    // address, sdr, expiration
    assert_eq!(output.unlock_conditions().unwrap().len(), 3);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 42600,
                assets: None,
                features: Some(Features {
                    metadata: Some(prefix_hex::encode(b"Large metadata".repeat(100))),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Return),
                    use_excess_if_low: None,
                }),
            },
            None,
        )
        .await?;
    let rent_structure = wallet.client().get_rent_structure().await?;
    let minimum_storage_deposit = output.rent_cost(rent_structure);
    assert_eq!(output.amount(), minimum_storage_deposit);
    assert_eq!(output.amount(), 187900);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 145300);
    // address and storage deposit unlock condition, because of the metadata feature block, 42600 is not enough for the
    // required storage deposit
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn output_preparation_sdr() -> Result<()> {
    let storage_path = "test-storage/output_preparation_sdr";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;

    let rent_structure = wallet.client().get_rent_structure().await?;
    let token_supply = wallet.client().get_token_supply().await?;

    let recipient_address_bech32 = String::from("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu");
    // Roundtrip to get the correct bech32 HRP
    let recipient_address =
        Address::try_from_bech32(&recipient_address_bech32)?.to_bech32(wallet.client().get_bech32_hrp().await?);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 8001,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    assert_eq!(output.amount(), 50601);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 42600);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 42599,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    assert_eq!(output.amount(), 85199);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 42600);

    // ReturnStrategy::Return provided
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 42599,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Return),
                    use_excess_if_low: None,
                }),
            },
            None,
        )
        .await?;
    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    assert_eq!(output.amount(), 85199);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 42600);

    // ReturnStrategy::Gift provided
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address,
                amount: 42599,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Gift),
                    use_excess_if_low: None,
                }),
            },
            None,
        )
        .await?;
    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    // The additional 1 amount will be added, because the storage deposit should be gifted and not returned
    assert_eq!(output.amount(), 42600);
    // storage deposit gifted, only address unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn prepare_nft_output_features_update() -> Result<()> {
    let storage_path = "test-storage/prepare_nft_output_features_update";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;
    let wallet_address = wallet.address_as_bech32().await;

    let nft_options = [MintNftParams::new()
        .with_address(wallet_address)
        .with_sender(wallet_address)
        .with_metadata(b"some nft metadata".to_vec())
        .with_tag(b"some nft tag".to_vec())
        .with_issuer(wallet_address)
        .with_immutable_metadata(b"some immutable nft metadata".to_vec())];

    let transaction = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let nft_id = *wallet.sync(None).await?.nfts().first().unwrap();

    let nft = wallet
        .prepare_output(
            OutputParams {
                recipient_address: wallet_address,
                amount: 1_000_000,
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(nft_id),
                }),
                features: Some(Features {
                    metadata: Some("0x2a".to_string()),
                    tag: None,
                    issuer: None,
                    sender: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?
        .as_nft()
        .clone();

    assert_eq!(nft.amount(), 1_000_000);
    assert_eq!(nft.address(), &wallet.address().await);
    assert!(nft.features().sender().is_none());
    assert!(nft.features().tag().is_none());
    assert_eq!(nft.features().metadata().unwrap().data(), &[42]);
    assert_eq!(
        nft.immutable_features().metadata().unwrap().data(),
        b"some immutable nft metadata"
    );
    assert_eq!(
        nft.immutable_features().issuer().unwrap().address(),
        &wallet.address().await
    );

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn prepare_output_remainder_dust() -> Result<()> {
    let storage_path_0 = "test-storage/prepare_output_remainder_dust_0";
    let storage_path_1 = "test-storage/prepare_output_remainder_dust_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None, None).await?;
    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    let rent_structure = wallet_0.client().get_rent_structure().await?;
    let token_supply = wallet_0.client().get_token_supply().await?;

    let balance = wallet_0.sync(None).await?;
    let minimum_required_storage_deposit =
        MinimumStorageDepositBasicOutput::new(rent_structure, token_supply).finish()?;

    // Send away most balance so we can test with leaving dust
    let output = wallet_0
        .prepare_output(
            OutputParams {
                recipient_address: wallet_1.address_as_bech32().await,
                amount: balance.base_coin().available() - 63900,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    let transaction = wallet_0.send_outputs(vec![output], None).await?;
    wallet_0
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = wallet_0.sync(None).await?;

    // 63900 left
    let output = wallet_0
        .prepare_output(
            OutputParams {
                recipient_address: wallet_1.address_as_bech32().await,
                amount: minimum_required_storage_deposit - 1, // Leave less than min. deposit
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Gift),
                    use_excess_if_low: Some(true),
                }),
            },
            None,
        )
        .await?;

    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    // The left over 21299 is too small to keep, so we donate it
    assert_eq!(output.amount(), balance.base_coin().available());
    // storage deposit gifted, only address unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);

    let result = wallet_0
        .prepare_output(
            OutputParams {
                recipient_address: wallet_1.address_as_bech32().await,
                amount: minimum_required_storage_deposit - 1, // Leave less than min. deposit
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Return),
                    use_excess_if_low: Some(true),
                }),
            },
            None,
        )
        .await;
    assert!(
        matches!(result, Err(iota_sdk::wallet::Error::InsufficientFunds{available, required}) if available == balance.base_coin().available() && required == 85199)
    );

    let output = wallet_0
        .prepare_output(
            OutputParams {
                recipient_address: wallet_1.address_as_bech32().await,
                amount: 100, // leave more behind than min. deposit
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Gift),
                    use_excess_if_low: Some(true),
                }),
            },
            None,
        )
        .await?;

    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    // We use excess if leftover is too small, so amount == all available balance
    assert_eq!(output.amount(), 63900);
    // storage deposit gifted, only address unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);

    let output = wallet_0
        .prepare_output(
            OutputParams {
                recipient_address: wallet_1.address_as_bech32().await,
                amount: 100, // leave more behind than min. deposit
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Return),
                    use_excess_if_low: Some(true),
                }),
            },
            None,
        )
        .await?;

    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(rent_structure, token_supply)?;
    // We use excess if leftover is too small, so amount == all available balance
    assert_eq!(output.amount(), 63900);
    // storage deposit returned, address and SDR unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // We have ReturnStrategy::Return, so leftover amount gets returned
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 63900 - 100);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn prepare_output_only_single_nft() -> Result<()> {
    let storage_path_0 = "test-storage/prepare_output_only_single_nft_0";
    let storage_path_1 = "test-storage/prepare_output_only_single_nft_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None, None).await?;
    request_funds(&wallet_0).await?;

    // Create second wallet without funds, so it only gets the NFT
    let wallet_1 = make_wallet(storage_path_1, None, None, None).await?;

    let wallet_0_address = wallet_0.address_as_bech32().await;
    let wallet_1_address = wallet_1.address_as_bech32().await;

    // Send NFT to second account
    let tx = wallet_0
        .mint_nfts([MintNftParams::new().try_with_address(wallet_1_address)?], None)
        .await?;
    wallet_0
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await?;
    assert_eq!(balance.nfts().len(), 1);

    let nft_data = &wallet_1.unspent_outputs(None).await?[0];
    let nft_id = *balance.nfts().first().unwrap();
    // Send NFT back to first account
    let output = wallet_1
        .prepare_output(
            OutputParams {
                recipient_address: wallet_0_address,
                amount: nft_data.output.amount(),
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(nft_id),
                }),
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    let tx = wallet_1.send_outputs([output], None).await?;
    wallet_1
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    // account_0 now has the NFT
    let balance_0 = wallet_0.sync(None).await?;
    assert_eq!(*balance_0.nfts().first().unwrap(), nft_id);

    // account_1 has no NFT and also no base coin amount
    let balance_1 = wallet_1.sync(None).await?;
    assert!(balance_1.nfts().is_empty());
    assert_eq!(balance_1.base_coin().total(), 0);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn prepare_existing_nft_output_gift() -> Result<()> {
    let storage_path = "test-storage/prepare_existing_nft_output_gift";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;
    request_funds(&wallet).await?;
    let address = wallet.address_as_bech32().await;

    let nft_options = [MintNftParams::new()
        .with_address(address)
        .with_sender(address)
        .with_metadata(b"some nft metadata".to_vec())
        .with_tag(b"some nft tag".to_vec())
        .with_issuer(address)
        .with_immutable_metadata(b"some immutable nft metadata".to_vec())];

    let transaction = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let nft_id = *wallet.sync(None).await?.nfts().first().unwrap();

    let nft = wallet
        .prepare_output(
            OutputParams {
                recipient_address: address,
                amount: 0,
                assets: Some(Assets {
                    native_tokens: None,
                    nft_id: Some(nft_id),
                }),
                features: None,
                unlocks: None,
                storage_deposit: Some(StorageDeposit {
                    return_strategy: Some(ReturnStrategy::Gift),
                    use_excess_if_low: None,
                }),
            },
            None,
        )
        .await?
        .as_nft()
        .clone();

    let rent_structure = wallet.client().get_rent_structure().await?;
    let minimum_storage_deposit = Output::Nft(nft.clone()).rent_cost(rent_structure);
    assert_eq!(nft.amount(), minimum_storage_deposit);

    assert_eq!(nft.amount(), 52300);
    assert_eq!(nft.address(), &wallet.address().await);
    assert!(nft.features().is_empty());
    assert_eq!(
        nft.immutable_features().metadata().unwrap().data(),
        b"some immutable nft metadata"
    );
    assert_eq!(
        nft.immutable_features().issuer().unwrap().address(),
        &wallet.address().await
    );

    tear_down(storage_path)
}
