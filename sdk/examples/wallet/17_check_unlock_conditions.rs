// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we check if an output has only an address unlock condition and that the address is from the account.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_account.rs` example!
//!
//! ```sh
//! cargo run --release --all-features --example check_unlock_conditions
//! ```

use iota_sdk::{
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, UnlockCondition},
    },
    wallet::Result,
    Wallet,
};

// The amount to build the basic output with
const AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH"] {
        if std::env::var(var).is_err() {
            panic!(".env variable '{}' is undefined, see .env.example", var);
        }
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    let account_addresses = account
        .addresses()
        .await?
        .into_iter()
        .map(|a| *a.address())
        .collect::<Vec<Bech32Address>>();

    println!("ADDRESSES:\n{:#?}", account_addresses);

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
