// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::{ConsolidationParams, SendParams};
use pretty_assertions::assert_eq;

use crate::wallet::common::{make_wallet, request_funds, setup, tear_down};

#[ignore]
#[tokio::test]
async fn consolidation() -> Result<(), Box<dyn std::error::Error>> {
    let storage_path_0 = "test-storage/consolidation_0";
    let storage_path_1 = "test-storage/consolidation_1";
    setup(storage_path_0)?;
    setup(storage_path_1)?;

    let wallet_0 = make_wallet(storage_path_0, None, None).await?;
    let wallet_1 = make_wallet(storage_path_1, None, None).await?;

    request_funds(&wallet_0).await?;
    request_funds(&wallet_1).await?;

    // Send 10 outputs to wallet_1
    let amount = 1_000_000;
    let tx = wallet_0
        .send_with_params(vec![SendParams::new(amount, wallet_1.address().await)?; 10], None)
        .await?;

    wallet_0
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await.unwrap();
    assert_eq!(balance.base_coin().available(), 2009968300);
    assert_eq!(wallet_1.ledger().await.unspent_outputs().len(), 12);

    let tx = wallet_1
        .consolidate_outputs(ConsolidationParams::new().with_force(true))
        .await?;
    wallet_1
        .wait_for_transaction_acceptance(&tx.transaction_id, None, None)
        .await?;

    let balance = wallet_1.sync(None).await.unwrap();
    // Balance still the same
    assert_eq!(balance.base_coin().available(), 2009968300);
    // Account and basic unspent output
    assert_eq!(wallet_1.ledger().await.unspent_outputs().len(), 2);

    tear_down(storage_path_0)?;
    tear_down(storage_path_1)?;

    Ok(())
}
