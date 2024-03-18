// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    types::block::{
        address::{Address, Bech32Address, ToBech32Ext},
        output::{feature::MetadataFeature, BasicOutput, MinimumOutputAmount, NativeToken, NftId, TokenId},
        protocol::CommittableAgeRange,
        slot::SlotIndex,
    },
    wallet::{Assets, Features, MintNftParams, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn output_preparation() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/output_preparation";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;

    let recipient_address_bech32 = String::from("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu");
    // Roundtrip to get the correct bech32 HRP
    let recipient_address =
        Address::try_from_bech32(&recipient_address_bech32)?.to_bech32(wallet.client().get_bech32_hrp().await?);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 18300);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 17800);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
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
                recipient_address: recipient_address.clone(),
                amount: 500000,
                assets: Some(Assets { nft_id: None }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: None,
                    sender: None,
                    native_token: Some(native_token),
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 500000);
    // only address condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);
    assert_eq!(output.native_token(), Some(&native_token));

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 300000,
                assets: None,
                features: Some(Features {
                    metadata: Some(MetadataFeature::new([("data".to_owned(), b"Hello world".to_vec())]).unwrap()),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                    native_token: None,
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
                recipient_address: recipient_address.clone(),
                amount: 1,
                assets: None,
                features: Some(Features {
                    metadata: Some(MetadataFeature::new([("data".to_owned(), b"Hello world".to_vec())]).unwrap()),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                    native_token: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    let min_amount_with_metadata_and_tag = 21100;
    assert_eq!(output.amount(), min_amount_with_metadata_and_tag);
    let unlock_conditions = output.unlock_conditions().unwrap();
    // address + sdr
    assert_eq!(unlock_conditions.len(), 2);
    let storage_deposit_return = unlock_conditions.storage_deposit_return().unwrap();
    assert_eq!(storage_deposit_return.amount(), min_amount_with_metadata_and_tag - 1);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 12000,
                assets: None,
                features: Some(Features {
                    metadata: Some(MetadataFeature::new([("data".to_owned(), b"Hello world".to_vec())]).unwrap()),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                    native_token: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), 26100);
    // address and storage deposit unlock condition, because of the metadata feature block, 12000 is not enough for the
    // required storage deposit
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 1,
                assets: None,
                features: Some(Features {
                    metadata: Some(MetadataFeature::new([("data".to_owned(), b"Hello world".to_vec())]).unwrap()),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                    native_token: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    assert_eq!(output.amount(), min_amount_with_metadata_and_tag);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), min_amount_with_metadata_and_tag - 1);

    // address and storage deposit unlock condition, because of the metadata feature block, 213000 is not enough for the
    // required storage deposit
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    // Error if this NftId is not in the wallet
    let error = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500000,
                assets: Some(Assets {
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
        iota_sdk::wallet::WalletError::NftNotFoundInUnspentOutputs => {}
        _ => panic!("should return NftNotFoundInUnspentOutputs error"),
    }

    if let Ok(output) = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500000,
                assets: Some(Assets {
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
    let issuer_and_sender_address = issuer_and_sender_address_bech32.to_bech32(wallet.client().get_bech32_hrp().await?);
    let expected_address = issuer_and_sender_address.inner();

    // sender address present when building basic output
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500000,
                assets: Some(Assets { nft_id: None }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: None,
                    sender: Some(issuer_and_sender_address.clone()),
                    native_token: Some(native_token),
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;

    assert_eq!(output.kind(), BasicOutput::KIND);
    assert_eq!(output.amount(), 500000);
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);
    let features = output.features().unwrap();
    assert_eq!(features.len(), 2);
    assert_eq!(features.sender().unwrap().address(), expected_address);

    // error when adding issuer when building basic output
    let error = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500000,
                assets: None,
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: Some(issuer_and_sender_address.clone()),
                    sender: None,
                    native_token: None,
                }),
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await
        .unwrap_err();
    match error {
        iota_sdk::wallet::WalletError::MissingParameter(_) => {}
        _ => panic!("should return MissingParameter error"),
    }

    // issuer and sender address present when building nft output
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500000,
                assets: Some(Assets {
                    nft_id: Some(NftId::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )?),
                }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: Some(issuer_and_sender_address.clone()),
                    sender: Some(issuer_and_sender_address.clone()),
                    native_token: None,
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
    assert!(conditions.is_timelocked(0, 0));
    assert_eq!(
        conditions.is_expired(2, CommittableAgeRange { min: 0, max: 0 }),
        Some(true)
    );

    // nft with expiration
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 500,
                assets: Some(Assets {
                    nft_id: Some(NftId::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )?),
                }),
                features: Some(Features {
                    metadata: None,
                    tag: None,
                    issuer: None,
                    sender: None,
                    native_token: None,
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
    assert_eq!(output.amount(), 25400);
    // address, sdr, expiration
    assert_eq!(output.unlock_conditions().unwrap().len(), 3);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 42600,
                assets: None,
                features: Some(Features {
                    metadata: Some(
                        MetadataFeature::new([("data".to_owned(), b"Large metadata".repeat(100).to_vec())]).unwrap(),
                    ),
                    tag: Some(prefix_hex::encode(b"My Tag")),
                    issuer: None,
                    sender: None,
                    native_token: None,
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
    let storage_score_params = wallet.client().get_storage_score_parameters().await?;
    let minimum_amount = output.minimum_amount(storage_score_params);
    assert_eq!(output.amount(), minimum_amount);
    assert_eq!(output.amount(), 160000);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 117400);
    // address and storage deposit unlock condition, because of the metadata feature block, 42600 is not enough for the
    // required storage deposit
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    // metadata and tag features
    assert_eq!(output.features().unwrap().len(), 2);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn output_preparation_sdr() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/output_preparation_sdr";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;

    let storage_score_params = wallet.client().get_storage_score_parameters().await?;

    let recipient_address_bech32 = String::from("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu");
    // Roundtrip to get the correct bech32 HRP
    let recipient_address =
        Address::try_from_bech32(&recipient_address_bech32)?.to_bech32(wallet.client().get_bech32_hrp().await?);

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: 4001,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(storage_score_params)?;
    assert_eq!(output.amount(), 18300);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), 14299);

    let min_amount = 14100;

    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: min_amount - 1,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    // Check if the output has enough amount to cover the storage deposit
    output.verify_storage_deposit(storage_score_params)?;
    assert_eq!(output.amount(), (min_amount * 2) - 1);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), min_amount);

    // ReturnStrategy::Return provided
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: min_amount - 1,
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
    output.verify_storage_deposit(storage_score_params)?;
    assert_eq!(output.amount(), (min_amount * 2) - 1);
    // address and sdr unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 2);
    let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
    assert_eq!(sdr.amount(), min_amount);

    // ReturnStrategy::Gift provided
    let output = wallet
        .prepare_output(
            OutputParams {
                recipient_address: recipient_address.clone(),
                amount: min_amount - 1,
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
    output.verify_storage_deposit(storage_score_params)?;
    // The additional 1 amount will be added, because the storage deposit should be gifted and not returned
    assert_eq!(output.amount(), min_amount);
    // storage deposit gifted, only address unlock condition
    assert_eq!(output.unlock_conditions().unwrap().len(), 1);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn prepare_nft_output_features_update() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/prepare_nft_output_features_update";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;
    let wallet_address = wallet.address().await;

    let nft_options = [MintNftParams::new()
        .with_address(wallet_address.clone())
        .with_sender(wallet_address.clone())
        .with_metadata(MetadataFeature::new([("data".to_owned(), vec![42])])?)
        .with_tag(b"some nft tag".to_vec())
        .with_issuer(wallet_address.clone())
        .with_immutable_metadata(MetadataFeature::new([("data".to_owned(), vec![42])])?)];

    let transaction = wallet.mint_nfts(nft_options, None).await.unwrap();
    assert_eq!(
        transaction.payload.transaction().outputs()[0]
            .as_nft()
            .immutable_features()
            .metadata()
            .unwrap()
            .get("data")
            .unwrap(),
        vec![42]
    );
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    let nft_id = *wallet.sync(None).await?.nfts().first().unwrap();

    let nft = wallet
        .prepare_output(
            OutputParams {
                recipient_address: wallet_address,
                amount: 1_000_000,
                assets: Some(Assets { nft_id: Some(nft_id) }),
                features: Some(Features {
                    metadata: Some(MetadataFeature::new([("data".to_owned(), b"0x2a".to_vec())]).unwrap()),
                    tag: None,
                    issuer: None,
                    sender: None,
                    native_token: None,
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
    assert_eq!(nft.address(), wallet.address().await.inner());
    assert!(nft.features().sender().is_none());
    assert!(nft.features().tag().is_none());
    assert_eq!(nft.features().metadata().unwrap().get("data").unwrap(), b"0x2a");
    assert_eq!(
        nft.immutable_features().issuer().unwrap().address(),
        wallet.address().await.inner()
    );

    tear_down(storage_path)
}

// TODO: adjust amounts
// #[ignore]
// #[tokio::test]
// async fn prepare_output_remainder_dust() -> Result<(), Box<dyn std::error::Error>> {
//     let storage_path_0 = "test-storage/prepare_output_remainder_dust_0";
//     let storage_path_1 = "test-storage/prepare_output_remainder_dust_1";
//     setup(storage_path_0)?;
//     setup(storage_path_1)?;

//     let wallet_0 = make_wallet(storage_path_0, None, None).await?;
//     let wallet_1 = make_wallet(storage_path_1, None, None).await?;
//     request_funds(&wallet_0).await?;
//     request_funds(&wallet_1).await?;

//     let storage_score_params = wallet_0.client().get_storage_score_parameters().await?;

//     let balance = wallet_0.sync(None).await?;
//     let minimum_amount = BasicOutput::minimum_amount(&*wallet_1.address().await, storage_score_params);

//     // Send away most balance so we can test with leaving dust
//     let output = wallet_0
//         .prepare_output(
//             OutputParams {
//                 recipient_address: wallet_1.address().await,
//                 amount: balance.base_coin().available() - 63900,
//                 assets: None,
//                 features: None,
//                 unlocks: None,
//                 storage_deposit: None,
//             },
//             None,
//         )
//         .await?;
//     let transaction = wallet_0.send_outputs(vec![output], None).await?;
//     wallet_0
//         .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
//         .await?;
//     let balance = wallet_0.sync(None).await?;

//     // 63900 left
//     let output = wallet_0
//         .prepare_output(
//             OutputParams {
//                 recipient_address: wallet_1.address().await,
//                 amount: minimum_amount - 1, // Leave less than min. deposit
//                 assets: None,
//                 features: None,
//                 unlocks: None,
//                 storage_deposit: Some(StorageDeposit {
//                     return_strategy: Some(ReturnStrategy::Gift),
//                     use_excess_if_low: Some(true),
//                 }),
//             },
//             None,
//         )
//         .await?;

//     // Check if the output has enough amount to cover the storage deposit
//     output.verify_storage_deposit(storage_score_params)?;
//     // The left over 21299 is too small to keep, so we donate it
//     assert_eq!(output.amount(), balance.base_coin().available());
//     // storage deposit gifted, only address unlock condition
//     assert_eq!(output.unlock_conditions().unwrap().len(), 1);

//     let result = wallet_0
//         .prepare_output(
//             OutputParams {
//                 recipient_address: wallet_1.address().await,
//                 amount: minimum_amount - 1, // Leave less than min. deposit
//                 assets: None,
//                 features: None,
//                 unlocks: None,
//                 storage_deposit: Some(StorageDeposit {
//                     return_strategy: Some(ReturnStrategy::Return),
//                     use_excess_if_low: Some(true),
//                 }),
//             },
//             None,
//         )
//         .await;
//     assert!(
//         matches!(result, Err(iota_sdk::wallet::WalletError::InsufficientFunds{available, required}) if available ==
// balance.base_coin().available() && required == 42599)     );

//     let output = wallet_0
//         .prepare_output(
//             OutputParams {
//                 recipient_address: wallet_1.address().await,
//                 amount: 100, // leave more behind than min. deposit
//                 assets: None,
//                 features: None,
//                 unlocks: None,
//                 storage_deposit: Some(StorageDeposit {
//                     return_strategy: Some(ReturnStrategy::Gift),
//                     use_excess_if_low: Some(true),
//                 }),
//             },
//             None,
//         )
//         .await?;

//     // Check if the output has enough amount to cover the storage deposit
//     output.verify_storage_deposit(storage_score_params)?;
//     // We use excess if leftover is too small, so amount == all available balance
//     assert_eq!(output.amount(), 63900);
//     // storage deposit gifted, only address unlock condition
//     assert_eq!(output.unlock_conditions().unwrap().len(), 1);

//     let output = wallet_0
//         .prepare_output(
//             OutputParams {
//                 recipient_address: wallet_1.address().await,
//                 amount: 100, // leave more behind than min. deposit
//                 assets: None,
//                 features: None,
//                 unlocks: None,
//                 storage_deposit: Some(StorageDeposit {
//                     return_strategy: Some(ReturnStrategy::Return),
//                     use_excess_if_low: Some(true),
//                 }),
//             },
//             None,
//         )
//         .await?;

//     // Check if the output has enough amount to cover the storage deposit
//     output.verify_storage_deposit(storage_score_params)?;
//     // We use excess if leftover is too small, so amount == all available balance
//     assert_eq!(output.amount(), 63900);
//     // storage deposit returned, address and SDR unlock condition
//     assert_eq!(output.unlock_conditions().unwrap().len(), 2);
//     // We have ReturnStrategy::Return, so leftover amount gets returned
//     let sdr = output.unlock_conditions().unwrap().storage_deposit_return().unwrap();
//     assert_eq!(sdr.amount(), 63900 - 100);

//     tear_down(storage_path_0)?;
//     tear_down(storage_path_1)?;

//     Ok(())
// }

#[ignore]
#[tokio::test]
async fn prepare_output_only_single_nft() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/prepare_output_only_single_nft_0";
    let storage_path_1 = "test-storage/prepare_output_only_single_nft_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    request_funds(&wallet_0).await?;

    // Create second wallet, so it only gets the NFT
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;
    request_funds(&wallet_1).await?;

    let wallet_0_address = wallet_0.address().await;
    let wallet_1_address = wallet_1.address().await;

    // Send NFT to second wallet
    let tx = wallet_0
        .mint_nfts([MintNftParams::new().try_with_address(wallet_1_address)?], None)
        .await?;
    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await?;
    assert_eq!(balance.nfts().len(), 1);

    let nft_amount = wallet_1
        .ledger()
        .await
        .unspent_outputs()
        .values()
        .next()
        .unwrap()
        .output
        .amount();
    let nft_id = *balance.nfts().first().unwrap();
    // Send NFT back to first wallet
    let output = wallet_1
        .prepare_output(
            OutputParams {
                recipient_address: wallet_0_address,
                amount: nft_amount,
                assets: Some(Assets { nft_id: Some(nft_id) }),
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    let tx = wallet_1.send_outputs([output], None).await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    // wallet_0 now has the NFT
    let balance_0 = wallet_0.sync(None).await?;
    assert_eq!(*balance_0.nfts().first().unwrap(), nft_id);

    // wallet_1 has no NFT
    let balance_1 = wallet_1.sync(None).await?;
    assert!(balance_1.nfts().is_empty());

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn prepare_existing_nft_output_gift() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "test-storage/prepare_existing_nft_output_gift";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    request_funds(&wallet).await?;
    let address = wallet.address().await;

    let nft_options = [MintNftParams::new()
        .with_address(address.clone())
        .with_sender(address.clone())
        .with_metadata(MetadataFeature::new([("42".to_owned(), vec![42])])?)
        .with_tag(b"some nft tag".to_vec())
        .with_issuer(address.clone())
        .with_immutable_metadata(MetadataFeature::new([("43".to_owned(), vec![43])])?)];

    let transaction = wallet.mint_nfts(nft_options, None).await.unwrap();
    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    let nft_id = *wallet.sync(None).await?.nfts().first().unwrap();

    let nft = wallet
        .prepare_output(
            OutputParams {
                recipient_address: address,
                amount: 0,
                assets: Some(Assets { nft_id: Some(nft_id) }),
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

    let storage_score_params = wallet.client().get_storage_score_parameters().await?;
    let minimum_storage_deposit = nft.minimum_amount(storage_score_params);
    assert_eq!(nft.amount(), minimum_storage_deposit);

    assert_eq!(nft.amount(), 21600);
    assert_eq!(nft.address(), wallet.address().await.inner());
    assert!(nft.features().is_empty());
    assert_eq!(
        nft.immutable_features()
            .metadata()
            .unwrap()
            .first_key_value()
            .unwrap()
            .1
            .to_vec(),
        [43]
    );
    assert_eq!(
        nft.immutable_features().issuer().unwrap().address(),
        wallet.address().await.inner()
    );

    tear_down(storage_path)
}
