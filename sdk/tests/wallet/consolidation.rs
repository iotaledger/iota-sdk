// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::{Result, SendAmountParams};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn consolidation() -> Result<()> {
    let storage_path = "test-storage/consolidation";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;

    // Send 10 outputs to account_1
    let amount = 1_000_000;
    let tx = account_0
        .send_amount(
            vec![SendAmountParams::new(account_1.addresses().await?[0].address().to_string(), amount); 10],
            None,
        )
        .await?;

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin().available(), 10 * amount);
    assert_eq!(account_1.unspent_outputs(None).await?.len(), 10);

    let tx = account_1.consolidate_outputs(true, None).await?;
    account_1
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    let balance = account_1.sync(None).await.unwrap();
    // Balance still the same
    assert_eq!(balance.base_coin().available(), 10 * amount);
    // Only one unspent output
    assert_eq!(account_1.unspent_outputs(None).await?.len(), 1);

    tear_down(storage_path)
}
