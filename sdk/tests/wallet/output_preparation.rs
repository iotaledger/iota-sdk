// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    types::block::{
        address::{Address, Bech32Address, ToBech32Ext},
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeToken, NftId, RentCost, TokenId},
    },
    wallet::{
        account::{Assets, Features, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
        MintNftParams, Result,
    },
};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn output_preparation() -> Result<()> {
    let storage_path = "test-storage/output_preparation";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = &create_accounts_with_funds(&wallet, 1).await?[0];

    let recipient_address_bech32 = String::from("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu");
    // Roundtrip to get the correct bech32 HRP
    let recipient_address =
        Address::try_from_bech32(&recipient_address_bech32)?.to_bech32(account.client().get_bech32_hrp().await?);

    let output = account
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

    let output = account
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
    let output = account
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

    let output = account
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
    let output = account
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

    let output = account
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

    let output = account
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
    let error = account
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

    if let Ok(output) = account
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
        .to_bech32(account.client().get_bech32_hrp().await?);
    let expected_address = issuer_and_sender_address.inner();

    // sender address present when building basic output
    let output = account
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
    let error = account
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
    let output = account
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
                    expiration_unix_time: Some(1),
                    timelock_unix_time: Some(1),
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
    let output = account
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
                    expiration_unix_time: Some(1),
                    timelock_unix_time: None,
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

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn output_preparation_sdr() -> Result<()> {
    let storage_path = "test-storage/output_preparation_sdr";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let account = &create_accounts_with_funds(&wallet, 1).await?[0];

    let rent_structure = account.client().get_rent_structure().await?;
    let token_supply = account.client().get_token_supply().await?;

    let recipient_address_bech32 = String::from("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu");
    // Roundtrip to get the correct bech32 HRP
    let recipient_address =
        Address::try_from_bech32(&recipient_address_bech32)?.to_bech32(account.client().get_bech32_hrp().await?);

    let output = account
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

    let output = account
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
    let output = account
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
    let output = account
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

    let wallet = make_wallet(storage_path, None, None).await?;
    let accounts = &create_accounts_with_funds(&wallet, 1).await?;
    let addresses = accounts[0].addresses().await?;
    let address = addresses[0].address();

    let nft_options = [MintNftParams::new()
        .with_address(*address)
        .with_sender(*address)
        .with_metadata(b"some nft metadata".to_vec())
        .with_tag(b"some nft tag".to_vec())
        .with_issuer(*address)
        .with_immutable_metadata(b"some immutable nft metadata".to_vec())];

    let transaction = accounts[0].mint_nfts(nft_options, None).await.unwrap();
    accounts[0]
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    let nft_id = *accounts[0].sync(None).await?.nfts().first().unwrap();

    let nft = accounts[0]
        .prepare_output(
            OutputParams {
                recipient_address: *address,
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
    assert_eq!(nft.address(), accounts[0].addresses().await?[0].address().as_ref());
    assert!(nft.features().sender().is_none());
    assert!(nft.features().tag().is_none());
    assert_eq!(nft.features().metadata().unwrap().data(), &[42]);
    assert_eq!(
        nft.immutable_features().metadata().unwrap().data(),
        b"some immutable nft metadata"
    );
    assert_eq!(
        nft.immutable_features().issuer().unwrap().address(),
        accounts[0].addresses().await?[0].address().as_ref()
    );

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn prepare_output_remainder_dust() -> Result<()> {
    let storage_path = "test-storage/prepare_output_remainder_dust";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let accounts = &create_accounts_with_funds(&wallet, 2).await?;
    let account = &accounts[0];
    let addresses = &accounts[1].addresses().await?;
    let address = addresses[0].address();

    let rent_structure = account.client().get_rent_structure().await?;
    let token_supply = account.client().get_token_supply().await?;

    let balance = account.sync(None).await?;
    let minimum_required_storage_deposit = BasicOutputBuilder::new_with_amount(0)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .rent_cost(rent_structure);

    // Send away most balance so we can test with leaving dust
    let output = account
        .prepare_output(
            OutputParams {
                recipient_address: *address,
                amount: balance.base_coin().available() - 63900,
                assets: None,
                features: None,
                unlocks: None,
                storage_deposit: None,
            },
            None,
        )
        .await?;
    let transaction = account.send_outputs(vec![output], None).await?;
    account
        .reissue_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;

    // 63900 left
    let output = account
        .prepare_output(
            OutputParams {
                recipient_address: *address,
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

    let result = account
        .prepare_output(
            OutputParams {
                recipient_address: *address,
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

    let output = account
        .prepare_output(
            OutputParams {
                recipient_address: *address,
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

    let output = account
        .prepare_output(
            OutputParams {
                recipient_address: *address,
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

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn prepare_output_only_single_nft() -> Result<()> {
    let storage_path = "test-storage/prepare_output_only_single_nft";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;
    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    // Create second account without funds, so it only gets the NFT
    let account_1 = wallet.create_account().finish().await?;
    let addresses = &account_0.addresses().await?;
    let account_0_address = addresses[0].address();
    let addresses = &account_1.addresses().await?;
    let account_1_address = addresses[0].address();

    // Send NFT to second account
    let tx = account_0
        .mint_nfts([MintNftParams::new().try_with_address(account_1_address)?], None)
        .await?;
    account_0
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await?;
    assert_eq!(balance.nfts().len(), 1);

    let nft_data = &account_1.unspent_outputs(None).await?[0];
    let nft_id = *balance.nfts().first().unwrap();
    // Send NFT back to first account
    let output = account_1
        .prepare_output(
            OutputParams {
                recipient_address: *account_0_address,
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
    let tx = account_1.send_outputs([output], None).await?;
    account_1
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    // account_0 now has the NFT
    let balance_0 = account_0.sync(None).await?;
    assert_eq!(*balance_0.nfts().first().unwrap(), nft_id);

    // account_1 has no NFT and also no base coin amount
    let balance_1 = account_1.sync(None).await?;
    assert!(balance_1.nfts().is_empty());
    assert_eq!(balance_1.base_coin().total(), 0);

    tear_down(storage_path)
}
