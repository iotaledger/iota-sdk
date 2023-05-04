// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we check if an output has only an address unlock condition and that the address is from the account.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! ```sh
//! cargo run --all-features --example check_unlock_conditions --release
//! ```

use iota_sdk::{
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, UnlockCondition},
    },
    wallet::{Result, Wallet},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";
// The amount to build the basic output with
const AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    let account_addresses: Vec<Bech32Address> = account
        .addresses()
        .await?
        .into_iter()
        .map(|address| address.address().clone())
        .collect();

    println!("Account addresses: {:#?}", account_addresses);

    let output = BasicOutputBuilder::new_with_amount(AMOUNT)
        .add_unlock_condition(AddressUnlockCondition::new(*account_addresses[0].as_ref()))
        .finish_output(account.client().get_token_supply().await?)?;

    let controlled_by_account = if let [UnlockCondition::Address(address_unlock_condition)] = output
        .unlock_conditions()
        .expect("output needs to have unlock conditions")
        .as_ref()
    {
        // Check that address in the unlock condition belongs to the account
        account_addresses
            .iter()
            .any(|address| address.as_ref() == address_unlock_condition.address())
    } else {
        false
    };

    println!(
        "The output has only an address unlock condition and the address is from the account: {controlled_by_account:?}"
    );

    Ok(())
}
