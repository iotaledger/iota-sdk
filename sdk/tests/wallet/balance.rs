// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::{
        address::Bech32Address,
        output::{
            feature::SenderFeature,
            unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
            BasicOutputBuilder, UnlockCondition,
        },
    },
    wallet::{account::types::Balance, Result},
};

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[test]
fn balance_add_assign() {
    use iota_sdk::U256;

    let mut balance1 = Balance::rand_mock();
    let total1 = balance1.base_coin().total();
    let available1 = balance1.base_coin().available();
    #[cfg(feature = "participation")]
    let voting_power1 = balance1.base_coin().voting_power();

    let sdr_account_1 = balance1.required_storage_deposit().account();
    let sdr_basic1 = balance1.required_storage_deposit().basic();
    let sdr_foundry1 = balance1.required_storage_deposit().foundry();
    let sdr_nft1 = balance1.required_storage_deposit().nft();

    let native_tokens1 = balance1.native_tokens().clone();
    let num_accounts_1 = balance1.accounts().len();
    let num_foundries1 = balance1.foundries().len();
    let num_nfts1 = balance1.nfts().len();

    let balance2 = Balance::rand_mock();
    let total2 = balance2.base_coin().total();
    let available2 = balance2.base_coin().available();
    #[cfg(feature = "participation")]
    let voting_power2 = balance2.base_coin().voting_power();

    let sdr_account_2 = balance2.required_storage_deposit().account();
    let sdr_basic2 = balance2.required_storage_deposit().basic();
    let sdr_foundry2 = balance2.required_storage_deposit().foundry();
    let sdr_nft2 = balance2.required_storage_deposit().nft();

    let native_tokens2 = balance2.native_tokens().clone();
    let num_accounts_2 = balance2.accounts().len();
    let num_foundries2 = balance2.foundries().len();
    let num_nfts2 = balance2.nfts().len();

    balance1 += balance2;

    assert_eq!(balance1.base_coin().total(), total1 + total2);
    assert_eq!(balance1.base_coin().available(), available1 + available2);
    #[cfg(feature = "participation")]
    assert_eq!(balance1.base_coin().voting_power(), voting_power1 + voting_power2);

    assert_eq!(
        balance1.required_storage_deposit().account(),
        sdr_account_1 + sdr_account_2
    );
    assert_eq!(balance1.required_storage_deposit().basic(), sdr_basic1 + sdr_basic2);
    assert_eq!(
        balance1.required_storage_deposit().foundry(),
        sdr_foundry1 + sdr_foundry2
    );
    assert_eq!(balance1.required_storage_deposit().nft(), sdr_nft1 + sdr_nft2);

    assert_eq!(balance1.accounts().len(), num_accounts_1 + num_accounts_2);
    assert_eq!(balance1.foundries().len(), num_foundries1 + num_foundries2);
    assert_eq!(balance1.nfts().len(), num_nfts1 + num_nfts2);

    let mut expected = std::collections::HashMap::new();
    for nt in native_tokens1.iter().chain(native_tokens2.iter()) {
        let v = expected
            .entry(nt.token_id())
            .or_insert((U256::default(), U256::default()));
        v.0 += nt.total();
        v.1 += nt.available();
    }

    assert_eq!(balance1.native_tokens().len(), expected.len());
    for nt in balance1.native_tokens().iter() {
        assert_eq!(nt.total(), expected.get(nt.token_id()).unwrap().0);
        assert_eq!(nt.available(), expected.get(nt.token_id()).unwrap().1);
    }
}

#[ignore]
#[tokio::test]
async fn balance_expiration() -> Result<()> {
    let storage_path_0 = "test-storage/balance_expiration_0";
    let storage_path_1 = "test-storage/balance_expiration_1";
    let storage_path_2 = "test-storage/balance_expiration_2";
    setup(storage_path_0)?;
    setup(storage_path_1)?;
    setup(storage_path_2)?;

    let wallet_0 = make_wallet(storage_path_0, None, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None, None).await?;
    let wallet_2 = make_wallet(storage_path_2, None, None, None).await?;

    request_funds(&wallet_0).await?;

    let slots_until_expired = 20;
    let token_supply = wallet_0.client().get_token_supply().await?;
    let outputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        // Send to account 1 with expiration to account 2, both have no amount yet
        .with_unlock_conditions([
            UnlockCondition::Address(AddressUnlockCondition::new(wallet_1.address().await)),
            UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                wallet_2.address().await,
                wallet_0.client().get_slot_index().await? + slots_until_expired,
            )?),
        ])
        .with_features([SenderFeature::new(wallet_0.address().await)])
        .finish_output(token_supply)?];

    let balance_before_tx = wallet_0.balance().await?;
    let tx = wallet_0.send_outputs(outputs, None).await?;
    let balance_after_tx = wallet_0.balance().await?;
    // Total doesn't change before syncing after tx got confirmed
    assert_eq!(
        balance_before_tx.base_coin().total(),
        balance_after_tx.base_coin().total()
    );
    assert_eq!(balance_after_tx.base_coin().available(), 0);

    wallet_0
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    // Wallet 1 balance before expiration
    let balance = wallet_1.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Wallet 2 balance before expiration
    let balance = wallet_2.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Wait until expired
    // TODO wait for slots, not seconds
    tokio::time::sleep(std::time::Duration::from_secs(slots_until_expired)).await;

    // Wallet 1 balance after expiration
    let balance = wallet_1.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Wallet 2 balance after expiration
    let balance = wallet_2.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.base_coin().total(), 1_000_000);
    assert_eq!(balance.base_coin().available(), 1_000_000);

    // It's possible to send the expired output
    let outputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        // Send to wallet 1 with expiration to wallet 2, both have no amount yet
        .with_unlock_conditions([AddressUnlockCondition::new(wallet_1.address().await)])
        .finish_output(token_supply)?];
    let _tx = wallet_2.send_outputs(outputs, None).await?;

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;
    tear_down(storage_path_2)?;
    Ok(())
}

#[ignore]
#[tokio::test]
async fn balance_transfer() -> Result<()> {
    let storage_path_0 = "test-storage/addresses_balance_0";
    let storage_path_1 = "test-storage/addresses_balance_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None, None).await?;

    request_funds(&wallet_0).await?;

    let balance_0 = wallet_0.balance().await?;
    let balance_0_sync = wallet_0.sync(None).await?;
    let to_send = balance_0.base_coin().available();

    // Check if 0 has balance and sync() and address_balance() match
    assert!(to_send > 0);
    assert_eq!(balance_0, balance_0_sync);

    // Make sure 1 is empty
    let balance_1 = wallet_1.sync(None).await?;
    assert_eq!(balance_1.base_coin().available(), 0);

    // Send to 1
    let tx = wallet_0.send(to_send, wallet_1.address().await, None).await?;

    // Balance should update without sync
    let balance_0 = wallet_0.balance().await?;
    let balance_0_sync = wallet_0.sync(None).await?;
    assert_eq!(balance_0.base_coin().available(), 0);
    assert_eq!(balance_0, balance_0_sync);

    wallet_0
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    wallet_1.sync(None).await?;

    // Balance should have transferred entirely
    let balance_1_sync = wallet_1.balance().await?;
    assert!(balance_1.base_coin().available() > 0);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;
    Ok(())
}

#[ignore]
#[tokio::test]
#[cfg(feature = "participation")]
async fn balance_voting_power() -> Result<()> {
    let storage_path = "test-storage/balance_voting_power";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None, None).await?;

    request_funds(&wallet).await?;

    let faucet_amount = 100_000_000_000;

    let balance = wallet.balance().await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), faucet_amount);

    let voting_power = 1_000_000;
    // Only use a part as voting power
    let tx = wallet.increase_voting_power(voting_power).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), faucet_amount - voting_power);
    let account_voting_power = wallet.get_voting_power().await?;
    assert_eq!(account_voting_power, voting_power);

    // Increase voting power to total amount
    let tx = wallet.increase_voting_power(faucet_amount - voting_power).await?;
    wallet
        .reissue_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = wallet.sync(None).await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), 0);
    let account_voting_power = wallet.get_voting_power().await?;
    assert_eq!(account_voting_power, faucet_amount);

    tear_down(storage_path)?;
    Ok(())
}
