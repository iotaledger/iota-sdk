// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::output::{
        feature::SenderFeature,
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        BasicOutputBuilder, UnlockCondition,
    },
    wallet::{types::Balance, Result},
};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[test]
fn rand_balance_add_assign() {
    use iota_sdk::U256;

    let old_balance = Balance::rand();
    let add_balance = Balance::rand();

    let mut new_balance = old_balance.clone();
    assert_eq!(new_balance, old_balance);

    let rhs_balance = add_balance.clone();
    assert_eq!(rhs_balance, add_balance);

    new_balance += rhs_balance;

    // Base Coin
    assert_eq!(
        new_balance.base_coin().total(),
        old_balance.base_coin().total() + add_balance.base_coin().total()
    );
    assert_eq!(
        new_balance.base_coin().available(),
        old_balance.base_coin().available() + add_balance.base_coin().available()
    );
    #[cfg(feature = "participation")]
    assert_eq!(
        new_balance.base_coin().voting_power(),
        old_balance.base_coin().voting_power() + add_balance.base_coin().voting_power()
    );

    // Required Storage Deposit
    assert_eq!(
        new_balance.required_storage_deposit().basic(),
        old_balance.required_storage_deposit().basic() + add_balance.required_storage_deposit().basic()
    );
    assert_eq!(
        new_balance.required_storage_deposit().account(),
        old_balance.required_storage_deposit().account() + add_balance.required_storage_deposit().account()
    );
    assert_eq!(
        new_balance.required_storage_deposit().foundry(),
        old_balance.required_storage_deposit().foundry() + add_balance.required_storage_deposit().foundry()
    );
    assert_eq!(
        new_balance.required_storage_deposit().nft(),
        old_balance.required_storage_deposit().nft() + add_balance.required_storage_deposit().nft()
    );
    assert_eq!(
        new_balance.required_storage_deposit().delegation(),
        old_balance.required_storage_deposit().delegation() + add_balance.required_storage_deposit().delegation()
    );

    // Assets
    assert_eq!(
        new_balance.accounts(),
        &old_balance
            .accounts()
            .iter()
            .chain(add_balance.accounts().iter())
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        new_balance.foundries(),
        &old_balance
            .foundries()
            .iter()
            .chain(add_balance.foundries().iter())
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        new_balance.nfts(),
        &old_balance
            .nfts()
            .iter()
            .chain(add_balance.nfts().iter())
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        new_balance.delegations(),
        &old_balance
            .delegations()
            .iter()
            .chain(add_balance.delegations().iter())
            .cloned()
            .collect::<Vec<_>>()
    );
    let mut expected_native_tokens = std::collections::HashMap::new();
    for native_token in old_balance
        .native_tokens()
        .iter()
        .chain(add_balance.native_tokens().iter())
    {
        let v = expected_native_tokens
            .entry(native_token.token_id())
            .or_insert((U256::default(), U256::default()));
        v.0 += native_token.total();
        v.1 += native_token.available();
    }
    assert_eq!(new_balance.native_tokens().len(), expected_native_tokens.len());
    for native_token in new_balance.native_tokens().iter() {
        assert_eq!(
            native_token.total(),
            expected_native_tokens.get(native_token.token_id()).unwrap().0
        );
        assert_eq!(
            native_token.available(),
            expected_native_tokens.get(native_token.token_id()).unwrap().1
        );
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

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;
    let wallet_2 = make_wallet(storage_path_2, None, None).await?;

    request_funds(&wallet_0).await?;

    let slots_until_expired = 20;
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
        .finish_output()?];

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
    tokio::time::sleep(std::time::Duration::from_secs(slots_until_expired as u64)).await;

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
        .finish_output()?];
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

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

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

    // Balance should have transferred entirely
    let balance_1_sync = wallet_1.sync(None).await?;
    assert!(balance_1.base_coin().available() > 0);
    assert_eq!(balance_1, balance_1_sync);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;
    Ok(())
}

// #[ignore]
// #[tokio::test]
// #[cfg(feature = "participation")]
// async fn balance_voting_power() -> Result<()> {
//     let storage_path = "test-storage/balance_voting_power";
//     setup(storage_path)?;

//     let wallet = make_wallet(storage_path, None, None).await?;

//     request_funds(&wallet).await?;

//     let faucet_amount = 100_000_000_000;

//     let balance = wallet.balance().await?;
//     assert_eq!(balance.base_coin().total(), faucet_amount);
//     assert_eq!(balance.base_coin().available(), faucet_amount);

//     let voting_power = 1_000_000;
//     // Only use a part as voting power
//     let tx = wallet.increase_voting_power(voting_power).await?;
//     wallet
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await?;
//     let balance = wallet.sync(None).await?;
//     assert_eq!(balance.base_coin().total(), faucet_amount);
//     assert_eq!(balance.base_coin().available(), faucet_amount - voting_power);
//     let wallet_voting_power = wallet.get_voting_power().await?;
//     assert_eq!(wallet_voting_power, voting_power);

//     // Increase voting power to total amount
//     let tx = wallet.increase_voting_power(faucet_amount - voting_power).await?;
//     wallet
//         .reissue_transaction_until_included(&tx.transaction_id, None, None)
//         .await?;
//     let balance = wallet.sync(None).await?;
//     assert_eq!(balance.base_coin().total(), faucet_amount);
//     assert_eq!(balance.base_coin().available(), 0);
//     let wallet_voting_power = wallet.get_voting_power().await?;
//     assert_eq!(wallet_voting_power, faucet_amount);

//     tear_down(storage_path)?;
//     Ok(())
// }
