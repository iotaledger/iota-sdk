// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::output::{
        feature::SenderFeature,
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        BasicOutputBuilder, UnlockCondition,
    },
    wallet::{account::types::Balance, Result},
};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[test]
fn balance_add_assign() {
    use iota_sdk::U256;

    let mut balance1 = Balance::rand_mock();
    let total1 = balance1.base_coin().total();
    let available1 = balance1.base_coin().available();
    #[cfg(feature = "participation")]
    let voting_power1 = balance1.base_coin().voting_power();

    let sdr_alias1 = balance1.required_storage_deposit().alias();
    let sdr_basic1 = balance1.required_storage_deposit().basic();
    let sdr_foundry1 = balance1.required_storage_deposit().foundry();
    let sdr_nft1 = balance1.required_storage_deposit().nft();

    let native_tokens1 = balance1.native_tokens().clone();
    let num_aliases1 = balance1.aliases().len();
    let num_foundries1 = balance1.foundries().len();
    let num_nfts1 = balance1.nfts().len();

    let balance2 = Balance::rand_mock();
    let total2 = balance2.base_coin().total();
    let available2 = balance2.base_coin().available();
    #[cfg(feature = "participation")]
    let voting_power2 = balance2.base_coin().voting_power();

    let sdr_alias2 = balance2.required_storage_deposit().alias();
    let sdr_basic2 = balance2.required_storage_deposit().basic();
    let sdr_foundry2 = balance2.required_storage_deposit().foundry();
    let sdr_nft2 = balance2.required_storage_deposit().nft();

    let native_tokens2 = balance2.native_tokens().clone();
    let num_aliases2 = balance2.aliases().len();
    let num_foundries2 = balance2.foundries().len();
    let num_nfts2 = balance2.nfts().len();

    balance1 += balance2;

    assert_eq!(balance1.base_coin().total(), total1 + total2);
    assert_eq!(balance1.base_coin().available(), available1 + available2);
    #[cfg(feature = "participation")]
    assert_eq!(balance1.base_coin().voting_power(), voting_power1 + voting_power2);

    assert_eq!(balance1.required_storage_deposit().alias(), sdr_alias1 + sdr_alias2);
    assert_eq!(balance1.required_storage_deposit().basic(), sdr_basic1 + sdr_basic2);
    assert_eq!(
        balance1.required_storage_deposit().foundry(),
        sdr_foundry1 + sdr_foundry2
    );
    assert_eq!(balance1.required_storage_deposit().nft(), sdr_nft1 + sdr_nft2);

    assert_eq!(balance1.aliases().len(), num_aliases1 + num_aliases2);
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
    let storage_path = "test-storage/balance_expiration";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;
    let account_2 = wallet.create_account().finish().await?;

    let seconds_until_expired = 20;
    let token_supply = account_0.client().get_token_supply().await?;
    let outputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        // Send to account 1 with expiration to account 2, both have no amount yet
        .with_unlock_conditions([
            UnlockCondition::Address(AddressUnlockCondition::new(
                *account_1.addresses().await?[0].address().as_ref(),
            )),
            UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                *account_2.addresses().await?[0].address().as_ref(),
                // Current time + 20s
                account_0.client().get_time_checked().await? + seconds_until_expired,
            )?),
        ])
        .with_features([SenderFeature::new(*account_0.addresses().await?[0].address().as_ref())])
        .finish_output(token_supply)?];

    let balance_before_tx = account_0.balance().await?;
    let tx = account_0.send_outputs(outputs, None).await?;
    let balance_after_tx = account_0.balance().await?;
    // Total doesn't change before syncing after tx got confirmed
    assert_eq!(
        balance_before_tx.base_coin().total(),
        balance_after_tx.base_coin().total()
    );
    assert_eq!(balance_after_tx.base_coin().available(), 0);

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    // Account 1 balance before expiration
    let balance = account_1.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Account 2 balance before expiration
    let balance = account_2.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Wait until expired
    tokio::time::sleep(std::time::Duration::from_secs(seconds_until_expired.into())).await;

    // Account 1 balance after expiration
    let balance = account_1.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Account 2 balance after expiration
    let balance = account_2.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.base_coin().total(), 1_000_000);
    assert_eq!(balance.base_coin().available(), 1_000_000);

    // It's possible to send the expired output
    let outputs = [BasicOutputBuilder::new_with_amount(1_000_000)
        // Send to account 1 with expiration to account 2, both have no amount yet
        .with_unlock_conditions([AddressUnlockCondition::new(
            *account_1.addresses().await?[0].address().as_ref(),
        )])
        .finish_output(token_supply)?];
    let _tx = account_2.send_outputs(outputs, None).await?;

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
async fn addresses_balance() -> Result<()> {
    let storage_path = "test-storage/addresses_balance";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;
    let addresses_0 = account_0.addresses_with_unspent_outputs().await?;
    let acc_1_addr = &account_1.generate_ed25519_addresses(1, None).await?[0];

    let balance_0 = account_0
        .addresses_balance(addresses_0.iter().map(|a| a.address()).collect())
        .await?;
    let balance_0_sync = account_0.balance().await?;
    let to_send = balance_0.base_coin().available();

    // Check if 0 has balance and sync() and address_balance() match
    assert!(to_send > 0);
    assert_eq!(balance_0, balance_0_sync);

    // Make sure 1 is empty
    let balance_1 = account_1.sync(None).await?;
    assert_eq!(balance_1.base_coin().available(), 0);

    // Send to 1
    let tx = account_0.send(to_send, acc_1_addr.address(), None).await?;
    // Balance should update without sync
    let balance_0 = account_0
        .addresses_balance(addresses_0.iter().map(|a| a.address()).collect())
        .await?;
    let balance_0_sync = account_0.balance().await?;
    assert_eq!(balance_0.base_coin().available(), 0);
    assert_eq!(balance_0, balance_0_sync);

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    account_1.sync(None).await?;

    // Balance should have transferred entirely
    let balance_1 = account_1.addresses_balance(vec![acc_1_addr.address()]).await?;
    let balance_1_sync = account_1.balance().await?;
    assert!(balance_1.base_coin().available() > 0);
    assert_eq!(balance_1, balance_1_sync);

    // Internal transfer on account 1
    let acc_1_addr_2 = &account_1.generate_ed25519_addresses(1, None).await?[0];

    let tx = account_1.send(to_send / 2, acc_1_addr_2.address(), None).await?;
    account_1
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance_1_sync = account_1.sync(None).await?;

    // Check the new address
    let balance_1 = account_1.addresses_balance(vec![acc_1_addr_2.address()]).await?;
    assert_eq!(to_send / 2, balance_1.base_coin().available());

    // Check old and new together
    let balance_1_total = account_1
        .addresses_balance(vec![acc_1_addr.address(), acc_1_addr_2.address()])
        .await?;
    assert_eq!(balance_1_total, balance_1_sync);

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
#[cfg(feature = "participation")]
async fn balance_voting_power() -> Result<()> {
    let storage_path = "test-storage/balance_voting_power";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account = &create_accounts_with_funds(&wallet, 1).await?[0];

    let faucet_amount = 100_000_000_000;

    let balance = account.balance().await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), faucet_amount);

    let voting_power = 1_000_000;
    // Only use a part as voting power
    let tx = account.increase_voting_power(voting_power).await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), faucet_amount - voting_power);
    let account_voting_power = account.get_voting_power().await?;
    assert_eq!(account_voting_power, voting_power);

    // Increase voting power to total amount
    let tx = account.increase_voting_power(faucet_amount - voting_power).await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), 0);
    let account_voting_power = account.get_voting_power().await?;
    assert_eq!(account_voting_power, faucet_amount);

    tear_down(storage_path)
}
