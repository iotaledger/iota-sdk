// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    wallet::{account::SyncOptions, CreateNativeTokenParams, Result},
    U256,
};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn create_and_mint_native_token() -> Result<()> {
    let storage_path = "test-storage/create_and_mint_native_token";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account = &create_accounts_with_funds(&wallet, 1).await?[0];
    
    tear_down(storage_path)
}
