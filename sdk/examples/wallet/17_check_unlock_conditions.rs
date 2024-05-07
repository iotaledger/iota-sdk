// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we check if an output has only an address unlock condition and that the address is from the wallet.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! ```sh
//! cargo run --release --all-features --example check_unlock_conditions
//! ```

use iota_sdk::{
    types::block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, UnlockCondition},
    Wallet,
};

// The amount to build the basic output with
const AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    let wallet_address = wallet.address().await;

    println!("Wallet address:\n{:#?}", wallet_address);

    let output = BasicOutputBuilder::new_with_amount(AMOUNT)
        .add_unlock_condition(AddressUnlockCondition::new(wallet_address.clone()))
        .finish_output()?;

    let controlled_by_account =
        if let [UnlockCondition::Address(address_unlock_condition)] = output.unlock_conditions().as_ref() {
            // Check that the address in the unlock condition belongs to the wallet
            wallet_address.inner() == address_unlock_condition.address()
        } else {
            false
        };

    println!(
        "The output has only an address unlock condition and the address is from the account: {controlled_by_account:?}"
    );

    Ok(())
}
